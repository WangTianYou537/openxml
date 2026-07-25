//! SpreadsheetDocument — Excel (`.xlsx`) package.

use super::open_xml_package::{OpenSettings, OpenXmlPackage};
use crate::element::{parse_element, write_element, OpenXmlElement};
use crate::error::{Error, Result};
use crate::namespace::{content_type, rel};
use crate::opc::{
    CustomProperties, ExtendedProperties, OpcPackage, PackageMode, PackageProperties, PackUri,
    RelationshipTargetMode,
};
use crate::spreadsheet::{
    area_chart_space, auto_filter, bar_chart_space, bubble_chart_space, calc_chain, calc_chain_cell,
    cell_formula, cell_inline_str, cell_number, cell_number_styled, cell_ref_to_row_col,
    cell_shared_string, cf_rule_cell_is,
    cf_rule_color_scale, cf_rule_data_bar, cf_rule_icon_set, column, column_with_hidden, columns,
    comments_for_author,
    conditional_formatting, data_validation_list, data_validation_whole, data_validations,
    doughnut_chart_space, dxf_fill, dxfs, freeze_panes_views, line_chart_space, merge_cells,
    minimal_stylesheet, one_cell_anchor_picture, page_margins, page_setup, pie_chart_space,
    pivot_cache_definition, pivot_cache_records, pivot_cache_records_from_rows,
    pivot_table_definition, radar_chart_space, row, scatter_chart_space, sheet, sheet_data,
    sheet_protection, sheets, sparkline, sparkline_ext, sparkline_group, sparkline_groups, stylesheet_with_border,
    stylesheet_with_fill, stylesheet_with_named_styles, stylesheet_with_num_fmt, table_definition,
    two_cell_anchor_chart,
    vml_comments_drawing, workbook, workbook_pivot_cache, workbook_pivot_caches,
    workbook_protection, worksheet, worksheet_drawing, worksheet_drawing_ref,
    SharedStringTableBuilder,
};
use std::path::Path;

/// Type of SpreadsheetML package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpreadsheetDocumentType {
    #[default]
    Workbook,
    Template,
    MacroEnabledWorkbook,
    MacroEnabledTemplate,
}

impl SpreadsheetDocumentType {
    pub fn content_type(self) -> &'static str {
        match self {
            Self::Workbook => content_type::SPREADSHEET_SHEET,
            Self::Template => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml"
            }
            Self::MacroEnabledWorkbook => "application/vnd.ms-excel.sheet.macroEnabled.main+xml",
            Self::MacroEnabledTemplate => {
                "application/vnd.ms-excel.template.macroEnabled.main+xml"
            }
        }
    }

    pub fn from_content_type(ct: &str) -> Option<Self> {
        match ct {
            content_type::SPREADSHEET_SHEET => Some(Self::Workbook),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml" => {
                Some(Self::Template)
            }
            "application/vnd.ms-excel.sheet.macroEnabled.main+xml" => {
                Some(Self::MacroEnabledWorkbook)
            }
            "application/vnd.ms-excel.template.macroEnabled.main+xml" => {
                Some(Self::MacroEnabledTemplate)
            }
            _ => None,
        }
    }
}

const WORKBOOK_URI: &str = "/xl/workbook.xml";
const WORKSHEET1_URI: &str = "/xl/worksheets/sheet1.xml";
const SHARED_STRINGS_URI: &str = "/xl/sharedStrings.xml";

/// Descriptor for a worksheet in the workbook.
#[derive(Debug, Clone)]
pub struct WorksheetInfo {
    pub name: String,
    pub sheet_id: u32,
    pub relationship_id: String,
    pub uri: PackUri,
}

/// An Open XML spreadsheet document (`.xlsx`).
#[derive(Debug)]
pub struct SpreadsheetDocument {
    package: OpenXmlPackage,
    document_type: SpreadsheetDocumentType,
    /// Tracked sheets when we create/manage them (name → info).
    sheets: Vec<WorksheetInfo>,
    /// Optional shared string table builder (created on demand).
    sst: Option<SharedStringTableBuilder>,
    next_sheet_index: u32,
}

impl SpreadsheetDocument {
    /// Create a new spreadsheet at `path`.
    pub fn create(
        path: impl AsRef<Path>,
        document_type: SpreadsheetDocumentType,
    ) -> Result<Self> {
        Self::create_with_settings(path, document_type, OpenSettings::default())
    }

    /// Create a new spreadsheet at `path` with custom open settings.
    pub fn create_with_settings(
        path: impl AsRef<Path>,
        document_type: SpreadsheetDocumentType,
        settings: OpenSettings,
    ) -> Result<Self> {
        let opc = OpcPackage::create_file(path.as_ref());
        let mut package = OpenXmlPackage::from_opc(opc, settings);
        package.set_application_type(crate::features::ApplicationType::EXCEL);
        package.set_package_factory_feature("SpreadsheetDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "SpreadsheetDocument",
        ));
        Ok(Self {
            package,
            document_type,
            sheets: Vec::new(),
            sst: None,
            next_sheet_index: 1,
        })
    }

    /// Create an in-memory spreadsheet.
    pub fn create_in_memory(document_type: SpreadsheetDocumentType) -> Result<Self> {
        let opc = OpcPackage::create();
        let mut package = OpenXmlPackage::from_opc(opc, OpenSettings::default());
        package.set_application_type(crate::features::ApplicationType::EXCEL);
        package.set_package_factory_feature("SpreadsheetDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "SpreadsheetDocument",
        ));
        Ok(Self {
            package,
            document_type,
            sheets: Vec::new(),
            sst: None,
            next_sheet_index: 1,
        })
    }

    /// Quick-create a workbook at `path` with a single sheet of string data.
    ///
    /// Sheet name defaults to `"Sheet1"`. For in-memory use, prefer
    /// [`create_in_memory`](Self::create_in_memory) + [`write_sheet_strings`](Self::write_sheet_strings).
    pub fn create_simple(
        path: impl AsRef<Path>,
        sheet_name: &str,
        rows: &[Vec<&str>],
    ) -> Result<Self> {
        let mut wb = Self::create(path, SpreadsheetDocumentType::Workbook)?;
        let name = if sheet_name.is_empty() {
            "Sheet1"
        } else {
            sheet_name
        };
        wb.write_sheet_strings(name, rows)?;
        Ok(wb)
    }

    /// Open an existing spreadsheet.
    pub fn open(path: impl AsRef<Path>, is_editable: bool) -> Result<Self> {
        Self::open_with_settings(path, is_editable, OpenSettings::default())
    }

    /// Open an existing spreadsheet with custom open settings.
    pub fn open_with_settings(
        path: impl AsRef<Path>,
        is_editable: bool,
        mut settings: OpenSettings,
    ) -> Result<Self> {
        if !is_editable {
            settings.auto_save = false;
        }
        let opc = OpcPackage::open(path)?;
        Self::from_opc(opc, settings)
    }

    /// Open from raw package bytes.
    pub fn open_bytes(data: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = data.as_ref().to_vec();
        let opc = OpcPackage::open_bytes(&bytes)?;
        let mut settings = OpenSettings::default();
        settings.auto_save = false;
        let mut doc = Self::from_opc(opc, settings)?;
        doc.package_mut().set_package_stream_bytes(bytes);
        Ok(doc)
    }

    /// Open a Spreadsheet package from any `Read + Seek` stream (C# `Open(Stream, …)`).
    pub fn open_stream<R: std::io::Read + std::io::Seek>(
        reader: R,
        is_editable: bool,
    ) -> Result<Self> {
        Self::open_stream_with_settings(reader, is_editable, OpenSettings::default())
    }

    /// Open from a stream with custom [`OpenSettings`].
    pub fn open_stream_with_settings<R: std::io::Read + std::io::Seek>(
        reader: R,
        is_editable: bool,
        mut settings: OpenSettings,
    ) -> Result<Self> {
        if !is_editable {
            settings.auto_save = false;
        }
        let opc = OpcPackage::open_reader(reader)?;
        Self::from_opc(opc, settings)
    }

    /// Write the package ZIP to a stream (C# stream save).
    pub fn write_to<W: std::io::Write>(&mut self, writer: W) -> Result<()> {
        self.flush_shared_strings()?;
        self.package.write_to(writer)
    }


    fn from_opc(opc: OpcPackage, settings: OpenSettings) -> Result<Self> {
        let mut package = OpenXmlPackage::from_opc(opc, settings);
        package.set_application_type(crate::features::ApplicationType::EXCEL);
        package.set_package_factory_feature("SpreadsheetDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "SpreadsheetDocument",
        ));
        let document_type = package
            .opc()
            .main_part_uri(rel::OFFICE_DOCUMENT)
            .ok()
            .and_then(|uri| {
                package
                    .opc()
                    .content_types()
                    .content_type_for(uri.as_str())
                    .and_then(SpreadsheetDocumentType::from_content_type)
            })
            .unwrap_or_default();
        if let Ok(uri) = package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ct = package
                .opc()
                .content_types()
                .content_type_for(uri.as_str())
                .unwrap_or("")
                .to_string();
            package.set_main_part_feature(crate::features::MainPartFeature::new(
                rel::OFFICE_DOCUMENT,
                ct,
                Some(uri.as_str().to_string()),
            ));
        }

        let mut doc = Self {
            package,
            document_type,
            sheets: Vec::new(),
            sst: None,
            next_sheet_index: 1,
        };
        doc.reload_sheet_index()?;
        doc.reload_shared_strings()?;
        Ok(doc)
    }

    fn reload_sheet_index(&mut self) -> Result<()> {
        self.sheets.clear();
        let wb_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(()),
        };
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(());
        };
        let root = parse_element(data)?;
        let Some(sheets_el) = root.child("sheets") else {
            return Ok(());
        };

        for sheet_el in sheets_el.children_by_name("sheet") {
            let name = sheet_el
                .get_attribute("name")
                .unwrap_or("Sheet")
                .to_string();
            let sheet_id = sheet_el
                .get_attribute("sheetId")
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);
            let rid = sheet_el
                .get_attribute_qname("r:id")
                .or_else(|| sheet_el.get_attribute("id"))
                .unwrap_or("")
                .to_string();
            if rid.is_empty() {
                continue;
            }
            let uri = if let Some(rel) = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .and_then(|rels| rels.get(&rid))
            {
                self.package
                    .opc()
                    .resolve_relationship(Some(&wb_uri), rel)?
            } else {
                continue;
            };
            if sheet_id >= self.next_sheet_index {
                self.next_sheet_index = sheet_id + 1;
            }
            self.sheets.push(WorksheetInfo {
                name,
                sheet_id,
                relationship_id: rid,
                uri,
            });
        }
        Ok(())
    }

    fn reload_shared_strings(&mut self) -> Result<()> {
        let uri = PackUri::new(SHARED_STRINGS_URI);
        let Some(data) = self.package.opc().get_part(&uri) else {
            self.sst = None;
            return Ok(());
        };
        let root = parse_element(data)?;
        self.sst = Some(SharedStringTableBuilder::from_element(&root));
        Ok(())
    }

    pub fn document_type(&self) -> SpreadsheetDocumentType {
        self.document_type
    }

    pub fn package(&self) -> &OpenXmlPackage {
        &self.package
    }

    pub fn package_mut(&mut self) -> &mut OpenXmlPackage {
        &mut self.package
    }


    /// Alias for [`open_bytes`](Self::open_bytes).
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Result<Self> {
        Self::open_bytes(data)
    }

    /// Sheets currently known in the workbook.
    pub fn worksheets(&self) -> &[WorksheetInfo] {
        &self.sheets
    }

    /// Ensure the workbook part exists (package-level relationship).
    fn ensure_workbook(&mut self) -> Result<PackUri> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        if self.package.opc().has_part(&wb_uri) {
            return Ok(wb_uri);
        }
        self.package.set_part(
            wb_uri.clone(),
            self.document_type.content_type(),
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><x:workbook xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"><x:sheets/></x:workbook>".to_vec(),
        );
        self.package.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &wb_uri,
            RelationshipTargetMode::Internal,
        );
        Ok(wb_uri)
    }

    /// Rewrite workbook.xml sheet list while preserving other children (e.g. definedNames).
    fn rewrite_workbook(&mut self) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let sheet_els: Vec<_> = self
            .sheets
            .iter()
            .map(|s| sheet(&s.name, s.sheet_id, &s.relationship_id))
            .collect();

        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data).unwrap_or_else(|_| workbook(Vec::<crate::element::OpenXmlElement>::new()))
        } else {
            workbook(Vec::<crate::element::OpenXmlElement>::new())
        };

        // Replace or insert sheets
        if let Some(pos) = root.children.iter().position(|c| c.local_name == "sheets") {
            root.children[pos] = sheets(sheet_els);
        } else {
            // Insert sheets after bookViews if present, else at start
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "bookViews")
                .map(|i| i + 1)
                .unwrap_or(0);
            root.children.insert(insert_at, sheets(sheet_els));
        }

        let wb_xml = write_element(&root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            wb_xml,
        );
        Ok(())
    }

    /// Set workbook defined names. Each entry is `(name, refers_to)` e.g. `("Sales", "Sheet1!$A$1:$B$10")`.
    pub fn set_defined_names(&mut self, names: &[(&str, &str)]) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            workbook(Vec::<crate::element::OpenXmlElement>::new())
        };
        root.children.retain(|c| c.local_name != "definedNames");
        if !names.is_empty() {
            let kids: Vec<_> = names
                .iter()
                .map(|(n, r)| crate::spreadsheet::defined_name(n, r))
                .collect();
            // Place definedNames after sheets (Excel convention)
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheets")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, crate::spreadsheet::defined_names(kids));
        }
        let xml = write_element(&root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            xml,
        );
        Ok(())
    }

    /// Read workbook defined names as `(name, refers_to)`.
    pub fn defined_names(&self) -> Result<Vec<(String, String)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("definedNames") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("definedName")
            .filter_map(|el| {
                let name = el.get_attribute("name")?.to_string();
                let refers = el.inner_text();
                Some((name, refers))
            })
            .collect())
    }

    /// Add or replace a single defined name.
    pub fn add_defined_name(&mut self, name: &str, refers_to: &str) -> Result<()> {
        let mut names = self.defined_names()?;
        if let Some(existing) = names.iter_mut().find(|(n, _)| n == name) {
            existing.1 = refers_to.to_string();
        } else {
            names.push((name.to_string(), refers_to.to_string()));
        }
        let refs: Vec<(&str, &str)> = names
            .iter()
            .map(|(n, r)| (n.as_str(), r.as_str()))
            .collect();
        self.set_defined_names(&refs)
    }

    /// Look up a defined name's `refers_to` value.
    pub fn get_defined_name(&self, name: &str) -> Result<Option<String>> {
        Ok(self
            .defined_names()?
            .into_iter()
            .find(|(n, _)| n == name)
            .map(|(_, r)| r))
    }

    /// Number of defined names in the workbook.
    pub fn defined_name_count(&self) -> Result<usize> {
        Ok(self.defined_names()?.len())
    }

    /// Whether a workbook-level defined name exists.
    pub fn has_defined_name(&self, name: &str) -> Result<bool> {
        Ok(self.defined_names()?.iter().any(|(n, _)| n == name))
    }


    /// Alias for [`defined_names`](Self::defined_names).
    pub fn list_defined_names(&self) -> Result<Vec<(String, String)>> {
        self.defined_names()
    }

    /// Remove all defined names.
    pub fn clear_defined_names(&mut self) -> Result<usize> {
        let n = self.defined_name_count()?;
        if n == 0 {
            return Ok(0);
        }
        self.set_defined_names(&[])?;
        Ok(n)
    }


    /// Remove all table definition parts and related worksheet relationships.
    pub fn clear_tables(&mut self) -> Result<usize> {
        let infos = self.table_infos()?;
        let n = infos.len();
        for (name, _, _) in infos {
            if !name.is_empty() {
                let _ = self.remove_table(&name)?;
            }
        }
        // also drop any leftover table parts
        for uri in self.list_tables() {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Whether any defined names exist.
    pub fn has_defined_names(&self) -> Result<bool> {
        Ok(self.defined_name_count()? > 0)
    }

    /// Remove a defined name by name. Returns whether it was present.
    pub fn remove_defined_name(&mut self, name: &str) -> Result<bool> {
        let names = self.defined_names()?;
        let before = names.len();
        let filtered: Vec<(&str, &str)> = names
            .iter()
            .filter(|(n, _)| n != name)
            .map(|(n, r)| (n.as_str(), r.as_str()))
            .collect();
        let removed = filtered.len() < before;
        self.set_defined_names(&filtered)?;
        Ok(removed)
    }

    /// Set attributes on a defined name (`hidden`, `comment`, optional new `refers_to` text).
    pub fn set_defined_name_attrs(
        &mut self,
        name: &str,
        hidden: Option<bool>,
        comment: Option<&str>,
        refers_to: Option<&str>,
    ) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(container) = root.child_mut("definedNames") else {
            return Ok(false);
        };
        let mut found = false;
        for dn in container
            .children
            .iter_mut()
            .filter(|c| c.local_name == "definedName")
        {
            if dn.get_attribute("name").unwrap_or("") != name {
                continue;
            }
            found = true;
            if let Some(h) = hidden {
                dn.set_attribute("hidden", if h { "1" } else { "0" });
            }
            if let Some(c) = comment {
                dn.set_attribute("comment", c);
            }
            if let Some(r) = refers_to {
                dn.set_text(r);
                dn.children.clear();
            }
            break;
        }
        if found {
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        }
        Ok(found)
    }

    /// Read defined name attributes as `(refers_to, hidden, comment?, local_sheet_id?)`.
    pub fn defined_name_details(
        &self,
        name: &str,
    ) -> Result<Option<(String, bool, Option<String>, Option<u32>)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("definedNames") else {
            return Ok(None);
        };
        for dn in container.children_by_name("definedName") {
            if dn.get_attribute("name").unwrap_or("") != name {
                continue;
            }
            let hidden = dn
                .get_attribute("hidden")
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            let comment = dn.get_attribute("comment").map(|s| s.to_string());
            let local = dn
                .get_attribute("localSheetId")
                .and_then(|s| s.parse().ok());
            return Ok(Some((dn.inner_text(), hidden, comment, local)));
        }
        Ok(None)
    }

    /// 0-based index of a sheet by name.
    pub fn sheet_index(&self, name: &str) -> Option<usize> {
        self.sheets.iter().position(|s| s.name == name)
    }

    /// Set an auto-filter on a worksheet range (e.g. `"A1:C10"`).
    pub fn set_auto_filter(&mut self, sheet_name: &str, reference: &str) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "autoFilter");
        // autoFilter typically follows sheetData
        let insert_at = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
            .map(|i| i + 1)
            .unwrap_or(root.children.len());
        root.children
            .insert(insert_at, auto_filter(reference));
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read the autoFilter `ref` on a sheet, if present.
    pub fn auto_filter_ref(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("autoFilter")
            .and_then(|af| af.get_attribute("ref").map(|s| s.to_string())))
    }

    /// Whether an autoFilter is present on the sheet.
    pub fn has_auto_filter(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.auto_filter_ref(sheet_name)?.is_some())
    }

    /// Sheet names that have an autoFilter.
    pub fn sheets_with_auto_filter(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_auto_filter(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has an autoFilter.
    pub fn has_sheets_with_auto_filter(&self) -> Result<bool> {
        Ok(!self.sheets_with_auto_filter()?.is_empty())
    }

    /// Remove the autoFilter from a sheet.
    pub fn clear_auto_filter(&mut self, sheet_name: &str) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "autoFilter");
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Clear auto-filters on every sheet. Returns sheets modified.
    pub fn clear_all_auto_filters(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.has_auto_filter(&name)? {
                self.clear_auto_filter(&name)?;
                n += 1;
            }
        }
        Ok(n)
    }

    /// Add a value filter on an autoFilter column (`filterColumn` with `filters/filter`).
    ///
    /// `col_id` is 0-based column index within the filter range.
    /// Creates `autoFilter` with `ref` if missing (caller should set range first ideally).
    pub fn add_auto_filter_values(
        &mut self,
        sheet_name: &str,
        col_id: u32,
        values: &[&str],
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("autoFilter").is_none() {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "autoFilter").with_attribute("ref", "A1"),
            );
        }
        let af = root.child_mut("autoFilter").unwrap();
        // remove existing filter for this col
        af.children.retain(|c| {
            !(c.local_name == "filterColumn"
                && c.get_attribute("colId") == Some(&col_id.to_string()))
        });
        let mut filters = OpenXmlElement::new("x", x, "filters");
        for v in values {
            filters.append_child(
                OpenXmlElement::new("x", x, "filter").with_attribute("val", *v),
            );
        }
        af.append_child(
            OpenXmlElement::new("x", x, "filterColumn")
                .with_attribute("colId", col_id.to_string())
                .with_child(filters),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a top-10 filter on an autoFilter column.
    pub fn add_auto_filter_top10(
        &mut self,
        sheet_name: &str,
        col_id: u32,
        top: bool,
        percent: bool,
        val: f64,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("autoFilter").is_none() {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "autoFilter").with_attribute("ref", "A1"),
            );
        }
        let af = root.child_mut("autoFilter").unwrap();
        af.children.retain(|c| {
            !(c.local_name == "filterColumn"
                && c.get_attribute("colId") == Some(&col_id.to_string()))
        });
        let top10 = OpenXmlElement::new("x", x, "top10")
            .with_attribute("top", if top { "1" } else { "0" })
            .with_attribute("percent", if percent { "1" } else { "0" })
            .with_attribute("val", val.to_string());
        af.append_child(
            OpenXmlElement::new("x", x, "filterColumn")
                .with_attribute("colId", col_id.to_string())
                .with_child(top10),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List filter column ids under autoFilter.
    pub fn list_auto_filter_columns(&self, sheet_name: &str) -> Result<Vec<u32>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(af) = root.child("autoFilter") else {
            return Ok(Vec::new());
        };
        Ok(af
            .children_by_name("filterColumn")
            .filter_map(|c| c.get_attribute("colId").and_then(|s| s.parse().ok()))
            .collect())
    }

    /// Number of filter columns configured on a sheet.
    pub fn auto_filter_column_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_auto_filter_columns(sheet_name)?.len())
    }


    /// Whether any filter columns are configured on a sheet.
    pub fn has_auto_filter_columns(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.auto_filter_column_count(sheet_name)? > 0)
    }

    /// Clear filter columns but keep autoFilter ref. Returns how many columns cleared.
    pub fn clear_auto_filter_columns(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(af) = root.child_mut("autoFilter") else {
            return Ok(0);
        };
        let before = af.children.len();
        af.children.retain(|c| c.local_name != "filterColumn");
        let n = before - af.children.len();
        if n > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    fn ensure_auto_filter_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("autoFilter").is_none() {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "autoFilter").with_attribute("ref", "A1"),
            );
        }
        root.child_mut("autoFilter").expect("autoFilter")
    }

    /// Add custom filters on an autoFilter column (`customFilters` with one or two conditions).
    ///
    /// `filters` are `(operator, value)` pairs, e.g. `("greaterThan", "50")`.
    /// When two filters are provided, `and` controls AND vs OR combination.
    pub fn add_auto_filter_custom(
        &mut self,
        sheet_name: &str,
        col_id: u32,
        filters: &[(&str, &str)],
        and: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let af = self.ensure_auto_filter_mut(&mut root);
        af.children.retain(|c| {
            !(c.local_name == "filterColumn"
                && c.get_attribute("colId") == Some(&col_id.to_string()))
        });
        let mut custom = OpenXmlElement::new("x", x, "customFilters")
            .with_attribute("and", if and { "1" } else { "0" });
        for (op, val) in filters {
            custom.append_child(
                OpenXmlElement::new("x", x, "customFilter")
                    .with_attribute("operator", *op)
                    .with_attribute("val", *val),
            );
        }
        af.append_child(
            OpenXmlElement::new("x", x, "filterColumn")
                .with_attribute("colId", col_id.to_string())
                .with_child(custom),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a dynamic filter on an autoFilter column (e.g. type `"today"`, `"thisWeek"`, `"aboveAverage"`).
    pub fn add_auto_filter_dynamic(
        &mut self,
        sheet_name: &str,
        col_id: u32,
        type_: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let af = self.ensure_auto_filter_mut(&mut root);
        af.children.retain(|c| {
            !(c.local_name == "filterColumn"
                && c.get_attribute("colId") == Some(&col_id.to_string()))
        });
        let dynf = OpenXmlElement::new("x", x, "dynamicFilter").with_attribute("type", type_);
        af.append_child(
            OpenXmlElement::new("x", x, "filterColumn")
                .with_attribute("colId", col_id.to_string())
                .with_child(dynf),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read filter kind for a column: `"values"`, `"top10"`, `"custom"`, `"dynamic"`, or `None`.
    pub fn auto_filter_column_kind(
        &self,
        sheet_name: &str,
        col_id: u32,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(af) = root.child("autoFilter") else {
            return Ok(None);
        };
        for fc in af.children_by_name("filterColumn") {
            if fc.get_attribute("colId") == Some(&col_id.to_string()) {
                if fc.child("filters").is_some() {
                    return Ok(Some("values".into()));
                }
                if fc.child("top10").is_some() {
                    return Ok(Some("top10".into()));
                }
                if fc.child("customFilters").is_some() {
                    return Ok(Some("custom".into()));
                }
                if fc.child("dynamicFilter").is_some() {
                    return Ok(Some("dynamic".into()));
                }
                if fc.child("colorFilter").is_some() {
                    return Ok(Some("color".into()));
                }
                if fc.child("iconFilter").is_some() {
                    return Ok(Some("icon".into()));
                }
                return Ok(Some("unknown".into()));
            }
        }
        Ok(None)
    }

    /// Add a data reference under dataConsolidate/dataRefs.
    pub fn add_data_consolidate_ref(
        &mut self,
        sheet_name: &str,
        reference: &str,
        sheet: Option<&str>,
        name: Option<&str>,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("dataConsolidate").is_none() {
            root.children.push(
                OpenXmlElement::new("x", x, "dataConsolidate")
                    .with_attribute("function", "sum")
                    .with_child(OpenXmlElement::new("x", x, "dataRefs")),
            );
        }
        let dc = root.child_mut("dataConsolidate").unwrap();
        if dc.child("dataRefs").is_none() {
            dc.append_child(OpenXmlElement::new("x", x, "dataRefs"));
        }
        let refs = dc.child_mut("dataRefs").unwrap();
        let mut r = OpenXmlElement::new("x", x, "dataRef").with_attribute("ref", reference);
        if let Some(s) = sheet {
            r.set_attribute("sheet", s);
        }
        if let Some(n) = name {
            r.set_attribute("name", n);
        }
        refs.append_child(r);
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List data consolidate refs as `(ref, sheet?, name?)`.
    pub fn list_data_consolidate_refs(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, Option<String>, Option<String>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(refs) = root
            .child("dataConsolidate")
            .and_then(|d| d.child("dataRefs"))
        else {
            return Ok(Vec::new());
        };
        Ok(refs
            .children_by_name("dataRef")
            .map(|r| {
                (
                    r.get_attribute("ref").unwrap_or("").to_string(),
                    r.get_attribute("sheet").map(|s| s.to_string()),
                    r.get_attribute("name").map(|s| s.to_string()),
                )
            })
            .collect())
    }

    /// Hide or show a single 1-based row.
    pub fn set_row_hidden(
        &mut self,
        sheet_name: &str,
        row_num: u32,
        hidden: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if root.child("sheetData").is_none() {
            root.append_child(sheet_data(Vec::<crate::element::OpenXmlElement>::new()));
        }
        let sd = root.child_mut("sheetData").unwrap();
        let existing = sd.children.iter_mut().find(|c| {
            c.local_name == "row"
                && c.get_attribute("r").and_then(|s| s.parse::<u32>().ok()) == Some(row_num)
        });
        if let Some(row_el) = existing {
            if hidden {
                row_el.set_attribute("hidden", "1");
            } else {
                row_el.attributes.retain(|a| a.local_name != "hidden");
            }
        } else if hidden {
            let mut r = row(row_num, Vec::<crate::element::OpenXmlElement>::new());
            r.set_attribute("hidden", "1");
            sd.append_child(r);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Clear hidden on a row (set hidden=false).
    pub fn clear_row_hidden(&mut self, sheet_name: &str, row_idx: u32) -> Result<()> {
        self.set_row_hidden(sheet_name, row_idx, false)
    }

    /// Whether a 1-based row is marked hidden.
    /// Unhide every hidden row on a sheet. Returns how many rows were unhidden.
    pub fn unhide_all_rows(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut n = 0usize;
        if let Some(sd) = root.child_mut("sheetData") {
            for row in sd.children.iter_mut() {
                if row.local_name != "row" {
                    continue;
                }
                let hidden = row
                    .get_attribute("hidden")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                if hidden {
                    row.attributes.retain(|a| a.local_name != "hidden");
                    n += 1;
                }
            }
        }
        if n > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    pub fn is_row_hidden(&self, sheet_name: &str, row_num: u32) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        let Some(sd) = root.child("sheetData") else {
            return Ok(false);
        };
        Ok(sd.children.iter().any(|c| {
            c.local_name == "row"
                && c.get_attribute("r").and_then(|s| s.parse::<u32>().ok()) == Some(row_num)
                && c.get_attribute("hidden").map(|s| s == "1").unwrap_or(false)
        }))
    }

    /// List 1-based row indices that are marked hidden.
    pub fn list_hidden_rows(&self, sheet_name: &str) -> Result<Vec<u32>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(sd) = root.child("sheetData") else {
            return Ok(Vec::new());
        };
        Ok(sd
            .children_by_name("row")
            .filter(|c| c.get_attribute("hidden").map(|s| s == "1").unwrap_or(false))
            .filter_map(|c| c.get_attribute("r").and_then(|s| s.parse().ok()))
            .collect())
    }

    /// Add a list-style data validation to a worksheet.
    ///
    /// `sqref` is the target cells (e.g. `"B2:B100"`); `formula` is the list source
    /// (e.g. `"\"Yes,No\""` or a range reference).
    pub fn add_data_validation_list(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        formula: &str,
        allow_blank: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let dv = data_validation_list(sqref, formula, allow_blank);
        if let Some(container) = root.child_mut("dataValidations") {
            container.append_child(dv);
            if let Some(count) = container.children.len().checked_sub(0) {
                container.set_attribute("count", count.to_string());
            }
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, data_validations(vec![dv]));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Remove all data validations from a sheet. Returns whether any were present.
    pub fn clear_data_validations(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "dataValidations");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Create an Excel table definition part and link it from the worksheet.
    ///
    /// Returns `(table_uri, relationship_id)`.
    /// Clear data validations on every sheet. Returns sheets modified.
    /// Clear data validations on every sheet. Returns how many sheets were modified.
    pub fn clear_all_data_validations(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_data_validations(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn add_table(
        &mut self,
        sheet_name: &str,
        name: &str,
        reference: &str,
        columns: &[&str],
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let table_uri = loop {
            let candidate = PackUri::new(format!("/xl/tables/table{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let table_xml = write_element(&table_definition(
            index,
            name,
            name,
            reference,
            columns,
        ))?;
        self.package.set_part(
            table_uri.clone(),
            content_type::SPREADSHEET_TABLE,
            table_xml,
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::TABLE,
            &table_uri,
            RelationshipTargetMode::Internal,
        );

        let mut root = self.load_sheet_root(&sheet_uri)?;
        // Ensure tableParts container
        let table_part_el = crate::element::OpenXmlElement::new(
            "x",
            crate::namespace::ns::SPREADSHEETML.uri,
            "tablePart",
        )
        .with_attribute_qname("r:id", &rid);
        if let Some(parts) = root.child_mut("tableParts") {
            parts.append_child(table_part_el);
            parts.set_attribute("count", parts.children.len().to_string());
        } else {
            let parts = crate::element::OpenXmlElement::new(
                "x",
                crate::namespace::ns::SPREADSHEETML.uri,
                "tableParts",
            )
            .with_attribute("count", "1")
            .with_child(table_part_el);
            root.append_child(parts);
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok((table_uri, rid))
    }

    /// List table part URIs under `/xl/tables/`.
    pub fn list_tables(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/tables/"))
            
            .collect()
    }

    /// Number of table definition parts.
    pub fn table_count(&self) -> usize {
        self.list_tables().len()
    }

    /// List tables as `(name, ref, uri)` triples from table definition parts.
    pub fn table_infos(&self) -> Result<Vec<(String, String, PackUri)>> {
        let mut out = Vec::new();
        for uri in self.list_tables() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            let name = root
                .get_attribute("name")
                .or_else(|| root.get_attribute("displayName"))
                .unwrap_or("")
                .to_string();
            let reference = root.get_attribute("ref").unwrap_or("").to_string();
            out.push((name, reference, uri));
        }
        Ok(out)
    }

    /// List table names only.
    pub fn table_names(&self) -> Result<Vec<String>> {
        Ok(self
            .table_infos()?
            .into_iter()
            .map(|(n, _, _)| n)
            .filter(|n| !n.is_empty())
            .collect())
    }

    /// List column names for a table by table name.
    /// Table names related from a specific worksheet.
    /// Whether the sheet has any related table parts.
    pub fn sheet_has_table(&self, sheet_name: &str) -> Result<bool> {
        Ok(!self.sheet_table_names(sheet_name)?.is_empty())
    }

    /// Sheet names that have at least one related table.
    pub fn sheets_with_tables(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.sheet_has_table(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has tables.
    pub fn has_sheets_with_tables(&self) -> Result<bool> {
        Ok(!self.sheets_with_tables()?.is_empty())
    }

    pub fn sheet_table_names(&self, sheet_name: &str) -> Result<Vec<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(rels) = self.package.opc().part_relationships(&sheet_uri) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for r in rels.find_all_by_type(rel::TABLE) {
            let target = r.target.clone();
            let uri = if target.starts_with('/') {
                PackUri::new(target)
            } else {
                // resolve relative to sheet dir /xl/worksheets/
                PackUri::new(format!("/xl/tables/{}", target.rsplit('/').next().unwrap_or(&target)))
            };
            if let Some(data) = self.package.opc().get_part(&uri) {
                if let Ok(root) = parse_element(data) {
                    if let Some(name) = root.get_attribute("name") {
                        out.push(name.to_string());
                    }
                }
            }
        }
        Ok(out)
    }

    pub fn table_columns(&self, table_name: &str) -> Result<Vec<String>> {
        let infos = self.table_infos()?;
        let Some((_, _, uri)) = infos.into_iter().find(|(n, _, _)| n == table_name) else {
            return Ok(Vec::new());
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("tableColumns") else {
            return Ok(Vec::new());
        };
        Ok(cols
            .children_by_name("tableColumn")
            .filter_map(|c| c.get_attribute("name").map(|s| s.to_string()))
            .collect())
    }

    /// Resolve a table definition part URI by table `name` or `displayName`.
    /// Whether a table has a column with the given name.
    pub fn has_table_column(&self, table_name: &str, column_name: &str) -> Result<bool> {
        Ok(self
            .table_columns(table_name)?
            .iter()
            .any(|c| c == column_name))
    }

    pub fn table_uri(&self, table_name: &str) -> Result<Option<PackUri>> {
        Ok(self
            .table_infos()?
            .into_iter()
            .find(|(n, _, _)| n == table_name)
            .map(|(_, _, uri)| uri))
    }

    /// Read table numeric `id` attribute.
    pub fn table_id(&self, table_name: &str) -> Result<Option<u32>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("id").and_then(|s| s.parse().ok()))
    }

    /// Whether table `@id` is set.
    pub fn has_table_id(&self, table_name: &str) -> Result<bool> {
        Ok(self.table_id(table_name)?.is_some())
    }

    /// Clear table `@id`.
    pub fn clear_table_id(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "id")
    }

    /// Set table numeric `id` attribute.
    pub fn set_table_id(&mut self, table_name: &str, id: u32) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("id", id.to_string());
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table `ref` range.
    pub fn table_ref(&self, table_name: &str) -> Result<Option<String>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("ref").map(|s| s.to_string()))
    }

    /// Whether table `@ref` is set.
    pub fn has_table_ref(&self, table_name: &str) -> Result<bool> {
        Ok(self.table_ref(table_name)?.is_some())
    }

    /// Clear table `@ref`.
    pub fn clear_table_ref(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "ref")
    }

    /// Read `tableStyleInfo` as `(name, show_first_col, show_last_col, show_row_stripes, show_col_stripes)`.
    pub fn table_style_info(
        &self,
        table_name: &str,
    ) -> Result<Option<(String, bool, bool, bool, bool)>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(info) = root.child("tableStyleInfo") else {
            return Ok(None);
        };
        let on = |name: &str| {
            info.get_attribute(name)
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        };
        Ok(Some((
            info.get_attribute("name").unwrap_or("").to_string(),
            on("showFirstColumn"),
            on("showLastColumn"),
            on("showRowStripes"),
            on("showColumnStripes"),
        )))
    }

    /// Set or replace `tableStyleInfo` on a table definition.
    pub fn set_table_style_info(
        &mut self,
        table_name: &str,
        style_name: &str,
        show_first_column: bool,
        show_last_column: bool,
        show_row_stripes: bool,
        show_column_stripes: bool,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.children.retain(|c| c.local_name != "tableStyleInfo");
        let flag = |b: bool| if b { "1" } else { "0" };
        let info = crate::element::OpenXmlElement::new(
            "x",
            crate::namespace::ns::SPREADSHEETML.uri,
            "tableStyleInfo",
        )
        .with_attribute("name", style_name)
        .with_attribute("showFirstColumn", flag(show_first_column))
        .with_attribute("showLastColumn", flag(show_last_column))
        .with_attribute("showRowStripes", flag(show_row_stripes))
        .with_attribute("showColumnStripes", flag(show_column_stripes));
        root.append_child(info);
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Remove tableStyleInfo from a table.
    pub fn clear_table_style_info(&mut self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "tableStyleInfo");
        if root.children.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table `headerRowCount` (defaults to 1 when absent).
    pub fn table_header_row_count(&self, table_name: &str) -> Result<Option<u32>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("headerRowCount")
            .and_then(|s| s.parse().ok())
            .or(Some(1)))
    }

    /// Whether table `@headerRowCount` is set.
    pub fn has_table_header_row_count(&self, table_name: &str) -> Result<bool> {
        Ok(self.table_header_row_count(table_name)?.is_some())
    }

    /// Clear table `@headerRowCount`.
    pub fn clear_table_header_row_count(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "headerRowCount")
    }

    /// Set whether the table has a totals row (`totalsRowCount` 0/1).
    pub fn set_table_totals_row(&mut self, table_name: &str, enabled: bool) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("totalsRowCount", if enabled { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Whether the table has a totals row.
    pub fn table_has_totals_row(&self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("totalsRowCount")
            .map(|s| s != "0" && !s.is_empty())
            .unwrap_or(false))
    }

    /// Rename a table (`name` and `displayName` attributes).
    /// Whether a table definition has an `autoFilter` child.
    /// Disable totals row (`totalsRowCount=0`). Returns whether table was found.
    pub fn clear_table_totals_row(&mut self, table_name: &str) -> Result<bool> {
        self.set_table_totals_row(table_name, false)
    }


    pub fn has_table_auto_filter(&self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("autoFilter").is_some())
    }

    /// Read table `autoFilter/@ref` when present.
    pub fn table_auto_filter_ref(&self, table_name: &str) -> Result<Option<String>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("autoFilter")
            .and_then(|a| a.get_attribute("ref").map(|s| s.to_string())))
    }

    /// Set or replace table `autoFilter` with the given ref (typically the table ref).
    pub fn set_table_auto_filter(&mut self, table_name: &str, reference: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.children.retain(|c| c.local_name != "autoFilter");
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        root.append_child(
            OpenXmlElement::new("x", x, "autoFilter").with_attribute("ref", reference),
        );
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Remove table `autoFilter`. Returns whether one was present.
    pub fn clear_table_auto_filter(&mut self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "autoFilter");
        if root.children.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    pub fn rename_table(&mut self, old_name: &str, new_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(old_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("name", new_name);
        root.set_attribute("displayName", new_name);
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Update the table `ref` (range) attribute.
    pub fn set_table_ref(&mut self, table_name: &str, reference: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("ref", reference);
        if let Some(af) = root.child_mut("autoFilter") {
            af.set_attribute("ref", reference);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Set `headerRowCount` on a table (0 or 1 typically).
    pub fn set_table_header_row_count(&mut self, table_name: &str, count: u32) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("headerRowCount", count.to_string());
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read `totalsRowCount` (defaults to 0 when absent).
    pub fn table_totals_row_count(&self, table_name: &str) -> Result<Option<u32>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("totalsRowCount")
            .and_then(|s| s.parse().ok())
            .or(Some(0)))
    }

    /// Set totals-row function and optional label on a table column by name.
    ///
    /// `function` is typically `sum`, `count`, `average`, `min`, `max`, `none`, or `custom`.
    pub fn set_table_column_totals(
        &mut self,
        table_name: &str,
        column_name: &str,
        function: &str,
        label: Option<&str>,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "tableColumn" {
                continue;
            }
            if col.get_attribute("name") != Some(column_name) {
                continue;
            }
            col.set_attribute("totalsRowFunction", function);
            if let Some(lbl) = label {
                col.set_attribute("totalsRowLabel", lbl);
            }
            found = true;
            break;
        }
        if !found {
            return Ok(false);
        }
        // Enabling a totals function implies the table has a totals row.
        if function != "none" {
            root.set_attribute("totalsRowCount", "1");
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Clear totalsRowFunction/totalsRowLabel on a table column.
    pub fn clear_table_column_totals(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "tableColumn" {
                continue;
            }
            if col.get_attribute("name") != Some(column_name) {
                continue;
            }
            let before = col.attributes.len();
            col.attributes.retain(|a| {
                a.local_name != "totalsRowFunction" && a.local_name != "totalsRowLabel"
            });
            found = col.attributes.len() < before;
            break;
        }
        if found {
            let xml = write_element(&root)?;
            self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        }
        Ok(found)
    }

    /// Read `(totalsRowFunction, totalsRowLabel)` for a table column.
    pub fn table_column_totals(
        &self,
        table_name: &str,
        column_name: &str,
    ) -> Result<Option<(String, Option<String>)>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("tableColumns") else {
            return Ok(None);
        };
        for col in cols.children_by_name("tableColumn") {
            if col.get_attribute("name") != Some(column_name) {
                continue;
            }
            let func = col
                .get_attribute("totalsRowFunction")
                .unwrap_or("none")
                .to_string();
            let label = col.get_attribute("totalsRowLabel").map(|s| s.to_string());
            return Ok(Some((func, label)));
        }
        Ok(None)
    }

    /// Set `uniqueName` on a table column.
    pub fn set_table_column_unique_name(
        &mut self,
        table_name: &str,
        column_name: &str,
        unique_name: &str,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name == "tableColumn" && col.get_attribute("name") == Some(column_name) {
                col.set_attribute("uniqueName", unique_name);
                found = true;
                break;
            }
        }
        if !found {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Clear table column `@uniqueName`.
    pub fn clear_table_column_unique_name(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "tableColumn" {
                continue;
            }
            if col.get_attribute("name") != Some(column_name) {
                continue;
            }
            if col.get_attribute("uniqueName").is_some() {
                col.remove_attribute("uniqueName");
                found = true;
            }
            break;
        }
        if found {
            let xml = write_element(&root)?;
            self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        }
        Ok(found)
    }

    /// Read `uniqueName` for a table column.
    pub fn table_column_unique_name(
        &self,
        table_name: &str,
        column_name: &str,
    ) -> Result<Option<String>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("tableColumns") else {
            return Ok(None);
        };
        for col in cols.children_by_name("tableColumn") {
            if col.get_attribute("name") == Some(column_name) {
                return Ok(col.get_attribute("uniqueName").map(|s| s.to_string()));
            }
        }
        Ok(None)
    }

    /// Set dxf ids on a table column (`headerRowDxfId`, `dataDxfId`, `totalsRowDxfId`).
    pub fn set_table_column_dxf_ids(
        &mut self,
        table_name: &str,
        column_name: &str,
        header_row: Option<u32>,
        data: Option<u32>,
        totals_row: Option<u32>,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(d) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(d)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name == "tableColumn" && col.get_attribute("name") == Some(column_name) {
                if let Some(id) = header_row {
                    col.set_attribute("headerRowDxfId", id.to_string());
                }
                if let Some(id) = data {
                    col.set_attribute("dataDxfId", id.to_string());
                }
                if let Some(id) = totals_row {
                    col.set_attribute("totalsRowDxfId", id.to_string());
                }
                found = true;
                break;
            }
        }
        if !found {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Clear headerRowDxfId/dataDxfId/totalsRowDxfId on a table column.
    pub fn clear_table_column_dxf_ids(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(d) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(d)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "tableColumn" {
                continue;
            }
            if col.get_attribute("name") != Some(column_name) {
                continue;
            }
            let before = col.attributes.len();
            col.attributes.retain(|a| {
                !matches!(
                    a.local_name.as_str(),
                    "headerRowDxfId" | "dataDxfId" | "totalsRowDxfId"
                )
            });
            found = col.attributes.len() < before;
            break;
        }
        if found {
            let xml = write_element(&root)?;
            self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        }
        Ok(found)
    }

    /// Set cell style names on a table column.
    pub fn set_table_column_cell_styles(
        &mut self,
        table_name: &str,
        column_name: &str,
        header_row: Option<&str>,
        data: Option<&str>,
        totals_row: Option<&str>,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(d) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(d)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name == "tableColumn" && col.get_attribute("name") == Some(column_name) {
                if let Some(s) = header_row {
                    col.set_attribute("headerRowCellStyle", s);
                }
                if let Some(s) = data {
                    col.set_attribute("dataCellStyle", s);
                }
                if let Some(s) = totals_row {
                    col.set_attribute("totalsRowCellStyle", s);
                }
                found = true;
                break;
            }
        }
        if !found {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read column cell styles as `(header, data, totals)`.
    pub fn table_column_cell_styles(
        &self,
        table_name: &str,
        column_name: &str,
    ) -> Result<Option<(Option<String>, Option<String>, Option<String>)>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("tableColumns") else {
            return Ok(None);
        };
        for col in cols.children_by_name("tableColumn") {
            if col.get_attribute("name") != Some(column_name) {
                continue;
            }
            let get = |name: &str| col.get_attribute(name).map(|s| s.to_string());
            return Ok(Some((
                get("headerRowCellStyle"),
                get("dataCellStyle"),
                get("totalsRowCellStyle"),
            )));
        }
        Ok(None)
    }

    /// Set `queryTableFieldId` on a table column.
    pub fn set_table_column_query_field_id(
        &mut self,
        table_name: &str,
        column_name: &str,
        field_id: u32,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name == "tableColumn" && col.get_attribute("name") == Some(column_name) {
                col.set_attribute("queryTableFieldId", field_id.to_string());
                found = true;
                break;
            }
        }
        if !found {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Clear table column `@queryTableFieldId`.
    pub fn clear_table_column_query_field_id(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "tableColumn" {
                continue;
            }
            if col.get_attribute("name") != Some(column_name) {
                continue;
            }
            if col.get_attribute("queryTableFieldId").is_some() {
                col.remove_attribute("queryTableFieldId");
                found = true;
            }
            break;
        }
        if found {
            let xml = write_element(&root)?;
            self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        }
        Ok(found)
    }

    /// Read `queryTableFieldId` for a table column.
    pub fn table_column_query_field_id(
        &self,
        table_name: &str,
        column_name: &str,
    ) -> Result<Option<u32>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("tableColumns") else {
            return Ok(None);
        };
        for col in cols.children_by_name("tableColumn") {
            if col.get_attribute("name") == Some(column_name) {
                return Ok(col
                    .get_attribute("queryTableFieldId")
                    .and_then(|s| s.parse().ok()));
            }
        }
        Ok(None)
    }

    /// Rename a table column by current name.
    pub fn rename_table_column(
        &mut self,
        table_name: &str,
        old_name: &str,
        new_name: &str,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name == "tableColumn" && col.get_attribute("name") == Some(old_name) {
                col.set_attribute("name", new_name);
                found = true;
                break;
            }
        }
        if !found {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Remove a table column by name. Updates `tableColumns/@count` and shrinks `ref`
    /// when the removed column was the last column of the table range.
    ///
    /// Does not rewrite worksheet cells (values remain in the grid).
    pub fn remove_table_column(&mut self, table_name: &str, column_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        // Capture column index of the target before removal.
        let col_index = {
            let Some(cols) = root.child("tableColumns") else {
                return Ok(false);
            };
            let names: Vec<String> = cols
                .children_by_name("tableColumn")
                .filter_map(|c| c.get_attribute("name").map(|s| s.to_string()))
                .collect();
            match names.iter().position(|n| n == column_name) {
                Some(i) => i,
                None => return Ok(false),
            }
        };
        let col_count_before = {
            let cols = root.child("tableColumns").unwrap();
            cols.children_by_name("tableColumn").count()
        };
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        cols.children.retain(|c| {
            !(c.local_name == "tableColumn" && c.get_attribute("name") == Some(column_name))
        });
        let count = cols
            .children
            .iter()
            .filter(|c| c.local_name == "tableColumn")
            .count();
        cols.set_attribute("count", count.to_string());

        // Shrink table ref when removing the rightmost column.
        if col_index + 1 == col_count_before {
            if let Some(r) = root.get_attribute("ref").map(|s| s.to_string()) {
                if let Some(shrunk) = shrink_table_ref_last_col(&r) {
                    root.set_attribute("ref", shrunk);
                }
            }
        }

        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table column numeric `id`.
    pub fn table_column_id(&self, table_name: &str, column_name: &str) -> Result<Option<u32>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("tableColumns") else {
            return Ok(None);
        };
        for col in cols.children_by_name("tableColumn") {
            if col.get_attribute("name") == Some(column_name) {
                return Ok(col.get_attribute("id").and_then(|s| s.parse().ok()));
            }
        }
        Ok(None)
    }

    /// Set table column numeric `id`.
    pub fn set_table_column_id(
        &mut self,
        table_name: &str,
        column_name: &str,
        id: u32,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name == "tableColumn" && col.get_attribute("name") == Some(column_name) {
                col.set_attribute("id", id.to_string());
                found = true;
                break;
            }
        }
        if !found {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Clear table column `@id`.
    pub fn clear_table_column_id(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cols) = root.child_mut("tableColumns") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "tableColumn" {
                continue;
            }
            if col.get_attribute("name") != Some(column_name) {
                continue;
            }
            if col.get_attribute("id").is_some() {
                col.remove_attribute("id");
                found = true;
            }
            break;
        }
        if found {
            let xml = write_element(&root)?;
            self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        }
        Ok(found)
    }

    /// Update `and` flag on customFilters for a filter column.
    pub fn set_auto_filter_custom_and(
        &mut self,
        sheet_name: &str,
        col_id: u32,
        and: bool,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(af) = root.child_mut("autoFilter") else {
            return Ok(false);
        };
        let mut found = false;
        for fc in af.children.iter_mut() {
            if fc.local_name == "filterColumn"
                && fc.get_attribute("colId") == Some(&col_id.to_string())
            {
                if let Some(custom) = fc.child_mut("customFilters") {
                    custom.set_attribute("and", if and { "1" } else { "0" });
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear `and` on customFilters for a filter column.
    pub fn clear_auto_filter_custom_and(
        &mut self,
        sheet_name: &str,
        col_id: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(af) = root.child_mut("autoFilter") else {
            return Ok(false);
        };
        let mut found = false;
        for fc in af.children.iter_mut() {
            if fc.local_name == "filterColumn"
                && fc.get_attribute("colId") == Some(&col_id.to_string())
            {
                if let Some(custom) = fc.child_mut("customFilters") {
                    if custom.get_attribute("and").is_some() {
                        custom.remove_attribute("and");
                        found = true;
                    }
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Set table `comment` attribute.
    pub fn set_table_comment(&mut self, table_name: &str, comment: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("comment", comment);
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table `comment`.
    pub fn table_comment(&self, table_name: &str) -> Result<Option<String>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("comment").map(|s| s.to_string()))
    }

    /// Enable or disable the insert row on a table (`insertRow`).
    /// Whether a table has a non-empty comment attribute.
    pub fn has_table_comment(&self, table_name: &str) -> Result<bool> {
        Ok(self
            .table_comment(table_name)?
            .map(|c| !c.is_empty())
            .unwrap_or(false))
    }

    /// Clear table comment attribute. Returns whether it was present.
    fn clear_table_attr(&mut self, table_name: &str, attr: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute(attr).is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != attr);
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    pub fn clear_table_comment(&mut self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.attributes.len();
        root.attributes.retain(|a| a.local_name != "comment");
        if root.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }


    pub fn set_table_insert_row(&mut self, table_name: &str, enabled: bool) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("insertRow", if enabled { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Whether the table has an insert row enabled.
    pub fn table_insert_row(&self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("insertRow")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Clear table `@insertRow`.
    pub fn clear_table_insert_row(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "insertRow")
    }

    /// Set `insertRowShift` on a table.
    pub fn set_table_insert_row_shift(&mut self, table_name: &str, enabled: bool) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("insertRowShift", if enabled { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Whether insertRowShift is enabled.
    pub fn table_insert_row_shift(&self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("insertRowShift")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Clear table `@insertRowShift`.
    pub fn clear_table_insert_row_shift(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "insertRowShift")
    }

    /// Set border dxf ids on a table.
    pub fn set_table_border_dxf_ids(
        &mut self,
        table_name: &str,
        header_row: Option<u32>,
        table: Option<u32>,
        totals_row: Option<u32>,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(d) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(d)?;
        if let Some(id) = header_row {
            root.set_attribute("headerRowBorderDxfId", id.to_string());
        }
        if let Some(id) = table {
            root.set_attribute("tableBorderDxfId", id.to_string());
        }
        if let Some(id) = totals_row {
            root.set_attribute("totalsRowBorderDxfId", id.to_string());
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Clear headerRowBorderDxfId/tableBorderDxfId/totalsRowBorderDxfId on a table.
    pub fn clear_table_border_dxf_ids(&mut self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(d) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(d)?;
        let before = root.attributes.len();
        root.attributes.retain(|a| {
            !matches!(
                a.local_name.as_str(),
                "headerRowBorderDxfId" | "tableBorderDxfId" | "totalsRowBorderDxfId"
            )
        });
        if root.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read border dxf ids as `(header_row, table, totals_row)`.
    pub fn table_border_dxf_ids(
        &self,
        table_name: &str,
    ) -> Result<Option<(Option<u32>, Option<u32>, Option<u32>)>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let parse = |name: &str| root.get_attribute(name).and_then(|s| s.parse().ok());
        Ok(Some((
            parse("headerRowBorderDxfId"),
            parse("tableBorderDxfId"),
            parse("totalsRowBorderDxfId"),
        )))
    }

    /// Set whether the totals row is shown (`totalsRowShown`).
    pub fn set_table_totals_row_shown(&mut self, table_name: &str, shown: bool) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("totalsRowShown", if shown { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Whether the totals row is shown (defaults true when absent).
    pub fn table_totals_row_shown(&self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(true);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("totalsRowShown")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Clear table `@totalsRowShown`.
    pub fn clear_table_totals_row_shown(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "totalsRowShown")
    }

    /// Set table published flag.
    pub fn set_table_published(&mut self, table_name: &str, published: bool) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("published", if published { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Whether the table is published.
    pub fn table_published(&self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("published")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Clear table `@published`.
    pub fn clear_table_published(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "published")
    }

    /// Set table type (`worksheet`, `xml`, `queryTable`).
    pub fn set_table_type(&mut self, table_name: &str, table_type: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("tableType", table_type);
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table type attribute.
    pub fn table_type(&self, table_name: &str) -> Result<Option<String>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("tableType").map(|s| s.to_string()))
    }

    /// Whether table `@type` is set.
    pub fn has_table_type(&self, table_name: &str) -> Result<bool> {
        Ok(self.table_type(table_name)?.is_some())
    }

    /// Clear table `@type`.
    pub fn clear_table_type(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "type")
    }

    /// Set table connection id (for query tables).
    pub fn set_table_connection_id(&mut self, table_name: &str, id: u32) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("connectionId", id.to_string());
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table connection id.
    pub fn table_connection_id(&self, table_name: &str) -> Result<Option<u32>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("connectionId")
            .and_then(|s| s.parse().ok()))
    }

    /// Whether table `@connectionId` is set.
    pub fn has_table_connection_id(&self, table_name: &str) -> Result<bool> {
        Ok(self.table_connection_id(table_name)?.is_some())
    }

    /// Clear table `@connectionId`.
    pub fn clear_table_connection_id(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "connectionId")
    }

    /// Set table `displayName` without changing the internal `name`.
    pub fn set_table_display_name(
        &mut self,
        table_name: &str,
        display_name: &str,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("displayName", display_name);
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table `displayName` (falls back to `name`).
    pub fn table_display_name(&self, table_name: &str) -> Result<Option<String>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("displayName")
            .or_else(|| root.get_attribute("name"))
            .map(|s| s.to_string()))
    }

    /// Whether table `@displayName` is set (does not fall back to `@name`).
    pub fn has_table_display_name(&self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("displayName").is_some())
    }

    /// Clear table `@displayName`.
    pub fn clear_table_display_name(&mut self, table_name: &str) -> Result<bool> {
        self.clear_table_attr(table_name, "displayName")
    }

    /// Set differential formatting ids on a table (`headerRowDxfId`, `dataDxfId`, `totalsRowDxfId`).
    pub fn set_table_dxf_ids(
        &mut self,
        table_name: &str,
        header_row: Option<u32>,
        data: Option<u32>,
        totals_row: Option<u32>,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(d) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(d)?;
        if let Some(id) = header_row {
            root.set_attribute("headerRowDxfId", id.to_string());
        }
        if let Some(id) = data {
            root.set_attribute("dataDxfId", id.to_string());
        }
        if let Some(id) = totals_row {
            root.set_attribute("totalsRowDxfId", id.to_string());
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Clear headerRowDxfId/dataDxfId/totalsRowDxfId on a table.
    pub fn clear_table_dxf_ids(&mut self, table_name: &str) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(d) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(d)?;
        let before = root.attributes.len();
        root.attributes.retain(|a| {
            !matches!(
                a.local_name.as_str(),
                "headerRowDxfId" | "dataDxfId" | "totalsRowDxfId"
            )
        });
        if root.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table dxf ids as `(header_row, data, totals_row)`.
    pub fn table_dxf_ids(
        &self,
        table_name: &str,
    ) -> Result<Option<(Option<u32>, Option<u32>, Option<u32>)>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let parse = |name: &str| root.get_attribute(name).and_then(|s| s.parse().ok());
        Ok(Some((
            parse("headerRowDxfId"),
            parse("dataDxfId"),
            parse("totalsRowDxfId"),
        )))
    }

    /// Set cell style names on a table (`headerRowCellStyle`, `dataCellStyle`, `totalsRowCellStyle`).
    pub fn set_table_cell_styles(
        &mut self,
        table_name: &str,
        header_row: Option<&str>,
        data: Option<&str>,
        totals_row: Option<&str>,
    ) -> Result<bool> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(false);
        };
        let Some(d) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(d)?;
        if let Some(s) = header_row {
            root.set_attribute("headerRowCellStyle", s);
        }
        if let Some(s) = data {
            root.set_attribute("dataCellStyle", s);
        }
        if let Some(s) = totals_row {
            root.set_attribute("totalsRowCellStyle", s);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_TABLE, xml);
        Ok(true)
    }

    /// Read table cell style names as `(header, data, totals)`.
    pub fn table_cell_styles(
        &self,
        table_name: &str,
    ) -> Result<Option<(Option<String>, Option<String>, Option<String>)>> {
        let Some(uri) = self.table_uri(table_name)? else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let get = |name: &str| root.get_attribute(name).map(|s| s.to_string());
        Ok(Some((
            get("headerRowCellStyle"),
            get("dataCellStyle"),
            get("totalsRowCellStyle"),
        )))
    }

    /// Remove a table definition by display/name. Returns whether it was found.
    pub fn remove_table(&mut self, name: &str) -> Result<bool> {
        let infos = self.table_infos()?;
        let Some((_, _, table_uri)) = infos.into_iter().find(|(n, _, _)| n == name) else {
            return Ok(false);
        };
        // Find owning sheet relationship
        let mut owner: Option<(PackUri, String)> = None;
        for sheet in &self.sheets {
            if let Some(rels) = self.package.opc().part_relationships(&sheet.uri) {
                for r in rels.iter() {
                    if r.relationship_type != rel::TABLE {
                        continue;
                    }
                    if let Ok(uri) = self
                        .package
                        .opc()
                        .resolve_relationship(Some(&sheet.uri), r)
                    {
                        if uri == table_uri {
                            owner = Some((sheet.uri.clone(), r.id.clone()));
                            break;
                        }
                    }
                }
            }
            if owner.is_some() {
                break;
            }
        }
        if let Some((sheet_uri, rid)) = owner {
            // Drop tablePart entry
            if let Ok(mut root) = self.load_sheet_root(&sheet_uri) {
                if let Some(parts) = root.child_mut("tableParts") {
                    parts.children.retain(|c| {
                        c.get_attribute_qname("r:id")
                            .or_else(|| c.get_attribute("id"))
                            != Some(rid.as_str())
                    });
                    if parts.children.is_empty() {
                        root.children.retain(|c| c.local_name != "tableParts");
                    } else {
                        parts.set_attribute("count", parts.children.len().to_string());
                    }
                    let _ = self.save_sheet_root(&sheet_uri, &root);
                }
            }
            let _ = self.package.delete_reference_relationship(Some(&sheet_uri), &rid);
        }
        self.package.delete_part(&table_uri);
        Ok(true)
    }

    /// Whether any pivot table parts exist.
    /// Remove every table definition and sheet relationships. Returns tables removed.
    pub fn clear_all_tables(&mut self) -> Result<usize> {
        let names: Vec<String> = self
            .table_infos()?
            .into_iter()
            .map(|(n, _, _)| n)
            .filter(|n| !n.is_empty())
            .collect();
        let mut n = 0usize;
        for name in names {
            if self.remove_table(&name)? {
                n += 1;
            }
        }
        // Drop any orphan table parts still present
        for uri in self.list_tables() {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    pub fn has_pivot_tables(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("/xl/pivotTables/") || s.contains("pivotTable")
        })
    }


    /// Whether any pivot cache parts exist under `/xl/pivotCache/`.
    pub fn has_pivot_caches(&self) -> bool {
        self.pivot_cache_count() > 0
    }

    /// Count pivot cache parts under `/xl/pivotCache/`.
    pub fn pivot_cache_count(&self) -> usize {
        self.list_pivot_caches().len()
    }

    /// List pivot cache part URIs under `/xl/pivotCache/`.
    pub fn list_pivot_caches(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/pivotCache/"))
            
            .collect()
    }

    /// Remove pivot cache parts and workbook pivotCaches relationships/element.
    pub fn clear_pivot_caches(&mut self) -> Result<usize> {
        let uris = self.list_pivot_caches();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::PIVOT_CACHE_DEFINITION
                                || r.relationship_type == rel::PIVOT_CACHE_RECORDS
                                || r.target.contains("pivotCache")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
            if let Some(data) = self.package.opc().get_part(&wb_uri).map(|b| b.to_vec()) {
                if let Ok(mut root) = parse_element(&data) {
                    root.children.retain(|c| c.local_name != "pivotCaches");
                    let xml = write_element(&root)?;
                    self.package.set_part(
                        wb_uri.clone(),
                        self.document_type.content_type(),
                        xml,
                    );
                }
            }
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Sheets that currently have sparkline groups.
    pub fn sheets_with_sparklines(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for s in &self.sheets {
            if self.has_sparklines(&s.name)? {
                out.push(s.name.clone());
            }
        }
        Ok(out)
    }

    /// Whether any slicer parts exist.
    /// Whether any sheet has sparklines.
    pub fn has_sheets_with_sparklines(&self) -> Result<bool> {
        Ok(!self.sheets_with_sparklines()?.is_empty())
    }

    pub fn has_slicers(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/xl/slicers/") || u.as_str().contains("/xl/slicerCaches/"))
    }

    /// Count slicer definition parts under `/xl/slicers/`.
    pub fn slicer_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/slicers/"))
            .count()
    }

    /// Count timeline definition parts under `/xl/timelines/`.
    pub fn timeline_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/timelines/"))
            .count()
    }

    /// Whether a connections part exists.
    pub fn has_connections(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/xl/connections.xml"))
    }

    /// Count connection entries in `/xl/connections.xml` (0 if part missing).
    pub fn connection_count(&self) -> Result<usize> {
        let uri = PackUri::new("/xl/connections.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root.children_by_name("connection").count())
    }

    /// List connections as `(id, name, type?, source?)`.
    pub fn list_connections(&self) -> Result<Vec<(u32, String, Option<String>, Option<String>)>> {
        let uri = PackUri::new("/xl/connections.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("connection")
            .map(|c| {
                let id = c
                    .get_attribute("id")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let name = c.get_attribute("name").unwrap_or("").to_string();
                let ty = c.get_attribute("type").map(|s| s.to_string());
                let source = c
                    .child("dbPr")
                    .and_then(|d| d.get_attribute("connection").map(|s| s.to_string()));
                (id, name, ty, source)
            })
            .collect())
    }

    /// Update a connection by id (name / refreshedVersion / background / dbPr connection).
    /// Whether a connection with the given name exists.
    pub fn has_connection(&self, name: &str) -> Result<bool> {
        Ok(self
            .list_connections()?
            .iter()
            .any(|(_, n, _, _)| n == name))
    }

    pub fn set_connection_attrs(
        &mut self,
        id: u32,
        name: Option<&str>,
        refreshed_version: Option<u32>,
        background: Option<bool>,
        source: Option<&str>,
    ) -> Result<bool> {
        let uri = PackUri::new("/xl/connections.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for c in root
            .children
            .iter_mut()
            .filter(|c| c.local_name == "connection")
        {
            let cid = c
                .get_attribute("id")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            if cid != id {
                continue;
            }
            found = true;
            if let Some(n) = name {
                c.set_attribute("name", n);
            }
            if let Some(v) = refreshed_version {
                c.set_attribute("refreshedVersion", v.to_string());
            }
            if let Some(b) = background {
                c.set_attribute("background", if b { "1" } else { "0" });
            }
            if let Some(s) = source {
                if let Some(db) = c.child_mut("dbPr") {
                    db.set_attribute("connection", s);
                }
            }
            break;
        }
        if found {
            self.package.set_part(
                uri,
                content_type::SPREADSHEET_CONNECTIONS,
                write_element(&root)?,
            );
        }
        Ok(found)
    }

    /// Remove a connection by id. Returns whether found.
    pub fn remove_connection(&mut self, id: u32) -> Result<bool> {
        let uri = PackUri::new("/xl/connections.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| {
            !(c.local_name == "connection"
                && c.get_attribute("id")
                    .and_then(|s| s.parse::<u32>().ok())
                    == Some(id))
        });
        let removed = root.children.len() < before;
        if removed {
            self.package.set_part(
                uri,
                content_type::SPREADSHEET_CONNECTIONS,
                write_element(&root)?,
            );
        }
        Ok(removed)
    }

    /// Remove slicer and slicerCache parts and their relationships.
    pub fn clear_slicers(&mut self) -> Result<usize> {
        self.clear_parts_under(&[
            "/xl/slicers/",
            "/xl/slicerCaches/",
        ], &[rel::SLICER, rel::SLICER_CACHE])
    }

    /// Whether any timeline parts exist.
    pub fn has_timelines(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("/xl/timelines/") || s.contains("/xl/timelineCaches/")
        })
    }

    /// Remove timeline and timelineCache parts and their relationships.
    pub fn clear_timelines(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["/xl/timelines/", "/xl/timelineCaches/"],
            &[rel::TIMELINE, rel::TIMELINE_CACHE],
        )
    }

    /// Remove the connections part and workbook relationship.
    pub fn clear_connections(&mut self) -> Result<bool> {
        let uri = PackUri::new("/xl/connections.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::CONNECTIONS)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Count query table parts.
    pub fn query_table_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/queryTables/"))
            .count()
    }

    /// List query table part URIs.
    pub fn list_query_tables(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/queryTables/"))
            
            .collect()
    }

    /// List query tables as `(name, connection_id, uri)`.
    pub fn query_table_infos(&self) -> Result<Vec<(String, u32, PackUri)>> {
        let mut out = Vec::new();
        for uri in self.list_query_tables() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            let name = root.get_attribute("name").unwrap_or("").to_string();
            let conn = root
                .get_attribute("connectionId")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            out.push((name, conn, uri));
        }
        Ok(out)
    }

    /// Whether any query tables exist.
    pub fn has_query_tables(&self) -> bool {
        self.query_table_count() > 0
    }

    /// Whether a query table with the given name exists.
    pub fn has_query_table(&self, name: &str) -> Result<bool> {
        Ok(self.query_table_infos()?.iter().any(|(n, _, _)| n == name))
    }

    /// Rename a query table part's `name` attribute. Returns whether found.
    pub fn rename_query_table(&mut self, old_name: &str, new_name: &str) -> Result<bool> {
        let infos = self.query_table_infos()?;
        let Some((_, _, uri)) = infos.into_iter().find(|(n, _, _)| n == old_name) else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("name", new_name);
        let xml = write_element(&root)?;
        let ct = self
            .package
            .opc()
            .content_types()
            .content_type_for(uri.as_str())
            .unwrap_or(content_type::SPREADSHEET_QUERY_TABLE)
            .to_string();
        self.package.set_part(uri, ct, xml);
        Ok(true)
    }

    /// Remove a query table part by name. Returns whether found.
    pub fn remove_query_table(&mut self, name: &str) -> Result<bool> {
        let infos = self.query_table_infos()?;
        let Some((_, _, uri)) = infos.into_iter().find(|(n, _, _)| n == name) else {
            return Ok(false);
        };
        let target = uri.as_str().to_string();
        let part_uris: Vec<PackUri> = self.package.opc().part_uris();
        for src in part_uris {
            let Some(rels) = self.package.opc().part_relationships(&src) else {
                continue;
            };
            let ids: Vec<String> = rels
                .iter()
                .filter(|r| {
                    r.relationship_type == rel::QUERY_TABLE
                        || r.target.contains("queryTable")
                })
                .filter(|r| {
                    r.target == target
                        || r.target.ends_with(target.trim_start_matches('/'))
                        || target.ends_with(r.target.trim_start_matches("./"))
                })
                .map(|r| r.id.clone())
                .collect();
            if ids.is_empty() {
                continue;
            }
            self.package
                .delete_reference_relationships(Some(&src), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Remove all query table parts and worksheet relationships.
    pub fn clear_query_tables(&mut self) -> Result<usize> {
        self.clear_parts_under(&["/xl/queryTables/"], &[rel::QUERY_TABLE])
    }

    /// Whether a volatile dependencies part exists.
    pub fn has_volatile_dependencies(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/xl/volatileDependencies.xml"))
    }

    /// Whether any threaded comment parts exist.
    pub fn has_threaded_comments(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/threadedComments/"))
    }

    /// Count threaded comment parts.
    pub fn threaded_comment_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/threadedComments/"))
            .count()
    }

    /// List threaded comment part URIs.
    pub fn list_threaded_comments(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/threadedComments/"))
            
            .collect()
    }

    /// List threaded comments as `(id, person_id, text)` across all parts.
    pub fn list_threaded_comment_entries(&self) -> Result<Vec<(String, String, String)>> {
        let mut out = Vec::new();
        for uri in self.list_threaded_comments() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            for c in root.descendants().filter(|e| e.local_name == "threadedComment") {
                let id = c.get_attribute("id").unwrap_or("").to_string();
                let person = c.get_attribute("personId").unwrap_or("").to_string();
                let text = c
                    .child("text")
                    .map(|t| t.inner_text())
                    .unwrap_or_default();
                out.push((id, person, text));
            }
        }
        Ok(out)
    }

    /// Whether a persons part exists for threaded comments.


    /// Whether any threaded comment entries exist.
    pub fn has_threaded_comment_entries(&self) -> Result<bool> {
        Ok(!self.list_threaded_comment_entries()?.is_empty())
    }

    /// Count threaded comment entries across all sheets.
    pub fn threaded_comment_entry_count(&self) -> Result<usize> {
        Ok(self.list_threaded_comment_entries()?.len())
    }

    /// Whether a persons part exists for threaded comments.
    pub fn has_persons(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/xl/persons/person.xml"))
            || self
                .package
                .opc()
                .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/persons/"))
    }

    /// Count person entries in the persons part.
    pub fn person_count(&self) -> Result<usize> {
        Ok(self.list_persons()?.len())
    }

    /// List persons as `(id, display_name)`.
    pub fn list_persons(&self) -> Result<Vec<(String, String)>> {
        let uri = PackUri::new("/xl/persons/person.xml");
        let data = if let Some(d) = self.package.opc().get_part(&uri) {
            d
        } else {
            let alt: Vec<_> = self
                .package
                .opc()
                .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/persons/"))
                
                .collect();
            if alt.is_empty() {
                return Ok(Vec::new());
            }
            match self.package.opc().get_part(&alt[0]) {
                Some(d) => d,
                None => return Ok(Vec::new()),
            }
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "person")
            .map(|p| {
                let id = p.get_attribute("id").unwrap_or("").to_string();
                let name = p.get_attribute("displayName").unwrap_or("").to_string();
                (id, name)
            })
            .collect())
    }

    /// Whether any chart style parts exist under `/xl/charts/`.
    pub fn has_chart_styles(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.starts_with("/xl/charts/style") || s.contains("/chartstyles/")
        })
    }

    /// List chart style part URIs.
    pub fn list_chart_styles(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.starts_with("/xl/charts/style") || s.contains("/chartstyles/")
            })
            
            .collect()
    }

    /// Whether named sheet views parts exist.
    pub fn has_named_sheet_views(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("namedSheetViews") || u.as_str().contains("/namedSheetView"))
    }

    /// Whether rich value / rich data shells exist.
    pub fn has_rich_data(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("/xl/richData/") || s.contains("richValue") || s.contains("rdrichvalue")
        })
    }

    /// Whether a feature property bag part exists.
    pub fn has_feature_property_bag(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("featurePropertyBag") || s.contains("FeaturePropertyBag")
        })
    }

    /// Whether any chartsheet parts exist.
    pub fn has_chartsheets(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/chartsheets/"))
    }

    /// Count chartsheet parts.
    pub fn chartsheet_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/chartsheets/"))
            .count()
    }

    /// List chartsheet part URIs.
    pub fn list_chartsheets(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/chartsheets/"))
            
            .collect()
    }

    /// Remove chartsheet parts and workbook relationships.
    pub fn clear_chartsheets(&mut self) -> Result<usize> {
        self.clear_parts_under(&["/xl/chartsheets/"], &[rel::CHARTSHEET])
    }

    /// Whether any dialogsheet parts exist.
    pub fn has_dialogsheets(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/dialogsheets/"))
    }

    /// Count dialogsheet parts.
    pub fn dialogsheet_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/dialogsheets/"))
            .count()
    }

    /// Remove dialogsheet parts and workbook relationships.
    pub fn clear_dialogsheets(&mut self) -> Result<usize> {
        self.clear_parts_under(&["/xl/dialogsheets/"], &[rel::DIALOGSHEET])
    }

    /// Whether any macrosheet parts exist.
    pub fn has_macrosheets(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/macrosheets/"))
    }

    /// Count macrosheet parts (including intl macrosheets).
    pub fn macrosheet_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/macrosheets/"))
            .count()
    }

    /// Remove macrosheet parts and workbook relationships.
    pub fn clear_macrosheets(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["/xl/macrosheets/"],
            &[rel::MACRO_SHEET, rel::INT_MACRO_SHEET],
        )
    }

    /// Whether an xmlMaps part exists.
    pub fn has_xml_maps(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/xl/xmlMaps.xml"))
    }

    /// Remove xmlMaps part and workbook relationship.
    pub fn clear_xml_maps(&mut self) -> Result<bool> {
        let uri = PackUri::new("/xl/xmlMaps.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::CUSTOM_XML_MAPPINGS)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Whether any sort map parts exist.
    pub fn has_sort_maps(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("worksheetSortMap") || u.as_str().contains("sortMap"))
    }

    /// Remove sort map parts.
    pub fn clear_sort_maps(&mut self) -> Result<usize> {
        self.clear_parts_under(&["worksheetSortMap", "sortMap"], &[rel::SORT_MAP])
    }

    /// Whether revision tracking shells exist.
    pub fn has_revision_tracking(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/revisions/"))
    }

    /// Remove revision tracking shells.
    pub fn clear_revision_tracking(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["/xl/revisions/"],
            &[rel::REVISION_HEADERS, rel::REVISION_LOG, rel::USERS],
        )
    }

    /// Whether any single-cell table parts exist.
    pub fn has_single_cell_tables(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("tableSingleCells") || s.contains("singleXmlCells")
        })
    }

    /// Remove single-cell table parts.
    pub fn clear_single_cell_tables(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["tableSingleCells", "singleXmlCells"],
            &[rel::SINGLE_CELL_TABLE],
        )
    }

    /// Whether any embedded ActiveX control parts exist.
    pub fn has_embedded_controls(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/xl/activeX/"))
    }

    /// Remove embedded ActiveX control parts.
    pub fn clear_embedded_controls(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["/xl/activeX/"],
            &[rel::EMBEDDED_CONTROL, rel::EMBEDDED_CONTROL_PERSISTENCE],
        )
    }

    /// Whether Excel attached toolbars part exists.
    pub fn has_attached_toolbars(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("attachedToolbars") || s.contains("AttachedToolbars")
        })
    }

    /// Remove Excel attached toolbars parts.
    pub fn clear_attached_toolbars(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["attachedToolbars", "AttachedToolbars"],
            &[rel::ATTACHED_TOOLBARS],
        )
    }

    /// Whether cell metadata part exists.
    pub fn has_cell_metadata(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("/xl/metadata") || s.contains("sheetMetadata") || s.contains("cellMetadata")
        })
    }

    /// Remove cell metadata parts.
    pub fn clear_cell_metadata(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["/xl/metadata", "sheetMetadata", "cellMetadata"],
            &[rel::CELL_METADATA],
        )
    }

    /// Whether any chart drawing (user shapes) parts exist.
    pub fn has_chart_drawings(&self) -> bool {
        // Parts may live under /xl/drawings/ with chartDrawing content type,
        // or under chartshapes paths. Detect via content type or chartDrawing rels.
        if self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("chartshapes") || s.contains("chartDrawing")
        }) {
            return true;
        }
        // Fall back: any part related via CHART_DRAWING from a chart
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            if !s.starts_with("/xl/charts/") {
                return false;
            }
            self.package
                .opc()
                .part_relationships(&u)
                .map(|rels| rels.iter().any(|r| r.relationship_type == rel::CHART_DRAWING))
                .unwrap_or(false)
        })
    }

    /// Remove chart drawing parts.
    pub fn clear_chart_drawings(&mut self) -> Result<usize> {
        // Collect targets of CHART_DRAWING relationships from chart parts
        let mut uris: Vec<PackUri> = Vec::new();
        let chart_uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/charts/"))
            
            .collect();
        for chart_uri in &chart_uris {
            if let Some(rels) = self.package.opc().part_relationships(chart_uri) {
                for r in rels.iter() {
                    if r.relationship_type == rel::CHART_DRAWING {
                        if let Ok(uri) = self
                            .package
                            .opc()
                            .resolve_relationship(Some(chart_uri), r)
                        {
                            if !uris.iter().any(|x| x == &uri) {
                                uris.push(uri);
                            }
                        }
                    }
                }
            }
        }
        // Also path-hint matches
        for u in self.package.opc().part_uris() {
            let s = u.as_str();
            if s.contains("chartshapes") || s.contains("chartDrawing") {
                if !uris.iter().any(|x| x == &u) {
                    uris.push(u.clone());
                }
            }
        }
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        for chart_uri in chart_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&chart_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type == rel::CHART_DRAWING)
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                self.package
                    .delete_reference_relationships(Some(&chart_uri), &ids);
            }
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Whether a theme override part exists.
    pub fn has_theme_override(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("themeOverride") || s.contains("theme/themeOverride")
        })
    }


    /// Count theme parts under `/xl/theme/`.
    pub fn theme_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/xl/theme/"))
            .count()
    }

    /// List theme part URIs.
    pub fn list_themes(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/xl/theme/"))
            
            .collect()
    }

    /// Remove theme override parts.
    pub fn clear_theme_override(&mut self) -> Result<usize> {
        self.clear_parts_under(&["themeOverride"], &[rel::THEME_OVERRIDE])
    }

    /// Whether custom data parts exist.
    pub fn has_custom_data(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("/xl/customData/") || s.contains("customData")
        })
    }

    /// Remove custom data parts.
    pub fn clear_custom_data(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["/xl/customData/", "customData"],
            &[rel::CUSTOM_DATA, rel::CUSTOM_DATA_PROPS],
        )
    }

    /// Whether supporting property bag parts exist.
    pub fn has_supporting_property_bags(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("supportingPropertyBag")
                || s.contains("rdSupportingPropertyBag")
                || s.contains("rdsupportingpropertybag")
        })
    }

    /// Remove supporting property bag parts.
    pub fn clear_supporting_property_bags(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &[
                "supportingPropertyBag",
                "rdSupportingPropertyBag",
                "rdsupportingpropertybag",
            ],
            &[
                rel::SUPPORTING_PROPERTY_BAG,
                rel::SUPPORTING_PROPERTY_BAG_STRUCTURE,
            ],
        )
    }

    /// Remove threaded comment parts and relationships.
    pub fn clear_threaded_comments(&mut self) -> Result<usize> {
        self.clear_parts_under(&["/xl/threadedComments/"], &[rel::THREADED_COMMENT])
    }

    /// Remove persons parts and workbook relationships.
    pub fn clear_persons(&mut self) -> Result<usize> {
        self.clear_parts_under(&["/xl/persons/"], &[rel::PERSON])
    }

    /// Remove chart style/color style parts (does not remove charts themselves).
    pub fn clear_chart_styles(&mut self) -> Result<usize> {
        // Only style/colors under charts, not chartN.xml
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.starts_with("/xl/charts/style")
                    || s.starts_with("/xl/charts/colors")
                    || s.contains("/chartstyles/")
            })
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        // Drop chart-style relationships from chart parts
        let chart_uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.starts_with("/xl/charts/")
                    && s.ends_with(".xml")
                    && !s.contains("style")
                    && !s.contains("colors")
            })
            .collect();
        for parent in chart_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&parent)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::CHART_STYLE
                                || r.relationship_type.contains("chartColorStyle")
                                || r.relationship_type.contains("chartStyle")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                self.package
                    .delete_reference_relationships(Some(&parent), &ids);
            }
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Remove named sheet view parts.
    pub fn clear_named_sheet_views(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["namedSheetViews", "namedSheetView"],
            &[rel::NAMED_SHEET_VIEW],
        )
    }

    /// Remove rich data / rich value shells.
    pub fn clear_rich_data(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["/xl/richData/", "richValue", "rdrichvalue"],
            &[
                rel::RICH_VALUE,
                rel::RICH_VALUE_STRUCTURE,
                rel::RICH_VALUE_TYPES,
                rel::RICH_STYLES,
            ],
        )
    }

    /// Remove feature property bag parts.
    pub fn clear_feature_property_bag(&mut self) -> Result<usize> {
        self.clear_parts_under(
            &["featurePropertyBag", "FeaturePropertyBag"],
            &[rel::FEATURE_PROPERTY_BAG],
        )
    }

    /// Remove the volatile dependencies part.
    pub fn clear_volatile_dependencies(&mut self) -> Result<bool> {
        let uri = PackUri::new("/xl/volatileDependencies.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::VOLATILE_DEPENDENCIES)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Remove parts whose URI contains any of `path_hints` and drop matching relationship types
    /// from workbook and all worksheets. Returns number of parts removed.
    fn clear_parts_under(&mut self, path_hints: &[&str], rel_types: &[&str]) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| path_hints.iter().any(|h| u.as_str().contains(h)))
            
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        let mut parents: Vec<PackUri> = self.sheets.iter().map(|s| s.uri.clone()).collect();
        if let Ok(wb) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            parents.push(wb);
        }
        for parent in parents {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&parent)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| rel_types.iter().any(|t| r.relationship_type == *t))
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                self.package
                    .delete_reference_relationships(Some(&parent), &ids);
            }
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Count pivot table parts.
    pub fn pivot_table_count(&self) -> usize {
        self.list_pivot_tables().len()
    }

    /// Whether a pivot table with the given name exists.
    pub fn has_pivot_table(&self, name: &str) -> Result<bool> {
        Ok(self.pivot_table_infos()?.iter().any(|(n, _)| n == name))
    }

    /// List pivot table definition part URIs.
    pub fn list_pivot_tables(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/xl/pivotTables/"))
            
            .collect()
    }

    /// List pivot tables as `(name, uri)` pairs when the definition has a `name` attribute.
    pub fn pivot_table_infos(&self) -> Result<Vec<(String, PackUri)>> {
        let mut out = Vec::new();
        for uri in self.list_pivot_tables() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            let name = root
                .get_attribute("name")
                .unwrap_or_else(|| {
                    uri.as_str()
                        .rsplit('/')
                        .next()
                        .unwrap_or(uri.as_str())
                })
                .to_string();
            out.push((name, uri));
        }
        Ok(out)
    }

    /// Remove a pivot table definition part by name. Returns whether found.
    ///
    /// Drops sheet relationships to the part. Pivot cache parts are left intact.
    pub fn remove_pivot_table(&mut self, name: &str) -> Result<bool> {
        let infos = self.pivot_table_infos()?;
        let Some((_, uri)) = infos.into_iter().find(|(n, _)| n == name) else {
            return Ok(false);
        };
        let target = uri.as_str().to_string();
        // Drop relationships from sheets / workbook
        let part_uris: Vec<PackUri> = self.package.opc().part_uris();
        for src in part_uris {
            let Some(rels) = self.package.opc().part_relationships(&src) else {
                continue;
            };
            let ids: Vec<String> = rels
                .iter()
                .filter(|r| {
                    r.relationship_type == rel::PIVOT_TABLE
                        || r.target.contains("pivotTable")
                })
                .filter(|r| {
                    r.target == target
                        || r.target.ends_with(target.trim_start_matches('/'))
                        || target.ends_with(r.target.trim_start_matches("./"))
                })
                .map(|r| r.id.clone())
                .collect();
            if ids.is_empty() {
                continue;
            }
            self.package
                .delete_reference_relationships(Some(&src), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Rename a pivot table definition part by current name.
    pub fn rename_pivot_table(&mut self, old_name: &str, new_name: &str) -> Result<bool> {
        for (name, uri) in self.pivot_table_infos()? {
            if name != old_name {
                continue;
            }
            let Some(data) = self.package.opc().get_part(&uri) else {
                return Ok(false);
            };
            let mut root = parse_element(data)?;
            root.set_attribute("name", new_name);
            self.package.set_part(
                uri,
                content_type::SPREADSHEET_PIVOT_TABLE,
                write_element(&root)?,
            );
            return Ok(true);
        }
        Ok(false)
    }

    /// Set pivot table attributes (`dataCaption`, `showDrill`, `itemPrintTitles`, etc.).
    pub fn set_pivot_table_attrs(
        &mut self,
        name: &str,
        data_caption: Option<&str>,
        show_drill: Option<bool>,
        item_print_titles: Option<bool>,
        field_list_sort_ascending: Option<bool>,
        page_over_then_down: Option<bool>,
    ) -> Result<bool> {
        for (n, uri) in self.pivot_table_infos()? {
            if n != name {
                continue;
            }
            let Some(data) = self.package.opc().get_part(&uri) else {
                return Ok(false);
            };
            let mut root = parse_element(data)?;
            if let Some(c) = data_caption {
                root.set_attribute("dataCaption", c);
            }
            if let Some(v) = show_drill {
                root.set_attribute("showDrill", if v { "1" } else { "0" });
            }
            if let Some(v) = item_print_titles {
                root.set_attribute("itemPrintTitles", if v { "1" } else { "0" });
            }
            if let Some(v) = field_list_sort_ascending {
                root.set_attribute("fieldListSortAscending", if v { "1" } else { "0" });
            }
            if let Some(v) = page_over_then_down {
                root.set_attribute("pageOverThenDown", if v { "1" } else { "0" });
            }
            self.package.set_part(
                uri,
                content_type::SPREADSHEET_PIVOT_TABLE,
                write_element(&root)?,
            );
            return Ok(true);
        }
        Ok(false)
    }

    /// Read pivot table location `ref` when present.
    pub fn pivot_table_location(&self, name: &str) -> Result<Option<String>> {
        for (n, uri) in self.pivot_table_infos()? {
            if n != name {
                continue;
            }
            let Some(data) = self.package.opc().get_part(&uri) else {
                return Ok(None);
            };
            let root = parse_element(data)?;
            return Ok(root
                .child("location")
                .and_then(|l| l.get_attribute("ref").map(|s| s.to_string())));
        }
        Ok(None)
    }

    /// Remove pivot table definition parts (and related pivotCaches when only used by them).
    pub fn clear_pivot_tables(&mut self) -> Result<usize> {
        let n = self.clear_parts_under(
            &["/xl/pivotTables/", "/xl/pivotCache/"],
            &[
                rel::PIVOT_TABLE,
                rel::PIVOT_CACHE_DEFINITION,
                rel::PIVOT_CACHE_RECORDS,
            ],
        )?;
        // Drop pivotCaches element from workbook if present
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            if let Some(data) = self.package.opc().get_part(&wb_uri).map(|d| d.to_vec()) {
                if let Ok(mut root) = parse_element(&data) {
                    let before = root.children.len();
                    root.children.retain(|c| c.local_name != "pivotCaches");
                    if root.children.len() < before {
                        let xml = write_element(&root)?;
                        self.package.set_part(
                            wb_uri,
                            self.document_type.content_type(),
                            xml,
                        );
                    }
                }
            }
        }
        Ok(n)
    }

    /// Remove all chart parts under `/xl/charts/` and chart relationships.
    pub fn clear_charts(&mut self) -> Result<usize> {
        self.clear_parts_under(&["/xl/charts/"], &[rel::CHART, rel::EXTENDED_CHART])
    }

    /// Whether the sheet is protected (`sheetProtection` present).
    pub fn is_sheet_protected(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_protection_flags(sheet_name)?.is_some())
    }


    /// Alias for [`is_sheet_protected`](Self::is_sheet_protected).
    pub fn has_sheet_protection(&self, sheet_name: &str) -> Result<bool> {
        self.is_sheet_protected(sheet_name)
    }

    /// Read sheet protection flags as `(sheet, objects, scenarios)` when present.
    /// Sheet names that have sheet protection enabled.
    pub fn sheets_with_sheet_protection(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_sheet_protection(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet is returned by [`sheets_with_sheet_protection`](Self::sheets_with_sheet_protection).
    pub fn has_sheets_with_sheet_protection(&self) -> Result<bool> {
        Ok(!self.sheets_with_sheet_protection()?.is_empty())
    }

    pub fn sheet_protection_flags(
        &self,
        sheet_name: &str,
    ) -> Result<Option<(bool, bool, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(sp) = root.child("sheetProtection") else {
            return Ok(None);
        };
        let on = |name: &str| {
            sp.get_attribute(name)
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        };
        Ok(Some((on("sheet"), on("objects"), on("scenarios"))))
    }

    /// Enable sheet protection on a worksheet (no password hashing).
    pub fn set_sheet_protection(
        &mut self,
        sheet_name: &str,
        sheet: bool,
        objects: bool,
        scenarios: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "sheetProtection");
        // sheetProtection typically follows sheetData / before page margins
        let insert_at = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
            .map(|i| i + 1)
            .unwrap_or(root.children.len());
        root.children
            .insert(insert_at, sheet_protection(sheet, objects, scenarios));
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Set additional sheet protection permission flags on existing `sheetProtection`.
    ///
    /// Creates a basic protection element if missing. Flag names match OOXML attributes,
    /// e.g. `"formatCells"`, `"insertRows"`, `"deleteColumns"`, `"sort"`, `"autoFilter"`,
    /// `"selectLockedCells"`, `"selectUnlockedCells"`, `"pivotTables"`.
    pub fn set_sheet_protection_flags(
        &mut self,
        sheet_name: &str,
        flags: &[(&str, bool)],
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if root.child("sheetProtection").is_none() {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, sheet_protection(true, false, false));
        }
        if let Some(sp) = root.child_mut("sheetProtection") {
            for (name, enabled) in flags {
                sp.set_attribute(*name, if *enabled { "1" } else { "0" });
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Clear sheetProtection flag attributes listed in `flags` (names only).
    pub fn clear_sheet_protection_flags(
        &mut self,
        sheet_name: &str,
        flags: &[&str],
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sp) = root.child_mut("sheetProtection") else {
            return Ok(false);
        };
        let before = sp.attributes.len();
        sp.attributes.retain(|a| !flags.iter().any(|f| a.local_name == *f));
        if sp.attributes.len() == before {
            return Ok(false);
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Read a single sheet protection permission flag (default false when unset/missing).
    pub fn sheet_protection_flag(&self, sheet_name: &str, flag: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetProtection")
            .and_then(|sp| sp.get_attribute(flag))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Remove `sheetProtection` from a worksheet. Returns whether it was present.
    pub fn clear_sheet_protection(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "sheetProtection");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Add a protected range shell (`protectedRanges/protectedRange`) without password hashing.
    /// Clear sheet protection on every sheet. Returns sheets modified.
    pub fn clear_all_sheet_protection(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_sheet_protection(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Sheet names that have sheet protection.
    pub fn sheets_with_protection(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_sheet_protection(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    pub fn add_protected_range(
        &mut self,
        sheet_name: &str,
        name: &str,
        sqref: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let pr = OpenXmlElement::new("x", x, "protectedRange")
            .with_attribute("name", name)
            .with_attribute("sqref", sqref);
        if let Some(container) = root.child_mut("protectedRanges") {
            // replace same name
            container
                .children
                .retain(|c| c.get_attribute("name") != Some(name));
            container.append_child(pr);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| {
                    matches!(
                        c.local_name.as_str(),
                        "drawing" | "legacyDrawing" | "tableParts" | "extLst"
                    )
                })
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "protectedRanges").with_child(pr),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List protected ranges as `(name, sqref)`.
    pub fn list_protected_ranges(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("protectedRanges") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("protectedRange")
            .map(|pr| {
                (
                    pr.get_attribute("name").unwrap_or("").to_string(),
                    pr.get_attribute("sqref").unwrap_or("").to_string(),
                )
            })
            .collect())
    }

    /// Number of protected ranges on the sheet.
    pub fn protected_range_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_protected_ranges(sheet_name)?.len())
    }

    /// Whether any protected ranges exist.
    pub fn has_protected_ranges(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.protected_range_count(sheet_name)? > 0)
    }

    /// Remove a protected range by name. Returns whether present.
    pub fn remove_protected_range(&mut self, sheet_name: &str, name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("protectedRanges") else {
            return Ok(false);
        };
        let before = container.children.len();
        container
            .children
            .retain(|c| c.get_attribute("name") != Some(name));
        let removed = container.children.len() < before;
        if container.children.is_empty() {
            root.children.retain(|c| c.local_name != "protectedRanges");
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear all protected ranges. Returns how many were removed.
    pub fn clear_protected_ranges(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.protected_range_count(sheet_name)?;
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "protectedRanges");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Add an ignored error entry (`ignoredErrors/ignoredError`).
    ///
    /// `flags` is a list of boolean attribute names to set true, e.g. `["numberStoredAsText", "formula"]`.
    pub fn add_ignored_error(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        flags: &[&str],
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut err = OpenXmlElement::new("x", x, "ignoredError").with_attribute("sqref", sqref);
        for f in flags {
            err.set_attribute(*f, "1");
        }
        if let Some(container) = root.child_mut("ignoredErrors") {
            container.append_child(err);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| {
                    matches!(
                        c.local_name.as_str(),
                        "drawing" | "legacyDrawing" | "tableParts" | "extLst"
                    )
                })
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "ignoredErrors").with_child(err),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List ignored errors as `(sqref, flags)`.
    pub fn list_ignored_errors(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, Vec<String>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("ignoredErrors") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("ignoredError")
            .map(|e| {
                let sqref = e.get_attribute("sqref").unwrap_or("").to_string();
                let flags = e
                    .attributes
                    .iter()
                    .filter(|a| a.local_name != "sqref" && (a.value == "1" || a.value.eq_ignore_ascii_case("true")))
                    .map(|a| a.local_name.clone())
                    .collect();
                (sqref, flags)
            })
            .collect())
    }

    /// Number of ignored error entries.
    pub fn ignored_error_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_ignored_errors(sheet_name)?.len())
    }

    /// Whether ignored errors exist.
    pub fn has_ignored_errors(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.ignored_error_count(sheet_name)? > 0)
    }

    /// Clear all ignored errors. Returns how many were removed.
    pub fn clear_ignored_errors(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.ignored_error_count(sheet_name)?;
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "ignoredErrors");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Remove ignored error entries matching `sqref`. Returns how many were removed.
    pub fn remove_ignored_error(&mut self, sheet_name: &str, sqref: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("ignoredErrors") else {
            return Ok(0);
        };
        let before = container.children.len();
        container.children.retain(|c| {
            !(c.local_name == "ignoredError" && c.get_attribute("sqref").unwrap_or("") == sqref)
        });
        let n = before - container.children.len();
        if n > 0 {
            if container.children.is_empty() {
                root.children.retain(|c| c.local_name != "ignoredErrors");
            }
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    /// Set data consolidate start/left labels and link flags.
    pub fn set_data_consolidate_attrs(
        &mut self,
        sheet_name: &str,
        function: Option<&str>,
        start_labels: Option<bool>,
        left_labels: Option<bool>,
        top_labels: Option<bool>,
        link: Option<bool>,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("dataConsolidate").is_none() {
            root.children.push(
                OpenXmlElement::new("x", x, "dataConsolidate")
                    .with_child(OpenXmlElement::new("x", x, "dataRefs")),
            );
        }
        let dc = root.child_mut("dataConsolidate").expect("dataConsolidate");
        if let Some(f) = function {
            dc.set_attribute("function", f);
        }
        if let Some(v) = start_labels {
            dc.set_attribute("startLabels", if v { "1" } else { "0" });
        }
        if let Some(v) = left_labels {
            dc.set_attribute("leftLabels", if v { "1" } else { "0" });
        }
        if let Some(v) = top_labels {
            dc.set_attribute("topLabels", if v { "1" } else { "0" });
        }
        if let Some(v) = link {
            dc.set_attribute("link", if v { "1" } else { "0" });
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read data consolidate flags as `(function?, start_labels, left_labels, top_labels, link)`.
    pub fn data_consolidate_attrs(
        &self,
        sheet_name: &str,
    ) -> Result<Option<(Option<String>, bool, bool, bool, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(dc) = root.child("dataConsolidate") else {
            return Ok(None);
        };
        let flag = |name: &str| {
            dc.get_attribute(name)
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        };
        Ok(Some((
            dc.get_attribute("function").map(|s| s.to_string()),
            flag("startLabels"),
            flag("leftLabels"),
            flag("topLabels"),
            flag("link"),
        )))
    }

    /// Clear all data consolidate refs while keeping the parent element.
    pub fn clear_data_consolidate_refs(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.list_data_consolidate_refs(sheet_name)?.len();
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(dc) = root.child_mut("dataConsolidate") {
            if let Some(refs) = dc.child_mut("dataRefs") {
                refs.children.clear();
            }
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    /// Add a what-if scenario shell (`scenarios/scenario`) with optional input cells.
    ///
    /// `inputs` are `(cell_ref, value)` pairs.
    pub fn add_scenario(
        &mut self,
        sheet_name: &str,
        name: &str,
        inputs: &[(&str, &str)],
        comment: Option<&str>,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut scenario = OpenXmlElement::new("x", x, "scenario")
            .with_attribute("name", name)
            .with_attribute("count", inputs.len().to_string());
        if let Some(c) = comment {
            scenario.set_attribute("comment", c);
        }
        for (cell, val) in inputs {
            scenario.append_child(
                OpenXmlElement::new("x", x, "inputCells")
                    .with_attribute("r", *cell)
                    .with_attribute("val", *val),
            );
        }
        if let Some(container) = root.child_mut("scenarios") {
            container
                .children
                .retain(|c| c.get_attribute("name") != Some(name));
            container.append_child(scenario);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| {
                    matches!(
                        c.local_name.as_str(),
                        "drawing" | "legacyDrawing" | "tableParts" | "extLst"
                    )
                })
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "scenarios").with_child(scenario),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List scenarios as `(name, comment, input_count)`.
    pub fn list_scenarios(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, Option<String>, usize)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("scenarios") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("scenario")
            .map(|s| {
                (
                    s.get_attribute("name").unwrap_or("").to_string(),
                    s.get_attribute("comment").map(|c| c.to_string()),
                    s.children_by_name("inputCells").count(),
                )
            })
            .collect())
    }

    /// Number of scenarios.
    pub fn scenario_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_scenarios(sheet_name)?.len())
    }

    /// Whether scenarios exist.
    pub fn has_scenarios(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.scenario_count(sheet_name)? > 0)
    }

    /// Remove a scenario by name. Returns whether present.
    pub fn remove_scenario(&mut self, sheet_name: &str, name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("scenarios") else {
            return Ok(false);
        };
        let before = container.children.len();
        container
            .children
            .retain(|c| c.get_attribute("name") != Some(name));
        let removed = container.children.len() < before;
        if container.children.is_empty() {
            root.children.retain(|c| c.local_name != "scenarios");
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear all scenarios. Returns how many were removed.
    pub fn clear_scenarios(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.scenario_count(sheet_name)?;
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "scenarios");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Set scenario flags (`locked`, `hidden`, `user`) and optional comment.
    pub fn set_scenario_attrs(
        &mut self,
        sheet_name: &str,
        name: &str,
        locked: Option<bool>,
        hidden: Option<bool>,
        user: Option<bool>,
        comment: Option<&str>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("scenarios") else {
            return Ok(false);
        };
        let mut found = false;
        for sc in container
            .children
            .iter_mut()
            .filter(|c| c.local_name == "scenario")
        {
            if sc.get_attribute("name").unwrap_or("") != name {
                continue;
            }
            found = true;
            if let Some(v) = locked {
                sc.set_attribute("locked", if v { "1" } else { "0" });
            }
            if let Some(v) = hidden {
                sc.set_attribute("hidden", if v { "1" } else { "0" });
            }
            if let Some(v) = user {
                sc.set_attribute("user", if v { "1" } else { "0" });
            }
            if let Some(c) = comment {
                sc.set_attribute("comment", c);
            }
            break;
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// List scenario input cells for a named scenario as `(cell_ref, val)`.
    pub fn list_scenario_inputs(
        &self,
        sheet_name: &str,
        name: &str,
    ) -> Result<Vec<(String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("scenarios") else {
            return Ok(Vec::new());
        };
        for sc in container.children_by_name("scenario") {
            if sc.get_attribute("name").unwrap_or("") != name {
                continue;
            }
            return Ok(sc
                .children_by_name("inputCells")
                .map(|c| {
                    (
                        c.get_attribute("r").unwrap_or("").to_string(),
                        c.get_attribute("val").unwrap_or("").to_string(),
                    )
                })
                .collect());
        }
        Ok(Vec::new())
    }

    /// Add a cell watch entry (`cellWatches/cellWatch@r`).


    /// Whether a named scenario has any input cells.
    pub fn has_scenario_inputs(&self, sheet_name: &str, name: &str) -> Result<bool> {
        Ok(!self.list_scenario_inputs(sheet_name, name)?.is_empty())
    }

    /// Count input cells for a named scenario.
    pub fn scenario_input_count(&self, sheet_name: &str, name: &str) -> Result<usize> {
        Ok(self.list_scenario_inputs(sheet_name, name)?.len())
    }

    /// Set/replace the value of a scenario input cell. Creates the input if missing.
    pub fn set_scenario_input(
        &mut self,
        sheet_name: &str,
        name: &str,
        cell_ref: &str,
        val: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("scenarios") else {
            return Ok(false);
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut found = false;
        for sc in container
            .children
            .iter_mut()
            .filter(|c| c.local_name == "scenario")
        {
            if sc.get_attribute("name").unwrap_or("") != name {
                continue;
            }
            found = true;
            let mut updated = false;
            for ic in sc
                .children
                .iter_mut()
                .filter(|c| c.local_name == "inputCells")
            {
                if ic.get_attribute("r") == Some(cell_ref) {
                    ic.set_attribute("val", val);
                    updated = true;
                    break;
                }
            }
            if !updated {
                sc.append_child(
                    OpenXmlElement::new("x", x, "inputCells")
                        .with_attribute("r", cell_ref)
                        .with_attribute("val", val),
                );
            }
            let count = sc.children_by_name("inputCells").count();
            sc.set_attribute("count", count.to_string());
            break;
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Remove one scenario input cell by reference. Returns whether found.
    pub fn remove_scenario_input(
        &mut self,
        sheet_name: &str,
        name: &str,
        cell_ref: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("scenarios") else {
            return Ok(false);
        };
        let mut removed = false;
        for sc in container
            .children
            .iter_mut()
            .filter(|c| c.local_name == "scenario")
        {
            if sc.get_attribute("name").unwrap_or("") != name {
                continue;
            }
            let before = sc.children.len();
            sc.children.retain(|c| {
                !(c.local_name == "inputCells" && c.get_attribute("r") == Some(cell_ref))
            });
            removed = sc.children.len() < before;
            if removed {
                let count = sc.children_by_name("inputCells").count();
                sc.set_attribute("count", count.to_string());
            }
            break;
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear all input cells on a named scenario. Returns how many were removed.
    pub fn clear_scenario_inputs(&mut self, sheet_name: &str, name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("scenarios") else {
            return Ok(0);
        };
        let mut removed = 0usize;
        for sc in container
            .children
            .iter_mut()
            .filter(|c| c.local_name == "scenario")
        {
            if sc.get_attribute("name").unwrap_or("") != name {
                continue;
            }
            let before = sc.children.len();
            sc.children.retain(|c| c.local_name != "inputCells");
            removed = before - sc.children.len();
            sc.set_attribute("count", "0");
            break;
        }
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Add a cell watch entry (`cellWatches/cellWatch@r`).
    pub fn add_cell_watch(&mut self, sheet_name: &str, cell_ref: &str) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let watch = OpenXmlElement::new("x", x, "cellWatch").with_attribute("r", cell_ref);
        if let Some(container) = root.child_mut("cellWatches") {
            container.children.retain(|c| c.get_attribute("r") != Some(cell_ref));
            container.append_child(watch);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| {
                    matches!(
                        c.local_name.as_str(),
                        "drawing" | "legacyDrawing" | "tableParts" | "extLst"
                    )
                })
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "cellWatches").with_child(watch),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List watched cells.
    pub fn list_cell_watches(&self, sheet_name: &str) -> Result<Vec<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("cellWatches") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("cellWatch")
            .filter_map(|c| c.get_attribute("r").map(|s| s.to_string()))
            .collect())
    }


    /// Whether any cell watches are configured on a sheet.
    pub fn has_cell_watches(&self, sheet_name: &str) -> Result<bool> {
        Ok(!self.list_cell_watches(sheet_name)?.is_empty())
    }

    /// Count cell watches on a sheet.
    pub fn cell_watch_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_cell_watches(sheet_name)?.len())
    }

    /// Clear all cell watches. Returns how many were removed.
    pub fn clear_cell_watches(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.list_cell_watches(sheet_name)?.len();
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "cellWatches");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Remove one cell watch by reference. Returns whether found.
    pub fn remove_cell_watch(&mut self, sheet_name: &str, cell_ref: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("cellWatches") else {
            return Ok(false);
        };
        let before = container.children.len();
        container
            .children
            .retain(|c| !(c.local_name == "cellWatch" && c.get_attribute("r") == Some(cell_ref)));
        let removed = container.children.len() < before;
        if container.children.is_empty() {
            root.children.retain(|c| c.local_name != "cellWatches");
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear cell watches on every sheet. Returns total removed.
    pub fn clear_all_cell_watches(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut total = 0usize;
        for name in names {
            total += self.clear_cell_watches(&name)?;
        }
        Ok(total)
    }

    /// Add an OLE object reference shell under `oleObjects` (relationship to embedding optional).
    pub fn add_ole_object(
        &mut self,
        sheet_name: &str,
        prog_id: &str,
        shape_id: u32,
        relationship_id: Option<&str>,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut ole = OpenXmlElement::new("x", x, "oleObject")
            .with_attribute("progId", prog_id)
            .with_attribute("shapeId", shape_id.to_string());
        if let Some(rid) = relationship_id {
            ole = ole.with_attribute_qname("r:id", rid);
        }
        if let Some(container) = root.child_mut("oleObjects") {
            container.append_child(ole);
        } else {
            let insert_at = root.children.len();
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "oleObjects").with_child(ole),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List OLE objects as `(prog_id, shape_id, r_id?)`.
    pub fn list_ole_objects(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, u32, Option<String>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("oleObjects") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("oleObject")
            .map(|o| {
                (
                    o.get_attribute("progId").unwrap_or("").to_string(),
                    o.get_attribute("shapeId")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                    o.get_attribute_qname("r:id")
                        .or_else(|| o.get_attribute("id"))
                        .map(|s| s.to_string()),
                )
            })
            .collect())
    }

    /// Number of OLE objects on the sheet.
    pub fn ole_object_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_ole_objects(sheet_name)?.len())
    }

    /// Whether OLE objects exist.
    pub fn has_ole_objects(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.ole_object_count(sheet_name)? > 0)
    }

    /// Clear all OLE objects. Returns how many were removed.
    pub fn clear_ole_objects(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.ole_object_count(sheet_name)?;
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "oleObjects");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Remove OLE objects matching `shape_id`. Returns how many were removed.
    pub fn remove_ole_object(&mut self, sheet_name: &str, shape_id: u32) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("oleObjects") else {
            return Ok(0);
        };
        let sid = shape_id.to_string();
        let before = container.children.len();
        container.children.retain(|c| {
            !(c.local_name == "oleObject"
                && c.get_attribute("shapeId") == Some(sid.as_str()))
        });
        let removed = before - container.children.len();
        if container.children.is_empty() {
            root.children.retain(|c| c.local_name != "oleObjects");
        }
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Add an ActiveX control reference shell under `controls`.
    pub fn add_control(
        &mut self,
        sheet_name: &str,
        name: &str,
        shape_id: u32,
        relationship_id: Option<&str>,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut ctrl = OpenXmlElement::new("x", x, "control")
            .with_attribute("name", name)
            .with_attribute("shapeId", shape_id.to_string());
        if let Some(rid) = relationship_id {
            ctrl = ctrl.with_attribute_qname("r:id", rid);
        }
        if let Some(container) = root.child_mut("controls") {
            container.append_child(ctrl);
        } else {
            root.children.push(
                OpenXmlElement::new("x", x, "controls").with_child(ctrl),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List controls as `(name, shape_id, r_id?)`.
    pub fn list_controls(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, u32, Option<String>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("controls") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("control")
            .map(|c| {
                (
                    c.get_attribute("name").unwrap_or("").to_string(),
                    c.get_attribute("shapeId")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                    c.get_attribute_qname("r:id")
                        .or_else(|| c.get_attribute("id"))
                        .map(|s| s.to_string()),
                )
            })
            .collect())
    }

    /// Number of controls.
    pub fn control_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_controls(sheet_name)?.len())
    }

    /// Whether controls exist.
    pub fn has_controls(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.control_count(sheet_name)? > 0)
    }

    /// Clear all controls. Returns how many were removed.
    pub fn clear_controls(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.control_count(sheet_name)?;
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "controls");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Remove controls matching `shape_id`. Returns how many were removed.
    pub fn remove_control(&mut self, sheet_name: &str, shape_id: u32) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("controls") else {
            return Ok(0);
        };
        let sid = shape_id.to_string();
        let before = container.children.len();
        container.children.retain(|c| {
            !(c.local_name == "control" && c.get_attribute("shapeId") == Some(sid.as_str()))
        });
        let removed = before - container.children.len();
        if container.children.is_empty() {
            root.children.retain(|c| c.local_name != "controls");
        }
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Add a web publish item shell.
    pub fn add_web_publish_item(
        &mut self,
        sheet_name: &str,
        id: u32,
        source_type: &str,
        destination_file: &str,
        title: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let item = OpenXmlElement::new("x", x, "webPublishItem")
            .with_attribute("id", id.to_string())
            .with_attribute("divId", format!("div_{id}"))
            .with_attribute("sourceType", source_type)
            .with_attribute("destinationFile", destination_file)
            .with_attribute("title", title);
        if let Some(container) = root.child_mut("webPublishItems") {
            container.append_child(item);
            container.set_attribute("count", container.children.len().to_string());
        } else {
            root.children.push(
                OpenXmlElement::new("x", x, "webPublishItems")
                    .with_attribute("count", "1")
                    .with_child(item),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List web publish items as `(id, source_type, destination, title)`.
    pub fn list_web_publish_items(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(u32, String, String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("webPublishItems") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("webPublishItem")
            .map(|i| {
                (
                    i.get_attribute("id")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                    i.get_attribute("sourceType").unwrap_or("").to_string(),
                    i.get_attribute("destinationFile")
                        .unwrap_or("")
                        .to_string(),
                    i.get_attribute("title").unwrap_or("").to_string(),
                )
            })
            .collect())
    }

    /// Number of web publish items.
    pub fn web_publish_item_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_web_publish_items(sheet_name)?.len())
    }


    /// Whether a sheet has any web publish items.
    pub fn has_web_publish_items(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.web_publish_item_count(sheet_name)? > 0)
    }

    /// Clear web publish items. Returns how many were removed.
    pub fn clear_web_publish_items(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.web_publish_item_count(sheet_name)?;
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children
            .retain(|c| c.local_name != "webPublishItems");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Set workbook-level structure/windows protection (no password).
    pub fn set_workbook_protection(
        &mut self,
        lock_structure: bool,
        lock_windows: bool,
    ) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            workbook(Vec::<crate::element::OpenXmlElement>::new())
        };
        root.children
            .retain(|c| c.local_name != "workbookProtection");
        // workbookProtection typically before bookViews/sheets
        let insert_at = root
            .children
            .iter()
            .position(|c| matches!(c.local_name.as_str(), "bookViews" | "sheets"))
            .unwrap_or(0);
        root.children.insert(
            insert_at,
            workbook_protection(lock_structure, lock_windows),
        );
        let xml = write_element(&root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            xml,
        );
        Ok(())
    }

    /// Whether the workbook has `workbookProtection`.
    pub fn is_workbook_protected(&self) -> Result<bool> {
        Ok(self.workbook_protection_flags()?.is_some())
    }


    /// Alias for [`is_workbook_protected`](Self::is_workbook_protected).
    pub fn has_workbook_protection(&self) -> Result<bool> {
        self.is_workbook_protected()
    }

    /// Read workbook protection as `(lock_structure, lock_windows)` when present.
    pub fn workbook_protection_flags(&self) -> Result<Option<(bool, bool)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(wp) = root.child("workbookProtection") else {
            return Ok(None);
        };
        let on = |name: &str| {
            wp.get_attribute(name)
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        };
        Ok(Some((on("lockStructure"), on("lockWindows"))))
    }

    /// Remove workbook-level protection. Returns whether it was present.
    pub fn clear_workbook_protection(&mut self) -> Result<bool> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Ok(false);
        };
        let before = root.children.len();
        root.children
            .retain(|c| c.local_name != "workbookProtection");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package.set_part(
                wb_uri,
                self.document_type.content_type(),
                xml,
            );
        }
        Ok(removed)
    }

    /// Set workbook protection including lockRevision flag.
    pub fn set_workbook_protection_ex(
        &mut self,
        lock_structure: bool,
        lock_windows: bool,
        lock_revision: bool,
    ) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            workbook(Vec::<crate::element::OpenXmlElement>::new())
        };
        root.children
            .retain(|c| c.local_name != "workbookProtection");
        let mut wp = workbook_protection(lock_structure, lock_windows);
        if lock_revision {
            wp.set_attribute("lockRevision", "1");
        }
        let insert_at = root
            .children
            .iter()
            .position(|c| matches!(c.local_name.as_str(), "bookViews" | "sheets"))
            .unwrap_or(0);
        root.children.insert(insert_at, wp);
        let xml = write_element(&root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            xml,
        );
        Ok(())
    }

    /// Clear workbook protection (alias).
    pub fn clear_workbook_protection_ex(&mut self) -> Result<bool> {
        self.clear_workbook_protection()
    }

    /// Whether lockRevision is set on workbookProtection.
    pub fn workbook_lock_revision(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookProtection")
            .and_then(|wp| wp.get_attribute("lockRevision"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Read workbook protection as `(lock_structure, lock_windows, lock_revision)`.
    pub fn workbook_protection_flags_ex(&self) -> Result<Option<(bool, bool, bool)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(wp) = root.child("workbookProtection") else {
            return Ok(None);
        };
        let on = |name: &str| {
            wp.get_attribute(name)
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        };
        Ok(Some((
            on("lockStructure"),
            on("lockWindows"),
            on("lockRevision"),
        )))
    }

    /// Write a rich-text cell with bold/normal segments.
    ///
    /// `segments` is `[(text, bold), ...]`.
    pub fn set_rich_text_cell(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
        segments: &[(&str, bool)],
    ) -> Result<()> {
        use crate::spreadsheet::cell_rich_text;
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let sheet_data = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("worksheet has no sheetData".into()))?;
        let row_idx: u32 = cell_ref
            .bytes()
            .skip_while(|b| b.is_ascii_alphabetic())
            .map(|b| b as char)
            .collect::<String>()
            .parse()
            .unwrap_or(1);
        let cell_el = cell_rich_text(cell_ref, segments);
        let row_el = if let Some(r) = sheet_data.children.iter_mut().find(|c| {
            c.local_name == "row"
                && c.get_attribute("r").and_then(|s| s.parse().ok()) == Some(row_idx)
        }) {
            r
        } else {
            sheet_data.append_child(row(row_idx, Vec::<crate::element::OpenXmlElement>::new()));
            sheet_data.children.last_mut().unwrap()
        };
        row_el
            .children
            .retain(|c| c.get_attribute("r") != Some(cell_ref));
        row_el.append_child(cell_el);
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a chartsheet that hosts an existing chart part via a drawing.
    ///
    /// Creates a minimal chartsheet + drawing that anchors the chart.
    /// Returns `(chartsheet_uri, drawing_uri)`.
    pub fn add_chartsheet(
        &mut self,
        name: &str,
        chart_uri: &PackUri,
    ) -> Result<(PackUri, PackUri)> {
        use crate::spreadsheet::chartsheet;
        let wb_uri = self.ensure_workbook()?;
        let mut index = 1u32;
        let sheet_uri = loop {
            let candidate = PackUri::new(format!("/xl/chartsheets/sheet{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let mut dindex = 1u32;
        let drawing_uri = loop {
            let candidate = PackUri::new(format!("/xl/drawings/drawing{dindex}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            dindex += 1;
        };
        // Placeholder drawing then chart rel
        self.package.set_part(
            drawing_uri.clone(),
            content_type::SPREADSHEET_DRAWING,
            b"<?xml version=\"1.0\"?><xdr:wsDr xmlns:xdr=\"http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing\"/>".to_vec(),
        );
        let chart_rel = self.package.add_part_relationship(
            &drawing_uri,
            rel::CHART,
            chart_uri,
            RelationshipTargetMode::Internal,
        );
        let anchor = two_cell_anchor_chart(0, 0, 10, 15, &chart_rel, name);
        let drawing_xml = write_element(&worksheet_drawing(vec![anchor]))?;
        self.package.set_part(
            drawing_uri.clone(),
            content_type::SPREADSHEET_DRAWING,
            drawing_xml,
        );
        // Chartsheet with drawing rel
        self.package.set_part(
            sheet_uri.clone(),
            content_type::SPREADSHEET_CHARTSHEET,
            b"placeholder".to_vec(),
        );
        let drawing_rel = self.package.add_part_relationship(
            &sheet_uri,
            rel::DRAWING,
            &drawing_uri,
            RelationshipTargetMode::Internal,
        );
        let cs_xml = write_element(&chartsheet(&drawing_rel))?;
        self.package.set_part(
            sheet_uri.clone(),
            content_type::SPREADSHEET_CHARTSHEET,
            cs_xml,
        );
        // Workbook relationship + sheet entry
        let sheet_rid = self.package.add_part_relationship(
            &wb_uri,
            rel::CHARTSHEET,
            &sheet_uri,
            RelationshipTargetMode::Internal,
        );
        let sheet_id = (self.sheets.len() as u32) + 1;
        // Track as worksheet-like for enumeration? Skip sheets vec for chartsheets
        let mut wb_root = parse_element(
            self.package
                .opc()
                .get_part(&wb_uri)
                .ok_or_else(|| Error::PartNotFound(wb_uri.to_string()))?,
        )?;
        if let Some(sheets_el) = wb_root.child_mut("sheets") {
            sheets_el.append_child(sheet(name, sheet_id + 1000, &sheet_rid));
        }
        let wb_xml = write_element(&wb_root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            wb_xml,
        );
        Ok((sheet_uri, drawing_uri))
    }

    /// Set an array formula spanning `array_ref`, writing the formula on `cell_ref`.
    pub fn set_array_formula(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
        formula: &str,
        array_ref: &str,
        cached: Option<&str>,
    ) -> Result<()> {
        use crate::spreadsheet::cell_array_formula;
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let sheet_data = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("worksheet has no sheetData".into()))?;
        let row_idx: u32 = cell_ref
            .bytes()
            .skip_while(|b| b.is_ascii_alphabetic())
            .map(|b| b as char)
            .collect::<String>()
            .parse()
            .unwrap_or(1);
        let cell_el = cell_array_formula(cell_ref, formula, array_ref, cached);
        let row_el = if let Some(r) = sheet_data.children.iter_mut().find(|c| {
            c.local_name == "row"
                && c.get_attribute("r").and_then(|s| s.parse().ok()) == Some(row_idx)
        }) {
            r
        } else {
            sheet_data.append_child(row(row_idx, Vec::<crate::element::OpenXmlElement>::new()));
            sheet_data.children.last_mut().unwrap()
        };
        row_el
            .children
            .retain(|c| c.get_attribute("r") != Some(cell_ref));
        row_el.append_child(cell_el);
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a sheet-local defined name.
    pub fn set_local_defined_name(
        &mut self,
        sheet_name: &str,
        name: &str,
        refers_to: &str,
    ) -> Result<()> {
        use crate::spreadsheet::defined_name_local;
        let sheet_index = self
            .sheets
            .iter()
            .position(|s| s.name == sheet_name)
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?
            as u32;
        let wb_uri = self.ensure_workbook()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&wb_uri)
                .ok_or_else(|| Error::PartNotFound(wb_uri.to_string()))?,
        )?;
        let dn = defined_name_local(name, refers_to, sheet_index);
        if let Some(container) = root.child_mut("definedNames") {
            // Remove existing with same name and localSheetId
            container.children.retain(|c| {
                !(c.get_attribute("name") == Some(name)
                    && c.get_attribute("localSheetId") == Some(&sheet_index.to_string()))
            });
            container.append_child(dn);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheets")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                crate::spreadsheet::defined_names(vec![dn]),
            );
        }
        let xml = write_element(&root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            xml,
        );
        Ok(())
    }

    /// List sheet-local defined names as `(name, refers_to, local_sheet_id)`.
    pub fn list_local_defined_names(&self) -> Result<Vec<(String, String, u32)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("definedNames") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("definedName")
            .filter_map(|el| {
                let local = el.get_attribute("localSheetId")?.parse().ok()?;
                let name = el.get_attribute("name")?.to_string();
                let refers = el.inner_text();
                Some((name, refers, local))
            })
            .collect())
    }

    /// Whether any local (sheet-scoped) defined names exist.
    pub fn has_local_defined_names(&self) -> Result<bool> {
        Ok(!self.list_local_defined_names()?.is_empty())
    }

    /// Whether a local defined name exists for a sheet.
    pub fn has_local_defined_name(&self, sheet_name: &str, name: &str) -> Result<bool> {
        let sheet_id = match self.sheets.iter().position(|s| s.name == sheet_name) {
            Some(i) => i as u32,
            None => return Ok(false),
        };
        Ok(self
            .list_local_defined_names()?
            .iter()
            .any(|(n, _, sid)| n == name && *sid == sheet_id))
    }

    pub fn remove_local_defined_name(&mut self, sheet_name: &str, name: &str) -> Result<bool> {
        let sheet_index = match self.sheets.iter().position(|s| s.name == sheet_name) {
            Some(i) => i as u32,
            None => return Ok(false),
        };
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(container) = root.child_mut("definedNames") else {
            return Ok(false);
        };
        let before = container.children.len();
        let idx_str = sheet_index.to_string();
        container.children.retain(|c| {
            !(c.get_attribute("name") == Some(name)
                && c.get_attribute("localSheetId") == Some(idx_str.as_str()))
        });
        let removed = container.children.len() < before;
        if removed {
            if container.children.is_empty() {
                root.children.retain(|c| c.local_name != "definedNames");
            }
            let xml = write_element(&root)?;
            self.package.set_part(
                wb_uri,
                self.document_type.content_type(),
                xml,
            );
        }
        Ok(removed)
    }

    /// List worksheet tab colors as `(sheet_name, rgb)` for sheets that have one.
    /// Alias for [`remove_local_defined_name`](Self::remove_local_defined_name).
    pub fn clear_local_defined_name(&mut self, sheet_name: &str, name: &str) -> Result<bool> {
        self.remove_local_defined_name(sheet_name, name)
    }

    /// Remove all local (sheet-scoped) defined names. Returns how many were removed.
    pub fn clear_local_defined_names(&mut self) -> Result<usize> {
        let entries = self.list_local_defined_names()?;
        let mut n = 0usize;
        for (name, _refers, sheet_id) in entries {
            let Some(sheet) = self.sheets.get(sheet_id as usize).map(|s| s.name.clone()) else {
                continue;
            };
            if self.remove_local_defined_name(&sheet, &name)? {
                n += 1;
            }
        }
        Ok(n)
    }


    pub fn list_tab_colors(&self) -> Result<Vec<(String, String)>> {
        let mut out = Vec::new();
        for s in &self.sheets {
            if let Ok(Some(rgb)) = self.tab_color(&s.name) {
                out.push((s.name.clone(), rgb));
            }
        }
        Ok(out)
    }

    /// Toggle "show formulas" on a sheet view.
    /// Sheet names that have a tab color set.
    pub fn sheets_with_tab_color(&self) -> Result<Vec<String>> {
        Ok(self
            .list_tab_colors()?
            .into_iter()
            .map(|(n, _)| n)
            .collect())
    }

    /// Whether any sheet is returned by [`sheets_with_tab_color`](Self::sheets_with_tab_color).
    pub fn has_sheets_with_tab_color(&self) -> Result<bool> {
        Ok(!self.sheets_with_tab_color()?.is_empty())
    }

    pub fn set_show_formulas(&mut self, sheet_name: &str, show: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("showFormulas", if show { "1" } else { "0" });
            }
        } else {
            let x = crate::namespace::ns::SPREADSHEETML.uri;
            let view = OpenXmlElement::new("x", x, "sheetView")
                .with_attribute("workbookViewId", "0")
                .with_attribute("showFormulas", if show { "1" } else { "0" });
            let views = OpenXmlElement::new("x", x, "sheetViews").with_child(view);
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }


    /// Show or hide zero values on a worksheet view (`showZeros`).
    pub fn set_show_zeros(&mut self, sheet_name: &str, show: bool) -> Result<()> {
        // reuse gridlines-like path
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("showZeros", if show { "1" } else { "0" });
            }
        } else {
            use crate::spreadsheet::sheet_views_zoom;
            let mut views = sheet_views_zoom(100);
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("showZeros", if show { "1" } else { "0" });
            }
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }


    /// Show or hide outline symbols (`showOutlineSymbols`).
    pub fn set_show_outline_symbols(&mut self, sheet_name: &str, show: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("showOutlineSymbols", if show { "1" } else { "0" });
            }
        } else {
            use crate::spreadsheet::sheet_views_zoom;
            let mut views = sheet_views_zoom(100);
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("showOutlineSymbols", if show { "1" } else { "0" });
            }
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether outline symbols are shown (default true when unset).
    pub fn show_outline_symbols(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(true),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute("showOutlineSymbols"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set sheet view type (`view` attribute), e.g. `"normal"`, `"pageBreakPreview"`, `"pageLayout"`.
    /// Disable `show outline symbols` on a sheet. Returns whether it was enabled.
    pub fn clear_show_outline_symbols(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "showOutlineSymbols")
    }

    /// Whether `showOutlineSymbols` is explicitly set.
    pub fn has_show_outline_symbols_attr(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "showOutlineSymbols")?.is_some())
    }


    pub fn set_sheet_view_type(&mut self, sheet_name: &str, view: &str) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(v) = views.child_mut("sheetView") {
                v.set_attribute("view", view);
            }
        } else {
            use crate::spreadsheet::sheet_views_zoom;
            let mut views = sheet_views_zoom(100);
            if let Some(v) = views.child_mut("sheetView") {
                v.set_attribute("view", view);
            }
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read sheet view type attribute.
    pub fn sheet_view_type(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute("view").map(|s| s.to_string())))
    }

    /// Whether zero values are shown (default true when unset).
    /// Whether sheet view type is explicitly set.
    pub fn has_sheet_view_type(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_type(sheet_name)?.is_some())
    }

    /// Clear sheet view `view` attribute.
    pub fn clear_sheet_view_type(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "view")
    }


    pub fn show_zeros(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(true),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute("showZeros"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Show or hide gridlines on a worksheet view.
    /// Disable `show zeros` on a sheet. Returns whether it was enabled.
    pub fn clear_show_zeros(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "showZeros")
    }

    /// Whether `showZeros` is explicitly set.
    pub fn has_show_zeros_attr(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "showZeros")?.is_some())
    }


    /// Clear showZeros override on every sheet. Returns sheets modified.
    pub fn clear_all_show_zeros(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_show_zeros(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }


    pub fn set_show_gridlines(&mut self, sheet_name: &str, show: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("showGridLines", if show { "1" } else { "0" });
            }
        } else {
            let x = crate::namespace::ns::SPREADSHEETML.uri;
            let view = OpenXmlElement::new("x", x, "sheetView")
                .with_attribute("workbookViewId", "0")
                .with_attribute("showGridLines", if show { "1" } else { "0" });
            let views = OpenXmlElement::new("x", x, "sheetViews").with_child(view);
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Show or hide row/column headers on a worksheet view.
    pub fn set_show_row_col_headers(&mut self, sheet_name: &str, show: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("showRowColHeaders", if show { "1" } else { "0" });
            }
        } else {
            let x = crate::namespace::ns::SPREADSHEETML.uri;
            let view = OpenXmlElement::new("x", x, "sheetView")
                .with_attribute("workbookViewId", "0")
                .with_attribute("showRowColHeaders", if show { "1" } else { "0" });
            let views = OpenXmlElement::new("x", x, "sheetViews").with_child(view);
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Set worksheet zoom scale (percent).
    pub fn set_zoom(&mut self, sheet_name: &str, zoom_scale: u32) -> Result<()> {
        use crate::spreadsheet::sheet_views_zoom;
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        // Preserve freeze panes if present by only setting zoom on existing view
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("zoomScale", zoom_scale.to_string());
                view.set_attribute("zoomScaleNormal", zoom_scale.to_string());
            }
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children
                .insert(insert_at, sheet_views_zoom(zoom_scale));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }


    /// Read sheet zoom scale from `sheetView/@zoomScale` when present.
    pub fn get_zoom(&self, sheet_name: &str) -> Result<Option<u32>> {
        self.zoom(sheet_name)
    }


    /// Whether the sheet has a `sheetViews/sheetView` element.
    pub fn has_sheet_view(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .is_some())
    }

    /// Read zoom scale from sheet views (`zoomScale`), if set.
    pub fn zoom(&self, sheet_name: &str) -> Result<Option<u32>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute("zoomScale"))
            .and_then(|s| s.parse().ok()))
    }

    /// Whether a zoom scale is set on the sheet view.
    pub fn has_zoom(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.zoom(sheet_name)?.is_some())
    }

    /// Clear zoom scale attributes from the sheet view. Returns whether zoom was set.
    pub fn clear_zoom(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = false;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                let before = view.attributes.len();
                view.attributes.retain(|a| {
                    a.local_name != "zoomScale" && a.local_name != "zoomScaleNormal"
                });
                removed = view.attributes.len() < before;
            }
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Freeze rows and/or columns on a worksheet.
    ///
    /// `freeze_cols`/`freeze_rows` are counts of frozen columns/rows.
    /// Example: freeze top row → `set_freeze_panes("S", 0, 1)`.
    /// Clear zoom settings on every sheet. Returns sheets modified.
    pub fn clear_all_zoom(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_zoom(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Sheet names that have an explicit zoom scale set.
    pub fn sheets_with_zoom(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_zoom(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    pub fn set_freeze_panes(
        &mut self,
        sheet_name: &str,
        freeze_cols: u32,
        freeze_rows: u32,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "sheetViews");
        let col_name = column_name(freeze_cols as usize + 1);
        let top_left = format!("{col_name}{}", freeze_rows + 1);
        let views = freeze_panes_views(freeze_cols as f64, freeze_rows as f64, &top_left);
        // sheetViews should come before sheetData
        let insert_at = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
            .unwrap_or(0);
        root.children.insert(insert_at, views);
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether the sheet has freeze panes (`pane` under `sheetViews`).
    pub fn has_freeze_panes(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.freeze_panes(sheet_name)?.is_some())
    }

    /// Read freeze pane splits as `(freeze_cols, freeze_rows)` when present.
    /// Sheet names that have freeze panes configured.
    pub fn sheets_with_freeze_panes(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_freeze_panes(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has freeze panes.
    pub fn has_sheets_with_freeze_panes(&self) -> Result<bool> {
        Ok(!self.sheets_with_freeze_panes()?.is_empty())
    }

    pub fn freeze_panes(&self, sheet_name: &str) -> Result<Option<(u32, u32)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(pane) = root.descendants().find(|e| e.local_name == "pane") else {
            return Ok(None);
        };
        let cols = pane
            .get_attribute("xSplit")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) as u32;
        let rows = pane
            .get_attribute("ySplit")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) as u32;
        Ok(Some((cols, rows)))
    }

    /// Remove freeze panes (`pane` elements) from the sheet views.
    ///
    /// Returns whether a pane was present and removed.
    pub fn clear_freeze_panes(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let had = root.descendants().any(|e| e.local_name == "pane");
        if !had {
            return Ok(false);
        }
        if let Some(views) = root.child_mut("sheetViews") {
            for view in views
                .children
                .iter_mut()
                .filter(|c| c.local_name == "sheetView")
            {
                view.children.retain(|c| c.local_name != "pane");
            }
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Clear freeze panes on every sheet. Returns sheets modified.
    pub fn clear_all_freeze_panes(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_freeze_panes(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Read freeze pane details as `(x_split, y_split, top_left_cell, active_pane, state)`.
    pub fn freeze_pane_details(
        &self,
        sheet_name: &str,
    ) -> Result<Option<(f64, f64, String, String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(pane) = root.descendants().find(|e| e.local_name == "pane") else {
            return Ok(None);
        };
        Ok(Some((
            pane.get_attribute("xSplit")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0),
            pane.get_attribute("ySplit")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0),
            pane.get_attribute("topLeftCell").unwrap_or("").to_string(),
            pane.get_attribute("activePane")
                .unwrap_or("bottomRight")
                .to_string(),
            pane.get_attribute("state").unwrap_or("frozen").to_string(),
        )))
    }

    /// Set freeze panes with explicit top-left cell / active pane / state.
    pub fn set_freeze_panes_ex(
        &mut self,
        sheet_name: &str,
        x_split: f64,
        y_split: f64,
        top_left_cell: &str,
        active_pane: &str,
        state: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("sheetViews").is_none() {
            let view =
                OpenXmlElement::new("x", x, "sheetView").with_attribute("workbookViewId", "0");
            let views = OpenXmlElement::new("x", x, "sheetViews").with_child(view);
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        if let Some(views) = root.child_mut("sheetViews") {
            if views.child("sheetView").is_none() {
                views.append_child(
                    OpenXmlElement::new("x", x, "sheetView").with_attribute("workbookViewId", "0"),
                );
            }
            if let Some(view) = views.child_mut("sheetView") {
                view.children.retain(|c| c.local_name != "pane");
                let mut pane = OpenXmlElement::new("x", x, "pane")
                    .with_attribute("topLeftCell", top_left_cell)
                    .with_attribute("activePane", active_pane)
                    .with_attribute("state", state);
                if x_split > 0.0 {
                    pane.set_attribute("xSplit", x_split.to_string());
                }
                if y_split > 0.0 {
                    pane.set_attribute("ySplit", y_split.to_string());
                }
                view.append_child(pane);
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Clear freeze panes (alias for [`clear_freeze_panes`](Self::clear_freeze_panes)).
    pub fn clear_freeze_panes_ex(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_freeze_panes(sheet_name)
    }

    /// Set the active cell selection on the sheet view (`selection/@activeCell` and `@sqref`).
    pub fn set_active_cell(&mut self, sheet_name: &str, cell_ref: &str) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        // Ensure sheetViews/sheetView
        if root.child("sheetViews").is_none() {
            let view = OpenXmlElement::new("x", x, "sheetView").with_attribute("workbookViewId", "0");
            let views = OpenXmlElement::new("x", x, "sheetViews").with_child(view);
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        if let Some(views) = root.child_mut("sheetViews") {
            if views.child("sheetView").is_none() {
                views.append_child(
                    OpenXmlElement::new("x", x, "sheetView").with_attribute("workbookViewId", "0"),
                );
            }
            if let Some(view) = views.child_mut("sheetView") {
                view.children.retain(|c| c.local_name != "selection");
                view.append_child(
                    OpenXmlElement::new("x", x, "selection")
                        .with_attribute("activeCell", cell_ref)
                        .with_attribute("sqref", cell_ref),
                );
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read active cell from sheet view selection.
    pub fn active_cell(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.child("selection"))
            .and_then(|s| s.get_attribute("activeCell").map(|x| x.to_string())))
    }

    /// Read selection `sqref` when present.
    pub fn selection_sqref(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.child("selection"))
            .and_then(|s| s.get_attribute("sqref").map(|x| x.to_string())))
    }

    /// Whether a selection element is present.
    pub fn has_selection(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.active_cell(sheet_name)?.is_some())
    }

    /// Clear selection from the sheet view. Returns whether present.
    pub fn clear_selection(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = false;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                let before = view.children.len();
                view.children.retain(|c| c.local_name != "selection");
                removed = view.children.len() < before;
            }
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Set selection `sqref` while keeping/creating activeCell.
    pub fn set_selection_sqref(
        &mut self,
        sheet_name: &str,
        active_cell: &str,
        sqref: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("sheetViews").is_none() {
            let view =
                OpenXmlElement::new("x", x, "sheetView").with_attribute("workbookViewId", "0");
            let views = OpenXmlElement::new("x", x, "sheetViews").with_child(view);
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        if let Some(views) = root.child_mut("sheetViews") {
            if views.child("sheetView").is_none() {
                views.append_child(
                    OpenXmlElement::new("x", x, "sheetView").with_attribute("workbookViewId", "0"),
                );
            }
            if let Some(view) = views.child_mut("sheetView") {
                view.children.retain(|c| c.local_name != "selection");
                view.append_child(
                    OpenXmlElement::new("x", x, "selection")
                        .with_attribute("activeCell", active_cell)
                        .with_attribute("sqref", sqref),
                );
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read the active tab index from workbook views (`activeTab`), if set.
    pub fn active_tab(&self) -> Result<Option<u32>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(views) = root.child("bookViews") else {
            return Ok(None);
        };
        let Some(view) = views.child("workbookView") else {
            return Ok(None);
        };
        Ok(view
            .get_attribute("activeTab")
            .and_then(|s| s.parse().ok()))
    }


    /// Whether workbook views (`bookViews/workbookView`) exist.
    pub fn has_workbook_view(&self) -> Result<bool> {
        Ok(self.active_tab()?.is_some() || self.workbook_view_count()? > 0)
    }

    /// Count `workbookView` elements.
    pub fn workbook_view_count(&self) -> Result<usize> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("bookViews")
            .map(|v| v.children_by_name("workbookView").count())
            .unwrap_or(0))
    }

    /// Clear all `bookViews/workbookView` entries. Returns how many were removed.
    pub fn clear_workbook_views(&mut self) -> Result<usize> {
        let n = self.workbook_view_count()?;
        if n == 0 {
            return Ok(0);
        }
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        root.children.retain(|c| c.local_name != "bookViews");
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(n)
    }


    /// Whether an active tab index is set in workbook views.
    /// Number of `sheetView` elements on a worksheet.
    pub fn sheet_view_count(&self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(0),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .map(|sv| sv.children_by_name("sheetView").count())
            .unwrap_or(0))
    }

    pub fn has_active_tab(&self) -> Result<bool> {
        Ok(self.active_tab()?.is_some())
    }

    /// Clear the active tab attribute from workbook views. Returns whether it was set.
    pub fn clear_active_tab(&mut self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut removed = false;
        if let Some(views) = root.child_mut("bookViews") {
            if let Some(view) = views.child_mut("workbookView") {
                let before = view.attributes.len();
                view.attributes.retain(|a| a.local_name != "activeTab");
                removed = view.attributes.len() < before;
            }
        }
        if removed {
            let xml = write_element(&root)?;
            self.package.set_part(
                wb_uri,
                self.document_type.content_type(),
                xml,
            );
        }
        Ok(removed)
    }

    /// Add manual row page breaks at 0-based row indices.
    pub fn set_row_breaks(&mut self, sheet_name: &str, row_ids: &[u32]) -> Result<()> {
        use crate::spreadsheet::{row_break, row_breaks};
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "rowBreaks");
        if !row_ids.is_empty() {
            let brks: Vec<_> = row_ids.iter().map(|id| row_break(*id, None, true)).collect();
            root.append_child(row_breaks(brks));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add manual column page breaks at 0-based column indices.
    pub fn set_col_breaks(&mut self, sheet_name: &str, col_ids: &[u32]) -> Result<()> {
        use crate::spreadsheet::{col_breaks, row_break};
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "colBreaks");
        if !col_ids.is_empty() {
            let brks: Vec<_> = col_ids.iter().map(|id| row_break(*id, None, true)).collect();
            root.append_child(col_breaks(brks));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read manual row page-break ids.
    pub fn row_breaks(&self, sheet_name: &str) -> Result<Vec<u32>> {
        self.break_ids(sheet_name, "rowBreaks")
    }

    /// Read manual column page-break ids.
    pub fn col_breaks(&self, sheet_name: &str) -> Result<Vec<u32>> {
        self.break_ids(sheet_name, "colBreaks")
    }

    /// Number of manual row page breaks on a sheet.
    pub fn row_break_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.row_breaks(sheet_name)?.len())
    }

    /// Number of manual column page breaks on a sheet.
    pub fn col_break_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.col_breaks(sheet_name)?.len())
    }

    /// Whether the sheet has any row page breaks.
    pub fn has_row_breaks(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.row_break_count(sheet_name)? > 0)
    }


    /// Whether the sheet has any manual row or column page breaks.
    pub fn has_page_breaks(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.has_row_breaks(sheet_name)? || self.has_col_breaks(sheet_name)?)
    }

    /// List all page breaks as `(kind, id)` where kind is `"row"` or `"col"`.
    pub fn list_page_breaks(&self, sheet_name: &str) -> Result<Vec<(String, u32)>> {
        let mut out = Vec::new();
        for id in self.row_breaks(sheet_name)? {
            out.push(("row".into(), id));
        }
        for id in self.col_breaks(sheet_name)? {
            out.push(("col".into(), id));
        }
        Ok(out)
    }

    /// Clear all manual row and column page breaks on a sheet.
    pub fn clear_page_breaks(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.has_page_breaks(sheet_name)?;
        if !had {
            return Ok(false);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "rowBreaks" && c.local_name != "colBreaks");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Whether the sheet has any column page breaks.
    pub fn has_col_breaks(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.col_break_count(sheet_name)? > 0)
    }

    /// Clear all manual row page breaks.
    pub fn clear_row_breaks(&mut self, sheet_name: &str) -> Result<bool> {
        let had = !self.row_breaks(sheet_name)?.is_empty();
        if had {
            self.set_row_breaks(sheet_name, &[])?;
        }
        Ok(had)
    }

    /// Clear all manual column page breaks.
    pub fn clear_col_breaks(&mut self, sheet_name: &str) -> Result<bool> {
        let had = !self.col_breaks(sheet_name)?.is_empty();
        if had {
            self.set_col_breaks(sheet_name, &[])?;
        }
        Ok(had)
    }

    fn break_ids(&self, sheet_name: &str, container: &str) -> Result<Vec<u32>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(brks) = root.child(container) else {
            return Ok(Vec::new());
        };
        Ok(brks
            .children_by_name("brk")
            .filter_map(|b| b.get_attribute("id")?.parse().ok())
            .collect())
    }

    /// List formula cells as `(reference, formula)` pairs.
    pub fn list_formulas(&self, sheet_name: &str) -> Result<Vec<(String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "c")
            .filter_map(|c| {
                let f = c.child("f")?;
                let reference = c.get_attribute("r")?.to_string();
                let formula = f.inner_text();
                if formula.is_empty() {
                    return None;
                }
                Some((reference, formula))
            })
            .collect())
    }

    /// Number of formula cells on a sheet.
    /// Remove formula elements from all cells on a sheet, keeping cached values when present.
    /// Returns how many formula cells were cleared.
    pub fn clear_formulas(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut n = 0usize;
        if let Some(sd) = root.child_mut("sheetData") {
            for row in sd.children.iter_mut() {
                if row.local_name != "row" {
                    continue;
                }
                for cell in row.children.iter_mut() {
                    if cell.local_name != "c" {
                        continue;
                    }
                    let before = cell.children.len();
                    cell.children.retain(|c| c.local_name != "f");
                    if cell.children.len() < before {
                        n += 1;
                    }
                }
            }
        }
        if n > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    /// Clear formulas on every sheet. Returns sheets modified.
    pub fn clear_all_formulas(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_formulas(&name)? > 0 {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn formula_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_formulas(sheet_name)?.len())
    }

    /// Whether the sheet has any formula cells.
    pub fn has_formulas(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.formula_count(sheet_name)? > 0)
    }

    /// Number of cell hyperlinks on a sheet.
    pub fn cell_hyperlink_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_cell_hyperlinks(sheet_name)?.len())
    }

    /// Whether the sheet has any cell hyperlinks.
    pub fn has_cell_hyperlinks(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.cell_hyperlink_count(sheet_name)? > 0)
    }

    /// Add an external hyperlink on a cell range.
    pub fn add_cell_hyperlink(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
        url: &str,
        display: Option<&str>,
    ) -> Result<String> {
        use crate::spreadsheet::{sheet_hyperlink, sheet_hyperlinks};
        let sheet_uri = self.sheet_uri(sheet_name)?;
        // External relationship from the worksheet
        let rid = self.package.add_external_relationship(
            Some(&sheet_uri),
            rel::HYPERLINK,
            url,
        );
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let link = sheet_hyperlink(cell_ref, &rid, display);
        if let Some(hl) = root.child_mut("hyperlinks") {
            hl.append_child(link);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "pageMargins")
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, sheet_hyperlinks(vec![link]));
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(rid)
    }

    /// List worksheet hyperlinks as `(cell_ref, relationship_id, display?)`.
    pub fn list_cell_hyperlinks(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String, Option<String>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(hl) = root.child("hyperlinks") else {
            return Ok(Vec::new());
        };
        Ok(hl
            .children_by_name("hyperlink")
            .filter_map(|h| {
                let r = h.get_attribute("ref")?.to_string();
                let id = h
                    .get_attribute("id")
                    .or_else(|| h.get_attribute_qname("r:id"))
                    .or_else(|| {
                        h.attributes
                            .iter()
                            .find(|a| a.local_name == "id")
                            .map(|a| a.value.as_str())
                    })?
                    .to_string();
                let display = h.get_attribute("display").map(|s| s.to_string());
                Some((r, id, display))
            })
            .collect())
    }

    /// Add a cell hyperlink with optional tooltip.
    pub fn add_cell_hyperlink_with_tooltip(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
        url: &str,
        display: Option<&str>,
        tooltip: Option<&str>,
    ) -> Result<String> {
        use crate::spreadsheet::{sheet_hyperlink_ex, sheet_hyperlinks};
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let rid = self
            .package
            .add_hyperlink_relationship(&sheet_uri, url, true);
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let link = sheet_hyperlink_ex(cell_ref, Some(&rid), display, tooltip, None);
        if let Some(hl) = root.child_mut("hyperlinks") {
            hl.append_child(link);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "pageMargins")
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, sheet_hyperlinks(vec![link]));
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(rid)
    }

    /// Add an in-sheet location hyperlink (no external relationship), e.g. location `"Sheet2!A1"`.
    pub fn add_cell_location_hyperlink(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
        location: &str,
        display: Option<&str>,
        tooltip: Option<&str>,
    ) -> Result<()> {
        use crate::spreadsheet::{sheet_hyperlink_ex, sheet_hyperlinks};
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let link = sheet_hyperlink_ex(cell_ref, None, display, tooltip, Some(location));
        if let Some(hl) = root.child_mut("hyperlinks") {
            hl.append_child(link);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "pageMargins")
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, sheet_hyperlinks(vec![link]));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List location-style hyperlinks as `(cell_ref, location, display?, tooltip?)`.
    pub fn list_location_hyperlinks(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String, Option<String>, Option<String>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(hl) = root.child("hyperlinks") else {
            return Ok(Vec::new());
        };
        Ok(hl
            .children_by_name("hyperlink")
            .filter_map(|h| {
                let loc = h.get_attribute("location")?.to_string();
                let r = h.get_attribute("ref")?.to_string();
                let display = h.get_attribute("display").map(|s| s.to_string());
                let tooltip = h.get_attribute("tooltip").map(|s| s.to_string());
                Some((r, loc, display, tooltip))
            })
            .collect())
    }

    /// Read tooltip for a hyperlink on a cell ref, if present.
    /// Whether a sheet has any location (internal) hyperlinks.
    pub fn has_location_hyperlinks(&self, sheet_name: &str) -> Result<bool> {
        Ok(!self.list_location_hyperlinks(sheet_name)?.is_empty())
    }

    /// Count location hyperlinks on a sheet.
    pub fn location_hyperlink_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_location_hyperlinks(sheet_name)?.len())
    }

    pub fn cell_hyperlink_tooltip(
        &self,
        sheet_name: &str,
        cell_ref: &str,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(hl) = root.child("hyperlinks") else {
            return Ok(None);
        };
        for h in hl.children_by_name("hyperlink") {
            if h.get_attribute("ref") == Some(cell_ref) {
                return Ok(h.get_attribute("tooltip").map(|s| s.to_string()));
            }
        }
        Ok(None)
    }

    /// Whether any worksheet hyperlink entries exist across sheets.
    pub fn has_hyperlinks(&self) -> bool {
        self.sheets.iter().any(|s| {
            self.list_cell_hyperlinks(&s.name)
                .map(|v| !v.is_empty())
                .unwrap_or(false)
        })
    }

    /// Total hyperlink entries across all worksheets.
    pub fn hyperlink_count(&self) -> Result<usize> {
        let mut n = 0;
        for s in &self.sheets {
            n += self.list_cell_hyperlinks(&s.name)?.len();
        }
        Ok(n)
    }

    /// List hyperlinks across all sheets as `(sheet, cell_ref, rel_id, display)`.
    pub fn list_hyperlinks(&self) -> Result<Vec<(String, String, String, Option<String>)>> {
        let mut out = Vec::new();
        for s in &self.sheets.clone() {
            for (cell, id, display) in self.list_cell_hyperlinks(&s.name)? {
                out.push((s.name.clone(), cell, id, display));
            }
        }
        Ok(out)
    }


    /// Alias for [`list_hyperlinks`](Self::list_hyperlinks) (Word-compatible name).
    pub fn list_external_hyperlinks(
        &self,
    ) -> Result<Vec<(String, String, String, Option<String>)>> {
        self.list_hyperlinks()
    }

    /// Alias for [`clear_hyperlinks`](Self::clear_hyperlinks).
    pub fn clear_external_hyperlinks(&mut self) -> Result<usize> {
        self.clear_hyperlinks()
    }

    /// Remove all hyperlink elements from all worksheets (relationships left in place).
    pub fn clear_hyperlinks(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheets.iter().map(|s| s.name.clone()).collect();
        let mut n = 0;
        for name in names {
            let sheet_uri = self.sheet_uri(&name)?;
            let mut root = self.load_sheet_root(&sheet_uri)?;
            let Some(hl) = root.child_mut("hyperlinks") else {
                continue;
            };
            let count = hl.children_by_name("hyperlink").count();
            if count == 0 {
                continue;
            }
            n += count;
            root.children.retain(|c| c.local_name != "hyperlinks");
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    /// Remove a hyperlink entry for `cell_ref` from the worksheet (relationship left in place).
    pub fn remove_cell_hyperlink(&mut self, sheet_name: &str, cell_ref: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(hl) = root.child_mut("hyperlinks") else {
            return Ok(false);
        };
        let before = hl.children.len();
        hl.children.retain(|c| {
            !(c.local_name == "hyperlink" && c.get_attribute("ref") == Some(cell_ref))
        });
        let removed = hl.children.len() < before;
        if hl.children.is_empty() {
            root.children.retain(|c| c.local_name != "hyperlinks");
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Update display / tooltip / location on a cell hyperlink identified by `ref`.
    pub fn set_cell_hyperlink_attrs(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
        display: Option<&str>,
        tooltip: Option<&str>,
        location: Option<&str>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(hl) = root.child_mut("hyperlinks") else {
            return Ok(false);
        };
        let mut found = false;
        for h in hl
            .children
            .iter_mut()
            .filter(|c| c.local_name == "hyperlink")
        {
            if h.get_attribute("ref") != Some(cell_ref) {
                continue;
            }
            found = true;
            if let Some(d) = display {
                h.set_attribute("display", d);
            }
            if let Some(t) = tooltip {
                h.set_attribute("tooltip", t);
            }
            if let Some(l) = location {
                h.set_attribute("location", l);
            }
            break;
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Read full hyperlink details for a cell as `(rel_id?, location?, display?, tooltip?)`.
    pub fn cell_hyperlink_details(
        &self,
        sheet_name: &str,
        cell_ref: &str,
    ) -> Result<Option<(Option<String>, Option<String>, Option<String>, Option<String>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(hl) = root.child("hyperlinks") else {
            return Ok(None);
        };
        for h in hl.children_by_name("hyperlink") {
            if h.get_attribute("ref") != Some(cell_ref) {
                continue;
            }
            let rid = h
                .get_attribute("id")
                .or_else(|| h.get_attribute_qname("r:id"))
                .map(|s| s.to_string());
            return Ok(Some((
                rid,
                h.get_attribute("location").map(|s| s.to_string()),
                h.get_attribute("display").map(|s| s.to_string()),
                h.get_attribute("tooltip").map(|s| s.to_string()),
            )));
        }
        Ok(None)
    }

    /// Remove all cell hyperlinks from a worksheet. Returns how many were removed.
    pub fn clear_cell_hyperlinks(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(hl) = root.child("hyperlinks") else {
            return Ok(0);
        };
        let n = hl.children_by_name("hyperlink").count();
        if n == 0 {
            return Ok(0);
        }
        root.children.retain(|c| c.local_name != "hyperlinks");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Number of shared strings currently in the in-memory SST builder (0 if none).
    /// Clear cell hyperlinks on every sheet. Returns sheets modified.
    pub fn clear_all_cell_hyperlinks(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_cell_hyperlinks(&name)? > 0 {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shared_string_count(&self) -> usize {
        self.sst.as_ref().map(|s| s.len()).unwrap_or(0)
    }

    /// Shared string table entries currently held in memory (empty if none loaded).
    pub fn shared_strings_list(&self) -> Vec<String> {
        self.sst
            .as_ref()
            .map(|s| s.strings().to_vec())
            .unwrap_or_default()
    }

    /// Whether this package is a macro-enabled workbook type.
    pub fn is_macro_enabled(&self) -> bool {
        matches!(
            self.document_type,
            SpreadsheetDocumentType::MacroEnabledWorkbook
                | SpreadsheetDocumentType::MacroEnabledTemplate
        )
    }

    /// Number of parts in the underlying OPC package.
    pub fn part_count(&self) -> usize {
        self.package.opc().part_uris().len()
    }

    /// Whether a stylesheet part is present.
    pub fn has_styles(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/xl/styles.xml"))
    }


    /// Count styles-related parts (`styles.xml` etc.).
    pub fn styles_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.contains("/xl/styles") || s.ends_with("/styles.xml")
            })
            .count()
    }


    /// List zero-based cell style indices present in `cellXfs` (as strings).
    pub fn list_style_ids(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let n = root
            .child("cellXfs")
            .map(|c| c.children_by_name("xf").count())
            .unwrap_or(0);
        Ok((0..n).map(|i| i.to_string()).collect())
    }

    /// Alias for number of `cellXfs` entries when styles part exists.
    /// Whether the stylesheet has any cell format records (xf).
    pub fn has_style_ids(&self) -> Result<bool> {
        Ok(!self.list_style_ids()?.is_empty())
    }

    /// Count cell format style ids (same as number of xf entries).
    pub fn style_id_count(&self) -> Result<usize> {
        Ok(self.list_style_ids()?.len())
    }

    pub fn style_count(&self) -> Result<usize> {
        Ok(self.list_style_ids()?.len())
    }

    /// List named cell styles as `(name, xfId)` from the stylesheet `cellStyles` section.
    pub fn list_named_styles(&self) -> Result<Vec<(String, u32)>> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(styles) = root.child("cellStyles") else {
            return Ok(Vec::new());
        };
        Ok(styles
            .children_by_name("cellStyle")
            .map(|s| {
                let name = s.get_attribute("name").unwrap_or("").to_string();
                let xf = s
                    .get_attribute("xfId")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0);
                (name, xf)
            })
            .collect())
    }

    /// List custom number formats as `(numFmtId, formatCode)` from the stylesheet.
    pub fn list_number_formats(&self) -> Result<Vec<(u32, String)>> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(fmts) = root.child("numFmts") else {
            return Ok(Vec::new());
        };
        Ok(fmts
            .children_by_name("numFmt")
            .filter_map(|f| {
                let id = f.get_attribute("numFmtId")?.parse().ok()?;
                let code = f.get_attribute("formatCode")?.to_string();
                Some((id, code))
            })
            .collect())
    }

    /// List font names declared in the stylesheet `fonts` section.
    /// Whether a custom number format id exists in the stylesheet.
    pub fn has_number_format(&self, num_fmt_id: u32) -> Result<bool> {
        Ok(self
            .list_number_formats()?
            .iter()
            .any(|(id, _)| *id == num_fmt_id))
    }

    /// Add or replace a custom number format entry.
    pub fn set_number_format(&mut self, num_fmt_id: u32, format_code: &str) -> Result<()> {
        let uri = PackUri::new("/xl/styles.xml");
        // Ensure styles part exists
        if !self.package.opc().has_part(&uri) {
            let _ = self.add_default_styles()?;
        }
        let data = self
            .package
            .opc()
            .get_part(&uri)
            .ok_or_else(|| Error::PartNotFound(uri.to_string()))?;
        let mut root = parse_element(data)?;
        if root.child("numFmts").is_none() {
            let x = crate::namespace::ns::SPREADSHEETML.uri;
            // insert near start
            root.children.insert(
                0,
                OpenXmlElement::new("x", x, "numFmts").with_attribute("count", "0"),
            );
        }
        let fmts = root.child_mut("numFmts").unwrap();
        let mut found = false;
        for f in fmts.children.iter_mut().filter(|c| c.local_name == "numFmt") {
            if f.get_attribute("numFmtId").and_then(|s| s.parse().ok()) == Some(num_fmt_id) {
                f.set_attribute("formatCode", format_code);
                found = true;
                break;
            }
        }
        if !found {
            let x = crate::namespace::ns::SPREADSHEETML.uri;
            fmts.append_child(
                OpenXmlElement::new("x", x, "numFmt")
                    .with_attribute("numFmtId", num_fmt_id.to_string())
                    .with_attribute("formatCode", format_code),
            );
        }
        let count = fmts
            .children
            .iter()
            .filter(|c| c.local_name == "numFmt")
            .count();
        fmts.set_attribute("count", count.to_string());
        self.package.set_part(
            uri,
            content_type::SPREADSHEET_STYLES,
            write_element(&root)?,
        );
        Ok(())
    }

    /// List font names declared in the stylesheet `fonts` section.
    pub fn list_style_fonts(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(fonts) = root.child("fonts") else {
            return Ok(Vec::new());
        };
        Ok(fonts
            .children_by_name("font")
            .filter_map(|f| {
                f.child("name")
                    .and_then(|n| n.get_attribute("val"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Whether a font name is declared in the stylesheet.
    pub fn has_style_font(&self, name: &str) -> Result<bool> {
        Ok(self.list_style_fonts()?.iter().any(|n| n == name))
    }

    /// Remove stylesheet `font` entries whose name matches. Returns how many were removed.
    ///
    /// Does not renumber cellXfs references; callers that share font indexes should rebuild styles.
    pub fn remove_style_font(&mut self, name: &str) -> Result<usize> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        let Some(fonts) = root.child_mut("fonts") else {
            return Ok(0);
        };
        let before = fonts.children.len();
        fonts.children.retain(|f| {
            if f.local_name != "font" {
                return true;
            }
            let n = f
                .child("name")
                .and_then(|el| el.get_attribute("val"))
                .unwrap_or("");
            n != name
        });
        let removed = before - fonts.children.len();
        if removed == 0 {
            return Ok(0);
        }
        fonts.set_attribute("count", fonts.children_by_name("font").count().to_string());
        let xml = write_element(&root)?;
        self.package
            .set_part(uri, content_type::SPREADSHEET_STYLES, xml);
        Ok(removed)
    }

    /// Alias for [`remove_style_font`](Self::remove_style_font).
    pub fn clear_style_font(&mut self, name: &str) -> Result<usize> {
        self.remove_style_font(name)
    }

    /// List fill pattern types (and optional RGB) from the stylesheet `fills` section.
    pub fn list_fills(&self) -> Result<Vec<(String, Option<String>)>> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(fills) = root.child("fills") else {
            return Ok(Vec::new());
        };
        Ok(fills
            .children_by_name("fill")
            .map(|f| {
                let pattern = f
                    .child("patternFill")
                    .and_then(|p| p.get_attribute("patternType"))
                    .unwrap_or("")
                    .to_string();
                let rgb = f
                    .descendants()
                    .find(|e| e.local_name == "fgColor" || e.local_name == "rgb")
                    .and_then(|e| e.get_attribute("rgb").map(|s| s.to_string()));
                (pattern, rgb)
            })
            .collect())
    }

    /// Number of named cell styles in the stylesheet.
    pub fn named_style_count(&self) -> Result<usize> {
        Ok(self.list_named_styles()?.len())
    }

    /// Whether any named cell styles exist in the stylesheet.
    pub fn has_named_styles(&self) -> Result<bool> {
        Ok(self.named_style_count()? > 0)
    }

    /// Whether a named cell style exists.
    pub fn has_named_style(&self, name: &str) -> Result<bool> {
        Ok(self.list_named_styles()?.iter().any(|(n, _)| n == name))
    }

    /// Rename a named cell style. Returns whether found.
    pub fn rename_named_style(&mut self, old_name: &str, new_name: &str) -> Result<bool> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(styles) = root.child_mut("cellStyles") else {
            return Ok(false);
        };
        let mut found = false;
        for s in styles.children.iter_mut().filter(|c| c.local_name == "cellStyle") {
            if s.get_attribute("name") == Some(old_name) {
                s.set_attribute("name", new_name);
                found = true;
                break;
            }
        }
        if found {
            self.package.set_part(
                uri,
                content_type::SPREADSHEET_STYLES,
                write_element(&root)?,
            );
        }
        Ok(found)
    }

    /// Remove a named cell style by name. Returns whether found.
    pub fn remove_named_style(&mut self, name: &str) -> Result<bool> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(styles) = root.child_mut("cellStyles") else {
            return Ok(false);
        };
        let before = styles.children.len();
        styles.children.retain(|c| {
            !(c.local_name == "cellStyle" && c.get_attribute("name") == Some(name))
        });
        let removed = styles.children.len() < before;
        if removed {
            if let Some(cs) = root.child_mut("cellStyles") {
                cs.set_attribute("count", cs.children.len().to_string());
            }
            self.package.set_part(
                uri,
                content_type::SPREADSHEET_STYLES,
                write_element(&root)?,
            );
        }
        Ok(removed)
    }

    /// Number of custom number formats in the stylesheet.
    /// Alias for [`remove_named_style`](Self::remove_named_style).
    pub fn clear_named_style(&mut self, name: &str) -> Result<bool> {
        self.remove_named_style(name)
    }

    /// Remove all named cell styles (`cellStyles`). Returns how many were removed.
    pub fn clear_named_styles(&mut self) -> Result<usize> {
        let names: Vec<String> = self
            .list_named_styles()?
            .into_iter()
            .map(|(n, _)| n)
            .collect();
        let mut n = 0usize;
        for name in names {
            if self.remove_named_style(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }


    pub fn number_format_count(&self) -> Result<usize> {
        Ok(self.list_number_formats()?.len())
    }

    /// Remove a custom number format by numFmtId. Returns whether found.
    pub fn remove_number_format(&mut self, num_fmt_id: u32) -> Result<bool> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(fmts) = root.child_mut("numFmts") else {
            return Ok(false);
        };
        let before = fmts.children.len();
        fmts.children.retain(|c| {
            !(c.local_name == "numFmt"
                && c.get_attribute("numFmtId").and_then(|s| s.parse().ok()) == Some(num_fmt_id))
        });
        let removed = fmts.children.len() < before;
        if removed {
            fmts.set_attribute("count", fmts.children.len().to_string());
            self.package.set_part(
                uri,
                content_type::SPREADSHEET_STYLES,
                write_element(&root)?,
            );
        }
        Ok(removed)
    }

    /// Update formatCode for an existing numFmtId.
    /// Alias for [`remove_number_format`](Self::remove_number_format).
    pub fn clear_number_format(&mut self, num_fmt_id: u32) -> Result<bool> {
        self.remove_number_format(num_fmt_id)
    }

    pub fn set_number_format_code(&mut self, num_fmt_id: u32, format_code: &str) -> Result<bool> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(fmts) = root.child_mut("numFmts") else {
            return Ok(false);
        };
        let mut found = false;
        for f in fmts.children.iter_mut().filter(|c| c.local_name == "numFmt") {
            if f.get_attribute("numFmtId").and_then(|s| s.parse().ok()) == Some(num_fmt_id) {
                f.set_attribute("formatCode", format_code);
                found = true;
                break;
            }
        }
        if found {
            self.package.set_part(
                uri,
                content_type::SPREADSHEET_STYLES,
                write_element(&root)?,
            );
        }
        Ok(found)
    }

    /// Clear/remove a number format by id (alias for [`clear_number_format`](Self::clear_number_format)).
    pub fn clear_number_format_code(&mut self, num_fmt_id: u32) -> Result<bool> {
        self.clear_number_format(num_fmt_id)
    }

    /// Number of font records in the stylesheet.
    pub fn style_font_count(&self) -> Result<usize> {
        Ok(self.list_style_fonts()?.len())
    }

    /// Number of fill records in the stylesheet.
    pub fn fill_count(&self) -> Result<usize> {
        Ok(self.list_fills()?.len())
    }

    /// Alias for [`fill_count`](Self::fill_count).
    pub fn fills_count(&self) -> Result<usize> {
        self.fill_count()
    }

    /// Whether any fills exist in the stylesheet.
    pub fn has_fills(&self) -> Result<bool> {
        Ok(self.fill_count()? > 0)
    }

    /// Count border definitions in the stylesheet (including the empty default).
    pub fn border_count(&self) -> Result<usize> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("borders")
            .map(|b| b.children_by_name("border").count())
            .unwrap_or(0))
    }

    /// Whether the stylesheet defines any borders.
    pub fn has_borders(&self) -> Result<bool> {
        Ok(self.border_count()? > 0)
    }

    /// List border style names from stylesheet `borders` (or empty string when absent).
    pub fn list_borders(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(borders) = root.child("borders") else {
            return Ok(Vec::new());
        };
        Ok(borders
            .children_by_name("border")
            .map(|b| {
                // summarize left style if present
                b.child("left")
                    .and_then(|l| l.get_attribute("style"))
                    .unwrap_or("")
                    .to_string()
            })
            .collect())
    }


    /// Count differential formatting records (`dxfs/dxf`) used by conditional formatting.
    pub fn dxf_count(&self) -> Result<usize> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("dxfs")
            .map(|d| d.children_by_name("dxf").count())
            .unwrap_or(0))
    }

    /// Whether any differential formats exist.
    pub fn has_dxfs(&self) -> Result<bool> {
        Ok(self.dxf_count()? > 0)
    }

    /// List dxf summary strings (fill RGB or font name when present).
    pub fn list_dxfs(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/xl/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(dxfs) = root.child("dxfs") else {
            return Ok(Vec::new());
        };
        Ok(dxfs
            .children_by_name("dxf")
            .map(|d| {
                if let Some(rgb) = d
                    .descendants()
                    .find(|e| e.local_name == "fgColor" || e.local_name == "rgb")
                    .and_then(|e| e.get_attribute("rgb"))
                {
                    return format!("fill:{rgb}");
                }
                if let Some(name) = d
                    .descendants()
                    .find(|e| e.local_name == "name")
                    .and_then(|e| e.get_attribute("val"))
                {
                    return format!("font:{name}");
                }
                String::new()
            })
            .collect())
    }


    /// Remove the stylesheet part and workbook relationship.
    pub fn clear_styles(&mut self) -> Result<bool> {
        let uri = PackUri::new("/xl/styles.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::STYLES)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Whether a theme part is present.
    pub fn has_theme(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/xl/theme/"))
    }

    /// Remove theme parts and workbook theme relationships.
    pub fn clear_theme(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/xl/theme/"))
            
            .collect();
        if uris.is_empty() {
            return Ok(false);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::THEME)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(true)
    }

    /// Ensure a minimal stylesheet exists (without bold).
    pub fn ensure_styles(&mut self) -> Result<String> {
        if self.has_styles() {
            let wb_uri = PackUri::new(WORKBOOK_URI);
            if let Some(rid) = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .and_then(|r| r.get_by_type(rel::STYLES).map(|x| x.id.clone()))
            {
                return Ok(rid);
            }
        }
        self.add_minimal_styles(false)
    }


    /// Alias for [`ensure_styles`](Self::ensure_styles).
    pub fn add_default_styles(&mut self) -> Result<String> {
        self.ensure_styles()
    }

    /// Count media parts under `/xl/media/`.
    pub fn media_count(&self) -> usize {
        self.list_media().len()
    }


    /// Whether any media parts exist under `/xl/media/`.
    pub fn has_media(&self) -> bool {
        self.media_count() > 0
    }

    /// List media part URIs under `/xl/media/`.
    pub fn list_media(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/media/"))
            
            .collect()
    }

    /// Remove a single media/image part by URI and drop relationships that target it.
    pub fn remove_media(&mut self, uri: &PackUri) -> Result<bool> {
        if !uri.as_str().starts_with("/xl/media/") {
            return Ok(false);
        }
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        let target = uri.as_str().to_string();
        let part_uris: Vec<PackUri> = self.package.opc().part_uris();
        for src in part_uris {
            let Some(rels) = self.package.opc().part_relationships(&src) else {
                continue;
            };
            let ids: Vec<String> = rels
                .iter()
                .filter(|r| relationship_targets_uri(&src, r.target.as_str(), &target))
                .map(|r| r.id.clone())
                .collect();
            if ids.is_empty() {
                continue;
            }
            self.package
                .delete_reference_relationships(Some(&src), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Remove media parts under `/xl/media/` (does not rewrite drawings that referenced them).
    pub fn clear_media(&mut self) -> Result<usize> {
        let media = self.list_media();
        let n = media.len();
        for uri in media {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Count chart parts under `/xl/charts/`.
    pub fn chart_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/charts/"))
            .count()
    }

    /// List drawing part URIs under `/xl/drawings/`.
    pub fn list_drawings(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/drawings/"))
            
            .collect()
    }

    /// Number of drawing parts under `/xl/drawings/`.
    pub fn drawing_count(&self) -> usize {
        self.list_drawings().len()
    }

    /// Whether any drawing parts exist.
    pub fn has_drawings(&self) -> bool {
        !self.list_drawings().is_empty()
    }

    /// Whether any Excel table parts exist.
    /// Whether a sheet has a drawing relationship.
    pub fn sheet_has_drawing(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(rels) = self.package.opc().part_relationships(&sheet_uri) else {
            return Ok(false);
        };
        Ok(rels.iter().any(|r| r.relationship_type.contains("drawing")))
    }

    /// Sheet names that have drawing relationships.
    pub fn sheets_with_drawings(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.sheet_has_drawing(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has drawings.
    pub fn has_sheets_with_drawings(&self) -> Result<bool> {
        Ok(!self.sheets_with_drawings()?.is_empty())
    }

    pub fn has_tables(&self) -> bool {
        self.table_count() > 0
    }

    /// Whether a table with the given name exists.
    pub fn has_table(&self, name: &str) -> Result<bool> {
        Ok(self.table_uri(name)?.is_some())
    }

    /// Remove all drawing parts under `/xl/drawings/` and worksheet drawing references.
    pub fn clear_drawings(&mut self) -> Result<usize> {
        let drawings = self.list_drawings();
        let n = drawings.len();
        if n == 0 {
            return Ok(0);
        }
        let sheet_uris: Vec<PackUri> = self.sheets.iter().map(|s| s.uri.clone()).collect();
        for sheet_uri in sheet_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&sheet_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type.contains("drawing"))
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                self.package
                    .delete_reference_relationships(Some(&sheet_uri), &ids);
                if let Ok(mut root) = self.load_sheet_root(&sheet_uri) {
                    root.children.retain(|c| c.local_name != "drawing");
                    let _ = self.save_sheet_root(&sheet_uri, &root);
                }
            }
        }
        for uri in drawings {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Remove a single drawing part by URI and drop sheet relationships / `drawing` elements.
    pub fn remove_drawing(&mut self, drawing_uri: &PackUri) -> Result<bool> {
        if !drawing_uri.as_str().starts_with("/xl/drawings/") {
            return Ok(false);
        }
        if !self.package.opc().has_part(drawing_uri) {
            return Ok(false);
        }
        let target = drawing_uri.as_str().to_string();
        let sheet_uris: Vec<PackUri> = self.sheets.iter().map(|s| s.uri.clone()).collect();
        for sheet_uri in sheet_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&sheet_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type.contains("drawing")
                                && (r.target == target
                                    || r.target.ends_with(target.trim_start_matches('/'))
                                    || target.ends_with(r.target.trim_start_matches("./")))
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if ids.is_empty() {
                continue;
            }
            self.package
                .delete_reference_relationships(Some(&sheet_uri), &ids);
            if let Ok(mut root) = self.load_sheet_root(&sheet_uri) {
                root.children.retain(|c| c.local_name != "drawing");
                let _ = self.save_sheet_root(&sheet_uri, &root);
            }
        }
        self.package.delete_part(drawing_uri);
        Ok(true)
    }

    /// List chart part URIs under `/xl/charts/`.
    pub fn list_charts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/charts/"))
            
            .collect()
    }

    /// Remove a chart part by URI, drop relationships that target it, and strip
    /// drawing anchors whose graphic frame references those relationship ids.
    ///
    /// Returns whether the part was present.
    pub fn remove_chart(&mut self, chart_uri: &PackUri) -> Result<bool> {
        if !chart_uri.as_str().starts_with("/xl/charts/") {
            return Ok(false);
        }
        if !self.package.opc().has_part(chart_uri) {
            return Ok(false);
        }
        let target = chart_uri.as_str().to_string();
        let part_uris: Vec<PackUri> = self.package.opc().part_uris();
        // Collect (drawing_uri, rid) pairs so we can prune anchors before dropping rels.
        let mut drawing_rids: Vec<(PackUri, String)> = Vec::new();
        for src in &part_uris {
            let Some(rels) = self.package.opc().part_relationships(&src) else {
                continue;
            };
            for r in rels.iter() {
                let matches = relationship_targets_uri(src, r.target.as_str(), &target);
                if matches && src.as_str().starts_with("/xl/drawings/") {
                    drawing_rids.push((src.clone(), r.id.clone()));
                }
            }
        }

        // Remove anchors in drawing parts that reference the chart rIds.
        for (drawing_uri, rid) in &drawing_rids {
            let Some(data) = self.package.opc().get_part(drawing_uri).map(|d| d.to_vec()) else {
                continue;
            };
            let Ok(mut root) = parse_element(&data) else {
                continue;
            };
            let before = root.children.len();
            root.children.retain(|anchor| !anchor_references_rid(anchor, rid));
            if root.children.len() != before {
                if let Ok(xml) = write_element(&root) {
                    // Preserve content type if present.
                    let ct = self
                        .package
                        .opc()
                        .content_types()
                        .overrides
                        .get(drawing_uri.as_str())
                        .cloned()
                        .unwrap_or_else(|| content_type::SPREADSHEET_DRAWING.to_string());
                    self.package.set_part(drawing_uri.clone(), ct, xml);
                }
            }
        }

        for src in part_uris {
            let Some(rels) = self.package.opc().part_relationships(&src) else {
                continue;
            };
            let ids: Vec<String> = rels
                .iter()
                .filter(|r| relationship_targets_uri(&src, r.target.as_str(), &target))
                .map(|r| r.id.clone())
                .collect();
            if ids.is_empty() {
                continue;
            }
            self.package
                .delete_reference_relationships(Some(&src), &ids);
        }
        self.package.delete_part(chart_uri);
        Ok(true)
    }

    /// Remove chart part at 0-based index among [`list_charts`](Self::list_charts).
    pub fn remove_chart_at(&mut self, index: usize) -> Result<bool> {
        let charts = self.list_charts();
        let Some(uri) = charts.get(index).cloned() else {
            return Ok(false);
        };
        self.remove_chart(&uri)
    }

    /// Whether any chart parts exist.
    pub fn has_charts(&self) -> bool {
        self.chart_count() > 0
    }

    /// List chart titles found under `/xl/charts/` (empty string when no title text).
    pub fn list_chart_titles(&self) -> Result<Vec<(PackUri, String)>> {
        let mut out = Vec::new();
        for uri in self.list_charts() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            let title = root
                .descendants()
                .find(|e| e.local_name == "title")
                .map(|t| {
                    t.descendants()
                        .filter(|e| e.local_name == "t")
                        .filter_map(|e| e.text.as_deref())
                        .collect::<Vec<_>>()
                        .join("")
                })
                .unwrap_or_default();
            out.push((uri, title));
        }
        Ok(out)
    }

    /// Set the chart title text on a chart part (first `c:title` rich text run).
    pub fn set_chart_title(&mut self, chart_uri: &PackUri, title: &str) -> Result<bool> {
        let Some(data) = self.package.opc().get_part(chart_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, title: &str, found: &mut bool) {
            if el.local_name == "title" && !*found {
                // Prefer existing t node; otherwise leave structure and just mark found false
                for d in el.descendants() {
                    // can't mut through descendants; handled below
                    let _ = d;
                }
                // recursive mut walk for t under this title
                fn set_t(el: &mut OpenXmlElement, title: &str) -> bool {
                    if el.local_name == "t" {
                        el.set_text(title);
                        return true;
                    }
                    for c in el.children.iter_mut() {
                        if set_t(c, title) {
                            return true;
                        }
                    }
                    false
                }
                if set_t(el, title) {
                    *found = true;
                }
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, title, found);
            }
        }
        visit(&mut root, title, &mut found);
        if found {
            let ct = self
                .package
                .opc()
                .content_types()
                .content_type_for(chart_uri.as_str())
                .unwrap_or(content_type::DRAWINGML_CHART)
                .to_string();
            self.package
            .set_part(chart_uri.clone(), &ct, write_element(&root)?);
        }
        Ok(found)
    }

    /// Count worksheet comments parts (`/xl/comments*.xml`).
    /// Whether a chart part has a non-empty title text.
    pub fn has_chart_title(&self, chart_uri: &PackUri) -> Result<bool> {
        Ok(self
            .list_chart_titles()?
            .into_iter()
            .any(|(u, t)| &u == chart_uri && !t.is_empty()))
    }

    /// Clear title text nodes under a chart part. Returns whether any title was cleared.
    pub fn clear_chart_title(&mut self, chart_uri: &PackUri) -> Result<bool> {
        let Some(data) = self.package.opc().get_part(chart_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, found: &mut bool) {
            if el.local_name == "title" {
                // clear a:t text under title
                fn clear_t(el: &mut OpenXmlElement, found: &mut bool) {
                    if el.local_name == "t" {
                        if el.text.as_deref().unwrap_or("").len() > 0 {
                            el.set_text("");
                            *found = true;
                        }
                    }
                    for c in el.children.iter_mut() {
                        clear_t(c, found);
                    }
                }
                clear_t(el, found);
                // also drop title entirely if preferred? keep structure, just clear text
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, found);
            }
        }
        visit(&mut root, &mut found);
        if found {
            let xml = write_element(&root)?;
            let ct = self
                .package
                .opc()
                .content_types()
                .content_type_for(chart_uri.as_str())
                .unwrap_or(content_type::DRAWINGML_CHART)
                .to_string();
            self.package
            .set_part(chart_uri.clone(), &ct, xml);
        }
        Ok(found)
    }


    /// Whether a chart has a `c:legend` element.
    pub fn has_chart_legend(&self, chart_uri: &PackUri) -> Result<bool> {
        let Some(data) = self.package.opc().get_part(chart_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.descendants().any(|e| e.local_name == "legend"))
    }

    /// Ensure a chart legend exists (adds empty `c:legend` under chart if missing).
    pub fn set_chart_legend(&mut self, chart_uri: &PackUri, enabled: bool) -> Result<bool> {
        let Some(data) = self.package.opc().get_part(chart_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let c_ns = "http://schemas.openxmlformats.org/drawingml/2006/chart";
        // find chart element
        let mut changed = false;
        fn visit(el: &mut OpenXmlElement, enabled: bool, c_ns: &str, changed: &mut bool) {
            if el.local_name == "chart" {
                let has = el.children.iter().any(|c| c.local_name == "legend");
                if enabled && !has {
                    el.append_child(OpenXmlElement::new("c", c_ns, "legend"));
                    *changed = true;
                } else if !enabled && has {
                    el.children.retain(|c| c.local_name != "legend");
                    *changed = true;
                }
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, c_ns, changed);
            }
        }
        visit(&mut root, enabled, c_ns, &mut changed);
        if changed {
            let xml = write_element(&root)?;
            let ct = self
                .package
                .opc()
                .content_types()
                .content_type_for(chart_uri.as_str())
                .unwrap_or(content_type::DRAWINGML_CHART)
                .to_string();
            self.package
            .set_part(chart_uri.clone(), &ct, xml);
        }
        Ok(changed)
    }

    /// Remove chart legend. Returns whether present.
    pub fn clear_chart_legend(&mut self, chart_uri: &PackUri) -> Result<bool> {
        self.set_chart_legend(chart_uri, false)
    }


    /// List chart axis titles as `(ax_id_or_index, title_text)` for cat/val/date/ser axes with title.
    pub fn list_chart_axis_titles(
        &self,
        chart_uri: &PackUri,
    ) -> Result<Vec<(String, String)>> {
        let Some(data) = self.package.opc().get_part(chart_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let mut out = Vec::new();
        for e in root.descendants() {
            if matches!(
                e.local_name.as_str(),
                "catAx" | "valAx" | "dateAx" | "serAx"
            ) {
                let id = e
                    .child("axId")
                    .and_then(|a| a.get_attribute("val"))
                    .unwrap_or(&e.local_name)
                    .to_string();
                if let Some(title_el) = e.child("title") {
                    let text = title_el
                        .descendants()
                        .filter(|t| t.local_name == "t")
                        .filter_map(|t| t.text.as_deref())
                        .collect::<Vec<_>>()
                        .join("");
                    if !text.is_empty() {
                        out.push((id, text));
                    }
                }
            }
        }
        Ok(out)
    }

    /// Whether any chart axis has a title.
    pub fn has_chart_axis_titles(&self, chart_uri: &PackUri) -> Result<bool> {
        Ok(!self.list_chart_axis_titles(chart_uri)?.is_empty())
    }

    /// Set text on the first title under the first matching axis kind (`catAx`/`valAx`/...).
    pub fn set_chart_axis_title(
        &mut self,
        chart_uri: &PackUri,
        axis_kind: &str,
        title: &str,
    ) -> Result<bool> {
        let Some(data) = self.package.opc().get_part(chart_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let c_ns = "http://schemas.openxmlformats.org/drawingml/2006/chart";
        let a_ns = crate::namespace::ns::DRAWINGML.uri;
        let mut found = false;
        fn set_t(el: &mut OpenXmlElement, title: &str) -> bool {
            if el.local_name == "t" {
                el.set_text(title);
                return true;
            }
            for c in el.children.iter_mut() {
                if set_t(c, title) {
                    return true;
                }
            }
            false
        }
        fn visit(
            el: &mut OpenXmlElement,
            axis_kind: &str,
            title: &str,
            c_ns: &str,
            a_ns: &str,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == axis_kind {
                if let Some(title_el) = el.child_mut("title") {
                    if set_t(title_el, title) {
                        *found = true;
                        return;
                    }
                }
                // create minimal title structure
                let title_el = OpenXmlElement::new("c", c_ns, "title").with_child(
                    OpenXmlElement::new("c", c_ns, "tx").with_child(
                        OpenXmlElement::new("c", c_ns, "rich").with_child(
                            OpenXmlElement::new("a", a_ns, "p").with_child(
                                OpenXmlElement::new("a", a_ns, "r").with_child(
                                    OpenXmlElement::new("a", a_ns, "t").with_text(title),
                                ),
                            ),
                        ),
                    ),
                );
                el.append_child(title_el);
                *found = true;
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, axis_kind, title, c_ns, a_ns, found);
            }
        }
        visit(&mut root, axis_kind, title, c_ns, a_ns, &mut found);
        if found {
            let xml = write_element(&root)?;
            let ct = self
                .package
                .opc()
                .content_types()
                .content_type_for(chart_uri.as_str())
                .unwrap_or(content_type::DRAWINGML_CHART)
                .to_string();
            self.package
            .set_part(chart_uri.clone(), &ct, xml);
        }
        Ok(found)
    }

    /// Clear all axis titles under a chart. Returns how many were cleared.
    pub fn clear_chart_axis_titles(&mut self, chart_uri: &PackUri) -> Result<usize> {
        let Some(data) = self.package.opc().get_part(chart_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if matches!(
                el.local_name.as_str(),
                "catAx" | "valAx" | "dateAx" | "serAx"
            ) {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "title");
                if el.children.len() < before {
                    *n += 1;
                }
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(&mut root, &mut n);
        if n > 0 {
            let xml = write_element(&root)?;
            let ct = self
                .package
                .opc()
                .content_types()
                .content_type_for(chart_uri.as_str())
                .unwrap_or(content_type::DRAWINGML_CHART)
                .to_string();
            self.package
            .set_part(chart_uri.clone(), &ct, xml);
        }
        Ok(n)
    }


    pub fn comments_part_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.starts_with("/xl/comments") && s.ends_with(".xml")
            })
            .count()
    }

    /// Whether the sheet has a comments relationship.
    pub fn sheet_has_comments(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        Ok(self
            .package
            .opc()
            .part_relationships(&sheet_uri)
            .map(|rels| {
                rels.iter()
                    .any(|r| r.relationship_type.contains("/comments"))
            })
            .unwrap_or(false))
    }

    /// Whether a shared strings part exists.
    /// Sheet names that have comments.
    pub fn sheets_with_comments(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.sheet_has_comments(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has comments.
    pub fn has_sheets_with_comments(&self) -> Result<bool> {
        Ok(!self.sheets_with_comments()?.is_empty())
    }

    pub fn has_shared_strings(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new(SHARED_STRINGS_URI))
            || self.sst.as_ref().map(|s| !s.is_empty()).unwrap_or(false)
    }

    /// Convert all `t="s"` cells on every worksheet to `inlineStr` using the SST.
    ///
    /// Leaves the shared strings part and in-memory builder intact. Useful before
    /// editing cells without maintaining SST indices. Returns the number of cells rewritten.
    pub fn materialize_shared_strings(&mut self) -> Result<usize> {
        let uri = PackUri::new(SHARED_STRINGS_URI);
        let strings: Vec<String> = if let Some(sst) = self.sst.as_ref() {
            sst.strings().to_vec()
        } else if self.package.opc().has_part(&uri) {
            if let Some(data) = self.package.opc().get_part(&uri) {
                if let Ok(root) = parse_element(data) {
                    root.children_by_name("si")
                        .map(|si| si.inner_text())
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        if strings.is_empty() {
            return Ok(0);
        }
        let mut total = 0usize;
        let sheet_uris: Vec<PackUri> = self.sheets.iter().map(|s| s.uri.clone()).collect();
        for sheet_uri in sheet_uris {
            if let Ok(mut root) = self.load_sheet_root(&sheet_uri) {
                let mut changed = false;
                if let Some(sd) = root.child_mut("sheetData") {
                    // Count cells before
                    let before = count_shared_string_cells(sd);
                    if rewrite_shared_string_cells(sd, &strings) {
                        changed = true;
                        total += before;
                    }
                }
                if changed {
                    self.save_sheet_root(&sheet_uri, &root)?;
                }
            }
        }
        Ok(total)
    }

    /// Remove the shared strings part and in-memory SST builder.
    ///
    /// Before removal, every worksheet cell with `t="s"` is rewritten to an
    /// `inlineStr` cell holding the resolved text so display values survive.
    pub fn clear_shared_strings(&mut self) -> Result<bool> {
        let uri = PackUri::new(SHARED_STRINGS_URI);
        let had_part = self.package.opc().has_part(&uri);
        let had_mem = self.sst.as_ref().map(|s| !s.is_empty()).unwrap_or(false);
        if !had_part && !had_mem {
            return Ok(false);
        }

        // Resolve strings: prefer in-memory SST, fall back to part parse.
        let strings: Vec<String> = if let Some(sst) = self.sst.as_ref() {
            sst.strings().to_vec()
        } else if had_part {
            if let Some(data) = self.package.opc().get_part(&uri) {
                if let Ok(root) = parse_element(data) {
                    root.children_by_name("si")
                        .map(|si| si.inner_text())
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Rewrite all worksheet cells that reference shared strings.
        if !strings.is_empty() {
            let sheet_uris: Vec<PackUri> = self.sheets.iter().map(|s| s.uri.clone()).collect();
            for sheet_uri in sheet_uris {
                if let Ok(mut root) = self.load_sheet_root(&sheet_uri) {
                    let mut changed = false;
                    if let Some(sd) = root.child_mut("sheetData") {
                        changed = rewrite_shared_string_cells(sd, &strings);
                    }
                    if changed {
                        self.save_sheet_root(&sheet_uri, &root)?;
                    }
                }
            }
        }

        if had_part {
            if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
                let ids: Vec<String> = self
                    .package
                    .opc()
                    .part_relationships(&wb_uri)
                    .map(|rels| {
                        rels.find_all_by_type(rel::SHARED_STRINGS)
                            .into_iter()
                            .map(|r| r.id.clone())
                            .collect()
                    })
                    .unwrap_or_default();
                self.package
                    .delete_reference_relationships(Some(&wb_uri), &ids);
            }
            self.package.delete_part(&uri);
        }
        self.sst = None;
        Ok(true)
    }

    /// Remove the shared strings part without rewriting worksheet cells.
    ///
    /// Prefer [`clear_shared_strings`](Self::clear_shared_strings) which converts
    /// `t="s"` cells to inline strings first.
    pub fn clear_shared_strings_part_only(&mut self) -> Result<bool> {
        let uri = PackUri::new(SHARED_STRINGS_URI);
        let had_part = self.package.opc().has_part(&uri);
        let had_mem = self.sst.as_ref().map(|s| !s.is_empty()).unwrap_or(false);
        if !had_part && !had_mem {
            return Ok(false);
        }
        if had_part {
            if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
                let ids: Vec<String> = self
                    .package
                    .opc()
                    .part_relationships(&wb_uri)
                    .map(|rels| {
                        rels.find_all_by_type(rel::SHARED_STRINGS)
                            .into_iter()
                            .map(|r| r.id.clone())
                            .collect()
                    })
                    .unwrap_or_default();
                self.package
                    .delete_reference_relationships(Some(&wb_uri), &ids);
            }
            self.package.delete_part(&uri);
        }
        self.sst = None;
        Ok(true)
    }

    /// Whether a calculation chain part exists.
    pub fn has_calc_chain(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/xl/calcChain.xml"))
    }

    /// Remove the calculation chain part and its workbook relationship.
    pub fn clear_calc_chain(&mut self) -> Result<bool> {
        let chain_uri = PackUri::new("/xl/calcChain.xml");
        if !self.package.opc().has_part(&chain_uri) {
            return Ok(false);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::CALC_CHAIN)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        self.package.delete_part(&chain_uri);
        Ok(true)
    }

    /// List all part URIs in the package.
    pub fn list_part_uris(&self) -> Vec<PackUri> {
        self.package.opc().part_uris()
    }

    /// Set a sort state on a range (metadata for Excel to apply).
    pub fn set_sort_state(
        &mut self,
        sheet_name: &str,
        range_ref: &str,
        column_ref: &str,
        descending: bool,
    ) -> Result<()> {
        use crate::spreadsheet::sort_state;
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "sortState");
        let insert_at = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
            .map(|i| i + 1)
            .unwrap_or(root.children.len());
        root.children
            .insert(insert_at, sort_state(range_ref, column_ref, descending));
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether the sheet has a sort state.
    pub fn has_sort_state(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root.child("sortState").is_some())
    }

    /// Remove sort state from a sheet. Returns whether it was present.
    pub fn clear_sort_state(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "sortState");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Read sort state as `(range_ref, column_ref, descending)` when present.
    /// Clear sort state on every sheet. Returns sheets modified.
    pub fn clear_all_sort_state(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_sort_state(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Sheet names that have a sort state.
    pub fn sheets_with_sort_state(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_sort_state(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    pub fn sort_state(&self, sheet_name: &str) -> Result<Option<(String, String, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(ss) = root.child("sortState") else {
            return Ok(None);
        };
        let range = ss.get_attribute("ref").unwrap_or("").to_string();
        let cond = ss.child("sortCondition");
        let col = cond
            .and_then(|c| c.get_attribute("ref"))
            .unwrap_or("")
            .to_string();
        let desc = cond
            .and_then(|c| c.get_attribute("descending"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Ok(Some((range, col, desc)))
    }

    /// Set case-sensitive flag on existing sort state (`sortState/@caseSensitive`).
    pub fn set_sort_case_sensitive(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(ss) = root.child_mut("sortState") {
            ss.set_attribute("caseSensitive", if enabled { "1" } else { "0" });
        } else {
            return Err(Error::Package("sortState missing; call set_sort_state first".into()));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Update the sortState range `ref` without replacing conditions.
    pub fn set_sort_range(&mut self, sheet_name: &str, range_ref: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        ss.set_attribute("ref", range_ref);
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Read sortState `@ref`.
    pub fn sort_range(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sortState")
            .and_then(|ss| ss.get_attribute("ref").map(|r| r.to_string())))
    }

    /// Whether sortState has a ref.
    pub fn has_sort_range(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sort_range(sheet_name)?.is_some())
    }

    /// Clear sortState `@ref` (keeps sortState element).
    pub fn clear_sort_range(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        if ss.get_attribute("ref").is_none() {
            return Ok(false);
        }
        ss.remove_attribute("ref");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Whether sort is case-sensitive.
    pub fn sort_case_sensitive(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sortState")
            .and_then(|s| s.get_attribute("caseSensitive"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set sort method on sortState (`"none"`, `"stroke"`, `"pinYin"`).
    /// Disable `sort case sensitive` on a sheet. Returns whether it was enabled.
    pub fn clear_sort_case_sensitive(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.sort_case_sensitive(sheet_name)?;
        if had {
            self.set_sort_case_sensitive(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_sort_method(&mut self, sheet_name: &str, method: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        ss.set_attribute("sortMethod", method);
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Read sort method.
    pub fn sort_method(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sortState")
            .and_then(|s| s.get_attribute("sortMethod").map(|m| m.to_string())))
    }

    /// Whether sortMethod is set on sortState.
    pub fn has_sort_method(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sort_method(sheet_name)?.is_some())
    }

    /// Clear sortMethod on sortState.
    pub fn clear_sort_method(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        if ss.get_attribute("sortMethod").is_none() {
            return Ok(false);
        }
        ss.remove_attribute("sortMethod");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Set columnSort flag on sortState.
    pub fn set_sort_column_sort(&mut self, sheet_name: &str, enabled: bool) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        ss.set_attribute("columnSort", if enabled { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Clear sortState `@columnSort`.
    pub fn clear_sort_column_sort(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        if ss.get_attribute("columnSort").is_none() {
            return Ok(false);
        }
        ss.remove_attribute("columnSort");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Whether columnSort is enabled.
    pub fn sort_column_sort(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sortState")
            .and_then(|s| s.get_attribute("columnSort"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set custom list on the first sort condition matching `column_ref`.
    pub fn set_sort_condition_custom_list(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
        custom_list: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let mut found = false;
        for cond in ss.children.iter_mut() {
            if cond.local_name == "sortCondition" && cond.get_attribute("ref") == Some(column_ref)
            {
                cond.set_attribute("customList", custom_list);
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear sortCondition `@customList` for a column ref.
    pub fn clear_sort_condition_custom_list(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
    ) -> Result<bool> {
        self.clear_sort_condition_attr(sheet_name, column_ref, "customList")
    }

    /// Set sortBy on a sort condition (`"value"`, `"cellColor"`, `"fontColor"`, `"icon"`).
    pub fn set_sort_condition_sort_by(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
        sort_by: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let mut found = false;
        for cond in ss.children.iter_mut() {
            if cond.local_name == "sortCondition" && cond.get_attribute("ref") == Some(column_ref)
            {
                cond.set_attribute("sortBy", sort_by);
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear sortCondition `@sortBy` for a column ref.
    pub fn clear_sort_condition_sort_by(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
    ) -> Result<bool> {
        self.clear_sort_condition_attr(sheet_name, column_ref, "sortBy")
    }

    /// Set icon sort attributes on a sort condition (`iconSet`, `iconId`).
    pub fn set_sort_condition_icon(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
        icon_set: &str,
        icon_id: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let mut found = false;
        for cond in ss.children.iter_mut() {
            if cond.local_name == "sortCondition" && cond.get_attribute("ref") == Some(column_ref)
            {
                cond.set_attribute("sortBy", "icon");
                cond.set_attribute("iconSet", icon_set);
                cond.set_attribute("iconId", icon_id.to_string());
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear sortCondition icon attributes for a column ref.
    pub fn clear_sort_condition_icon(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let mut found = false;
        for cond in ss.children.iter_mut() {
            if cond.local_name == "sortCondition" && cond.get_attribute("ref") == Some(column_ref)
            {
                let before = cond.attributes.len();
                cond.attributes.retain(|a| {
                    a.local_name != "iconSet" && a.local_name != "iconId"
                });
                if cond.attributes.len() < before {
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Set color/font sort via dxfId on a sort condition.
    pub fn set_sort_condition_dxf(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
        sort_by: &str,
        dxf_id: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let mut found = false;
        for cond in ss.children.iter_mut() {
            if cond.local_name == "sortCondition" && cond.get_attribute("ref") == Some(column_ref)
            {
                cond.set_attribute("sortBy", sort_by);
                cond.set_attribute("dxfId", dxf_id.to_string());
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear sortCondition `@dxfId` for a column ref.
    pub fn clear_sort_condition_dxf(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
    ) -> Result<bool> {
        self.clear_sort_condition_attr(sheet_name, column_ref, "dxfId")
    }

    /// Set descending flag on a sort condition.
    fn clear_sort_condition_attr(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
        attr: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let mut found = false;
        for cond in ss.children.iter_mut() {
            if cond.local_name == "sortCondition" && cond.get_attribute("ref") == Some(column_ref)
            {
                if cond.get_attribute(attr).is_some() {
                    cond.remove_attribute(attr);
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    pub fn set_sort_condition_descending(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
        descending: bool,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let mut found = false;
        for cond in ss.children.iter_mut() {
            if cond.local_name == "sortCondition" && cond.get_attribute("ref") == Some(column_ref)
            {
                cond.set_attribute("descending", if descending { "1" } else { "0" });
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear sortCondition `@descending` for a column ref.
    pub fn clear_sort_condition_descending(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
    ) -> Result<bool> {
        self.clear_sort_condition_attr(sheet_name, column_ref, "descending")
    }

    /// Read sort condition details as `(sort_by, custom_list, dxf_id, icon_set, icon_id)`.
    pub fn sort_condition_details(
        &self,
        sheet_name: &str,
        column_ref: &str,
    ) -> Result<Option<(Option<String>, Option<String>, Option<u32>, Option<String>, Option<u32>)>>
    {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(ss) = root.child("sortState") else {
            return Ok(None);
        };
        for cond in ss.children_by_name("sortCondition") {
            if cond.get_attribute("ref") != Some(column_ref) {
                continue;
            }
            return Ok(Some((
                cond.get_attribute("sortBy").map(|s| s.to_string()),
                cond.get_attribute("customList").map(|s| s.to_string()),
                cond.get_attribute("dxfId").and_then(|s| s.parse().ok()),
                cond.get_attribute("iconSet").map(|s| s.to_string()),
                cond.get_attribute("iconId").and_then(|s| s.parse().ok()),
            )));
        }
        Ok(None)
    }

    /// Count sort conditions under sortState.
    pub fn sort_condition_count(&self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sortState")
            .map(|ss| ss.children_by_name("sortCondition").count())
            .unwrap_or(0))
    }

    /// Append an additional sort condition to an existing sortState.
    pub fn add_sort_condition(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
        descending: bool,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut cond = OpenXmlElement::new("x", x, "sortCondition")
            .with_attribute("ref", column_ref);
        if descending {
            cond.set_attribute("descending", "1");
        }
        ss.append_child(cond);
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Remove the first sort condition matching `column_ref`. Returns whether found.
    pub fn remove_sort_condition(
        &mut self,
        sheet_name: &str,
        column_ref: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(false);
        };
        let before = ss.children.len();
        ss.children.retain(|c| {
            !(c.local_name == "sortCondition" && c.get_attribute("ref") == Some(column_ref))
        });
        let removed = ss.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// List sort conditions as `(column_ref, descending)`.
    pub fn list_sort_conditions(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(ss) = root.child("sortState") else {
            return Ok(Vec::new());
        };
        Ok(ss
            .children_by_name("sortCondition")
            .map(|c| {
                let col = c.get_attribute("ref").unwrap_or("").to_string();
                let desc = c
                    .get_attribute("descending")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                (col, desc)
            })
            .collect())
    }

    /// Whether a sheet has any sort conditions.
    pub fn has_sort_conditions(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sort_condition_count(sheet_name)? > 0)
    }

    /// Remove all sort conditions under sortState (keeps sortState shell if present).
    pub fn clear_sort_conditions(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(ss) = root.child_mut("sortState") else {
            return Ok(0);
        };
        let before = ss.children.len();
        ss.children.retain(|c| c.local_name != "sortCondition");
        let removed = before - ss.children.len();
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear sort conditions on every sheet. Returns total removed.
    pub fn clear_all_sort_conditions(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut total = 0usize;
        for name in names {
            total += self.clear_sort_conditions(&name)?;
        }
        Ok(total)
    }

    /// Add whole-number data validation.
    pub fn add_data_validation_whole(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        operator: &str,
        formula1: &str,
        formula2: Option<&str>,
        allow_blank: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let dv = data_validation_whole(sqref, operator, formula1, formula2, allow_blank);
        if let Some(container) = root.child_mut("dataValidations") {
            container.append_child(dv);
            container.set_attribute("count", container.children.len().to_string());
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, data_validations(vec![dv]));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Set outline levels on rows (group/collapse).
    ///
    /// Each entry is `(row_index_1_based, outline_level, hidden)`.
    pub fn set_row_outline_levels(
        &mut self,
        sheet_name: &str,
        levels: &[(u32, u8, bool)],
    ) -> Result<()> {
        use crate::spreadsheet::row_set_outline_level;
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let sheet_data = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("worksheet has no sheetData".into()))?;
        for &(row_idx, level, hidden) in levels {
            if let Some(row_el) = sheet_data.children.iter_mut().find(|c| {
                c.local_name == "row"
                    && c.get_attribute("r").and_then(|s| s.parse().ok()) == Some(row_idx)
            }) {
                row_set_outline_level(row_el, level, hidden);
            }
        }
        // Ensure sheetPr/outlinePr
        use crate::spreadsheet::outline_properties;
        if let Some(pr) = root.child_mut("sheetPr") {
            pr.children.retain(|c| c.local_name != "outlinePr");
            pr.append_child(outline_properties(true, true));
        } else {
            let x = crate::namespace::ns::SPREADSHEETML.uri;
            let pr = crate::element::OpenXmlElement::new("x", x, "sheetPr")
                .with_child(outline_properties(true, true));
            root.children.insert(0, pr);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List row outline levels as `(row_index, outline_level, hidden)`.
    pub fn row_outline_levels(&self, sheet_name: &str) -> Result<Vec<(u32, u8, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(sd) = root.child("sheetData") else {
            return Ok(Vec::new());
        };
        Ok(sd
            .children_by_name("row")
            .filter_map(|r| {
                let level = r.get_attribute("outlineLevel")?.parse::<u8>().ok()?;
                let idx = r
                    .get_attribute("r")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let hidden = r.get_attribute("hidden").map(|s| s == "1").unwrap_or(false);
                Some((idx, level, hidden))
            })
            .collect())
    }

    /// Number of rows with an outline level set.
    pub fn row_outline_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.row_outline_levels(sheet_name)?.len())
    }

    /// Whether any rows have an outline level set.
    pub fn has_row_outlines(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.row_outline_count(sheet_name)? > 0)
    }

    /// Whether any sheet has row or column outlines.
    pub fn has_sheets_with_outlines(&self) -> Result<bool> {
        for name in self.sheet_names() {
            if self.has_row_outlines(name)? || self.has_column_outlines(name)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Sheet names that have row or column outlines.
    pub fn sheets_with_outlines(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_row_outlines(name)? || self.has_column_outlines(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Clear row and column outlines on every sheet. Returns sheets modified.
    pub fn clear_all_outlines(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            let mut changed = false;
            if self.has_row_outlines(&name)? {
                self.clear_all_row_outlines(&name)?;
                changed = true;
            }
            if self.has_column_outlines(&name)? {
                self.clear_all_column_outlines(&name)?;
                changed = true;
            }
            if changed {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Clear outlineLevel (and collapsed) on all rows of a sheet. Returns rows cleared.
    pub fn clear_all_row_outlines(&mut self, sheet_name: &str) -> Result<usize> {
        self.clear_row_outline_levels(sheet_name, 1, u32::MAX)
    }

    /// Clear outlineLevel (and collapsed) on rows in `[from_row, to_row]` inclusive (1-based).
    /// Returns the number of rows cleared.
    pub fn clear_row_outline_levels(
        &mut self,
        sheet_name: &str,
        from_row: u32,
        to_row: u32,
    ) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sd) = root.child_mut("sheetData") else {
            return Ok(0);
        };
        let mut count = 0usize;
        for row in sd.children.iter_mut() {
            if row.local_name != "row" {
                continue;
            }
            let idx = row
                .get_attribute("r")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            if idx >= from_row && idx <= to_row {
                if row.get_attribute("outlineLevel").is_some()
                    || row.get_attribute("collapsed").is_some()
                {
                    row.remove_attribute("outlineLevel");
                    row.remove_attribute("collapsed");
                    count += 1;
                }
            }
        }
        if count > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(count)
    }

    /// Read sheet visibility state: `"visible"`, `"hidden"`, or `"veryHidden"`.
    pub fn sheet_state(&self, sheet_name: &str) -> Result<String> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok("visible".into());
        };
        let root = parse_element(data)?;
        let Some(sheets) = root.child("sheets") else {
            return Ok("visible".into());
        };
        for s in sheets.children_by_name("sheet") {
            if s.get_attribute("name") == Some(sheet_name) {
                return Ok(s
                    .get_attribute("state")
                    .unwrap_or("visible")
                    .to_string());
            }
        }
        Ok("visible".into())
    }

    /// Whether sheet `@state` is non-visible (hidden/veryHidden).
    pub fn has_sheet_state(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_state(sheet_name)? != "visible")
    }

    /// Clear sheet `@state` (makes visible). Returns whether it was non-visible.
    pub fn clear_sheet_state(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.has_sheet_state(sheet_name)?;
        if had {
            self.set_sheet_state(sheet_name, "visible")?;
        }
        Ok(had)
    }

    /// List all sheets as `(name, state)`.
    pub fn list_sheet_states(&self) -> Result<Vec<(String, String)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(sheets) = root.child("sheets") else {
            return Ok(Vec::new());
        };
        Ok(sheets
            .children_by_name("sheet")
            .map(|s| {
                (
                    s.get_attribute("name").unwrap_or("").to_string(),
                    s.get_attribute("state").unwrap_or("visible").to_string(),
                )
            })
            .collect())
    }

    /// Whether the sheet is hidden or veryHidden.
    pub fn is_sheet_hidden(&self, sheet_name: &str) -> Result<bool> {
        let state = self.sheet_state(sheet_name)?;
        Ok(state == "hidden" || state == "veryHidden")
    }

    /// List sheets that are hidden or veryHidden as `(name, state)`.
    pub fn list_hidden_sheets(&self) -> Result<Vec<(String, String)>> {
        Ok(self
            .list_sheet_states()?
            .into_iter()
            .filter(|(_, s)| s == "hidden" || s == "veryHidden")
            .collect())
    }

    /// Names of sheets that are hidden or veryHidden.
    pub fn list_hidden_sheet_names(&self) -> Result<Vec<String>> {
        Ok(self
            .list_hidden_sheets()?
            .into_iter()
            .map(|(n, _)| n)
            .collect())
    }

    /// Set every hidden/veryHidden sheet to visible. Returns how many sheets were unhidden.
    pub fn unhide_all_sheets(&mut self) -> Result<usize> {
        let hidden = self.list_hidden_sheet_names()?;
        let mut n = 0usize;
        for name in hidden {
            self.set_sheet_state(&name, "visible")?;
            n += 1;
        }
        Ok(n)
    }

    /// Whether any sheet is hidden or veryHidden.
    pub fn has_hidden_sheets(&self) -> Result<bool> {
        Ok(!self.list_hidden_sheet_names()?.is_empty())
    }

    /// Names of sheets with state `veryHidden`.
    pub fn list_very_hidden_sheet_names(&self) -> Result<Vec<String>> {
        Ok(self
            .list_hidden_sheets()?
            .into_iter()
            .filter(|(_, st)| st == "veryHidden")
            .map(|(n, _)| n)
            .collect())
    }

    /// Whether any sheets are veryHidden.
    pub fn has_very_hidden_sheets(&self) -> Result<bool> {
        Ok(!self.list_very_hidden_sheet_names()?.is_empty())
    }

    /// Names of sheets that are visible (not hidden/veryHidden).
    /// Set all veryHidden sheets to visible. Returns how many sheets were changed.
    pub fn unhide_very_hidden_sheets(&mut self) -> Result<usize> {
        let names = self.list_very_hidden_sheet_names()?;
        let mut n = 0usize;
        for name in names {
            self.set_sheet_state(&name, "visible")?;
            n += 1;
        }
        Ok(n)
    }


    pub fn list_visible_sheet_names(&self) -> Result<Vec<String>> {
        let hidden: std::collections::HashSet<String> = self
            .list_hidden_sheet_names()?
            .into_iter()
            .collect();
        Ok(self
            .sheet_names()
            .into_iter()
            .filter(|n| !hidden.contains(*n))
            .map(|s| s.to_string())
            .collect())
    }

    /// Whether any sheets are visible.
    pub fn has_visible_sheets(&self) -> Result<bool> {
        Ok(!self.list_visible_sheet_names()?.is_empty())
    }

    pub fn clear_merge_cells(&mut self, sheet_name: &str) -> Result<usize> {
        let existing = self.merge_cells(sheet_name)?;
        let n = existing.len();
        if n == 0 {
            return Ok(0);
        }
        self.set_merge_cells(sheet_name, &[])?;
        Ok(n)
    }

    /// List array-formula cells as `(reference, formula)`.
    pub fn list_array_formulas(&self, sheet_name: &str) -> Result<Vec<(String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "c")
            .filter_map(|c| {
                let f = c.child("f")?;
                let is_array = f.get_attribute("t") == Some("array")
                    || f.get_attribute("t") == Some("arrayEnter");
                if !is_array {
                    return None;
                }
                let reference = c.get_attribute("r")?.to_string();
                Some((reference, f.inner_text()))
            })
            .collect())
    }

    /// List shared formulas as `(reference, si, formula_text)` where `si` is the shared index.
    pub fn list_shared_formulas(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, u32, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "c")
            .filter_map(|c| {
                let f = c.child("f")?;
                if f.get_attribute("t") != Some("shared") {
                    return None;
                }
                let si = f.get_attribute("si")?.parse().ok()?;
                let reference = c.get_attribute("r")?.to_string();
                Some((reference, si, f.inner_text()))
            })
            .collect())
    }

    /// Number of array-formula cells on a sheet.
    pub fn array_formula_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_array_formulas(sheet_name)?.len())
    }

    /// Whether the sheet has any array formulas.
    pub fn has_array_formulas(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.array_formula_count(sheet_name)? > 0)
    }

    /// Clear all array formulas on a sheet (removes `f@t=array` children). Returns cells updated.
    /// Sheet names that have array formulas.
    pub fn sheets_with_array_formulas(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_array_formulas(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet is returned by [`sheets_with_array_formulas`](Self::sheets_with_array_formulas).
    pub fn has_sheets_with_array_formulas(&self) -> Result<bool> {
        Ok(!self.sheets_with_array_formulas()?.is_empty())
    }

    pub fn clear_array_formulas(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut n = 0;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "c" {
                if let Some(f) = el.child("f") {
                    let t = f.get_attribute("t");
                    if t == Some("array") || t == Some("arrayEnter") {
                        el.children.retain(|c| c.local_name != "f");
                        *n += 1;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(&mut root, &mut n);
        if n > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    /// Number of shared-formula cells on a sheet.
    /// Clear array formulas on every sheet. Returns sheets modified.
    pub fn clear_all_array_formulas(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_array_formulas(&name)? > 0 {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shared_formula_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_shared_formulas(sheet_name)?.len())
    }

    /// Clear shared formulas with the given `si` index (removes `f@t=shared` cells' formula attrs).
    ///
    /// Returns how many cells were updated.
    /// Whether the sheet has any shared formulas.
    pub fn has_shared_formulas(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.shared_formula_count(sheet_name)? > 0)
    }

    /// Sheet names that have shared formulas.
    pub fn sheets_with_shared_formulas(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_shared_formulas(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet is returned by [`sheets_with_shared_formulas`](Self::sheets_with_shared_formulas).
    pub fn has_sheets_with_shared_formulas(&self) -> Result<bool> {
        Ok(!self.sheets_with_shared_formulas()?.is_empty())
    }

    pub fn clear_shared_formula_group(&mut self, sheet_name: &str, si: u32) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut n = 0;
        fn visit(el: &mut OpenXmlElement, si: u32, n: &mut usize) {
            if el.local_name == "c" {
                if let Some(f) = el.child_mut("f") {
                    if f.get_attribute("t") == Some("shared")
                        && f.get_attribute("si").and_then(|s| s.parse().ok()) == Some(si)
                    {
                        el.children.retain(|c| c.local_name != "f");
                        *n += 1;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, si, n);
            }
        }
        visit(&mut root, si, &mut n);
        if n > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    /// Clear all shared formulas on a sheet. Returns cells updated.
    pub fn clear_shared_formulas(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut n = 0;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "c" {
                if let Some(f) = el.child("f") {
                    if f.get_attribute("t") == Some("shared") {
                        el.children.retain(|c| c.local_name != "f");
                        *n += 1;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(&mut root, &mut n);
        if n > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    /// Set sheet visibility state: `"visible"`, `"hidden"`, or `"veryHidden"`.
    /// Clear shared formulas on every sheet. Returns sheets modified.
    pub fn clear_all_shared_formulas(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_shared_formulas(&name)? > 0 {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn set_sheet_state(&mut self, sheet_name: &str, state: &str) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("no workbook".into()));
        };
        let sheets = root
            .child_mut("sheets")
            .ok_or_else(|| Error::Package("workbook has no sheets".into()))?;
        let sheet_el = sheets
            .children
            .iter_mut()
            .find(|c| c.get_attribute("name") == Some(sheet_name))
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;
        if state == "visible" {
            sheet_el.attributes.retain(|a| a.local_name != "state");
        } else {
            sheet_el.set_attribute("state", state);
        }
        let xml = write_element(&root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            xml,
        );
        Ok(())
    }

    /// Add a timeline + timeline cache shell (Excel 2013+).
    ///
    /// Returns `(timelines_uri, cache_uri)`.
    pub fn add_timeline_shell(
        &mut self,
        sheet_name: &str,
        name: &str,
        cache_name: &str,
    ) -> Result<(PackUri, PackUri)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut cindex = 1u32;
        let cache_uri = loop {
            let candidate =
                PackUri::new(format!("/xl/timelineCaches/timelineCache{cindex}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            cindex += 1;
        };
        let mut tindex = 1u32;
        let timelines_uri = loop {
            let candidate = PackUri::new(format!("/xl/timelines/timeline{tindex}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            tindex += 1;
        };
        let x15 = "http://schemas.microsoft.com/office/spreadsheetml/2010/11/main";
        let cache = OpenXmlElement::new("x15", x15, "timelineCacheDefinition")
            .with_ns_decl("x15", x15)
            .with_attribute("name", cache_name)
            .with_attribute("sourceName", name)
            .with_child(
                OpenXmlElement::new("x15", x15, "pivotTables").with_child(
                    OpenXmlElement::new("x15", x15, "pivotTable")
                        .with_attribute("name", "PivotTable1")
                        .with_attribute("tabId", "1"),
                ),
            );
        let timelines = OpenXmlElement::new("x15", x15, "timelines")
            .with_ns_decl("x15", x15)
            .with_child(
                OpenXmlElement::new("x15", x15, "timeline")
                    .with_attribute("name", name)
                    .with_attribute("cache", cache_name)
                    .with_attribute("caption", name),
            );
        self.package.set_part(
            cache_uri.clone(),
            content_type::TIMELINE_CACHE,
            write_element(&cache)?,
        );
        self.package.set_part(
            timelines_uri.clone(),
            content_type::TIMELINE,
            write_element(&timelines)?,
        );
        let wb_uri = self.ensure_workbook()?;
        self.package.add_part_relationship(
            &wb_uri,
            rel::TIMELINE_CACHE,
            &cache_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            &sheet_uri,
            rel::TIMELINE,
            &timelines_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((timelines_uri, cache_uri))
    }

    /// List timeline entries as `(name, cache)` from `/xl/timelines/` parts.
    pub fn list_timeline_entries(&self) -> Result<Vec<(String, String)>> {
        let mut out = Vec::new();
        for uri in self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/timelines/"))
            
            .collect::<Vec<_>>()
        {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let Ok(root) = parse_element(data) else {
                continue;
            };
            for t in root.descendants().filter(|e| e.local_name == "timeline") {
                let name = t.get_attribute("name").unwrap_or("").to_string();
                let cache = t.get_attribute("cache").unwrap_or("").to_string();
                if !name.is_empty() {
                    out.push((name, cache));
                }
            }
        }
        Ok(out)
    }

    /// Whether a timeline entry with the given name exists.
    pub fn has_timeline_entry(&self, name: &str) -> Result<bool> {
        Ok(self
            .list_timeline_entries()?
            .iter()
            .any(|(n, _)| n == name))
    }

    /// Remove timeline entries matching `name`. Returns count removed.
    pub fn remove_timeline_entry(&mut self, name: &str) -> Result<usize> {
        let mut removed = 0usize;
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/timelines/"))
            
            .collect();
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri).map(|d| d.to_vec()) else {
                continue;
            };
            let Ok(mut root) = parse_element(&data) else {
                continue;
            };
            fn visit(el: &mut OpenXmlElement, name: &str, count: &mut usize) {
                let before = el.children.len();
                el.children.retain(|c| {
                    !(c.local_name == "timeline" && c.get_attribute("name") == Some(name))
                });
                *count += before - el.children.len();
                for c in el.children.iter_mut() {
                    visit(c, name, count);
                }
            }
            let mut n = 0usize;
            visit(&mut root, name, &mut n);
            if n > 0 {
                removed += n;
                let xml = write_element(&root)?;
                let ct = self
                    .package
                    .opc()
                    .content_types()
                    .content_type_for(uri.as_str())
                    .unwrap_or("application/vnd.ms-excel.timeline+xml")
                    .to_string();
                self.package.set_part(uri, ct, xml);
            }
        }
        Ok(removed)
    }

    /// Attach chart style + color style shells to a chart part.
    pub fn add_chart_styles(
        &mut self,
        chart_uri: &PackUri,
    ) -> Result<(PackUri, PackUri)> {
        let mut index = 1u32;
        let style_uri = loop {
            let c = PackUri::new(format!("/xl/charts/style{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let colors_uri = PackUri::new(format!("/xl/charts/colors{index}.xml"));
        let cs = "http://schemas.microsoft.com/office/drawing/2012/chartStyle";
        let style = OpenXmlElement::new("cs", cs, "chartStyle")
            .with_ns_decl("cs", cs)
            .with_attribute("id", "201");
        let colors = OpenXmlElement::new("cs", cs, "colorStyle")
            .with_ns_decl("cs", cs)
            .with_attribute("meth", "cycle")
            .with_attribute("id", "10")
            .with_child(
                OpenXmlElement::new(
                    "a",
                    crate::namespace::ns::DRAWINGML.uri,
                    "schemeClr",
                )
                .with_attribute("val", "accent1"),
            );
        self.package.set_part(
            style_uri.clone(),
            content_type::CHART_STYLE,
            write_element(&style)?,
        );
        self.package.set_part(
            colors_uri.clone(),
            content_type::CHART_COLOR_STYLE,
            write_element(&colors)?,
        );
        self.package.add_part_relationship(
            chart_uri,
            rel::CHART_STYLE,
            &style_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            chart_uri,
            rel::CHART_COLOR_STYLE,
            &colors_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((style_uri, colors_uri))
    }

    /// Add a dialog sheet shell.
    pub fn add_dialogsheet(&mut self, name: &str) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/dialogsheets/sheet{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let root = OpenXmlElement::new("x", x, "dialogsheet")
            .with_ns_decl("x", x)
            .with_child(
                OpenXmlElement::new("x", x, "sheetPr").with_child(
                    OpenXmlElement::new("x", x, "pageSetUpPr")
                        .with_attribute("fitToPage", "0"),
                ),
            )
            .with_child(
                OpenXmlElement::new("x", x, "sheetViews").with_child(
                    OpenXmlElement::new("x", x, "sheetView")
                        .with_attribute("workbookViewId", "0"),
                ),
            );
        self.package.set_part(
            uri.clone(),
            content_type::SPREADSHEET_DIALOGSHEET,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::DIALOGSHEET,
            &uri,
            RelationshipTargetMode::Internal,
        );
        let sheet_id = (self.sheets.len() as u32) + 2000;
        let mut wb_root = parse_element(
            self.package
                .opc()
                .get_part(&wb_uri)
                .ok_or_else(|| Error::PartNotFound(wb_uri.to_string()))?,
        )?;
        if let Some(sheets_el) = wb_root.child_mut("sheets") {
            sheets_el.append_child(sheet(name, sheet_id, &rid));
        }
        let wb_xml = write_element(&wb_root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            wb_xml,
        );
        Ok((uri, rid))
    }

    /// Add a named sheet views part shell for a worksheet.
    pub fn add_named_sheet_views(
        &mut self,
        sheet_name: &str,
        view_name: &str,
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/namedSheetViews/namedSheetView{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let xnsv = "http://schemas.microsoft.com/office/spreadsheetml/2019/namedsheetviews";
        let root = OpenXmlElement::new("xnsv", xnsv, "namedSheetViews")
            .with_ns_decl("xnsv", xnsv)
            .with_child(
                OpenXmlElement::new("xnsv", xnsv, "namedSheetView")
                    .with_attribute("name", view_name)
                    .with_attribute("id", format!("{{{index}}}")),
            );
        self.package.set_part(
            uri.clone(),
            content_type::NAMED_SHEET_VIEW,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::NAMED_SHEET_VIEW,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// List named sheet views as `(name, uri)`.
    pub fn list_named_sheet_views(&self) -> Result<Vec<(String, PackUri)>> {
        let mut out = Vec::new();
        for uri in self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/namedSheetViews/"))
            
        {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            for v in root.descendants().filter(|e| e.local_name == "namedSheetView") {
                let name = v.get_attribute("name").unwrap_or("").to_string();
                out.push((name, uri.clone()));
            }
        }
        Ok(out)
    }

    /// Rename a named sheet view by current name.
    pub fn rename_named_sheet_view(&mut self, old_name: &str, new_name: &str) -> Result<bool> {
        for (name, uri) in self.list_named_sheet_views()? {
            if name != old_name {
                continue;
            }
            let Some(data) = self.package.opc().get_part(&uri) else {
                return Ok(false);
            };
            let mut root = parse_element(data)?;
            let mut found = false;
            fn visit(el: &mut OpenXmlElement, old: &str, new: &str, found: &mut bool) {
                if el.local_name == "namedSheetView" && el.get_attribute("name") == Some(old) {
                    el.set_attribute("name", new);
                    *found = true;
                    return;
                }
                for c in el.children.iter_mut() {
                    visit(c, old, new, found);
                }
            }
            visit(&mut root, old_name, new_name, &mut found);
            if found {
                self.package.set_part(
                    uri,
                    content_type::NAMED_SHEET_VIEW,
                    write_element(&root)?,
                );
            }
            return Ok(found);
        }
        Ok(false)
    }

    /// Add a custom sheet view shell under `customSheetViews` on the worksheet.
    ///
    /// `guid` should be a unique identifier string (braces optional).
    pub fn add_custom_sheet_view(
        &mut self,
        sheet_name: &str,
        guid: &str,
        scale: u32,
        show_grid_lines: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let view = OpenXmlElement::new("x", x, "customSheetView")
            .with_attribute("guid", guid)
            .with_attribute("scale", scale.to_string())
            .with_attribute("showGridLines", if show_grid_lines { "1" } else { "0" });
        if let Some(container) = root.child_mut("customSheetViews") {
            container.append_child(view);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| {
                    matches!(
                        c.local_name.as_str(),
                        "drawing" | "legacyDrawing" | "tableParts" | "extLst"
                    )
                })
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "customSheetViews").with_child(view),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// List custom sheet views as `(guid, scale)`.
    pub fn list_custom_sheet_views(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, u32)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("customSheetViews") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("customSheetView")
            .map(|v| {
                (
                    v.get_attribute("guid").unwrap_or("").to_string(),
                    v.get_attribute("scale")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(100),
                )
            })
            .collect())
    }

    /// Update attributes on a custom sheet view identified by `guid`.
    ///
    /// Pass `None` for any field to leave it unchanged.
    pub fn set_custom_sheet_view_attrs(
        &mut self,
        sheet_name: &str,
        guid: &str,
        scale: Option<u32>,
        show_grid_lines: Option<bool>,
        show_formulas: Option<bool>,
        show_row_col: Option<bool>,
        color_id: Option<u32>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("customSheetViews") else {
            return Ok(false);
        };
        let mut found = false;
        for view in container
            .children
            .iter_mut()
            .filter(|c| c.local_name == "customSheetView")
        {
            if view.get_attribute("guid").unwrap_or("") != guid {
                continue;
            }
            found = true;
            if let Some(s) = scale {
                view.set_attribute("scale", s.to_string());
            }
            if let Some(g) = show_grid_lines {
                view.set_attribute("showGridLines", if g { "1" } else { "0" });
            }
            if let Some(f) = show_formulas {
                view.set_attribute("showFormulas", if f { "1" } else { "0" });
            }
            if let Some(r) = show_row_col {
                view.set_attribute("showRowCol", if r { "1" } else { "0" });
            }
            if let Some(c) = color_id {
                view.set_attribute("colorId", c.to_string());
            }
            break;
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Remove a single custom sheet view by guid. Returns whether found.
    pub fn remove_custom_sheet_view(&mut self, sheet_name: &str, guid: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("customSheetViews") else {
            return Ok(false);
        };
        let before = container.children.len();
        container
            .children
            .retain(|c| {
                !(c.local_name == "customSheetView"
                    && c.get_attribute("guid").unwrap_or("") == guid)
            });
        let removed = container.children.len() < before;
        if removed {
            if container.children.is_empty() {
                root.children.retain(|c| c.local_name != "customSheetViews");
            }
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Number of custom sheet views.
    pub fn custom_sheet_view_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_custom_sheet_views(sheet_name)?.len())
    }

    /// Whether custom sheet views exist.
    pub fn has_custom_sheet_views(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.custom_sheet_view_count(sheet_name)? > 0)
    }

    /// Clear custom sheet views. Returns how many were removed.
    pub fn clear_custom_sheet_views(&mut self, sheet_name: &str) -> Result<usize> {
        let n = self.custom_sheet_view_count(sheet_name)?;
        if n == 0 {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children
            .retain(|c| c.local_name != "customSheetViews");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(n)
    }

    /// Set phonetic properties (`phoneticPr`) on a worksheet.
    pub fn set_phonetic_properties(
        &mut self,
        sheet_name: &str,
        font_id: u32,
        type_: &str,
        alignment: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        root.children.retain(|c| c.local_name != "phoneticPr");
        let el = OpenXmlElement::new("x", x, "phoneticPr")
            .with_attribute("fontId", font_id.to_string())
            .with_attribute("type", type_)
            .with_attribute("alignment", alignment);
        let insert_at = root
            .children
            .iter()
            .position(|c| {
                matches!(
                    c.local_name.as_str(),
                    "conditionalFormatting"
                        | "dataValidations"
                        | "hyperlinks"
                        | "printOptions"
                        | "pageMargins"
                        | "pageSetup"
                        | "headerFooter"
                        | "drawing"
                        | "tableParts"
                        | "extLst"
                )
            })
            .unwrap_or(root.children.len());
        root.children.insert(insert_at, el);
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read phonetic properties as `(font_id, type, alignment)`.
    pub fn phonetic_properties(
        &self,
        sheet_name: &str,
    ) -> Result<Option<(u32, String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(pp) = root.child("phoneticPr") else {
            return Ok(None);
        };
        Ok(Some((
            pp.get_attribute("fontId")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            pp.get_attribute("type").unwrap_or("fullwidthKatakana").to_string(),
            pp.get_attribute("alignment").unwrap_or("left").to_string(),
        )))
    }

    /// Whether phoneticPr is present.
    pub fn has_phonetic_properties(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.phonetic_properties(sheet_name)?.is_some())
    }

    /// Clear phoneticPr. Returns whether present.
    pub fn clear_phonetic_properties(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "phoneticPr");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Add a custom workbook view shell under workbook `customWorkbookViews`.
    pub fn add_custom_workbook_view(
        &mut self,
        name: &str,
        guid: &str,
        active_sheet_id: u32,
    ) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let view = OpenXmlElement::new("x", x, "customWorkbookView")
            .with_attribute("name", name)
            .with_attribute("guid", guid)
            .with_attribute("activeSheetId", active_sheet_id.to_string())
            .with_attribute("windowWidth", "20000")
            .with_attribute("windowHeight", "15000");
        if let Some(container) = root.child_mut("customWorkbookViews") {
            container.append_child(view);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| matches!(c.local_name.as_str(), "calcPr" | "extLst"))
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "customWorkbookViews").with_child(view),
            );
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// List custom workbook views as `(name, guid, active_sheet_id)`.
    pub fn list_custom_workbook_views(&self) -> Result<Vec<(String, String, u32)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("customWorkbookViews") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("customWorkbookView")
            .map(|v| {
                (
                    v.get_attribute("name").unwrap_or("").to_string(),
                    v.get_attribute("guid").unwrap_or("").to_string(),
                    v.get_attribute("activeSheetId")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                )
            })
            .collect())
    }

    /// Number of custom workbook views.
    pub fn custom_workbook_view_count(&self) -> Result<usize> {
        Ok(self.list_custom_workbook_views()?.len())
    }

    /// Whether custom workbook views exist.
    pub fn has_custom_workbook_views(&self) -> Result<bool> {
        Ok(self.custom_workbook_view_count()? > 0)
    }

    /// Clear custom workbook views. Returns how many were removed.
    pub fn clear_custom_workbook_views(&mut self) -> Result<usize> {
        let n = self.custom_workbook_view_count()?;
        if n == 0 {
            return Ok(0);
        }
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        root.children
            .retain(|c| c.local_name != "customWorkbookViews");
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(n)
    }

    /// Update attributes on a custom workbook view identified by `guid`.
    ///
    /// Pass `None` to leave a field unchanged.
    pub fn set_custom_workbook_view_attrs(
        &mut self,
        guid: &str,
        name: Option<&str>,
        active_sheet_id: Option<u32>,
        window_width: Option<u32>,
        window_height: Option<u32>,
        x_window: Option<i32>,
        y_window: Option<i32>,
        maximized: Option<bool>,
        minimzed: Option<bool>,
        show_horizontal_scroll: Option<bool>,
        show_vertical_scroll: Option<bool>,
        show_sheet_tabs: Option<bool>,
    ) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(container) = root.child_mut("customWorkbookViews") else {
            return Ok(false);
        };
        let mut found = false;
        for view in container
            .children
            .iter_mut()
            .filter(|c| c.local_name == "customWorkbookView")
        {
            if view.get_attribute("guid").unwrap_or("") != guid {
                continue;
            }
            found = true;
            if let Some(n) = name {
                view.set_attribute("name", n);
            }
            if let Some(id) = active_sheet_id {
                view.set_attribute("activeSheetId", id.to_string());
            }
            if let Some(w) = window_width {
                view.set_attribute("windowWidth", w.to_string());
            }
            if let Some(h) = window_height {
                view.set_attribute("windowHeight", h.to_string());
            }
            if let Some(x) = x_window {
                view.set_attribute("xWindow", x.to_string());
            }
            if let Some(y) = y_window {
                view.set_attribute("yWindow", y.to_string());
            }
            if let Some(m) = maximized {
                view.set_attribute("maximized", if m { "1" } else { "0" });
            }
            if let Some(m) = minimzed {
                view.set_attribute("minimized", if m { "1" } else { "0" });
            }
            if let Some(s) = show_horizontal_scroll {
                view.set_attribute("showHorizontalScroll", if s { "1" } else { "0" });
            }
            if let Some(s) = show_vertical_scroll {
                view.set_attribute("showVerticalScroll", if s { "1" } else { "0" });
            }
            if let Some(s) = show_sheet_tabs {
                view.set_attribute("showSheetTabs", if s { "1" } else { "0" });
            }
            break;
        }
        if found {
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        }
        Ok(found)
    }

    /// Remove a single custom workbook view by guid. Returns whether found.
    pub fn remove_custom_workbook_view(&mut self, guid: &str) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(container) = root.child_mut("customWorkbookViews") else {
            return Ok(false);
        };
        let before = container.children.len();
        container.children.retain(|c| {
            !(c.local_name == "customWorkbookView"
                && c.get_attribute("guid").unwrap_or("") == guid)
        });
        let removed = container.children.len() < before;
        if removed {
            if container.children.is_empty() {
                root.children
                    .retain(|c| c.local_name != "customWorkbookViews");
            }
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Set data consolidate shell (`dataConsolidate`) with function and optional link.
    pub fn set_data_consolidate(
        &mut self,
        sheet_name: &str,
        function: &str,
        link: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        root.children.retain(|c| c.local_name != "dataConsolidate");
        let el = OpenXmlElement::new("x", x, "dataConsolidate")
            .with_attribute("function", function)
            .with_attribute("link", if link { "1" } else { "0" })
            .with_child(OpenXmlElement::new("x", x, "dataRefs"));
        root.children.push(el);
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether dataConsolidate is present.
    pub fn has_data_consolidate(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root.child("dataConsolidate").is_some())
    }

    /// Read data consolidate function name.
    pub fn data_consolidate_function(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("dataConsolidate")
            .and_then(|d| d.get_attribute("function").map(|s| s.to_string())))
    }

    /// Clear dataConsolidate. Returns whether present.
    pub fn clear_data_consolidate(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "dataConsolidate");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Add threaded comments part for a worksheet (Excel 365).
    ///
    /// Each entry is `(id, person_id, text)`.
    pub fn add_threaded_comments(
        &mut self,
        sheet_name: &str,
        comments: &[(&str, &str, &str)],
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/threadedComments/threadedComment{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let x = "http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments";
        let mut root =
            OpenXmlElement::new("x", x, "ThreadedComments").with_ns_decl("x", x);
        for (id, person_id, text) in comments {
            root.append_child(
                OpenXmlElement::new("x", x, "threadedComment")
                    .with_attribute("ref", "A1")
                    .with_attribute("dT", "2020-01-01T00:00:00")
                    .with_attribute("personId", *person_id)
                    .with_attribute("id", *id)
                    .with_child(OpenXmlElement::new("x", x, "text").with_text(*text)),
            );
        }
        self.package.set_part(
            uri.clone(),
            content_type::THREADED_COMMENT,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::THREADED_COMMENT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add persons part for threaded comments.
    pub fn add_persons(&mut self, persons: &[(&str, &str)]) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/persons/person.xml");
        let x = "http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments";
        let mut root = OpenXmlElement::new("x", x, "personList").with_ns_decl("x", x);
        for (id, display_name) in persons {
            root.append_child(
                OpenXmlElement::new("x", x, "person")
                    .with_attribute("displayName", *display_name)
                    .with_attribute("id", *id)
                    .with_attribute("userId", *id)
                    .with_attribute("providerId", "None"),
            );
        }
        self.package.set_part(
            uri.clone(),
            content_type::PERSON,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::PERSON,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add shared workbook revision headers + empty log + users shells.
    pub fn add_revision_tracking_shell(&mut self) -> Result<(PackUri, PackUri, PackUri)> {
        let wb_uri = self.ensure_workbook()?;
        let headers_uri = PackUri::new("/xl/revisions/revisionHeaders.xml");
        let log_uri = PackUri::new("/xl/revisions/revisionLog1.xml");
        let users_uri = PackUri::new("/xl/revisions/userNames.xml");
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let headers = OpenXmlElement::new("x", x, "headers")
            .with_ns_decl("x", x)
            .with_ns_decl(
                "r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            )
            .with_attribute("guid", "{00000000-0000-0000-0000-000000000000}")
            .with_attribute("lastGuid", "{00000000-0000-0000-0000-000000000000}")
            .with_attribute("shared", "1")
            .with_attribute("diskRevisions", "1")
            .with_attribute("history", "1")
            .with_attribute("trackRevisions", "1")
            .with_attribute("exclusive", "0")
            .with_attribute("revisionId", "1")
            .with_attribute("version", "1")
            .with_child(
                OpenXmlElement::new("x", x, "header")
                    .with_attribute("guid", "{11111111-1111-1111-1111-111111111111}")
                    .with_attribute("dateTime", "2020-01-01T00:00:00")
                    .with_attribute("maxSheetId", "1")
                    .with_attribute("userName", "User")
                    .with_attribute_qname("r:id", "rId1")
                    .with_attribute("minRId", "1")
                    .with_attribute("maxRId", "1"),
            );
        let log = OpenXmlElement::new("x", x, "revisions")
            .with_ns_decl("x", x)
            .with_child(
                OpenXmlElement::new("x", x, "rrc")
                    .with_attribute("rId", "1")
                    .with_attribute("sId", "1")
                    .with_attribute("eol", "1"),
            );
        let users = OpenXmlElement::new("x", x, "users")
            .with_ns_decl("x", x)
            .with_attribute("count", "1")
            .with_child(
                OpenXmlElement::new("x", x, "userInfo")
                    .with_attribute("guid", "{11111111-1111-1111-1111-111111111111}")
                    .with_attribute("name", "User")
                    .with_attribute("id", "1")
                    .with_attribute("dateTime", "2020-01-01T00:00:00"),
            );
        self.package.set_part(
            headers_uri.clone(),
            content_type::REVISION_HEADERS,
            write_element(&headers)?,
        );
        self.package.set_part(
            log_uri.clone(),
            content_type::REVISION_LOG,
            write_element(&log)?,
        );
        self.package.set_part(
            users_uri.clone(),
            content_type::USERS,
            write_element(&users)?,
        );
        self.package.add_part_relationship(
            &headers_uri,
            rel::REVISION_LOG,
            &log_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            &wb_uri,
            rel::REVISION_HEADERS,
            &headers_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            &wb_uri,
            rel::USERS,
            &users_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((headers_uri, log_uri, users_uri))
    }

    /// Add worksheet sort map part shell.
    pub fn add_sort_map(&mut self, sheet_name: &str) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/worksheetSortMap/sheet{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let ns = "http://schemas.microsoft.com/office/excel/2006/main";
        let root = OpenXmlElement::new("ns", ns, "worksheetSortMap")
            .with_ns_decl("ns", ns)
            .with_child(
                OpenXmlElement::new("ns", ns, "rowCol")
                    .with_attribute("ref", "A1")
                    .with_child(
                        OpenXmlElement::new("ns", ns, "row")
                            .with_attribute("r", "1")
                            .with_attribute("s", "1"),
                    ),
            );
        self.package.set_part(
            uri.clone(),
            content_type::SORT_MAP,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::SORT_MAP,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add cell metadata part shell.
    pub fn add_cell_metadata(&mut self) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/metadata.xml");
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let root = OpenXmlElement::new("x", x, "metadata")
            .with_ns_decl("x", x)
            .with_child(
                OpenXmlElement::new("x", x, "metadataTypes")
                    .with_attribute("count", "0"),
            )
            .with_child(
                OpenXmlElement::new("x", x, "metadataStrings")
                    .with_attribute("count", "0"),
            )
            .with_child(
                OpenXmlElement::new("x", x, "mdxMetadata")
                    .with_attribute("count", "0"),
            )
            .with_child(OpenXmlElement::new("x", x, "futureMetadata").with_attribute("count", "0"))
            .with_child(
                OpenXmlElement::new("x", x, "cellMetadata").with_attribute("count", "0"),
            )
            .with_child(
                OpenXmlElement::new("x", x, "valueMetadata").with_attribute("count", "0"),
            );
        self.package.set_part(
            uri.clone(),
            content_type::CELL_METADATA,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::CELL_METADATA,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add spreadsheet printer settings binary shell for a worksheet.
    pub fn add_sheet_printer_settings(
        &mut self,
        sheet_name: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/printerSettings/printerSettings{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            content_type::SPREADSHEET_PRINTER_SETTINGS,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::PRINTER_SETTINGS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add Excel attached toolbars binary shell.
    pub fn add_attached_toolbars(&mut self, data: impl Into<Vec<u8>>) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/attachedToolbars.bin");
        self.package.set_part(
            uri.clone(),
            content_type::EXCEL_ATTACHED_TOOLBARS,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::ATTACHED_TOOLBARS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add rich styles part shell.
    pub fn add_rich_styles(&mut self) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/richData/richStyles.xml");
        let x = "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2";
        let root = OpenXmlElement::new("x", x, "richStyleSheet")
            .with_ns_decl("x", x);
        self.package.set_part(
            uri.clone(),
            content_type::RICH_STYLES,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::RICH_STYLES,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add supporting property bag parts shell.
    pub fn add_supporting_property_bags(&mut self) -> Result<(PackUri, PackUri)> {
        let wb_uri = self.ensure_workbook()?;
        let struct_uri = PackUri::new("/xl/richData/rdsupportingpropertybagstructure.xml");
        let data_uri = PackUri::new("/xl/richData/rdsupportingpropertybag.xml");
        let x = "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2";
        let structure = OpenXmlElement::new("x", x, "spbStructures")
            .with_ns_decl("x", x)
            .with_attribute("count", "0");
        let data = OpenXmlElement::new("x", x, "spbData")
            .with_ns_decl("x", x)
            .with_attribute("count", "0");
        self.package.set_part(
            struct_uri.clone(),
            content_type::SUPPORTING_PROPERTY_BAG_STRUCTURE,
            write_element(&structure)?,
        );
        self.package.set_part(
            data_uri.clone(),
            content_type::SUPPORTING_PROPERTY_BAG,
            write_element(&data)?,
        );
        self.package.add_part_relationship(
            &wb_uri,
            rel::SUPPORTING_PROPERTY_BAG_STRUCTURE,
            &struct_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            &wb_uri,
            rel::SUPPORTING_PROPERTY_BAG,
            &data_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((data_uri, struct_uri))
    }

    /// Add rdArray part shell.
    pub fn add_rd_array(&mut self) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/richData/rdarray.xml");
        let x = "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2";
        let root = OpenXmlElement::new("x", x, "arrayData")
            .with_ns_decl("x", x)
            .with_attribute("count", "0");
        self.package.set_part(
            uri.clone(),
            content_type::RD_ARRAY,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::RD_ARRAY,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add control properties part related from a worksheet.
    pub fn add_control_properties(
        &mut self,
        sheet_name: &str,
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/ctrlProps/ctrlProp{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let x = "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main";
        let root = OpenXmlElement::new("x", x, "formControlPr")
            .with_ns_decl("x", x)
            .with_attribute("objectType", "Drop")
            .with_attribute("dx", "15")
            .with_attribute("noThreeD", "1");
        self.package.set_part(
            uri.clone(),
            content_type::CONTROL_PROPS,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::CONTROL_PROPS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Attach a chart drawing (user shapes) part to a chart.
    pub fn add_chart_drawing(
        &mut self,
        chart_uri: &PackUri,
    ) -> Result<(PackUri, String)> {
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/drawings/drawing{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let cdr = "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing";
        let a = crate::namespace::ns::DRAWINGML.uri;
        let root = OpenXmlElement::new("cdr", cdr, "userShapes")
            .with_ns_decl("cdr", cdr)
            .with_ns_decl("a", a);
        self.package.set_part(
            uri.clone(),
            content_type::CHART_DRAWING,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            chart_uri,
            rel::CHART_DRAWING,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add an extended chart (chartEx) part shell.
    pub fn add_extended_chart(
        &mut self,
        title: &str,
    ) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/charts/chartEx{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let cx = "http://schemas.microsoft.com/office/drawing/2014/chartex";
        let a = crate::namespace::ns::DRAWINGML.uri;
        let root = OpenXmlElement::new("cx", cx, "chartSpace")
            .with_ns_decl("cx", cx)
            .with_ns_decl("a", a)
            .with_child(
                OpenXmlElement::new("cx", cx, "chart")
                    .with_child(
                        OpenXmlElement::new("cx", cx, "title").with_child(
                            OpenXmlElement::new("cx", cx, "tx").with_child(
                                OpenXmlElement::new("cx", cx, "rich").with_child(
                                    OpenXmlElement::new("a", a, "p").with_child(
                                        OpenXmlElement::new("a", a, "r").with_child(
                                            OpenXmlElement::new("a", a, "t").with_text(title),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    )
                    .with_child(OpenXmlElement::new("cx", cx, "plotArea")),
            );
        self.package.set_part(
            uri.clone(),
            content_type::EXTENDED_CHART,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::EXTENDED_CHART,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add international macrosheet shell.
    pub fn add_intl_macrosheet(&mut self, name: &str) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/macrosheets/intlSheet{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let root = OpenXmlElement::new("x", x, "intlMacrosheet")
            .with_ns_decl("x", x)
            .with_child(OpenXmlElement::new("x", x, "sheetData"));
        self.package.set_part(
            uri.clone(),
            content_type::INT_MACRO_SHEET,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::INT_MACRO_SHEET,
            &uri,
            RelationshipTargetMode::Internal,
        );
        let sheet_id = (self.sheets.len() as u32) + 4000;
        let mut wb_root = parse_element(
            self.package
                .opc()
                .get_part(&wb_uri)
                .ok_or_else(|| Error::PartNotFound(wb_uri.to_string()))?,
        )?;
        if let Some(sheets_el) = wb_root.child_mut("sheets") {
            sheets_el.append_child(sheet(name, sheet_id, &rid));
        }
        let wb_xml = write_element(&wb_root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            wb_xml,
        );
        Ok((uri, rid))
    }

    /// Add rich value web image part shell.
    pub fn add_rich_value_web_image(&mut self) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/richData/rdRichValueWebImage.xml");
        let x = "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2";
        let root = OpenXmlElement::new("x", x, "webImages")
            .with_ns_decl("x", x)
            .with_attribute("count", "0");
        self.package.set_part(
            uri.clone(),
            content_type::RD_RICH_VALUE_WEB_IMAGE,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::RD_RICH_VALUE_WEB_IMAGE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add an embedded ActiveX control persistence binary shell.
    pub fn add_embedded_control(
        &mut self,
        sheet_name: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/activeX/activeX{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            content_type::EMBEDDED_CONTROL_PERSISTENCE,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::EMBEDDED_CONTROL_PERSISTENCE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a custom property binary/XML part related from a worksheet.
    ///
    /// Corresponds to C# `CustomPropertyPart` (spreadsheet content type).
    /// Returns `(uri, relationship_id)`.
    pub fn add_custom_property(
        &mut self,
        sheet_name: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/customProperty{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            content_type::CUSTOM_PROPERTY_SPREADSHEET,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::CUSTOM_PROPERTY,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add an embedded font part related from the workbook.
    ///
    /// `content_type` should be one of `content_type::FONT_DATA`, `FONT_TTF`, or
    /// `FONT_ODTTF`. Returns `(uri, relationship_id)`.
    pub fn add_font_part(
        &mut self,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let ext = extension.trim_start_matches('.');
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/fonts/font{index}.{ext}"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package
            .set_part(uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::FONT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }


    /// Whether any embedded font parts exist under `/xl/fonts/`.
    pub fn has_font_parts(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/fonts/"))
    }

    /// Count embedded font parts under `/xl/fonts/`.
    pub fn font_part_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/fonts/"))
            .count()
    }

    /// List embedded font part URIs.
    pub fn list_font_parts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/fonts/"))
            
            .collect()
    }

    /// Remove all embedded font parts and related main-part font relationships.
    pub fn clear_font_parts(&mut self) -> Result<usize> {
        let uris = self.list_font_parts();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::FONT)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Add a single-cell tables part shell for a worksheet.
    pub fn add_single_cell_table(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
        id: u32,
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/tables/tableSingleCells{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let root = OpenXmlElement::new("x", x, "singleXmlCells")
            .with_ns_decl("x", x)
            .with_child(
                OpenXmlElement::new("x", x, "singleXmlCell")
                    .with_attribute("id", id.to_string())
                    .with_attribute("r", cell_ref)
                    .with_attribute("connectionId", "1")
                    .with_child(
                        OpenXmlElement::new("x", x, "xmlCellPr")
                            .with_attribute("id", "1")
                            .with_attribute("uniqueName", format!("Cell{id}"))
                            .with_child(
                                OpenXmlElement::new("x", x, "xmlPr")
                                    .with_attribute("mapId", "1")
                                    .with_attribute("xpath", "/root")
                                    .with_attribute("xmlDataType", "string"),
                            ),
                    ),
            );
        self.package.set_part(
            uri.clone(),
            content_type::SINGLE_CELL_TABLE,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::SINGLE_CELL_TABLE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add rich data value parts shell (Excel rich values).
    pub fn add_rich_value_shell(&mut self) -> Result<(PackUri, PackUri, PackUri)> {
        let wb_uri = self.ensure_workbook()?;
        let types_uri = PackUri::new("/xl/richData/rdRichValueTypes.xml");
        let struct_uri = PackUri::new("/xl/richData/rdrichvaluestructure.xml");
        let data_uri = PackUri::new("/xl/richData/rdrichvalue.xml");
        let rv = "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata";
        let types = OpenXmlElement::new("rv", rv, "rvTypesInfo")
            .with_ns_decl("rv", rv)
            .with_child(OpenXmlElement::new("rv", rv, "global"));
        let structure = OpenXmlElement::new("rv", rv, "rvStructures")
            .with_ns_decl("rv", rv)
            .with_attribute("count", "0");
        let data = OpenXmlElement::new("rv", rv, "rvData")
            .with_ns_decl("rv", rv)
            .with_attribute("count", "0");
        self.package.set_part(
            types_uri.clone(),
            content_type::RICH_VALUE_TYPES,
            write_element(&types)?,
        );
        self.package.set_part(
            struct_uri.clone(),
            content_type::RICH_VALUE_STRUCTURE,
            write_element(&structure)?,
        );
        self.package.set_part(
            data_uri.clone(),
            content_type::RICH_VALUE,
            write_element(&data)?,
        );
        for (uri, rel_ty) in [
            (&types_uri, rel::RICH_VALUE_TYPES),
            (&struct_uri, rel::RICH_VALUE_STRUCTURE),
            (&data_uri, rel::RICH_VALUE),
        ] {
            self.package.add_part_relationship(
                &wb_uri,
                rel_ty,
                uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok((data_uri, struct_uri, types_uri))
    }

    /// Add a feature property bag part shell.
    pub fn add_feature_property_bag(&mut self) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/featurePropertyBag/featurePropertyBag.xml");
        let xfpb = "http://schemas.microsoft.com/office/spreadsheetml/2022/featurepropertybag";
        let root = OpenXmlElement::new("xfpb", xfpb, "FeaturePropertyBags")
            .with_ns_decl("xfpb", xfpb)
            .with_attribute("count", "0");
        self.package.set_part(
            uri.clone(),
            content_type::FEATURE_PROPERTY_BAG,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::FEATURE_PROPERTY_BAG,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a macro sheet shell (Excel 4.0 macrosheet).
    pub fn add_macrosheet(&mut self, name: &str) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/macrosheets/sheet{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let root = OpenXmlElement::new("x", x, "macrosheet")
            .with_ns_decl("x", x)
            .with_child(
                OpenXmlElement::new("x", x, "sheetViews").with_child(
                    OpenXmlElement::new("x", x, "sheetView")
                        .with_attribute("workbookViewId", "0"),
                ),
            )
            .with_child(OpenXmlElement::new("x", x, "sheetData"));
        self.package.set_part(
            uri.clone(),
            content_type::MACRO_SHEET,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::MACRO_SHEET,
            &uri,
            RelationshipTargetMode::Internal,
        );
        let sheet_id = (self.sheets.len() as u32) + 3000;
        let mut wb_root = parse_element(
            self.package
                .opc()
                .get_part(&wb_uri)
                .ok_or_else(|| Error::PartNotFound(wb_uri.to_string()))?,
        )?;
        if let Some(sheets_el) = wb_root.child_mut("sheets") {
            sheets_el.append_child(sheet(name, sheet_id, &rid));
        }
        let wb_xml = write_element(&wb_root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            wb_xml,
        );
        Ok((uri, rid))
    }

    /// Add a theme override part related from a chart or drawing.
    pub fn add_theme_override(
        &mut self,
        parent_uri: &PackUri,
    ) -> Result<(PackUri, String)> {
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/theme/themeOverride{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let a = crate::namespace::ns::DRAWINGML.uri;
        let root = OpenXmlElement::new("a", a, "themeOverride")
            .with_ns_decl("a", a)
            .with_child(
                OpenXmlElement::new("a", a, "clrScheme")
                    .with_attribute("name", "Override")
                    .with_child(
                        OpenXmlElement::new("a", a, "dk1").with_child(
                            OpenXmlElement::new("a", a, "sysClr")
                                .with_attribute("val", "windowText")
                                .with_attribute("lastClr", "000000"),
                        ),
                    )
                    .with_child(
                        OpenXmlElement::new("a", a, "lt1").with_child(
                            OpenXmlElement::new("a", a, "sysClr")
                                .with_attribute("val", "window")
                                .with_attribute("lastClr", "FFFFFF"),
                        ),
                    ),
            );
        self.package.set_part(
            uri.clone(),
            content_type::THEME_OVERRIDE,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            parent_uri,
            rel::THEME_OVERRIDE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add custom data + properties parts shell.
    pub fn add_custom_data(
        &mut self,
        data: impl Into<Vec<u8>>,
        item_id: &str,
    ) -> Result<(PackUri, PackUri)> {
        let wb_uri = self.ensure_workbook()?;
        let mut index = 1u32;
        let data_uri = loop {
            let c = PackUri::new(format!("/xl/customData/customData{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let props_uri = PackUri::new(format!("/xl/customData/customDataProps{index}.xml"));
        self.package.set_part(
            data_uri.clone(),
            content_type::CUSTOM_DATA,
            data.into(),
        );
        let x = "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main";
        let props = OpenXmlElement::new("x", x, "datastoreItem")
            .with_ns_decl("x", x)
            .with_attribute("id", item_id);
        self.package.set_part(
            props_uri.clone(),
            content_type::CUSTOM_DATA_PROPS,
            write_element(&props)?,
        );
        self.package.add_part_relationship(
            &props_uri,
            rel::CUSTOM_DATA,
            &data_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            &wb_uri,
            rel::CUSTOM_DATA_PROPS,
            &props_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((data_uri, props_uri))
    }

    /// Add a custom XML maps / schema mappings part shell (`xl/xmlMaps.xml`).
    pub fn add_xml_maps(&mut self, map_id: u32, root_element: &str, schema: &str) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/xmlMaps.xml");
        // Simplified MapInfo structure
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<MapInfo xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" SelectionNamespaces="">
  <Schema ID="Schema1">{schema}</Schema>
  <Map ID="{map_id}" Name="Map{map_id}" RootElement="{root_element}" SchemaID="Schema1" ShowImportExportValidationErrors="false" AutoFit="true" Append="false" PreserveSortAFLayout="true" PreserveFormat="true"/>
</MapInfo>"#
        );
        self.package.set_part(
            uri.clone(),
            content_type::CUSTOM_XML_MAPPINGS,
            xml.into_bytes(),
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| {
                rels.get_by_type(rel::CUSTOM_XML_MAPPINGS)
                    .map(|r| r.id.clone())
            })
        {
            return Ok((uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::CUSTOM_XML_MAPPINGS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a query table part shell related from a worksheet.
    ///
    /// Returns `(query_table_uri, relationship_id)`.
    pub fn add_query_table(
        &mut self,
        sheet_name: &str,
        name: &str,
        connection_id: u32,
    ) -> Result<(PackUri, String)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut index = 1u32;
        let uri = loop {
            let candidate = PackUri::new(format!("/xl/queryTables/queryTable{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let root = OpenXmlElement::new("x", x, "queryTable")
            .with_ns_decl("x", x)
            .with_attribute("name", name)
            .with_attribute("connectionId", connection_id.to_string())
            .with_attribute("autoFormatId", "16")
            .with_attribute("applyNumberFormats", "0")
            .with_attribute("applyBorderFormats", "0")
            .with_attribute("applyFontFormats", "0")
            .with_attribute("applyPatternFormats", "0")
            .with_attribute("applyAlignmentFormats", "0")
            .with_attribute("applyWidthHeightFormats", "1");
        self.package.set_part(
            uri.clone(),
            content_type::SPREADSHEET_QUERY_TABLE,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &sheet_uri,
            rel::QUERY_TABLE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a volatile dependencies part shell.
    pub fn add_volatile_dependencies(&mut self) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/volatileDependencies.xml");
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let root = OpenXmlElement::new("x", x, "volTypes").with_ns_decl("x", x);
        self.package.set_part(
            uri.clone(),
            content_type::SPREADSHEET_VOLATILE_DEPS,
            write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| {
                rels.get_by_type(rel::VOLATILE_DEPENDENCIES)
                    .map(|r| r.id.clone())
            })
        {
            return Ok((uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::VOLATILE_DEPENDENCIES,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a workbook connections part shell listing named connections.
    ///
    /// Each entry is `(name, connection_type, source)` e.g. `("Query1", "OLEDB", "Provider=...")`.
    pub fn add_connections(
        &mut self,
        connections: &[(&str, &str, &str)],
    ) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let uri = PackUri::new("/xl/connections.xml");
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut root =
            OpenXmlElement::new("x", x, "connections").with_ns_decl("x", x);
        for (i, (name, ty, source)) in connections.iter().enumerate() {
            root.append_child(
                OpenXmlElement::new("x", x, "connection")
                    .with_attribute("id", (i + 1).to_string())
                    .with_attribute("name", *name)
                    .with_attribute("type", "1")
                    .with_attribute("refreshedVersion", "0")
                    .with_attribute("background", "1")
                    .with_child(
                        OpenXmlElement::new("x", x, "dbPr")
                            .with_attribute("connection", *source)
                            .with_attribute("command", *ty),
                    ),
            );
        }
        self.package.set_part(
            uri.clone(),
            content_type::SPREADSHEET_CONNECTIONS,
            write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| rels.get_by_type(rel::CONNECTIONS).map(|r| r.id.clone()))
        {
            return Ok((uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::CONNECTIONS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a slicer + slicer cache shell (Excel 2010+).
    ///
    /// Minimal valid structure for package round-trip; Excel may rewrite on open.
    /// Returns `(slicers_uri, cache_uri)`.
    pub fn add_slicer_shell(
        &mut self,
        sheet_name: &str,
        name: &str,
        cache_name: &str,
    ) -> Result<(PackUri, PackUri)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut cindex = 1u32;
        let cache_uri = loop {
            let candidate =
                PackUri::new(format!("/xl/slicerCaches/slicerCache{cindex}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            cindex += 1;
        };
        let mut sindex = 1u32;
        let slicers_uri = loop {
            let candidate = PackUri::new(format!("/xl/slicers/slicer{sindex}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            sindex += 1;
        };
        let x14 = "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main";
        let cache = OpenXmlElement::new("x14", x14, "slicerCacheDefinition")
            .with_ns_decl("x14", x14)
            .with_attribute("name", cache_name)
            .with_attribute("sourceName", name)
            .with_child(OpenXmlElement::new("x14", x14, "data").with_child(
                OpenXmlElement::new("x14", x14, "tabular")
                    .with_attribute("pivotCacheId", "0"),
            ));
        let slicers = OpenXmlElement::new("x14", x14, "slicers")
            .with_ns_decl("x14", x14)
            .with_child(
                OpenXmlElement::new("x14", x14, "slicer")
                    .with_attribute("name", name)
                    .with_attribute("cache", cache_name)
                    .with_attribute("caption", name),
            );
        self.package.set_part(
            cache_uri.clone(),
            content_type::SLICER_CACHE,
            write_element(&cache)?,
        );
        self.package.set_part(
            slicers_uri.clone(),
            content_type::SLICER,
            write_element(&slicers)?,
        );
        // workbook → cache, worksheet → slicers
        let wb_uri = self.ensure_workbook()?;
        self.package.add_part_relationship(
            &wb_uri,
            rel::SLICER_CACHE,
            &cache_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            &sheet_uri,
            rel::SLICER,
            &slicers_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((slicers_uri, cache_uri))
    }

    /// List slicer cache names from `/xl/slicerCaches/` as `(name, sourceName)`.
    pub fn list_slicer_cache_entries(&self) -> Result<Vec<(String, String)>> {
        let mut out = Vec::new();
        for uri in self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/slicerCaches/"))
            
            .collect::<Vec<_>>()
        {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let Ok(root) = parse_element(data) else {
                continue;
            };
            let name = root.get_attribute("name").unwrap_or("").to_string();
            let source = root.get_attribute("sourceName").unwrap_or("").to_string();
            if !name.is_empty() {
                out.push((name, source));
            }
        }
        Ok(out)
    }

    /// Whether a slicer cache with the given name exists.
    pub fn has_slicer_cache(&self, name: &str) -> Result<bool> {
        Ok(self
            .list_slicer_cache_entries()?
            .iter()
            .any(|(n, _)| n == name))
    }

    /// Remove a slicer cache part by name. Returns whether found.
    pub fn remove_slicer_cache(&mut self, name: &str) -> Result<bool> {
        let mut target: Option<PackUri> = None;
        for uri in self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/slicerCaches/"))
            
            .collect::<Vec<_>>()
        {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let Ok(root) = parse_element(data) else {
                continue;
            };
            if root.get_attribute("name") == Some(name) {
                target = Some(uri);
                break;
            }
        }
        let Some(uri) = target else {
            return Ok(false);
        };
        let target_s = uri.as_str().to_string();
        let sources: Vec<PackUri> = self.package.opc().part_uris();
        for src in sources {
            let Some(rels) = self.package.opc().part_relationships(&src) else {
                continue;
            };
            let ids: Vec<String> = rels
                .iter()
                .filter(|r| {
                    r.relationship_type == rel::SLICER_CACHE
                        || r.target.contains("slicerCache")
                })
                .filter(|r| {
                    r.target == target_s
                        || r.target.ends_with(target_s.trim_start_matches('/'))
                        || target_s.ends_with(r.target.trim_start_matches("./"))
                })
                .map(|r| r.id.clone())
                .collect();
            if ids.is_empty() {
                continue;
            }
            self.package
                .delete_reference_relationships(Some(&src), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Clear all slicer cache parts. Returns how many were removed.
    pub fn clear_slicer_caches(&mut self) -> Result<usize> {
        self.clear_parts_under(&["/xl/slicerCaches/"], &[rel::SLICER_CACHE])
    }

    /// List slicer names from all `/xl/slicers/` parts as `(name, cache, caption)`.
    pub fn list_slicer_entries(&self) -> Result<Vec<(String, String, String)>> {
        let mut out = Vec::new();
        for uri in self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/slicers/"))
            
            .collect::<Vec<_>>()
        {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let Ok(root) = parse_element(data) else {
                continue;
            };
            for s in root.descendants().filter(|e| e.local_name == "slicer") {
                let name = s.get_attribute("name").unwrap_or("").to_string();
                let cache = s.get_attribute("cache").unwrap_or("").to_string();
                let caption = s.get_attribute("caption").unwrap_or("").to_string();
                if !name.is_empty() {
                    out.push((name, cache, caption));
                }
            }
        }
        Ok(out)
    }

    /// Whether a slicer entry with the given name exists.
    pub fn has_slicer_entry(&self, name: &str) -> Result<bool> {
        Ok(self
            .list_slicer_entries()?
            .iter()
            .any(|(n, _, _)| n == name))
    }

    /// Set caption on slicer entries matching `name`. Returns count updated.
    pub fn set_slicer_caption(&mut self, name: &str, caption: &str) -> Result<usize> {
        let mut updated = 0usize;
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/slicers/"))
            
            .collect();
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri).map(|d| d.to_vec()) else {
                continue;
            };
            let Ok(mut root) = parse_element(&data) else {
                continue;
            };
            fn visit(el: &mut OpenXmlElement, name: &str, caption: &str, count: &mut usize) {
                if el.local_name == "slicer" && el.get_attribute("name") == Some(name) {
                    el.set_attribute("caption", caption);
                    *count += 1;
                }
                for c in el.children.iter_mut() {
                    visit(c, name, caption, count);
                }
            }
            let mut n = 0usize;
            visit(&mut root, name, caption, &mut n);
            if n > 0 {
                updated += n;
                let xml = write_element(&root)?;
                self.package
            .set_part(uri, content_type::SLICER, xml);
            }
        }
        Ok(updated)
    }

    /// Clear caption on slicers matching `name`.
    pub fn clear_slicer_caption(&mut self, name: &str) -> Result<usize> {
        let mut updated = 0usize;
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/slicers/"))
            
            .collect();
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let Ok(mut root) = parse_element(data) else {
                continue;
            };
            let mut changed = false;
            fn visit(el: &mut OpenXmlElement, name: &str, updated: &mut usize, changed: &mut bool) {
                if el.local_name == "slicer" {
                    let n = el.get_attribute("name").unwrap_or("");
                    if n == name && el.get_attribute("caption").is_some() {
                        el.attributes.retain(|a| a.local_name != "caption");
                        *changed = true;
                        *updated += 1;
                    }
                }
                for c in el.children.iter_mut() {
                    visit(c, name, updated, changed);
                }
            }
            visit(&mut root, name, &mut updated, &mut changed);
            if changed {
                let ct = self
                    .package
                    .opc()
                    .content_types()
                    .content_type_for(uri.as_str())
                    .unwrap_or("application/vnd.ms-excel.slicer+xml")
                    .to_string();
                self.package
            .set_part(uri, ct, write_element(&root)?);
            }
        }
        Ok(updated)
    }

    /// Remove slicer entries matching `name` from all slicer parts. Returns count removed.
    ///
    /// Does not remove empty slicer parts or slicer caches.
    pub fn remove_slicer_entry(&mut self, name: &str) -> Result<usize> {
        let mut removed = 0usize;
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/slicers/"))
            
            .collect();
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri).map(|d| d.to_vec()) else {
                continue;
            };
            let Ok(mut root) = parse_element(&data) else {
                continue;
            };
            fn visit(el: &mut OpenXmlElement, name: &str, count: &mut usize) {
                let before = el.children.len();
                el.children.retain(|c| {
                    if c.local_name == "slicer" && c.get_attribute("name") == Some(name) {
                        return false;
                    }
                    true
                });
                *count += before - el.children.len();
                for c in el.children.iter_mut() {
                    visit(c, name, count);
                }
            }
            let mut n = 0usize;
            visit(&mut root, name, &mut n);
            if n > 0 {
                removed += n;
                let xml = write_element(&root)?;
                self.package
            .set_part(uri, content_type::SLICER, xml);
            }
        }
        Ok(removed)
    }

    /// Add a theme part under the workbook (reuses DrawingML theme).
    pub fn add_default_theme(&mut self) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let theme_uri = PackUri::new("/xl/theme/theme1.xml");
        let theme = crate::wordprocessing::default_theme("Office Theme");
        let xml = write_element(&theme)?;
        self.package
            .set_part(theme_uri.clone(), content_type::THEME, xml);
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| rels.get_by_type(rel::THEME).map(|r| r.id.clone()))
        {
            return Ok((theme_uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::THEME,
            &theme_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((theme_uri, rid))
    }

    /// Add an external workbook link shell (no remote fetch).
    ///
    /// Creates `/xl/externalLinks/externalLinkN.xml` and a workbook relationship.
    /// Returns `(uri, relationship_id)`.
    pub fn add_external_link(&mut self, target_workbook: &str) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let mut index = 1u32;
        let link_uri = loop {
            let candidate =
                PackUri::new(format!("/xl/externalLinks/externalLink{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let root = crate::element::OpenXmlElement::new("x", x, "externalLink")
            .with_ns_decl("x", x)
            .with_ns_decl(
                "r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            )
            .with_child(
                crate::element::OpenXmlElement::new("x", x, "externalBook")
                    .with_attribute_qname("r:id", "rId1")
                    .with_child(
                        crate::element::OpenXmlElement::new("x", x, "sheetNames").with_child(
                            crate::element::OpenXmlElement::new("x", x, "sheetName")
                                .with_attribute("val", "Sheet1"),
                        ),
                    ),
            );
        let xml = write_element(&root)?;
        self.package.set_part(
            link_uri.clone(),
            content_type::SPREADSHEET_EXTERNAL_LINK,
            xml,
        );
        // External relationship from the link part to the target workbook path/url
        let _ = self.package.add_external_relationship(
            Some(&link_uri),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLinkPath",
            target_workbook,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::EXTERNAL_LINK,
            &link_uri,
            RelationshipTargetMode::Internal,
        );
        // Add externalReferences to workbook
        let mut wb_root = parse_element(
            self.package
                .opc()
                .get_part(&wb_uri)
                .ok_or_else(|| Error::PartNotFound(wb_uri.to_string()))?,
        )?;
        let xref = crate::element::OpenXmlElement::new("x", x, "externalReference")
            .with_attribute_qname("r:id", &rid);
        if let Some(refs) = wb_root.child_mut("externalReferences") {
            refs.append_child(xref);
        } else {
            let refs = crate::element::OpenXmlElement::new("x", x, "externalReferences")
                .with_child(xref);
            let insert_at = wb_root
                .children
                .iter()
                .position(|c| c.local_name == "sheets")
                .map(|i| i + 1)
                .unwrap_or(wb_root.children.len());
            wb_root.children.insert(insert_at, refs);
        }
        let wb_xml = write_element(&wb_root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            wb_xml,
        );
        Ok((link_uri, rid))
    }

    /// List external link part URIs under `/xl/externalLinks/`.
    pub fn list_external_links(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/externalLinks/"))
            
            .collect()
    }

    /// Whether any external workbook link parts exist.
    pub fn has_external_links(&self) -> bool {
        !self.list_external_links().is_empty()
    }

    /// Whether any external link targets the given workbook path/string.
    pub fn has_external_link_target(&self, target: &str) -> Result<bool> {
        Ok(self
            .list_external_link_targets()?
            .iter()
            .any(|(_, t)| t == target || t.ends_with(target)))
    }

    /// Number of external workbook link parts.
    pub fn external_link_count(&self) -> usize {
        self.list_external_links().len()
    }

    /// Remove all external link parts and workbook externalReferences.
    pub fn clear_external_links(&mut self) -> Result<usize> {
        let links = self.list_external_links();
        let n = links.len();
        if n == 0 {
            return Ok(0);
        }
        let wb_uri = self.ensure_workbook()?;
        let ids: Vec<String> = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .map(|rels| {
                rels.find_all_by_type(rel::EXTERNAL_LINK)
                    .into_iter()
                    .map(|r| r.id.clone())
                    .collect()
            })
            .unwrap_or_default();
        self.package
            .delete_reference_relationships(Some(&wb_uri), &ids);
        if let Some(data) = self.package.opc().get_part(&wb_uri).map(|d| d.to_vec()) {
            if let Ok(mut root) = parse_element(&data) {
                root.children
                    .retain(|c| c.local_name != "externalReferences");
                let xml = write_element(&root)?;
                self.package.set_part(
                    wb_uri,
                    self.document_type.content_type(),
                    xml,
                );
            }
        }
        for uri in links {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Remove a single external link part by URI. Returns whether found.
    pub fn remove_external_link(&mut self, link_uri: &PackUri) -> Result<bool> {
        if !self.package.opc().has_part(link_uri) {
            return Ok(false);
        }
        let target = link_uri.as_str().to_string();
        let wb_uri = self.ensure_workbook()?;
        let ids: Vec<String> = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .map(|rels| {
                rels.find_all_by_type(rel::EXTERNAL_LINK)
                    .into_iter()
                    .filter(|r| {
                        r.target == target
                            || r.target.ends_with(target.trim_start_matches('/'))
                            || target.ends_with(r.target.trim_start_matches("./"))
                    })
                    .map(|r| r.id.clone())
                    .collect()
            })
            .unwrap_or_default();
        self.package
            .delete_reference_relationships(Some(&wb_uri), &ids);
        // If no external links remain, drop externalReferences element
        let remaining = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .map(|rels| rels.find_all_by_type(rel::EXTERNAL_LINK).len())
            .unwrap_or(0);
        if remaining == 0 {
            if let Some(data) = self.package.opc().get_part(&wb_uri).map(|d| d.to_vec()) {
                if let Ok(mut root) = parse_element(&data) {
                    root.children
                        .retain(|c| c.local_name != "externalReferences");
                    let xml = write_element(&root)?;
                    self.package.set_part(
                        wb_uri,
                        self.document_type.content_type(),
                        xml,
                    );
                }
            }
        }
        self.package.delete_part(link_uri);
        Ok(true)
    }

    /// List external link targets as `(part_uri, target_path)`.
    pub fn list_external_link_targets(&self) -> Result<Vec<(PackUri, String)>> {
        let mut out = Vec::new();
        let path_rel =
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLinkPath";
        for uri in self.list_external_links() {
            if let Some(rels) = self.package.opc().part_relationships(&uri) {
                for r in rels.iter() {
                    if r.relationship_type == path_rel {
                        out.push((uri.clone(), r.target.clone()));
                    }
                }
            }
        }
        Ok(out)
    }

    /// Update the external path target for an external link part.
    pub fn set_external_link_target(
        &mut self,
        link_uri: &PackUri,
        target: &str,
    ) -> Result<bool> {
        let path_rel =
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLinkPath";
        if !self.package.opc().has_part(link_uri) {
            return Ok(false);
        }
        let ids: Vec<String> = self
            .package
            .opc()
            .part_relationships(link_uri)
            .map(|rels| {
                rels.iter()
                    .filter(|r| r.relationship_type == path_rel)
                    .map(|r| r.id.clone())
                    .collect()
            })
            .unwrap_or_default();
        self.package
            .delete_reference_relationships(Some(link_uri), &ids);
        let _ = self
            .package
            .add_external_relationship(Some(link_uri), path_rel, target);
        Ok(true)
    }

    /// Remove external path relationship(s) from an external link part.
    pub fn clear_external_link_target(&mut self, link_uri: &PackUri) -> Result<bool> {
        let path_rel =
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLinkPath";
        if !self.package.opc().has_part(link_uri) {
            return Ok(false);
        }
        let ids: Vec<String> = self
            .package
            .opc()
            .part_relationships(link_uri)
            .map(|rels| {
                rels.iter()
                    .filter(|r| r.relationship_type == path_rel)
                    .map(|r| r.id.clone())
                    .collect()
            })
            .unwrap_or_default();
        if ids.is_empty() {
            return Ok(false);
        }
        self.package
            .delete_reference_relationships(Some(link_uri), &ids);
        Ok(true)
    }


    /// Set workbook `workbookPr/@date1904`.
    pub fn set_date1904(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("date1904", enabled)
    }


    /// Set workbook VBA code name (`workbookPr/@codeName`).
    pub fn set_code_name(&mut self, name: &str) -> Result<()> {
        self.set_workbook_pr_str("codeName", name)
    }

    /// Read workbook codeName.
    pub fn code_name(&self) -> Result<Option<String>> {
        self.workbook_pr_str("codeName")
    }

    /// Whether codeName is set.
    pub fn has_code_name(&self) -> Result<bool> {
        Ok(self.code_name()?.is_some())
    }

    /// Set `workbookPr/@refreshAllConnections`.
    /// Clear workbook VBA code name. Returns whether it was present.
    pub fn clear_code_name(&mut self) -> Result<bool> {
        let had = self.has_code_name()?;
        if !had {
            return Ok(false);
        }
        // Remove codeName attribute from workbookPr
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if let Some(pr) = root.child_mut("workbookPr") {
            pr.attributes.retain(|a| a.local_name != "codeName");
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn set_refresh_all_connections(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("refreshAllConnections", enabled)
    }

    /// Whether refreshAllConnections is enabled.
    pub fn refresh_all_connections(&self) -> Result<bool> {
        self.workbook_pr_bool("refreshAllConnections", false)
    }

    /// Set `workbookPr/@defaultThemeVersion`.
    /// Whether `refreshAllConnections` is explicitly set on workbookPr.
    pub fn has_refresh_all_connections_attr(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("refreshAllConnections"))
            .is_some())
    }

    /// Clear `refreshAllConnections` from workbookPr.
    pub fn clear_refresh_all_connections(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("refreshAllConnections")
    }


    pub fn set_default_theme_version(&mut self, version: &str) -> Result<()> {
        self.set_workbook_pr_str("defaultThemeVersion", version)
    }

    /// Read defaultThemeVersion.
    pub fn default_theme_version(&self) -> Result<Option<String>> {
        self.workbook_pr_str("defaultThemeVersion")
    }

    fn set_workbook_pr_str(&mut self, attr: &str, value: &str) -> Result<()> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let x = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
        if let Some(pr) = root.child_mut("workbookPr") {
            pr.set_attribute(attr, value);
        } else {
            let pr = OpenXmlElement::new("x", x, "workbookPr").with_attribute(attr, value);
            root.children.insert(0, pr);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    fn workbook_pr_str(&self, attr: &str) -> Result<Option<String>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute(attr).map(|s| s.to_string())))
    }

    fn clear_workbook_pr_attr(&mut self, attr: &str) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(pr) = root.child_mut("workbookPr") else {
            return Ok(false);
        };
        let before = pr.attributes.len();
        pr.attributes.retain(|a| a.local_name != attr);
        if pr.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(true)
    }

    /// Whether workbook uses the 1904 date system.
    /// Whether defaultThemeVersion is set.
    pub fn has_default_theme_version(&self) -> Result<bool> {
        Ok(self.default_theme_version()?.is_some())
    }

    /// Clear defaultThemeVersion from workbookPr.
    pub fn clear_default_theme_version(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("defaultThemeVersion")
    }


    pub fn date1904(&self) -> Result<bool> {
        self.workbook_pr_bool("date1904", false)
    }

    /// Alias for [`date1904`](Self::date1904).
    pub fn has_date1904(&self) -> Result<bool> {
        self.date1904()
    }

    /// Set workbook `workbookPr/@filterPrivacy` or filterMode on sheet - workbook filterMode.
    /// Disable date1904 workbook mode. Returns whether it was enabled.
    pub fn clear_date1904(&mut self) -> Result<bool> {
        let had = self.has_date1904()?;
        if had {
            self.set_date1904(false)?;
        }
        Ok(had)
    }

    pub fn set_filter_mode(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("filterPrivacy", enabled)
    }

    /// Read workbookPr filterPrivacy as filter-mode helper.
    pub fn filter_mode(&self) -> Result<bool> {
        self.workbook_pr_bool("filterPrivacy", false)
    }

    /// Set workbook `workbookPr/@updateLinks` is string; provide promptForFullCalc instead:
    /// `workbookPr/@defaultThemeVersion` skip.
    /// Toggle `workbookPr/@backupFile`.
    /// Whether filterPrivacy (filter_mode helper) is explicitly set.
    pub fn has_filter_mode(&self) -> Result<bool> {
        self.has_filter_privacy()
    }

    /// Clear filterPrivacy (filter_mode helper).
    pub fn clear_filter_mode(&mut self) -> Result<bool> {
        self.clear_filter_privacy()
    }


    pub fn set_backup_file(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("backupFile", enabled)
    }

    /// Whether backupFile is enabled.
    pub fn backup_file(&self) -> Result<bool> {
        self.workbook_pr_bool("backupFile", false)
    }

    fn set_workbook_pr_bool(&mut self, attr: &str, value: bool) -> Result<()> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let x = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
        if let Some(pr) = root.child_mut("workbookPr") {
            pr.set_attribute(attr, if value { "1" } else { "0" });
        } else {
            let pr = OpenXmlElement::new("x", x, "workbookPr")
                .with_attribute(attr, if value { "1" } else { "0" });
            // workbookPr typically early
            root.children.insert(0, pr);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    fn workbook_pr_bool(&self, attr: &str, default: bool) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(default);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute(attr))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    /// Set workbook file version metadata (`x:fileVersion`).
    /// Whether `backupFile` is explicitly set on workbookPr.
    pub fn has_backup_file(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("backupFile"))
            .is_some())
    }

    /// Clear `backupFile` from workbookPr.
    pub fn clear_backup_file(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("backupFile")
    }


    pub fn set_file_version(
        &mut self,
        app_name: &str,
        last_edited: &str,
        lowest_edited: &str,
        rup_build: &str,
    ) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        root.children.retain(|c| c.local_name != "fileVersion");
        let fv = OpenXmlElement::new("x", x, "fileVersion")
            .with_attribute("appName", app_name)
            .with_attribute("lastEdited", last_edited)
            .with_attribute("lowestEdited", lowest_edited)
            .with_attribute("rupBuild", rup_build);
        root.children.insert(0, fv);
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Read file version as `(appName, lastEdited, lowestEdited, rupBuild)`.
    pub fn file_version(&self) -> Result<Option<(String, String, String, String)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(fv) = root.child("fileVersion") else {
            return Ok(None);
        };
        Ok(Some((
            fv.get_attribute("appName").unwrap_or("").to_string(),
            fv.get_attribute("lastEdited").unwrap_or("").to_string(),
            fv.get_attribute("lowestEdited").unwrap_or("").to_string(),
            fv.get_attribute("rupBuild").unwrap_or("").to_string(),
        )))
    }

    /// Whether fileVersion is present.
    pub fn has_file_version(&self) -> Result<bool> {
        Ok(self.file_version()?.is_some())
    }

    /// Clear fileVersion.
    pub fn clear_file_version(&mut self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "fileVersion");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Set file sharing shell (`x:fileSharing`) without password hashing.
    pub fn set_file_sharing(
        &mut self,
        read_only_recommended: bool,
        user_name: Option<&str>,
    ) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        root.children.retain(|c| c.local_name != "fileSharing");
        let mut fs = OpenXmlElement::new("x", x, "fileSharing").with_attribute(
            "readOnlyRecommended",
            if read_only_recommended { "1" } else { "0" },
        );
        if let Some(u) = user_name {
            fs.set_attribute("userName", u);
        }
        // after fileVersion if present
        let insert_at = root
            .children
            .iter()
            .position(|c| c.local_name == "fileVersion")
            .map(|i| i + 1)
            .unwrap_or(0);
        root.children.insert(insert_at, fs);
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Whether fileSharing is present.
    pub fn has_file_sharing(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("fileSharing").is_some())
    }

    /// Whether readOnlyRecommended is set.
    pub fn file_sharing_read_only_recommended(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("fileSharing")
            .and_then(|f| f.get_attribute("readOnlyRecommended"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Clear fileSharing.
    pub fn clear_file_sharing(&mut self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "fileSharing");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Set OLE size reference (`x:oleSize/@ref`), e.g. `"A1:H20"`.
    pub fn set_ole_size(&mut self, reference: &str) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        root.children.retain(|c| c.local_name != "oleSize");
        let el = OpenXmlElement::new("x", x, "oleSize").with_attribute("ref", reference);
        // typically after sheets
        let insert_at = root
            .children
            .iter()
            .position(|c| c.local_name == "sheets")
            .map(|i| i + 1)
            .unwrap_or(root.children.len());
        root.children.insert(insert_at, el);
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Read oleSize ref.
    pub fn ole_size(&self) -> Result<Option<String>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("oleSize")
            .and_then(|o| o.get_attribute("ref").map(|s| s.to_string())))
    }

    /// Clear oleSize.
    pub fn clear_ole_size(&mut self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "oleSize");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Add a custom function group name under `functionGroups`.
    pub fn add_function_group(&mut self, name: &str) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let fg = OpenXmlElement::new("x", x, "functionGroup").with_attribute("name", name);
        if let Some(container) = root.child_mut("functionGroups") {
            container
                .children
                .retain(|c| c.get_attribute("name") != Some(name));
            container.append_child(fg);
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| matches!(c.local_name.as_str(), "calcPr" | "extLst"))
                .unwrap_or(root.children.len());
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "functionGroups")
                    .with_attribute("builtInGroupCount", "16")
                    .with_child(fg),
            );
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// List custom function group names.
    pub fn list_function_groups(&self) -> Result<Vec<String>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("functionGroups") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("functionGroup")
            .filter_map(|g| g.get_attribute("name").map(|s| s.to_string()))
            .collect())
    }

    /// Number of custom function groups.
    pub fn function_group_count(&self) -> Result<usize> {
        Ok(self.list_function_groups()?.len())
    }

    /// Whether functionGroups exist.
    pub fn has_function_groups(&self) -> Result<bool> {
        Ok(self.function_group_count()? > 0)
    }

    /// Clear function groups. Returns how many were removed.
    pub fn clear_function_groups(&mut self) -> Result<usize> {
        let n = self.function_group_count()?;
        if n == 0 {
            return Ok(0);
        }
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        root.children.retain(|c| c.local_name != "functionGroups");
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(n)
    }

    /// Set `workbookPr/@dateCompatibility`.
    pub fn set_date_compatibility(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("dateCompatibility", enabled)
    }

    /// Whether dateCompatibility is enabled.
    pub fn date_compatibility(&self) -> Result<bool> {
        self.workbook_pr_bool("dateCompatibility", false)
    }

    /// Set `workbookPr/@showObjects` (e.g. `"all"`, `"placeholders"`, `"none"`).
    /// Whether `dateCompatibility` is explicitly set on workbookPr.
    pub fn has_date_compatibility(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("dateCompatibility"))
            .is_some())
    }

    /// Clear `dateCompatibility` from workbookPr.
    pub fn clear_date_compatibility(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("dateCompatibility")
    }


    pub fn set_show_objects(&mut self, value: &str) -> Result<()> {
        self.set_workbook_pr_str("showObjects", value)
    }

    /// Read `workbookPr/@showObjects`.
    pub fn show_objects(&self) -> Result<Option<String>> {
        self.workbook_pr_str("showObjects")
    }

    /// Set `workbookPr/@showBorderUnselectedTables`.
    /// Whether showObjects is set.
    pub fn has_show_objects(&self) -> Result<bool> {
        Ok(self.show_objects()?.is_some())
    }

    /// Clear showObjects from workbookPr.
    pub fn clear_show_objects(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("showObjects")
    }


    pub fn set_show_border_unselected_tables(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("showBorderUnselectedTables", enabled)
    }

    /// Whether showBorderUnselectedTables is enabled (default true when unset).
    pub fn show_border_unselected_tables(&self) -> Result<bool> {
        self.workbook_pr_bool("showBorderUnselectedTables", true)
    }

    /// Set `workbookPr/@promptedSolutions`.
    /// Whether `showBorderUnselectedTables` is explicitly set on workbookPr.
    pub fn has_show_border_unselected_tables(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("showBorderUnselectedTables"))
            .is_some())
    }

    /// Clear `showBorderUnselectedTables` from workbookPr.
    pub fn clear_show_border_unselected_tables(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("showBorderUnselectedTables")
    }


    pub fn set_prompted_solutions(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("promptedSolutions", enabled)
    }

    /// Whether promptedSolutions is enabled.
    pub fn prompted_solutions(&self) -> Result<bool> {
        self.workbook_pr_bool("promptedSolutions", false)
    }

    /// Set `workbookPr/@showInkAnnotation`.
    /// Whether promptedSolutions is explicitly set.
    pub fn has_prompted_solutions(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("promptedSolutions"))
            .is_some())
    }

    /// Clear promptedSolutions from workbookPr.
    pub fn clear_prompted_solutions(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("promptedSolutions")
    }


    pub fn set_show_ink_annotation(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("showInkAnnotation", enabled)
    }

    /// Whether showInkAnnotation is enabled (default true when unset).
    pub fn show_ink_annotation(&self) -> Result<bool> {
        self.workbook_pr_bool("showInkAnnotation", true)
    }

    /// Set `workbookPr/@saveExternalLinkValues`.
    /// Whether `showInkAnnotation` is explicitly set on workbookPr.
    pub fn has_show_ink_annotation(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("showInkAnnotation"))
            .is_some())
    }

    /// Clear `showInkAnnotation` from workbookPr.
    pub fn clear_show_ink_annotation(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("showInkAnnotation")
    }


    pub fn set_save_external_link_values(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("saveExternalLinkValues", enabled)
    }

    /// Whether saveExternalLinkValues is enabled (default true when unset).
    pub fn save_external_link_values(&self) -> Result<bool> {
        self.workbook_pr_bool("saveExternalLinkValues", true)
    }

    /// Set `workbookPr/@updateLinks` (e.g. `"userSet"`, `"never"`, `"always"`).
    /// Whether `saveExternalLinkValues` is explicitly set on workbookPr.
    pub fn has_save_external_link_values(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("saveExternalLinkValues"))
            .is_some())
    }

    /// Clear `saveExternalLinkValues` from workbookPr.
    pub fn clear_save_external_link_values(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("saveExternalLinkValues")
    }


    pub fn set_update_links(&mut self, value: &str) -> Result<()> {
        self.set_workbook_pr_str("updateLinks", value)
    }

    /// Read `workbookPr/@updateLinks`.
    pub fn update_links(&self) -> Result<Option<String>> {
        self.workbook_pr_str("updateLinks")
    }

    /// Set `workbookPr/@hidePivotFieldList`.
    /// Whether updateLinks is set.
    pub fn has_update_links(&self) -> Result<bool> {
        Ok(self.update_links()?.is_some())
    }

    /// Clear updateLinks from workbookPr.
    pub fn clear_update_links(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("updateLinks")
    }


    pub fn set_hide_pivot_field_list(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("hidePivotFieldList", enabled)
    }

    /// Whether hidePivotFieldList is enabled.
    pub fn hide_pivot_field_list(&self) -> Result<bool> {
        self.workbook_pr_bool("hidePivotFieldList", false)
    }

    /// Set `workbookPr/@showPivotChartFilter`.
    /// Whether `hidePivotFieldList` is explicitly set on workbookPr.
    pub fn has_hide_pivot_field_list(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("hidePivotFieldList"))
            .is_some())
    }

    /// Clear `hidePivotFieldList` from workbookPr.
    pub fn clear_hide_pivot_field_list(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("hidePivotFieldList")
    }


    pub fn set_show_pivot_chart_filter(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("showPivotChartFilter", enabled)
    }

    /// Whether showPivotChartFilter is enabled.
    pub fn show_pivot_chart_filter(&self) -> Result<bool> {
        self.workbook_pr_bool("showPivotChartFilter", false)
    }

    /// Set `workbookPr/@allowRefreshQuery`.
    /// Whether `showPivotChartFilter` is explicitly set on workbookPr.
    pub fn has_show_pivot_chart_filter_attr(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("showPivotChartFilter"))
            .is_some())
    }

    /// Clear `showPivotChartFilter` from workbookPr.
    pub fn clear_show_pivot_chart_filter(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("showPivotChartFilter")
    }


    pub fn set_allow_refresh_query(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("allowRefreshQuery", enabled)
    }

    /// Whether allowRefreshQuery is enabled.
    pub fn allow_refresh_query(&self) -> Result<bool> {
        self.workbook_pr_bool("allowRefreshQuery", false)
    }

    /// Set `workbookPr/@publishItems`.
    /// Whether `allowRefreshQuery` is explicitly set on workbookPr.
    pub fn has_allow_refresh_query_attr(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("allowRefreshQuery"))
            .is_some())
    }

    /// Clear `allowRefreshQuery` from workbookPr.
    pub fn clear_allow_refresh_query(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("allowRefreshQuery")
    }


    pub fn set_publish_items(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("publishItems", enabled)
    }

    /// Whether publishItems is enabled.
    pub fn publish_items(&self) -> Result<bool> {
        self.workbook_pr_bool("publishItems", false)
    }

    /// Set `workbookPr/@checkCompatibility`.
    /// Whether `publishItems` is explicitly set on workbookPr.
    pub fn has_publish_items_attr(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("publishItems"))
            .is_some())
    }

    /// Clear `publishItems` from workbookPr.
    pub fn clear_publish_items(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("publishItems")
    }


    pub fn set_check_compatibility(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("checkCompatibility", enabled)
    }

    /// Whether checkCompatibility is enabled.
    pub fn check_compatibility(&self) -> Result<bool> {
        self.workbook_pr_bool("checkCompatibility", false)
    }

    /// Set `workbookPr/@autoCompressPictures`.
    /// Whether `checkCompatibility` is explicitly set on workbookPr.
    pub fn has_check_compatibility_attr(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("checkCompatibility"))
            .is_some())
    }

    /// Clear `checkCompatibility` from workbookPr.
    pub fn clear_check_compatibility(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("checkCompatibility")
    }


    pub fn set_auto_compress_pictures(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_pr_bool("autoCompressPictures", enabled)
    }

    /// Whether autoCompressPictures is enabled (default true when unset).
    pub fn auto_compress_pictures(&self) -> Result<bool> {
        self.workbook_pr_bool("autoCompressPictures", true)
    }

    /// Whether filterPrivacy / filter mode is set.
    /// Whether `autoCompressPictures` is explicitly set on workbookPr.
    pub fn has_auto_compress_pictures(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("workbookPr")
            .and_then(|p| p.get_attribute("autoCompressPictures"))
            .is_some())
    }

    /// Clear `autoCompressPictures` from workbookPr.
    pub fn clear_auto_compress_pictures(&mut self) -> Result<bool> {
        self.clear_workbook_pr_attr("autoCompressPictures")
    }


    pub fn has_filter_privacy(&self) -> Result<bool> {
        self.filter_mode()
    }

    /// Alias for [`set_filter_mode`](Self::set_filter_mode).
    /// Disable filter privacy. Returns whether it was enabled.
    pub fn clear_filter_privacy(&mut self) -> Result<bool> {
        let had = self.has_filter_privacy()?;
        if had {
            self.set_filter_privacy(false)?;
        }
        Ok(had)
    }

    pub fn set_filter_privacy(&mut self, enabled: bool) -> Result<()> {
        self.set_filter_mode(enabled)
    }

    /// Alias for [`filter_mode`](Self::filter_mode).
    pub fn filter_privacy(&self) -> Result<bool> {
        self.filter_mode()
    }

    /// Set workbook calculation properties (`x:calcPr`).
    pub fn set_calc_properties(&mut self, full_calc_on_load: bool, calc_mode: &str) -> Result<()> {
        use crate::spreadsheet::calc_properties;
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            workbook(Vec::<crate::element::OpenXmlElement>::new())
        };
        root.children.retain(|c| c.local_name != "calcPr");
        // calcPr typically after sheets
        let insert_at = root
            .children
            .iter()
            .position(|c| c.local_name == "sheets")
            .map(|i| i + 1)
            .unwrap_or(root.children.len());
        root.children
            .insert(insert_at, calc_properties(full_calc_on_load, calc_mode));
        let xml = write_element(&root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            xml,
        );
        Ok(())
    }

    /// Read workbook `calcPr` as `(full_calc_on_load, calc_mode)` when present.
    pub fn get_calc_properties(&self) -> Result<Option<(bool, String)>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(cp) = root.child("calcPr") else {
            return Ok(None);
        };
        let full = cp
            .get_attribute("fullCalcOnLoad")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let mode = cp
            .get_attribute("calcMode")
            .unwrap_or("auto")
            .to_string();
        Ok(Some((full, mode)))
    }

    fn ensure_calc_pr_mut<'a>(&self, root: &'a mut OpenXmlElement) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("calcPr").is_none() {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheets")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, OpenXmlElement::new("x", x, "calcPr"));
        }
        root.child_mut("calcPr").expect("calcPr ensured")
    }

    fn set_calc_pr_attr(&mut self, attr: &str, value: &str) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let cp = self.ensure_calc_pr_mut(&mut root);
        cp.set_attribute(attr, value);
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    fn calc_pr_attr(&self, attr: &str) -> Result<Option<String>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("calcPr")
            .and_then(|c| c.get_attribute(attr).map(|s| s.to_string())))
    }

    fn clear_calc_pr_attr(&mut self, attr: &str) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(cp) = root.child_mut("calcPr") else {
            return Ok(false);
        };
        let before = cp.attributes.len();
        cp.attributes.retain(|a| a.local_name != attr);
        if cp.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(true)
    }

    fn calc_pr_bool(&self, attr: &str, default: bool) -> Result<bool> {
        Ok(self
            .calc_pr_attr(attr)?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    /// Set `calcPr/@calcId`.
    pub fn set_calc_id(&mut self, id: u32) -> Result<()> {
        self.set_calc_pr_attr("calcId", &id.to_string())
    }

    /// Read calcId.
    pub fn calc_id(&self) -> Result<Option<u32>> {
        Ok(self.calc_pr_attr("calcId")?.and_then(|s| s.parse().ok()))
    }

    /// Set `calcPr/@calcMode` (`"auto"`, `"autoNoTable"`, `"manual"`).
    /// Whether `calcId` is explicitly set on calcPr.
    pub fn has_calc_id_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("calcId")?.is_some())
    }

    /// Clear `calcId` from calcPr.
    pub fn clear_calc_id(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("calcId")
    }


    pub fn set_calc_mode(&mut self, mode: &str) -> Result<()> {
        self.set_calc_pr_attr("calcMode", mode)
    }

    /// Read calcMode.
    pub fn calc_mode(&self) -> Result<Option<String>> {
        self.calc_pr_attr("calcMode")
    }

    /// Set `calcPr/@fullCalcOnLoad`.
    /// Whether calc mode is set on workbook calcPr.
    pub fn has_calc_mode(&self) -> Result<bool> {
        Ok(self.calc_mode()?.is_some())
    }

    /// Clear workbook calc mode (removes `calcPr/@calcMode`). Returns whether it was set.
    pub fn clear_calc_mode(&mut self) -> Result<bool> {
        let had = self.has_calc_mode()?;
        if !had {
            return Ok(false);
        }
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if let Some(cp) = root.child_mut("calcPr") {
            cp.attributes.retain(|a| a.local_name != "calcMode");
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn set_full_calc_on_load(&mut self, enabled: bool) -> Result<()> {
        self.set_calc_pr_attr("fullCalcOnLoad", if enabled { "1" } else { "0" })
    }

    /// Whether fullCalcOnLoad is enabled.
    pub fn full_calc_on_load(&self) -> Result<bool> {
        self.calc_pr_bool("fullCalcOnLoad", false)
    }

    /// Set `calcPr/@calcCompleted`.
    /// Whether `fullCalcOnLoad` is explicitly set on calcPr.
    pub fn has_full_calc_on_load_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("fullCalcOnLoad")?.is_some())
    }

    /// Clear `fullCalcOnLoad` from calcPr.
    pub fn clear_full_calc_on_load(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("fullCalcOnLoad")
    }


    pub fn set_calc_completed(&mut self, completed: bool) -> Result<()> {
        self.set_calc_pr_attr("calcCompleted", if completed { "1" } else { "0" })
    }

    /// Whether calcCompleted is set (defaults true when absent).
    pub fn calc_completed(&self) -> Result<bool> {
        self.calc_pr_bool("calcCompleted", true)
    }

    /// Set `calcPr/@refMode` (`"A1"` or `"R1C1"`).
    /// Whether `calcCompleted` is explicitly set on calcPr.
    pub fn has_calc_completed_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("calcCompleted")?.is_some())
    }

    /// Clear `calcCompleted` from calcPr.
    pub fn clear_calc_completed(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("calcCompleted")
    }


    pub fn set_ref_mode(&mut self, mode: &str) -> Result<()> {
        self.set_calc_pr_attr("refMode", mode)
    }

    /// Read refMode.
    pub fn ref_mode(&self) -> Result<Option<String>> {
        self.calc_pr_attr("refMode")
    }

    /// Set `calcPr/@iterate`.
    /// Whether `refMode` is explicitly set on calcPr.
    pub fn has_ref_mode_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("refMode")?.is_some())
    }

    /// Clear `refMode` from calcPr.
    pub fn clear_ref_mode(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("refMode")
    }


    pub fn set_iterate(&mut self, enabled: bool) -> Result<()> {
        self.set_calc_pr_attr("iterate", if enabled { "1" } else { "0" })
    }

    /// Whether iterative calculation is enabled.
    pub fn iterate(&self) -> Result<bool> {
        self.calc_pr_bool("iterate", false)
    }

    /// Set `calcPr/@iterateCount`.
    /// Whether `iterate` is explicitly set on calcPr.
    pub fn has_iterate_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("iterate")?.is_some())
    }

    /// Clear `iterate` from calcPr.
    pub fn clear_iterate(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("iterate")
    }


    pub fn set_iterate_count(&mut self, count: u32) -> Result<()> {
        self.set_calc_pr_attr("iterateCount", &count.to_string())
    }

    /// Read iterateCount.
    pub fn iterate_count(&self) -> Result<Option<u32>> {
        Ok(self
            .calc_pr_attr("iterateCount")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set `calcPr/@iterateDelta`.
    /// Whether `iterateCount` is explicitly set on calcPr.
    pub fn has_iterate_count_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("iterateCount")?.is_some())
    }

    /// Clear `iterateCount` from calcPr.
    pub fn clear_iterate_count(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("iterateCount")
    }


    pub fn set_iterate_delta(&mut self, delta: f64) -> Result<()> {
        self.set_calc_pr_attr("iterateDelta", &delta.to_string())
    }

    /// Read iterateDelta.
    pub fn iterate_delta(&self) -> Result<Option<f64>> {
        Ok(self
            .calc_pr_attr("iterateDelta")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set `calcPr/@fullPrecision`.
    /// Whether `iterateDelta` is explicitly set on calcPr.
    pub fn has_iterate_delta_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("iterateDelta")?.is_some())
    }

    /// Clear `iterateDelta` from calcPr.
    pub fn clear_iterate_delta(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("iterateDelta")
    }


    pub fn set_full_precision(&mut self, enabled: bool) -> Result<()> {
        self.set_calc_pr_attr("fullPrecision", if enabled { "1" } else { "0" })
    }

    /// Whether fullPrecision is enabled (default true).
    pub fn full_precision(&self) -> Result<bool> {
        self.calc_pr_bool("fullPrecision", true)
    }

    /// Set `calcPr/@calcOnSave`.
    /// Whether `fullPrecision` is explicitly set on calcPr.
    pub fn has_full_precision_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("fullPrecision")?.is_some())
    }

    /// Clear `fullPrecision` from calcPr.
    pub fn clear_full_precision(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("fullPrecision")
    }


    pub fn set_calc_on_save(&mut self, enabled: bool) -> Result<()> {
        self.set_calc_pr_attr("calcOnSave", if enabled { "1" } else { "0" })
    }

    /// Whether calcOnSave is enabled (default true).
    pub fn calc_on_save(&self) -> Result<bool> {
        self.calc_pr_bool("calcOnSave", true)
    }

    /// Set `calcPr/@concurrentCalc`.
    /// Whether `calcOnSave` is explicitly set on calcPr.
    pub fn has_calc_on_save_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("calcOnSave")?.is_some())
    }

    /// Clear `calcOnSave` from calcPr.
    pub fn clear_calc_on_save(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("calcOnSave")
    }


    pub fn set_concurrent_calc(&mut self, enabled: bool) -> Result<()> {
        self.set_calc_pr_attr("concurrentCalc", if enabled { "1" } else { "0" })
    }

    /// Whether concurrentCalc is enabled (default true).
    pub fn concurrent_calc(&self) -> Result<bool> {
        self.calc_pr_bool("concurrentCalc", true)
    }

    /// Set `calcPr/@forceFullCalc`.
    /// Whether `concurrentCalc` is explicitly set on calcPr.
    pub fn has_concurrent_calc_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("concurrentCalc")?.is_some())
    }

    /// Clear `concurrentCalc` from calcPr.
    pub fn clear_concurrent_calc(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("concurrentCalc")
    }


    pub fn set_force_full_calc(&mut self, enabled: bool) -> Result<()> {
        self.set_calc_pr_attr("forceFullCalc", if enabled { "1" } else { "0" })
    }

    /// Whether forceFullCalc is enabled.
    pub fn force_full_calc(&self) -> Result<bool> {
        self.calc_pr_bool("forceFullCalc", false)
    }

    /// Whether a calcPr element is present.
    /// Whether `forceFullCalc` is explicitly set on calcPr.
    pub fn has_force_full_calc_attr(&self) -> Result<bool> {
        Ok(self.calc_pr_attr("forceFullCalc")?.is_some())
    }

    /// Clear `forceFullCalc` from calcPr.
    pub fn clear_force_full_calc(&mut self) -> Result<bool> {
        self.clear_calc_pr_attr("forceFullCalc")
    }


    pub fn has_calc_properties(&self) -> Result<bool> {
        Ok(self.get_calc_properties()?.is_some())
    }

    /// Clear calcPr. Returns whether it was present.
    pub fn clear_calc_properties(&mut self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "calcPr");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Set the active sheet tab index in workbook views (`activeTab`, 0-based).
    pub fn set_active_tab(&mut self, sheet_index: u32) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            workbook(Vec::<crate::element::OpenXmlElement>::new())
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if let Some(views) = root.child_mut("bookViews") {
            if let Some(view) = views.child_mut("workbookView") {
                view.set_attribute("activeTab", sheet_index.to_string());
            } else {
                views.append_child(
                    crate::element::OpenXmlElement::new("x", x, "workbookView")
                        .with_attribute("activeTab", sheet_index.to_string()),
                );
            }
        } else {
            let views = crate::element::OpenXmlElement::new("x", x, "bookViews").with_child(
                crate::element::OpenXmlElement::new("x", x, "workbookView")
                    .with_attribute("activeTab", sheet_index.to_string()),
            );
            // Insert before sheets
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheets")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        let xml = write_element(&root)?;
        self.package.set_part(
            wb_uri,
            self.document_type.content_type(),
            xml,
        );
        Ok(())
    }

    fn ensure_workbook_view_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("bookViews").is_none() {
            let views = OpenXmlElement::new("x", x, "bookViews").with_child(
                OpenXmlElement::new("x", x, "workbookView"),
            );
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheets")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        } else if root
            .child("bookViews")
            .and_then(|v| v.child("workbookView"))
            .is_none()
        {
            if let Some(views) = root.child_mut("bookViews") {
                views.append_child(OpenXmlElement::new("x", x, "workbookView"));
            }
        }
        root.child_mut("bookViews")
            .and_then(|v| v.child_mut("workbookView"))
            .expect("workbookView ensured")
    }

    fn set_workbook_view_attr(&mut self, attr: &str, value: &str) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let view = self.ensure_workbook_view_mut(&mut root);
        view.set_attribute(attr, value);
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    fn workbook_view_attr(&self, attr: &str) -> Result<Option<String>> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("bookViews")
            .and_then(|v| v.child("workbookView"))
            .and_then(|vw| vw.get_attribute(attr).map(|s| s.to_string())))
    }

    fn workbook_view_bool(&self, attr: &str, default: bool) -> Result<bool> {
        Ok(self
            .workbook_view_attr(attr)?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    /// Set workbook view `firstSheet` (0-based index of first visible sheet tab).
    pub fn set_first_sheet(&mut self, index: u32) -> Result<()> {
        self.set_workbook_view_attr("firstSheet", &index.to_string())
    }

    /// Read firstSheet.
    pub fn first_sheet(&self) -> Result<Option<u32>> {
        Ok(self
            .workbook_view_attr("firstSheet")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set workbook view `tabRatio` (0–1000).
    /// Whether firstSheet is set.
    pub fn has_first_sheet(&self) -> Result<bool> {
        Ok(self.first_sheet()?.is_some())
    }

    /// Clear firstSheet attribute from workbook view.
    pub fn clear_first_sheet(&mut self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(views) = root.child_mut("bookViews") else {
            return Ok(false);
        };
        let Some(view) = views.child_mut("workbookView") else {
            return Ok(false);
        };
        let before = view.attributes.len();
        view.attributes.retain(|a| a.local_name != "firstSheet");
        if view.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(true)
    }


    pub fn set_tab_ratio(&mut self, ratio: u32) -> Result<()> {
        self.set_workbook_view_attr("tabRatio", &ratio.to_string())
    }

    /// Read tabRatio.
    pub fn tab_ratio(&self) -> Result<Option<u32>> {
        Ok(self
            .workbook_view_attr("tabRatio")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set workbook view window position and size (`xWindow`, `yWindow`, `windowWidth`, `windowHeight`).
    /// Whether tabRatio is set.
    pub fn has_tab_ratio(&self) -> Result<bool> {
        Ok(self.tab_ratio()?.is_some())
    }

    /// Clear tabRatio from workbook view.
    pub fn clear_tab_ratio(&mut self) -> Result<bool> {
        self.clear_workbook_view_attr("tabRatio")
    }


    pub fn set_workbook_window(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let wb_uri = self.ensure_workbook()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&wb_uri) {
            parse_element(data)?
        } else {
            return Err(Error::Package("workbook missing".into()));
        };
        let view = self.ensure_workbook_view_mut(&mut root);
        view.set_attribute("xWindow", x.to_string());
        view.set_attribute("yWindow", y.to_string());
        view.set_attribute("windowWidth", width.to_string());
        view.set_attribute("windowHeight", height.to_string());
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Read workbook window as `(x, y, width, height)` when any attr present.
    pub fn workbook_window(&self) -> Result<Option<(i32, i32, u32, u32)>> {
        let x = self.workbook_view_attr("xWindow")?;
        let y = self.workbook_view_attr("yWindow")?;
        let w = self.workbook_view_attr("windowWidth")?;
        let h = self.workbook_view_attr("windowHeight")?;
        if x.is_none() && y.is_none() && w.is_none() && h.is_none() {
            return Ok(None);
        }
        Ok(Some((
            x.and_then(|s| s.parse().ok()).unwrap_or(0),
            y.and_then(|s| s.parse().ok()).unwrap_or(0),
            w.and_then(|s| s.parse().ok()).unwrap_or(0),
            h.and_then(|s| s.parse().ok()).unwrap_or(0),
        )))
    }

    /// Set `showHorizontalScroll`.

    fn clear_workbook_view_attr(&mut self, attr: &str) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(views) = root.child_mut("bookViews") else {
            return Ok(false);
        };
        let Some(view) = views.child_mut("workbookView") else {
            return Ok(false);
        };
        let before = view.attributes.len();
        view.attributes.retain(|a| a.local_name != attr);
        if view.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(true)
    }

    /// Whether workbook window position/size attrs are set.
    pub fn has_workbook_window(&self) -> Result<bool> {
        Ok(self.workbook_window()?.is_some())
    }

    /// Clear workbook view window position/size attributes.
    pub fn clear_workbook_window(&mut self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(views) = root.child_mut("bookViews") else {
            return Ok(false);
        };
        let Some(view) = views.child_mut("workbookView") else {
            return Ok(false);
        };
        let before = view.attributes.len();
        view.attributes.retain(|a| {
            !matches!(
                a.local_name.as_str(),
                "xWindow" | "yWindow" | "windowWidth" | "windowHeight"
            )
        });
        if view.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(true)
    }

    /// Whether showHorizontalScroll is explicitly set.
    pub fn has_show_horizontal_scroll(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("bookViews")
            .and_then(|v| v.child("workbookView"))
            .and_then(|v| v.get_attribute("showHorizontalScroll"))
            .is_some())
    }

    /// Clear showHorizontalScroll from workbook view.
    pub fn clear_show_horizontal_scroll(&mut self) -> Result<bool> {
        self.clear_workbook_view_attr("showHorizontalScroll")
    }

    pub fn set_show_horizontal_scroll(&mut self, show: bool) -> Result<()> {
        self.set_workbook_view_attr("showHorizontalScroll", if show { "1" } else { "0" })
    }

    /// Whether horizontal scroll is shown (default true).
    pub fn show_horizontal_scroll(&self) -> Result<bool> {
        self.workbook_view_bool("showHorizontalScroll", true)
    }

    /// Set `showVerticalScroll`.
    pub fn set_show_vertical_scroll(&mut self, show: bool) -> Result<()> {
        self.set_workbook_view_attr("showVerticalScroll", if show { "1" } else { "0" })
    }

    /// Whether vertical scroll is shown (default true).
    pub fn show_vertical_scroll(&self) -> Result<bool> {
        self.workbook_view_bool("showVerticalScroll", true)
    }

    /// Set `showSheetTabs`.
    /// Whether showVerticalScroll is explicitly set.
    pub fn has_show_vertical_scroll(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("bookViews")
            .and_then(|v| v.child("workbookView"))
            .and_then(|v| v.get_attribute("showVerticalScroll"))
            .is_some())
    }

    /// Clear showVerticalScroll from workbook view.
    pub fn clear_show_vertical_scroll(&mut self) -> Result<bool> {
        self.clear_workbook_view_attr("showVerticalScroll")
    }


    pub fn set_show_sheet_tabs(&mut self, show: bool) -> Result<()> {
        self.set_workbook_view_attr("showSheetTabs", if show { "1" } else { "0" })
    }

    /// Whether sheet tabs are shown (default true).
    pub fn show_sheet_tabs(&self) -> Result<bool> {
        self.workbook_view_bool("showSheetTabs", true)
    }

    /// Set workbook view `minimized`.
    /// Whether showSheetTabs is explicitly set.
    pub fn has_show_sheet_tabs(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("bookViews")
            .and_then(|v| v.child("workbookView"))
            .and_then(|v| v.get_attribute("showSheetTabs"))
            .is_some())
    }

    /// Clear showSheetTabs from workbook view.
    pub fn clear_show_sheet_tabs(&mut self) -> Result<bool> {
        self.clear_workbook_view_attr("showSheetTabs")
    }


    pub fn set_workbook_minimized(&mut self, minimized: bool) -> Result<()> {
        self.set_workbook_view_attr("minimized", if minimized { "1" } else { "0" })
    }

    /// Whether workbook window is minimized.
    pub fn workbook_minimized(&self) -> Result<bool> {
        self.workbook_view_bool("minimized", false)
    }

    /// Set workbook view `visibility` (`"visible"`, `"hidden"`, `"veryHidden"`).
    /// Whether minimized is explicitly set.
    pub fn has_workbook_minimized(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("bookViews")
            .and_then(|v| v.child("workbookView"))
            .and_then(|v| v.get_attribute("minimized"))
            .is_some())
    }

    /// Clear minimized attribute from workbook view.
    pub fn clear_workbook_minimized(&mut self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(views) = root.child_mut("bookViews") else {
            return Ok(false);
        };
        let Some(view) = views.child_mut("workbookView") else {
            return Ok(false);
        };
        let before = view.attributes.len();
        view.attributes.retain(|a| a.local_name != "minimized");
        if view.attributes.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(wb_uri, self.document_type.content_type(), xml);
        Ok(true)
    }


    pub fn set_workbook_visibility(&mut self, visibility: &str) -> Result<()> {
        self.set_workbook_view_attr("visibility", visibility)
    }

    /// Read workbook visibility.
    pub fn workbook_visibility(&self) -> Result<Option<String>> {
        self.workbook_view_attr("visibility")
    }

    /// Set `autoFilterDateGrouping`.
    /// Whether visibility is explicitly set.
    pub fn has_workbook_visibility(&self) -> Result<bool> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let Some(data) = self.package.opc().get_part(&wb_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("bookViews")
            .and_then(|v| v.child("workbookView"))
            .and_then(|v| v.get_attribute("visibility"))
            .is_some())
    }

    /// Clear visibility from workbook view.
    pub fn clear_workbook_visibility(&mut self) -> Result<bool> {
        self.clear_workbook_view_attr("visibility")
    }


    pub fn set_auto_filter_date_grouping(&mut self, enabled: bool) -> Result<()> {
        self.set_workbook_view_attr(
            "autoFilterDateGrouping",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether autoFilterDateGrouping is enabled (default true).
    pub fn auto_filter_date_grouping(&self) -> Result<bool> {
        self.workbook_view_bool("autoFilterDateGrouping", true)
    }

    /// Whether autoFilterDateGrouping attribute is present.
    pub fn has_auto_filter_date_grouping_attr(&self) -> Result<bool> {
        Ok(self.workbook_view_attr("autoFilterDateGrouping")?.is_some())
    }

    /// Clear autoFilterDateGrouping from workbook view.
    pub fn clear_auto_filter_date_grouping(&mut self) -> Result<bool> {
        self.clear_workbook_view_attr("autoFilterDateGrouping")
    }

    /// Set default row height (and optional column width) via `sheetFormatPr`.
    pub fn set_sheet_format(
        &mut self,
        sheet_name: &str,
        default_row_height: f64,
        default_col_width: Option<f64>,
    ) -> Result<()> {
        use crate::spreadsheet::sheet_format_properties;
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "sheetFormatPr");
        let insert_at = root
            .children
            .iter()
            .position(|c| matches!(c.local_name.as_str(), "cols" | "sheetData"))
            .unwrap_or(0);
        root.children.insert(
            insert_at,
            sheet_format_properties(default_row_height, default_col_width),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read `sheetFormatPr` as `(default_row_height, default_col_width?)`.
    pub fn sheet_format(
        &self,
        sheet_name: &str,
    ) -> Result<Option<(f64, Option<f64>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(fmt) = root.child("sheetFormatPr") else {
            return Ok(None);
        };
        let row_h = fmt
            .get_attribute("defaultRowHeight")
            .and_then(|s| s.parse().ok())
            .unwrap_or(15.0);
        let col_w = fmt
            .get_attribute("defaultColWidth")
            .and_then(|s| s.parse().ok());
        Ok(Some((row_h, col_w)))
    }

    /// Set only `defaultRowHeight` on sheetFormatPr.
    pub fn set_default_row_height(&mut self, sheet_name: &str, height: f64) -> Result<()> {
        self.set_sheet_format_attr(sheet_name, "defaultRowHeight", &height.to_string())
    }

    /// Read defaultRowHeight when sheetFormatPr is present.
    pub fn default_row_height(&self, sheet_name: &str) -> Result<Option<f64>> {
        Ok(self
            .sheet_format_attr(sheet_name, "defaultRowHeight")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set only `defaultColWidth` on sheetFormatPr.
    /// Whether `defaultRowHeight` is set on sheetFormatPr.
    pub fn has_default_row_height(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_format_attr(sheet_name, "defaultRowHeight")?.is_some())
    }

    /// Clear `defaultRowHeight` from sheetFormatPr.
    pub fn clear_default_row_height(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_format_attr(sheet_name, "defaultRowHeight")
    }


    pub fn set_default_col_width(&mut self, sheet_name: &str, width: f64) -> Result<()> {
        self.set_sheet_format_attr(sheet_name, "defaultColWidth", &width.to_string())
    }

    /// Read defaultColWidth when present.
    pub fn default_col_width(&self, sheet_name: &str) -> Result<Option<f64>> {
        Ok(self
            .sheet_format_attr(sheet_name, "defaultColWidth")?
            .and_then(|s| s.parse().ok()))
    }

    fn ensure_sheet_format_pr_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("sheetFormatPr").is_none() {
            let el = OpenXmlElement::new("x", x, "sheetFormatPr")
                .with_attribute("defaultRowHeight", "15");
            let insert_at = root
                .children
                .iter()
                .position(|c| matches!(c.local_name.as_str(), "cols" | "sheetData"))
                .unwrap_or(0);
            root.children.insert(insert_at, el);
        }
        root.child_mut("sheetFormatPr").expect("sheetFormatPr")
    }

    fn set_sheet_format_attr(
        &mut self,
        sheet_name: &str,
        attr: &str,
        value: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let fmt = self.ensure_sheet_format_pr_mut(&mut root);
        fmt.set_attribute(attr, value);
        self.save_sheet_root(&sheet_uri, &root)
    }

    fn clear_sheet_format_attr(&mut self, sheet_name: &str, attr: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(fmt) = root.child_mut("sheetFormatPr") else {
            return Ok(false);
        };
        let before = fmt.attributes.len();
        fmt.attributes.retain(|a| a.local_name != attr);
        if fmt.attributes.len() == before {
            return Ok(false);
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    fn sheet_format_attr(&self, sheet_name: &str, attr: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetFormatPr")
            .and_then(|f| f.get_attribute(attr).map(|s| s.to_string())))
    }

    /// Set `sheetFormatPr/@baseColWidth`.
    /// Whether `defaultColWidth` is set on sheetFormatPr.
    pub fn has_default_col_width(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_format_attr(sheet_name, "defaultColWidth")?.is_some())
    }

    /// Clear `defaultColWidth` from sheetFormatPr.
    pub fn clear_default_col_width(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_format_attr(sheet_name, "defaultColWidth")
    }


    pub fn set_base_col_width(&mut self, sheet_name: &str, width: u32) -> Result<()> {
        self.set_sheet_format_attr(sheet_name, "baseColWidth", &width.to_string())
    }

    /// Read baseColWidth.
    pub fn base_col_width(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_format_attr(sheet_name, "baseColWidth")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set `sheetFormatPr/@zeroHeight` (hide all rows by default height).
    /// Whether `baseColWidth` is set on sheetFormatPr.
    pub fn has_base_col_width(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_format_attr(sheet_name, "baseColWidth")?.is_some())
    }

    /// Clear `baseColWidth` from sheetFormatPr.
    pub fn clear_base_col_width(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_format_attr(sheet_name, "baseColWidth")
    }


    pub fn set_zero_height(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_format_attr(sheet_name, "zeroHeight", if enabled { "1" } else { "0" })
    }

    /// Whether zeroHeight is enabled.
    pub fn zero_height(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_format_attr(sheet_name, "zeroHeight")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `sheetFormatPr/@thickTop`.
    /// Disable `zero height` on a sheet. Returns whether it was enabled.
    pub fn clear_zero_height(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.zero_height(sheet_name)?;
        if had {
            self.set_zero_height(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_thick_top(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_format_attr(sheet_name, "thickTop", if enabled { "1" } else { "0" })
    }

    /// Whether thickTop is enabled.
    pub fn thick_top(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_format_attr(sheet_name, "thickTop")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `sheetFormatPr/@thickBottom`.
    /// Disable `thick top` on a sheet. Returns whether it was enabled.
    pub fn clear_thick_top(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.thick_top(sheet_name)?;
        if had {
            self.set_thick_top(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_thick_bottom(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_format_attr(sheet_name, "thickBottom", if enabled { "1" } else { "0" })
    }

    /// Whether thickBottom is enabled.
    pub fn thick_bottom(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_format_attr(sheet_name, "thickBottom")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `sheetFormatPr/@customHeight` (all rows use custom height by default).
    /// Disable `thick bottom` on a sheet. Returns whether it was enabled.
    pub fn clear_thick_bottom(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.thick_bottom(sheet_name)?;
        if had {
            self.set_thick_bottom(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_custom_height(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_format_attr(
            sheet_name,
            "customHeight",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether customHeight is enabled on sheetFormatPr.
    pub fn custom_height(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_format_attr(sheet_name, "customHeight")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set max outline level for rows (`outlineLevelRow`, 0–7).
    /// Disable `custom height` on a sheet. Returns whether it was enabled.
    pub fn clear_custom_height(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.custom_height(sheet_name)?;
        if had {
            self.set_custom_height(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_outline_level_row(&mut self, sheet_name: &str, level: u8) -> Result<()> {
        self.set_sheet_format_attr(sheet_name, "outlineLevelRow", &level.to_string())
    }

    /// Read outlineLevelRow.
    pub fn outline_level_row(&self, sheet_name: &str) -> Result<Option<u8>> {
        Ok(self
            .sheet_format_attr(sheet_name, "outlineLevelRow")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set max outline level for columns (`outlineLevelCol`, 0–7).
    /// Whether `outlineLevelRow` is set.
    pub fn has_outline_level_row(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.outline_level_row(sheet_name)?.is_some())
    }

    /// Clear `outlineLevelRow` from sheetFormatPr.
    pub fn clear_outline_level_row(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_format_attr(sheet_name, "outlineLevelRow")
    }


    pub fn set_outline_level_col(&mut self, sheet_name: &str, level: u8) -> Result<()> {
        self.set_sheet_format_attr(sheet_name, "outlineLevelCol", &level.to_string())
    }

    /// Read outlineLevelCol.
    pub fn outline_level_col(&self, sheet_name: &str) -> Result<Option<u8>> {
        Ok(self
            .sheet_format_attr(sheet_name, "outlineLevelCol")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether sheetFormatPr is present.
    /// Whether `outlineLevelCol` is set.
    pub fn has_outline_level_col(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.outline_level_col(sheet_name)?.is_some())
    }

    /// Clear `outlineLevelCol` from sheetFormatPr.
    pub fn clear_outline_level_col(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_format_attr(sheet_name, "outlineLevelCol")
    }


    pub fn has_sheet_format(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_format(sheet_name)?.is_some())
    }

    /// Clear sheetFormatPr. Returns whether it was present.
    pub fn clear_sheet_format(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "sheetFormatPr");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Set the worksheet used range dimension (`x:dimension`).
    /// Clear sheet format properties on every sheet. Returns sheets modified.
    pub fn clear_all_sheet_format(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_sheet_format(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn set_sheet_dimension(&mut self, sheet_name: &str, reference: &str) -> Result<()> {
        use crate::spreadsheet::sheet_dimension;
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "dimension");
        // dimension is typically the first child
        root.children.insert(0, sheet_dimension(reference));
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Set worksheet tab color RGB (e.g. `"FF0000"`) under `sheetPr/tabColor`.
    pub fn set_tab_color(&mut self, sheet_name: &str, rgb: &str) -> Result<()> {
        let x = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let tab = OpenXmlElement::new("x", x, "tabColor").with_attribute("rgb", rgb);
        if let Some(pr) = root.child_mut("sheetPr") {
            pr.children.retain(|c| c.local_name != "tabColor");
            pr.append_child(tab);
        } else {
            let pr = OpenXmlElement::new("x", x, "sheetPr").with_child(tab);
            // sheetPr typically first after dimension
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name != "dimension")
                .unwrap_or(0);
            root.children.insert(insert_at, pr);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read worksheet tab color RGB if present.
    pub fn tab_color(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.child("tabColor"))
            .and_then(|t| t.get_attribute("rgb").map(|s| s.to_string())))
    }

    /// Whether the sheet has a tab color.
    pub fn has_tab_color(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.tab_color(sheet_name)?.is_some())
    }

    /// Remove tab color from the sheet. Returns whether it was present.
    pub fn clear_tab_color(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = false;
        if let Some(pr) = root.child_mut("sheetPr") {
            let before = pr.children.len();
            pr.children.retain(|c| c.local_name != "tabColor");
            removed = pr.children.len() < before;
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Alias for [`sheet_dimension`](Self::sheet_dimension).
    /// Clear tab colors on every sheet. Returns sheets modified.
    pub fn clear_all_tab_colors(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_tab_color(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn get_sheet_dimension(&self, sheet_name: &str) -> Result<Option<String>> {
        self.sheet_dimension(sheet_name)
    }

    /// Alias for [`auto_filter_ref`](Self::auto_filter_ref) presence check.
    pub fn has_auto_filter_range(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.auto_filter_ref(sheet_name)?.is_some())
    }


    /// Read the worksheet `x:dimension` ref attribute, if present.
    pub fn sheet_dimension(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("dimension")
            .and_then(|d| d.get_attribute("ref").map(|s| s.to_string())))
    }

    /// Whether the sheet has an explicit `x:dimension` element.
    pub fn has_sheet_dimension(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_dimension(sheet_name)?.is_some())
    }

    /// Remove the sheet `x:dimension` element. Returns whether it was present.
    pub fn clear_sheet_dimension(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "dimension");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Compute the used range from existing cells (min/max row/col that have cells).
    ///
    /// Returns an A1-style range like `"A1:C10"`, or `None` if the sheet has no cells.
    /// Optionally writes the computed range into `x:dimension` when `update_dimension` is true.
    pub fn used_range(
        &mut self,
        sheet_name: &str,
        update_dimension: bool,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d.to_vec(),
            None => return Ok(None),
        };
        let root = parse_element(&data)?;
        let mut min_r = u32::MAX;
        let mut max_r = 0u32;
        let mut min_c = u32::MAX;
        let mut max_c = 0u32;
        let mut any = false;
        for cell in root.descendants().filter(|e| e.local_name == "c") {
            if let Some(r) = cell.get_attribute("r") {
                if let Some((rr, cc)) = cell_ref_to_row_col(r) {
                    any = true;
                    min_r = min_r.min(rr);
                    max_r = max_r.max(rr);
                    min_c = min_c.min(cc);
                    max_c = max_c.max(cc);
                }
            }
        }
        if !any {
            return Ok(None);
        }
        let range = format!(
            "{}{}:{}{}",
            column_name(min_c as usize),
            min_r + 1,
            column_name(max_c as usize),
            max_r + 1
        );
        if update_dimension {
            self.set_sheet_dimension(sheet_name, &range)?;
        }
        Ok(Some(range))
    }

    /// Write a shared formula across cells.
    ///
    /// The first cell in `cells` is the master (holds the formula); others reference `si`.
    /// `cells` are A1 references on `sheet_name`. `cached` optional parallel cached values.
    pub fn set_shared_formula(
        &mut self,
        sheet_name: &str,
        cells: &[&str],
        formula: &str,
        cached: &[Option<&str>],
        si: u32,
    ) -> Result<()> {
        use crate::spreadsheet::cell_shared_formula;
        if cells.is_empty() {
            return Ok(());
        }
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let sheet_data = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("worksheet has no sheetData".into()))?;

        for (i, cell_ref) in cells.iter().enumerate() {
            let cached_v = cached.get(i).copied().flatten();
            let formula_text = if i == 0 { Some(formula) } else { None };
            let cell_el = cell_shared_formula(cell_ref, si, formula_text, cached_v);
            // Parse row number from A1
            let row_idx: u32 = cell_ref
                .bytes()
                .skip_while(|b| b.is_ascii_alphabetic())
                .map(|b| b as char)
                .collect::<String>()
                .parse()
                .unwrap_or(1);
            // Find or create row
            let row_el = if let Some(r) = sheet_data.children.iter_mut().find(|c| {
                c.local_name == "row"
                    && c.get_attribute("r").and_then(|s| s.parse().ok()) == Some(row_idx)
            }) {
                r
            } else {
                sheet_data.append_child(row(row_idx, Vec::<crate::element::OpenXmlElement>::new()));
                sheet_data.children.last_mut().unwrap()
            };
            // Replace existing cell with same ref
            row_el
                .children
                .retain(|c| c.get_attribute("r") != Some(*cell_ref));
            row_el.append_child(cell_el);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Set the worksheet tab color.
    ///
    /// `rgb` is hex RGB (`"FF0000"`) or ARGB (`"FFFF0000"`).
    pub fn set_sheet_tab_color(&mut self, sheet_name: &str, rgb: &str) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let argb = if rgb.len() == 6 {
            format!("FF{rgb}")
        } else {
            rgb.to_string()
        };
        // sheetPr / tabColor
        if let Some(pr) = root.child_mut("sheetPr") {
            pr.children.retain(|c| c.local_name != "tabColor");
            pr.append_child(
                crate::element::OpenXmlElement::new(
                    "x",
                    crate::namespace::ns::SPREADSHEETML.uri,
                    "tabColor",
                )
                .with_attribute("rgb", argb),
            );
        } else {
            let pr = crate::element::OpenXmlElement::new(
                "x",
                crate::namespace::ns::SPREADSHEETML.uri,
                "sheetPr",
            )
            .with_child(
                crate::element::OpenXmlElement::new(
                    "x",
                    crate::namespace::ns::SPREADSHEETML.uri,
                    "tabColor",
                )
                .with_attribute("rgb", argb),
            );
            root.children.insert(0, pr);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read worksheet tab color RGB/ARGB, if set.
    pub fn sheet_tab_color(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|pr| pr.child("tabColor"))
            .and_then(|c| c.get_attribute("rgb").map(|s| s.to_string())))
    }

    /// Add a sparkline group to a worksheet (Office 2010+ `x14` extension).
    ///
    /// `sparkline_type` is `"line"`, `"column"`, or `"stacked"`.
    /// `data_ref` is the source range (e.g. `"Sheet1!A1:A12"`); `cell_ref` is where
    /// the sparkline is displayed (e.g. `"B1"`).
    pub fn add_sparkline(
        &mut self,
        sheet_name: &str,
        sparkline_type: &str,
        data_ref: &str,
        cell_ref: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let uri = "{05C60535-1F16-4fd2-B633-F4F36F0B64E0}";

        // Prefer appending into an existing sparklineGroups container.
        let mut appended = false;
        if let Some(ext_lst) = root.child_mut("extLst") {
            for ext in ext_lst
                .children
                .iter_mut()
                .filter(|c| c.get_attribute("uri") == Some(uri))
            {
                // child is sparklineGroups
                if let Some(groups) = ext.child_mut("sparklineGroups") {
                    // try same-type group first
                    if let Some(group) = groups
                        .children
                        .iter_mut()
                        .find(|g| {
                            g.local_name == "sparklineGroup"
                                && g.get_attribute("type").unwrap_or("line") == sparkline_type
                        })
                    {
                        if let Some(sps) = group.child_mut("sparklines") {
                            // replace same cell if present
                            sps.children.retain(|sp| {
                                if sp.local_name != "sparkline" {
                                    return true;
                                }
                                let cell = sp
                                    .child("sqref")
                                    .map(|s| s.inner_text())
                                    .or_else(|| {
                                        sp.descendants()
                                            .find(|e| e.local_name == "sqref")
                                            .map(|s| s.inner_text())
                                    })
                                    .unwrap_or_default();
                                cell != cell_ref
                            });
                            sps.append_child(sparkline(data_ref, cell_ref));
                            appended = true;
                            break;
                        }
                    }
                    if !appended {
                        groups.append_child(sparkline_group(sparkline_type, data_ref, cell_ref));
                        appended = true;
                        break;
                    }
                }
            }
        }

        if !appended {
            let group = sparkline_group(sparkline_type, data_ref, cell_ref);
            let groups = sparkline_groups(vec![group]);
            let ext = sparkline_ext(groups);
            if let Some(ext_lst) = root.child_mut("extLst") {
                ext_lst.children.retain(|c| c.get_attribute("uri") != Some(uri));
                ext_lst.append_child(ext);
            } else {
                let x = crate::namespace::ns::SPREADSHEETML.uri;
                let lst = crate::element::OpenXmlElement::new("x", x, "extLst").with_child(ext);
                root.append_child(lst);
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether the sheet has sparkline extensions.
    pub fn has_sparklines(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root.descendants().any(|e| {
            e.local_name == "sparklineGroup"
                || e.get_attribute("uri") == Some("{05C60535-1F16-4fd2-B633-F4F36F0B64E0}")
        }))
    }

    /// Remove sparkline extensions from a worksheet. Returns whether any were present.
    pub fn clear_sparklines(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = false;
        if let Some(ext_lst) = root.child_mut("extLst") {
            let before = ext_lst.children.len();
            ext_lst.children.retain(|c| {
                c.get_attribute("uri") != Some("{05C60535-1F16-4fd2-B633-F4F36F0B64E0}")
            });
            removed = ext_lst.children.len() < before;
            if ext_lst.children.is_empty() {
                root.children.retain(|c| c.local_name != "extLst");
            }
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Number of sparkline instances on a sheet.
    /// Clear sparklines on every sheet. Returns sheets modified.
    pub fn clear_all_sparklines(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_sparklines(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn sparkline_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_sparklines(sheet_name)?.len())
    }

    /// List sparkline groups as `(type, data_ref, cell_ref)`.
    pub fn list_sparklines(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let mut out = Vec::new();
        for group in root.descendants().filter(|e| e.local_name == "sparklineGroup") {
            let ty = group.get_attribute("type").unwrap_or("line").to_string();
            for sp in group.descendants().filter(|e| e.local_name == "sparkline") {
                let data_ref = sp
                    .child("f")
                    .map(|f| f.inner_text())
                    .or_else(|| {
                        sp.descendants()
                            .find(|e| e.local_name == "f")
                            .map(|f| f.inner_text())
                    })
                    .unwrap_or_default();
                let cell = sp
                    .child("sqref")
                    .map(|s| s.inner_text())
                    .or_else(|| {
                        sp.descendants()
                            .find(|e| e.local_name == "sqref")
                            .map(|s| s.inner_text())
                    })
                    .unwrap_or_default();
                out.push((ty.clone(), data_ref, cell));
            }
        }
        Ok(out)
    }


    /// Remove sparklines whose cell `sqref` matches `cell_ref`. Returns how many were removed.
    ///
    /// Empty sparkline groups (and the sparklines extension host when empty) are pruned.
    pub fn remove_sparkline(&mut self, sheet_name: &str, cell_ref: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = 0usize;
        fn sparkline_cell(sp: &OpenXmlElement) -> String {
            sp.child("sqref")
                .map(|s| s.inner_text())
                .or_else(|| {
                    sp.descendants()
                        .find(|e| e.local_name == "sqref")
                        .map(|s| s.inner_text())
                })
                .unwrap_or_default()
        }
        fn visit(el: &mut OpenXmlElement, cell_ref: &str, removed: &mut usize) {
            if el.local_name == "sparklineGroup" {
                let before = el.children.len();
                // sparklines may be nested under sparklines container
                for c in el.children.iter_mut() {
                    if c.local_name == "sparklines" {
                        let b = c.children.len();
                        c.children.retain(|sp| {
                            if sp.local_name != "sparkline" {
                                return true;
                            }
                            sparkline_cell(sp) != cell_ref
                        });
                        *removed += b - c.children.len();
                    }
                }
                el.children.retain(|c| {
                    if c.local_name == "sparkline" {
                        if sparkline_cell(c) == cell_ref {
                            *removed += 1;
                            return false;
                        }
                    }
                    if c.local_name == "sparklines" {
                        return c.children.iter().any(|s| s.local_name == "sparkline");
                    }
                    true
                });
                let _ = before;
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, cell_ref, removed);
            }
        }
        visit(&mut root, cell_ref, &mut removed);
        // drop empty sparklineGroup nodes
        fn prune_empty_groups(el: &mut OpenXmlElement) {
            for c in el.children.iter_mut() {
                prune_empty_groups(c);
            }
            el.children.retain(|c| {
                if c.local_name != "sparklineGroup" {
                    return true;
                }
                c.descendants().any(|e| e.local_name == "sparkline")
            });
        }
        prune_empty_groups(&mut root);
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Set attributes on the first sparkline group (type, displayEmptyCellsAs, markers, high/low).
    pub fn set_sparkline_group_attrs(
        &mut self,
        sheet_name: &str,
        sparkline_type: Option<&str>,
        display_empty: Option<&str>,
        markers: Option<bool>,
        high: Option<bool>,
        low: Option<bool>,
        first: Option<bool>,
        last: Option<bool>,
        negative: Option<bool>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, f: &mut impl FnMut(&mut OpenXmlElement) -> bool) -> bool {
            if f(el) {
                return true;
            }
            for c in el.children.iter_mut() {
                if visit(c, f) {
                    return true;
                }
            }
            false
        }
        visit(&mut root, &mut |group| {
            if group.local_name != "sparklineGroup" {
                return false;
            }
            found = true;
            if let Some(t) = sparkline_type {
                group.set_attribute("type", t);
            }
            if let Some(d) = display_empty {
                group.set_attribute("displayEmptyCellsAs", d);
            }
            if let Some(v) = markers {
                group.set_attribute("markers", if v { "1" } else { "0" });
            }
            if let Some(v) = high {
                group.set_attribute("high", if v { "1" } else { "0" });
            }
            if let Some(v) = low {
                group.set_attribute("low", if v { "1" } else { "0" });
            }
            if let Some(v) = first {
                group.set_attribute("first", if v { "1" } else { "0" });
            }
            if let Some(v) = last {
                group.set_attribute("last", if v { "1" } else { "0" });
            }
            if let Some(v) = negative {
                group.set_attribute("negative", if v { "1" } else { "0" });
            }
            true
        });
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear common sparkline group attributes (type/displayEmptyCellsAs/markers/high/low/first/last/negative).
    pub fn clear_sparkline_group_attrs(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        // find sparklineGroups/sparklineGroup
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, found: &mut bool) {
            if el.local_name == "sparklineGroup" {
                let before = el.attributes.len();
                el.attributes.retain(|a| {
                    !matches!(
                        a.local_name.as_str(),
                        "type"
                            | "displayEmptyCellsAs"
                            | "markers"
                            | "high"
                            | "low"
                            | "first"
                            | "last"
                            | "negative"
                    )
                });
                if el.attributes.len() < before {
                    *found = true;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, found);
            }
        }
        visit(&mut root, &mut found);
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Set print titles (repeating rows and/or columns) via defined names.
    ///
    /// `rows` e.g. `Some("$1:$1")`, `cols` e.g. `Some("$A:$A")`.
    pub fn set_print_titles(
        &mut self,
        sheet_name: &str,
        rows: Option<&str>,
        cols: Option<&str>,
    ) -> Result<()> {
        let mut refers_parts = Vec::new();
        if let Some(c) = cols {
            refers_parts.push(format!("'{sheet_name}'!{c}"));
        }
        if let Some(r) = rows {
            refers_parts.push(format!("'{sheet_name}'!{r}"));
        }
        if refers_parts.is_empty() {
            return Ok(());
        }
        let refers = refers_parts.join(",");
        let mut names = self.defined_names()?;
        names.retain(|(n, _)| n != "_xlnm.Print_Titles");
        names.push(("_xlnm.Print_Titles".into(), refers));
        let refs: Vec<(&str, &str)> = names
            .iter()
            .map(|(n, r)| (n.as_str(), r.as_str()))
            .collect();
        self.set_defined_names(&refs)
    }

    /// Define a print area via a defined name `_xlnm.Print_Area`.
    ///
    /// `range` is a sheet-local A1 range (e.g. `"$A$1:$G$50"`).
    pub fn set_print_area(&mut self, sheet_name: &str, range: &str) -> Result<()> {
        let refers = format!("'{sheet_name}'!{range}");
        // Merge with existing defined names
        let mut names = self.defined_names()?;
        names.retain(|(n, _)| n != "_xlnm.Print_Area");
        names.push(("_xlnm.Print_Area".into(), refers));
        let refs: Vec<(&str, &str)> = names
            .iter()
            .map(|(n, r)| (n.as_str(), r.as_str()))
            .collect();
        self.set_defined_names(&refs)
    }

    /// Read the print area defined name value, if present.
    pub fn print_area(&self) -> Result<Option<String>> {
        Ok(self
            .defined_names()?
            .into_iter()
            .find(|(n, _)| n == "_xlnm.Print_Area")
            .map(|(_, r)| r))
    }

    /// Read print area range for a sheet (strips `'Sheet'!` prefix when possible).
    pub fn print_area_for_sheet(&self, sheet_name: &str) -> Result<Option<String>> {
        let Some(full) = self.print_area()? else {
            return Ok(None);
        };
        // Match 'Sheet'!Range or Sheet!Range
        let quoted = format!("'{sheet_name}'!");
        let plain = format!("{sheet_name}!");
        if let Some(rest) = full.strip_prefix(&quoted) {
            return Ok(Some(rest.to_string()));
        }
        if let Some(rest) = full.strip_prefix(&plain) {
            return Ok(Some(rest.to_string()));
        }
        // multi-area: take first matching segment
        for part in full.split(',') {
            let part = part.trim();
            if let Some(rest) = part.strip_prefix(&quoted) {
                return Ok(Some(rest.to_string()));
            }
            if let Some(rest) = part.strip_prefix(&plain) {
                return Ok(Some(rest.to_string()));
            }
        }
        Ok(None)
    }

    /// Read print titles defined name value, if present.
    /// Clear print area for a sheet (local `Print_Area` and common global names).
    pub fn clear_print_area_for_sheet(&mut self, sheet_name: &str) -> Result<bool> {
        let mut removed = false;
        if self.remove_local_defined_name(sheet_name, "Print_Area")? {
            removed = true;
        }
        for name in [
            format!("{sheet_name}!Print_Area"),
            format!("'{sheet_name}'!Print_Area"),
        ] {
            if self.remove_defined_name(&name)? {
                removed = true;
            }
        }
        Ok(removed)
    }


    pub fn print_titles(&self) -> Result<Option<String>> {
        Ok(self
            .defined_names()?
            .into_iter()
            .find(|(n, _)| n == "_xlnm.Print_Titles")
            .map(|(_, r)| r))
    }

    /// Whether a print area defined name is present.
    pub fn has_print_area(&self) -> Result<bool> {
        Ok(self.print_area()?.is_some())
    }

    /// Whether print titles defined name is present.
    pub fn has_print_titles(&self) -> Result<bool> {
        Ok(self.print_titles()?.is_some())
    }

    /// Clear the print area defined name. Returns whether it was present.
    pub fn clear_print_area(&mut self) -> Result<bool> {
        self.remove_defined_name("_xlnm.Print_Area")
    }

    /// Clear print titles defined name. Returns whether it was present.
    pub fn clear_print_titles(&mut self) -> Result<bool> {
        self.remove_defined_name("_xlnm.Print_Titles")
    }

    /// Whether gridlines are shown on the worksheet view (default true when unset).
    pub fn show_gridlines(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(true),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute("showGridLines"))
            .map(|s| s != "0")
            .unwrap_or(true))
    }


    /// Remove explicit `showGridLines` attribute (restore default). Returns whether present.
    pub fn clear_show_gridlines(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "showGridLines")
    }

    /// Whether `showGridLines` is explicitly set.
    pub fn has_show_gridlines_attr(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "showGridLines")?.is_some())
    }

    /// Clear showGridLines override on every sheet. Returns sheets modified.
    pub fn clear_all_show_gridlines(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_show_gridlines(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }


    pub fn gridlines_visible(&self, sheet_name: &str) -> Result<bool> {
        self.show_gridlines(sheet_name)
    }

    /// Whether formulas are shown instead of values (default false when unset).
    pub fn show_formulas(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute("showFormulas"))
            .map(|s| s == "1")
            .unwrap_or(false))
    }

    /// Whether row/column headers are shown (default true when unset).
    /// Disable `show formulas` on a sheet. Returns whether it was enabled.
    pub fn clear_show_formulas(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "showFormulas")
    }

    /// Whether `showFormulas` is explicitly set.
    pub fn has_show_formulas_attr(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "showFormulas")?.is_some())
    }


    /// Clear showFormulas override on every sheet. Returns sheets modified.
    pub fn clear_all_show_formulas(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_show_formulas(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }


    pub fn show_row_col_headers(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(true),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute("showRowColHeaders"))
            .map(|s| s != "0")
            .unwrap_or(true))
    }


    /// Set worksheet view right-to-left mode (`sheetView/@rightToLeft`).
    /// Disable `show row col headers` on a sheet. Returns whether it was enabled.
    pub fn clear_show_row_col_headers(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "showRowColHeaders")
    }

    /// Whether `showRowColHeaders` is explicitly set.
    pub fn has_show_row_col_headers_attr(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "showRowColHeaders")?.is_some())
    }


    pub fn set_right_to_left(&mut self, sheet_name: &str, rtl: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("rightToLeft", if rtl { "1" } else { "0" });
            }
        } else {
            use crate::spreadsheet::sheet_views_zoom;
            let mut views = sheet_views_zoom(100);
            if let Some(view) = views.child_mut("sheetView") {
                view.set_attribute("rightToLeft", if rtl { "1" } else { "0" });
            }
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }


    
    /// Whether the sheet view is right-to-left.
    pub fn right_to_left(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute("rightToLeft"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Alias for [`right_to_left`](Self::right_to_left).
    pub fn has_right_to_left(&self, sheet_name: &str) -> Result<bool> {
        self.right_to_left(sheet_name)
    }

    fn ensure_sheet_view_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("sheetViews").is_none() {
            let view = OpenXmlElement::new("x", x, "sheetView").with_attribute("workbookViewId", "0");
            let views = OpenXmlElement::new("x", x, "sheetViews").with_child(view);
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(insert_at, views);
        } else if root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .is_none()
        {
            if let Some(views) = root.child_mut("sheetViews") {
                views.append_child(
                    OpenXmlElement::new("x", x, "sheetView").with_attribute("workbookViewId", "0"),
                );
            }
        }
        root.child_mut("sheetViews")
            .and_then(|v| v.child_mut("sheetView"))
            .expect("sheetView ensured")
    }

    fn set_sheet_view_attr(&mut self, sheet_name: &str, attr: &str, value: &str) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let view = self.ensure_sheet_view_mut(&mut root);
        view.set_attribute(attr, value);
        self.save_sheet_root(&sheet_uri, &root)
    }

    fn sheet_view_attr(&self, sheet_name: &str, attr: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetViews")
            .and_then(|v| v.child("sheetView"))
            .and_then(|sv| sv.get_attribute(attr).map(|s| s.to_string())))
    }

    fn sheet_view_bool_attr(&self, sheet_name: &str, attr: &str, default: bool) -> Result<bool> {
        Ok(self
            .sheet_view_attr(sheet_name, attr)?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    /// Set whether the sheet tab is selected (`tabSelected`).
    /// Disable right-to-left on a sheet. Returns whether it was enabled.
    pub fn clear_right_to_left(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "rightToLeft")
    }

    /// Whether `rightToLeft` is explicitly set.
    pub fn has_right_to_left_attr(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "rightToLeft")?.is_some())
    }


    pub fn set_tab_selected(&mut self, sheet_name: &str, selected: bool) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "tabSelected", if selected { "1" } else { "0" })
    }

    /// Whether the sheet tab is selected.
    pub fn tab_selected(&self, sheet_name: &str) -> Result<bool> {
        self.sheet_view_bool_attr(sheet_name, "tabSelected", false)
    }

    /// Set `showRuler` on the sheet view.
    /// Disable `tab selected` on a sheet. Returns whether it was enabled.
    pub fn clear_tab_selected(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "tabSelected")
    }

    /// Whether `tabSelected` is explicitly set.
    pub fn has_tab_selected(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "tabSelected")?.is_some())
    }


    pub fn set_show_ruler(&mut self, sheet_name: &str, show: bool) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "showRuler", if show { "1" } else { "0" })
    }

    /// Whether the ruler is shown (default true when unset).
    pub fn show_ruler(&self, sheet_name: &str) -> Result<bool> {
        self.sheet_view_bool_attr(sheet_name, "showRuler", true)
    }

    /// Set `showWhiteSpace` on the sheet view.
    /// Disable `show ruler` on a sheet. Returns whether it was enabled.
    pub fn clear_show_ruler(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "showRuler")
    }

    /// Whether `showRuler` is explicitly set.
    pub fn has_show_ruler_attr(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "showRuler")?.is_some())
    }


    pub fn set_show_white_space(&mut self, sheet_name: &str, show: bool) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "showWhiteSpace", if show { "1" } else { "0" })
    }

    /// Whether white space is shown (default true when unset).
    pub fn show_white_space(&self, sheet_name: &str) -> Result<bool> {
        self.sheet_view_bool_attr(sheet_name, "showWhiteSpace", true)
    }

    /// Set `defaultGridColor` on the sheet view.
    /// Disable `show white space` on a sheet. Returns whether it was enabled.
    pub fn clear_show_white_space(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "showWhiteSpace")
    }

    /// Whether `showWhiteSpace` is explicitly set.
    pub fn has_show_white_space_attr(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_view_attr(sheet_name, "showWhiteSpace")?.is_some())
    }


    pub fn set_default_grid_color(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_view_attr(
            sheet_name,
            "defaultGridColor",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether default grid color is used (default true when unset).
    pub fn default_grid_color(&self, sheet_name: &str) -> Result<bool> {
        self.sheet_view_bool_attr(sheet_name, "defaultGridColor", true)
    }

    /// Set `windowProtection` on the sheet view.
    /// Disable `default grid color` on a sheet. Returns whether it was enabled.
    pub fn clear_default_grid_color(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.default_grid_color(sheet_name)?;
        if had {
            self.set_default_grid_color(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_window_protection(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_view_attr(
            sheet_name,
            "windowProtection",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether window protection is enabled on the sheet view.
    pub fn window_protection(&self, sheet_name: &str) -> Result<bool> {
        self.sheet_view_bool_attr(sheet_name, "windowProtection", false)
    }

    /// Set the top-left visible cell (`topLeftCell`), e.g. `"B2"`.
    /// Disable `window protection` on a sheet. Returns whether it was enabled.
    pub fn clear_window_protection(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.window_protection(sheet_name)?;
        if had {
            self.set_window_protection(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_top_left_cell(&mut self, sheet_name: &str, cell_ref: &str) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "topLeftCell", cell_ref)
    }

    /// Read `topLeftCell` when present.
    pub fn top_left_cell(&self, sheet_name: &str) -> Result<Option<String>> {
        self.sheet_view_attr(sheet_name, "topLeftCell")
    }

    /// Set page-layout view zoom scale (`zoomScalePageLayoutView`).
    /// Whether topLeftCell is set.
    pub fn has_top_left_cell(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.top_left_cell(sheet_name)?.is_some())
    }

    /// Clear topLeftCell.
    pub fn clear_top_left_cell(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "topLeftCell")
    }


    pub fn set_zoom_scale_page_layout(&mut self, sheet_name: &str, percent: u32) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "zoomScalePageLayoutView", &percent.to_string())
    }

    /// Read `zoomScalePageLayoutView` when present.
    pub fn zoom_scale_page_layout(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_view_attr(sheet_name, "zoomScalePageLayoutView")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set sheet-layout (page break preview) zoom (`zoomScaleSheetLayoutView`).
    pub fn set_zoom_scale_sheet_layout(&mut self, sheet_name: &str, percent: u32) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "zoomScaleSheetLayoutView", &percent.to_string())
    }

    /// Read `zoomScaleSheetLayoutView` when present.
    pub fn zoom_scale_sheet_layout(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_view_attr(sheet_name, "zoomScaleSheetLayoutView")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set normal-view zoom scale (`zoomScaleNormal`).
    pub fn set_zoom_scale_normal(&mut self, sheet_name: &str, percent: u32) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "zoomScaleNormal", &percent.to_string())
    }

    /// Read `zoomScaleNormal` when present.
    pub fn zoom_scale_normal(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_view_attr(sheet_name, "zoomScaleNormal")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether `zoomScaleNormal` is set.
    pub fn has_zoom_scale_normal(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.zoom_scale_normal(sheet_name)?.is_some())
    }

    /// Clear `zoomScaleNormal`.
    pub fn clear_zoom_scale_normal(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "zoomScaleNormal")
    }

    /// Whether `zoomScalePageLayoutView` is set.
    pub fn has_zoom_scale_page_layout(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.zoom_scale_page_layout(sheet_name)?.is_some())
    }

    /// Clear `zoomScalePageLayoutView`.
    pub fn clear_zoom_scale_page_layout(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "zoomScalePageLayoutView")
    }

    /// Whether `zoomScaleSheetLayoutView` is set.
    pub fn has_zoom_scale_sheet_layout(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.zoom_scale_sheet_layout(sheet_name)?.is_some())
    }

    /// Clear `zoomScaleSheetLayoutView`.
    pub fn clear_zoom_scale_sheet_layout(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "zoomScaleSheetLayoutView")
    }

    /// Set custom grid color index (`colorId`). Use with `defaultGridColor=false`.
    pub fn set_color_id(&mut self, sheet_name: &str, color_id: u32) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "colorId", &color_id.to_string())
    }

    /// Read sheet view `colorId` when present.
    pub fn color_id(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_view_attr(sheet_name, "colorId")?
            .and_then(|s| s.parse().ok()))
    }

    /// Clear sheet view `colorId`. Returns whether present.
        /// Whether `colorId` is set on the sheet view.
    pub fn has_color_id(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.color_id(sheet_name)?.is_some())
    }

pub fn clear_color_id(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = false;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                let before = view.attributes.len();
                view.attributes.retain(|a| a.local_name != "colorId");
                removed = view.attributes.len() < before;
            }
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }


    fn clear_sheet_view_attr(&mut self, sheet_name: &str, attr: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = false;
        if let Some(views) = root.child_mut("sheetViews") {
            if let Some(view) = views.child_mut("sheetView") {
                let before = view.attributes.len();
                view.attributes.retain(|a| a.local_name != attr);
                removed = view.attributes.len() < before;
            }
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear colorId on every sheet. Returns sheets modified.
    pub fn clear_all_color_id(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_color_id(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }


    /// Set the workbook view index this sheet view is associated with (`workbookViewId`).
    pub fn set_workbook_view_id(&mut self, sheet_name: &str, id: u32) -> Result<()> {
        self.set_sheet_view_attr(sheet_name, "workbookViewId", &id.to_string())
    }

    /// Read `workbookViewId` (defaults to 0 when unset).
    pub fn workbook_view_id(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_view_attr(sheet_name, "workbookViewId")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether workbookViewId is set on the sheet view.
    pub fn has_workbook_view_id(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.workbook_view_id(sheet_name)?.is_some())
    }

    /// Clear `workbookViewId` from the sheet view.
    pub fn clear_workbook_view_id(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_view_attr(sheet_name, "workbookViewId")
    }


    fn ensure_sheet_child_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
        local_name: &str,
    ) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child(local_name).is_none() {
            let el = OpenXmlElement::new("x", x, local_name);
            // pageMargins/pageSetup/printOptions typically after sheetData
            let insert_at = root
                .children
                .iter()
                .position(|c| {
                    matches!(
                        c.local_name.as_str(),
                        "drawing" | "legacyDrawing" | "tableParts" | "extLst"
                    )
                })
                .unwrap_or(root.children.len());
            root.children.insert(insert_at, el);
        }
        root.child_mut(local_name).expect("child ensured")
    }

    fn set_sheet_child_attr(
        &mut self,
        sheet_name: &str,
        child: &str,
        attr: &str,
        value: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let el = self.ensure_sheet_child_mut(&mut root, child);
        el.set_attribute(attr, value);
        self.save_sheet_root(&sheet_uri, &root)
    }

    fn clear_sheet_child_attr(
        &mut self,
        sheet_name: &str,
        child: &str,
        attr: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(el) = root.child_mut(child) else {
            return Ok(false);
        };
        let before = el.attributes.len();
        el.attributes.retain(|a| a.local_name != attr);
        if el.attributes.len() == before {
            return Ok(false);
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    fn sheet_child_attr(
        &self,
        sheet_name: &str,
        child: &str,
        attr: &str,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child(child)
            .and_then(|c| c.get_attribute(attr).map(|s| s.to_string())))
    }

    /// Set print option `horizontalCentered`.
    pub fn set_print_horizontal_centered(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "printOptions",
            "horizontalCentered",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether print is horizontally centered.
    pub fn print_horizontal_centered(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "printOptions", "horizontalCentered")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set print option `verticalCentered`.
    /// Disable `print horizontal centered` on a sheet. Returns whether it was enabled.
    pub fn clear_print_horizontal_centered(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.print_horizontal_centered(sheet_name)?;
        if had {
            self.set_print_horizontal_centered(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_print_vertical_centered(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "printOptions",
            "verticalCentered",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether print is vertically centered.
    pub fn print_vertical_centered(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "printOptions", "verticalCentered")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set print option `headings` (print row/column headings).
    /// Disable `print vertical centered` on a sheet. Returns whether it was enabled.
    pub fn clear_print_vertical_centered(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.print_vertical_centered(sheet_name)?;
        if had {
            self.set_print_vertical_centered(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_print_headings(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "printOptions",
            "headings",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether print headings is enabled.
    pub fn print_headings(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "printOptions", "headings")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set print option `gridLines`.
    /// Disable `print headings` on a sheet. Returns whether it was enabled.
    pub fn clear_print_headings(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.print_headings(sheet_name)?;
        if had {
            self.set_print_headings(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_print_grid_lines(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "printOptions",
            "gridLines",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether print grid lines is enabled.
    pub fn print_grid_lines(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "printOptions", "gridLines")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set print option `gridLinesSet` (whether gridLines flag was explicitly set).
    /// Disable `print grid lines` on a sheet. Returns whether it was enabled.
    pub fn clear_print_grid_lines(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.print_grid_lines(sheet_name)?;
        if had {
            self.set_print_grid_lines(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_print_grid_lines_set(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "printOptions",
            "gridLinesSet",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether gridLinesSet is enabled (default true when unset).
    pub fn print_grid_lines_set(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "printOptions", "gridLinesSet")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Whether any `printOptions` element is present.
    /// Disable `print grid lines set` on a sheet. Returns whether it was enabled.
    pub fn clear_print_grid_lines_set(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.print_grid_lines_set(sheet_name)?;
        if had {
            self.set_print_grid_lines_set(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn has_print_options(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root.child("printOptions").is_some())
    }

    /// Clear `printOptions`. Returns whether it was present.
    pub fn clear_print_options(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "printOptions");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Set page setup `scale` percent.
    /// Clear print options on every sheet. Returns sheets modified.
    pub fn clear_all_print_options(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_print_options(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn set_page_scale(&mut self, sheet_name: &str, scale: u32) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "scale", &scale.to_string())
    }

    /// Read page setup scale.
    pub fn page_scale(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "scale")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set page setup `fitToWidth`.
    /// Whether pageSetup `scale` is set.
    pub fn has_page_scale(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "scale")?.is_some())
    }

    /// Clear pageSetup `scale`.
    pub fn clear_page_scale(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "scale")
    }


    pub fn set_fit_to_width(&mut self, sheet_name: &str, pages: u32) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "fitToWidth", &pages.to_string())
    }

    /// Read fitToWidth.
    pub fn fit_to_width(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "fitToWidth")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set page setup `fitToHeight`.
    /// Whether pageSetup `fitToWidth` is set.
    pub fn has_fit_to_width(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "fitToWidth")?.is_some())
    }

    /// Clear pageSetup `fitToWidth`.
    pub fn clear_fit_to_width(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "fitToWidth")
    }


    pub fn set_fit_to_height(&mut self, sheet_name: &str, pages: u32) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "fitToHeight", &pages.to_string())
    }

    /// Read fitToHeight.
    pub fn fit_to_height(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "fitToHeight")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set page setup `copies`.
    /// Whether pageSetup `fitToHeight` is set.
    pub fn has_fit_to_height(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "fitToHeight")?.is_some())
    }

    /// Clear pageSetup `fitToHeight`.
    pub fn clear_fit_to_height(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "fitToHeight")
    }


    pub fn set_page_copies(&mut self, sheet_name: &str, copies: u32) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "copies", &copies.to_string())
    }

    /// Read page copies.
    pub fn page_copies(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "copies")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set page setup `blackAndWhite`.
    /// Whether pageSetup `copies` is set.
    pub fn has_page_copies(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "copies")?.is_some())
    }

    /// Clear pageSetup `copies`.
    pub fn clear_page_copies(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "copies")
    }


    pub fn set_page_black_and_white(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "pageSetup",
            "blackAndWhite",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether black-and-white print is enabled.
    pub fn page_black_and_white(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "blackAndWhite")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set page setup `draft`.
    /// Disable `page black and white` on a sheet. Returns whether it was enabled.
    pub fn clear_page_black_and_white(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.page_black_and_white(sheet_name)?;
        if had {
            self.set_page_black_and_white(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_page_draft(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "pageSetup",
            "draft",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether draft print is enabled.
    pub fn page_draft(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "draft")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set page setup `firstPageNumber`.
    /// Disable `page draft` on a sheet. Returns whether it was enabled.
    pub fn clear_page_draft(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.page_draft(sheet_name)?;
        if had {
            self.set_page_draft(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_first_page_number(&mut self, sheet_name: &str, num: u32) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "pageSetup",
            "firstPageNumber",
            &num.to_string(),
        )
    }

    /// Read firstPageNumber.
    pub fn first_page_number(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "firstPageNumber")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set page setup `pageOrder` (`"downThenOver"` / `"overThenDown"`).
    /// Whether pageSetup `firstPageNumber` is set.
    pub fn has_first_page_number(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "firstPageNumber")?.is_some())
    }

    /// Clear pageSetup `firstPageNumber`.
    pub fn clear_first_page_number(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "firstPageNumber")
    }


    pub fn set_page_order(&mut self, sheet_name: &str, order: &str) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "pageOrder", order)
    }

    /// Read pageOrder.
    pub fn page_order(&self, sheet_name: &str) -> Result<Option<String>> {
        self.sheet_child_attr(sheet_name, "pageSetup", "pageOrder")
    }

    /// Set page setup `paperSize` (e.g. 1 = Letter, 9 = A4).
    /// Whether pageSetup `pageOrder` is set.
    pub fn has_page_order(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "pageOrder")?.is_some())
    }

    /// Clear pageSetup `pageOrder`.
    pub fn clear_page_order(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "pageOrder")
    }


    pub fn set_paper_size(&mut self, sheet_name: &str, size: u32) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "paperSize", &size.to_string())
    }

    /// Read paperSize when present.
    pub fn paper_size(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "paperSize")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set page setup `orientation` (`"portrait"` / `"landscape"`).
    /// Whether pageSetup `paperSize` is set.
    pub fn has_paper_size(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "paperSize")?.is_some())
    }

    /// Clear pageSetup `paperSize`.
    pub fn clear_paper_size(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "paperSize")
    }


    pub fn set_page_orientation(&mut self, sheet_name: &str, orientation: &str) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "orientation", orientation)
    }

    /// Read page orientation when present.
    pub fn page_orientation(&self, sheet_name: &str) -> Result<Option<String>> {
        self.sheet_child_attr(sheet_name, "pageSetup", "orientation")
    }

    /// Set page setup `usePrinterDefaults`.
    /// Whether pageSetup `orientation` is set.
    pub fn has_page_orientation(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "orientation")?.is_some())
    }

    /// Clear pageSetup `orientation`.
    pub fn clear_page_orientation(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "orientation")
    }


    pub fn set_use_printer_defaults(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "pageSetup",
            "usePrinterDefaults",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether usePrinterDefaults is enabled (default true when unset).
    pub fn use_printer_defaults(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "usePrinterDefaults")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set page setup `useFirstPageNumber`.
    /// Disable `use printer defaults` on a sheet. Returns whether it was enabled.
    pub fn clear_use_printer_defaults(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.use_printer_defaults(sheet_name)?;
        if had {
            self.set_use_printer_defaults(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_use_first_page_number(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        self.set_sheet_child_attr(
            sheet_name,
            "pageSetup",
            "useFirstPageNumber",
            if enabled { "1" } else { "0" },
        )
    }

    /// Whether useFirstPageNumber is enabled.
    pub fn use_first_page_number(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "useFirstPageNumber")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set page setup `cellComments` (`"none"`, `"asDisplayed"`, `"atEnd"`).
    /// Disable `use first page number` on a sheet. Returns whether it was enabled.
    pub fn clear_use_first_page_number(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.use_first_page_number(sheet_name)?;
        if had {
            self.set_use_first_page_number(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_cell_comments(&mut self, sheet_name: &str, mode: &str) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "cellComments", mode)
    }

    /// Read cellComments when present.
    pub fn cell_comments(&self, sheet_name: &str) -> Result<Option<String>> {
        self.sheet_child_attr(sheet_name, "pageSetup", "cellComments")
    }

    /// Set page setup `errors` print mode (`"displayed"`, `"blank"`, `"dash"`, `"NA"`).
    /// Whether pageSetup `cellComments` is set.
    pub fn has_cell_comments(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "cellComments")?.is_some())
    }

    /// Clear pageSetup `cellComments`.
    pub fn clear_cell_comments(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "cellComments")
    }


    pub fn set_print_errors(&mut self, sheet_name: &str, mode: &str) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "errors", mode)
    }

    /// Read print errors mode when present.
    pub fn print_errors(&self, sheet_name: &str) -> Result<Option<String>> {
        self.sheet_child_attr(sheet_name, "pageSetup", "errors")
    }

    /// Set page setup horizontal DPI.
    /// Whether pageSetup `errors` is set.
    pub fn has_print_errors(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageSetup", "errors")?.is_some())
    }

    /// Clear pageSetup `errors`.
    pub fn clear_print_errors(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "errors")
    }


    pub fn set_horizontal_dpi(&mut self, sheet_name: &str, dpi: u32) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "horizontalDpi", &dpi.to_string())
    }

    /// Read horizontalDpi when present.
    pub fn horizontal_dpi(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "horizontalDpi")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set page setup vertical DPI.
    pub fn set_vertical_dpi(&mut self, sheet_name: &str, dpi: u32) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageSetup", "verticalDpi", &dpi.to_string())
    }

    /// Read verticalDpi when present.
    pub fn vertical_dpi(&self, sheet_name: &str) -> Result<Option<u32>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "verticalDpi")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether pageSetup `horizontalDpi` is set.
    pub fn has_horizontal_dpi(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "horizontalDpi")?
            .is_some())
    }

    /// Clear pageSetup `horizontalDpi`.
    pub fn clear_horizontal_dpi(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "horizontalDpi")
    }

    /// Whether pageSetup `verticalDpi` is set.
    pub fn has_vertical_dpi(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageSetup", "verticalDpi")?
            .is_some())
    }

    /// Clear pageSetup `verticalDpi`.
    pub fn clear_vertical_dpi(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageSetup", "verticalDpi")
    }

    /// Clear page setup part child. Returns whether present.
    pub fn clear_page_setup(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "pageSetup");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    fn ensure_header_footer_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("headerFooter").is_none() {
            let el = OpenXmlElement::new("x", x, "headerFooter");
            let insert_at = root
                .children
                .iter()
                .position(|c| {
                    matches!(
                        c.local_name.as_str(),
                        "drawing" | "legacyDrawing" | "tableParts" | "extLst"
                    )
                })
                .unwrap_or(root.children.len());
            root.children.insert(insert_at, el);
        }
        root.child_mut("headerFooter").expect("headerFooter")
    }

    /// Set odd header text (`headerFooter/oddHeader`).
    /// Clear page setup on every sheet. Returns sheets modified.
    pub fn clear_all_page_setup(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_page_setup(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn set_odd_header(&mut self, sheet_name: &str, text: &str) -> Result<()> {
        self.set_header_footer_child(sheet_name, "oddHeader", text)
    }

    /// Read odd header text.
    pub fn odd_header(&self, sheet_name: &str) -> Result<Option<String>> {
        self.header_footer_child_text(sheet_name, "oddHeader")
    }

    /// Set odd footer text.
    /// Whether odd header is set.
    pub fn has_odd_header(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.odd_header(sheet_name)?.is_some())
    }

    /// Clear odd header.
    pub fn clear_odd_header(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_header_footer_child(sheet_name, "oddHeader")
    }


    pub fn set_odd_footer(&mut self, sheet_name: &str, text: &str) -> Result<()> {
        self.set_header_footer_child(sheet_name, "oddFooter", text)
    }

    /// Read odd footer text.
    pub fn odd_footer(&self, sheet_name: &str) -> Result<Option<String>> {
        self.header_footer_child_text(sheet_name, "oddFooter")
    }

    /// Set even header text.
    /// Whether odd footer is set.
    pub fn has_odd_footer(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.odd_footer(sheet_name)?.is_some())
    }

    /// Clear odd footer.
    pub fn clear_odd_footer(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_header_footer_child(sheet_name, "oddFooter")
    }


    pub fn set_even_header(&mut self, sheet_name: &str, text: &str) -> Result<()> {
        self.set_header_footer_child(sheet_name, "evenHeader", text)
    }

    /// Read even header text.
    pub fn even_header(&self, sheet_name: &str) -> Result<Option<String>> {
        self.header_footer_child_text(sheet_name, "evenHeader")
    }

    /// Set even footer text.
    /// Whether even header is set.
    pub fn has_even_header(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.even_header(sheet_name)?.is_some())
    }

    /// Clear even header.
    pub fn clear_even_header(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_header_footer_child(sheet_name, "evenHeader")
    }


    pub fn set_even_footer(&mut self, sheet_name: &str, text: &str) -> Result<()> {
        self.set_header_footer_child(sheet_name, "evenFooter", text)
    }

    /// Read even footer text.
    pub fn even_footer(&self, sheet_name: &str) -> Result<Option<String>> {
        self.header_footer_child_text(sheet_name, "evenFooter")
    }

    /// Set first-page header text.
    /// Whether even footer is set.
    pub fn has_even_footer(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.even_footer(sheet_name)?.is_some())
    }

    /// Clear even footer.
    pub fn clear_even_footer(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_header_footer_child(sheet_name, "evenFooter")
    }


    pub fn set_first_header(&mut self, sheet_name: &str, text: &str) -> Result<()> {
        self.set_header_footer_child(sheet_name, "firstHeader", text)
    }

    /// Read first header text.
    pub fn first_header(&self, sheet_name: &str) -> Result<Option<String>> {
        self.header_footer_child_text(sheet_name, "firstHeader")
    }

    /// Set first-page footer text.
    /// Whether first header is set.
    pub fn has_first_header(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.first_header(sheet_name)?.is_some())
    }

    /// Clear first header.
    pub fn clear_first_header(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_header_footer_child(sheet_name, "firstHeader")
    }


    pub fn set_first_footer(&mut self, sheet_name: &str, text: &str) -> Result<()> {
        self.set_header_footer_child(sheet_name, "firstFooter", text)
    }

    /// Read first footer text.
    pub fn first_footer(&self, sheet_name: &str) -> Result<Option<String>> {
        self.header_footer_child_text(sheet_name, "firstFooter")
    }

    fn set_header_footer_child(
        &mut self,
        sheet_name: &str,
        child: &str,
        text: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let hf = self.ensure_header_footer_mut(&mut root);
        hf.children.retain(|c| c.local_name != child);
        hf.append_child(
            OpenXmlElement::new("x", x, child).with_text(text),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    fn clear_header_footer_child(&mut self, sheet_name: &str, local: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(hf) = root.child_mut("headerFooter") else {
            return Ok(false);
        };
        let before = hf.children.len();
        hf.children.retain(|c| c.local_name != local);
        if hf.children.len() == before {
            return Ok(false);
        }
        if hf.children.is_empty() {
            root.children.retain(|c| c.local_name != "headerFooter");
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    fn header_footer_child_text(
        &self,
        sheet_name: &str,
        child: &str,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("headerFooter")
            .and_then(|hf| hf.child(child))
            .map(|c| c.inner_text()))
    }

    /// Set headerFooter flags: differentOddEven / differentFirst / scaleWithDoc / alignWithMargins.
    /// Whether first footer is set.
    pub fn has_first_footer(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.first_footer(sheet_name)?.is_some())
    }

    /// Clear first footer.
    pub fn clear_first_footer(&mut self, sheet_name: &str) -> Result<bool> {
        self.clear_header_footer_child(sheet_name, "firstFooter")
    }


    pub fn set_header_footer_flags(
        &mut self,
        sheet_name: &str,
        different_odd_even: Option<bool>,
        different_first: Option<bool>,
        scale_with_doc: Option<bool>,
        align_with_margins: Option<bool>,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let hf = self.ensure_header_footer_mut(&mut root);
        if let Some(v) = different_odd_even {
            hf.set_attribute("differentOddEven", if v { "1" } else { "0" });
        }
        if let Some(v) = different_first {
            hf.set_attribute("differentFirst", if v { "1" } else { "0" });
        }
        if let Some(v) = scale_with_doc {
            hf.set_attribute("scaleWithDoc", if v { "1" } else { "0" });
        }
        if let Some(v) = align_with_margins {
            hf.set_attribute("alignWithMargins", if v { "1" } else { "0" });
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Clear all headerFooter flag attributes.
    pub fn clear_header_footer_flags(&mut self, sheet_name: &str) -> Result<bool> {
        let a = self.clear_header_footer_different_odd_even(sheet_name)?;
        let b = self.clear_header_footer_different_first(sheet_name)?;
        let c = self.clear_header_footer_scale_with_doc(sheet_name)?;
        let d = self.clear_header_footer_align_with_margins(sheet_name)?;
        Ok(a || b || c || d)
    }

    /// Whether differentOddEven is enabled.
    pub fn header_footer_different_odd_even(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "headerFooter", "differentOddEven")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Whether differentFirst is enabled.
    pub fn header_footer_different_first(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "headerFooter", "differentFirst")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Whether scaleWithDoc is enabled (default true).
    pub fn header_footer_scale_with_doc(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "headerFooter", "scaleWithDoc")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Whether alignWithMargins is enabled (default true).
    pub fn header_footer_align_with_margins(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_child_attr(sheet_name, "headerFooter", "alignWithMargins")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Clear headerFooter differentOddEven attr.
    pub fn clear_header_footer_different_odd_even(
        &mut self,
        sheet_name: &str,
    ) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "headerFooter", "differentOddEven")
    }

    /// Clear headerFooter differentFirst attr.
    pub fn clear_header_footer_different_first(
        &mut self,
        sheet_name: &str,
    ) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "headerFooter", "differentFirst")
    }

    /// Clear headerFooter scaleWithDoc attr.
    pub fn clear_header_footer_scale_with_doc(
        &mut self,
        sheet_name: &str,
    ) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "headerFooter", "scaleWithDoc")
    }

    /// Clear headerFooter alignWithMargins attr.
    pub fn clear_header_footer_align_with_margins(
        &mut self,
        sheet_name: &str,
    ) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "headerFooter", "alignWithMargins")
    }

    /// Whether headerFooter element is present.
    pub fn has_header_footer(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root.child("headerFooter").is_some())
    }

    /// Clear headerFooter. Returns whether present.
    pub fn clear_header_footer(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "headerFooter");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    fn ensure_sheet_pr_mut<'a>(&self, root: &'a mut OpenXmlElement) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("sheetPr").is_none() {
            let el = OpenXmlElement::new("x", x, "sheetPr");
            root.children.insert(0, el);
        }
        root.child_mut("sheetPr").expect("sheetPr ensured")
    }

    /// Set worksheet VBA `codeName` on `sheetPr`.
    pub fn set_sheet_code_name(&mut self, sheet_name: &str, name: &str) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let pr = self.ensure_sheet_pr_mut(&mut root);
        pr.set_attribute("codeName", name);
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read worksheet `sheetPr/@codeName`.
    pub fn sheet_code_name(&self, sheet_name: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.get_attribute("codeName").map(|s| s.to_string())))
    }

    /// List sheets that have a VBA codeName as `(sheet_name, codeName)`.
    pub fn list_sheet_code_names(&self) -> Result<Vec<(String, String)>> {
        let mut out = Vec::new();
        for s in &self.sheets {
            if let Some(cn) = self.sheet_code_name(&s.name)? {
                if !cn.is_empty() {
                    out.push((s.name.clone(), cn));
                }
            }
        }
        Ok(out)
    }

    /// Whether a sheet has a VBA codeName set.
    pub fn has_sheet_code_name(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .sheet_code_name(sheet_name)?
            .map(|s| !s.is_empty())
            .unwrap_or(false))
    }

    /// Sheet names that have a VBA codeName.
    pub fn sheets_with_code_names(&self) -> Result<Vec<String>> {
        Ok(self
            .list_sheet_code_names()?
            .into_iter()
            .map(|(n, _)| n)
            .collect())
    }

    /// Clear worksheet `sheetPr/@codeName`. Returns whether it was present.
    /// Whether any sheet is returned by [`sheets_with_code_names`](Self::sheets_with_code_names).
    pub fn has_sheets_with_code_names(&self) -> Result<bool> {
        Ok(!self.sheets_with_code_names()?.is_empty())
    }

    pub fn clear_sheet_code_name(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(pr) = root.child_mut("sheetPr") else {
            return Ok(false);
        };
        let had = pr.get_attribute("codeName").is_some();
        if had {
            pr.remove_attribute("codeName");
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(had)
    }

    /// Set `sheetPr/@published`.
    /// Clear VBA code names on every sheet. Returns sheets modified.
    pub fn clear_all_sheet_code_names(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_sheet_code_name(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn set_sheet_published(&mut self, sheet_name: &str, published: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let pr = self.ensure_sheet_pr_mut(&mut root);
        pr.set_attribute("published", if published { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether the sheet is published (default true when unset).
    pub fn sheet_published(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(true),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.get_attribute("published"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set `sheetPr/@enableFormatConditionsCalculation`.
    /// Disable `sheet published` on a sheet. Returns whether it was enabled.
    pub fn clear_sheet_published(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.sheet_published(sheet_name)?;
        if had {
            self.set_sheet_published(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_enable_format_conditions_calculation(
        &mut self,
        sheet_name: &str,
        enabled: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let pr = self.ensure_sheet_pr_mut(&mut root);
        pr.set_attribute(
            "enableFormatConditionsCalculation",
            if enabled { "1" } else { "0" },
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether format conditions calculation is enabled (default true).
    pub fn enable_format_conditions_calculation(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(true),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.get_attribute("enableFormatConditionsCalculation"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Whether enableFormatConditionsCalculation attr is present on sheetPr.
    pub fn has_enable_format_conditions_calculation(
        &self,
        sheet_name: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.get_attribute("enableFormatConditionsCalculation"))
            .is_some())
    }

    /// Clear enableFormatConditionsCalculation from sheetPr.
    pub fn clear_enable_format_conditions_calculation(
        &mut self,
        sheet_name: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(pr) = root.child_mut("sheetPr") else {
            return Ok(false);
        };
        if pr.get_attribute("enableFormatConditionsCalculation").is_none() {
            return Ok(false);
        }
        pr.remove_attribute("enableFormatConditionsCalculation");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    fn ensure_sheet_pr_child_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
        child: &str,
    ) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let pr = self.ensure_sheet_pr_mut(root);
        if pr.child(child).is_none() {
            pr.append_child(OpenXmlElement::new("x", x, child));
        }
        pr.child_mut(child).expect("sheetPr child ensured")
    }

    /// Set `sheetPr/pageSetUpPr/@fitToPage`.
    pub fn set_fit_to_page(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let ps = self.ensure_sheet_pr_child_mut(&mut root, "pageSetUpPr");
        ps.set_attribute("fitToPage", if enabled { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether fitToPage is enabled.
    pub fn fit_to_page(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.child("pageSetUpPr"))
            .and_then(|ps| ps.get_attribute("fitToPage"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `sheetPr/pageSetUpPr/@autoPageBreaks`.
    /// Disable `fit to page` on a sheet. Returns whether it was enabled.
    pub fn clear_fit_to_page(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.fit_to_page(sheet_name)?;
        if had {
            self.set_fit_to_page(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_auto_page_breaks(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let ps = self.ensure_sheet_pr_child_mut(&mut root, "pageSetUpPr");
        ps.set_attribute("autoPageBreaks", if enabled { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether autoPageBreaks is enabled (default true).
    pub fn auto_page_breaks(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(true),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.child("pageSetUpPr"))
            .and_then(|ps| ps.get_attribute("autoPageBreaks"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set outline properties under `sheetPr/outlinePr`.
    /// Disable `auto page breaks` on a sheet. Returns whether it was enabled.
    pub fn clear_auto_page_breaks(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.auto_page_breaks(sheet_name)?;
        if had {
            self.set_auto_page_breaks(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_outline_properties(
        &mut self,
        sheet_name: &str,
        summary_below: bool,
        summary_right: bool,
        apply_styles: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let op = self.ensure_sheet_pr_child_mut(&mut root, "outlinePr");
        op.set_attribute("summaryBelow", if summary_below { "1" } else { "0" });
        op.set_attribute("summaryRight", if summary_right { "1" } else { "0" });
        op.set_attribute("applyStyles", if apply_styles { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read outline properties as `(summary_below, summary_right, apply_styles)`.
    ///
    /// Defaults are `(true, true, false)` when unset.
    pub fn outline_properties(&self, sheet_name: &str) -> Result<(bool, bool, bool)> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok((true, true, false)),
        };
        let root = parse_element(data)?;
        let Some(op) = root.child("sheetPr").and_then(|p| p.child("outlinePr")) else {
            return Ok((true, true, false));
        };
        let on = |name: &str, default: bool| {
            op.get_attribute(name)
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(default)
        };
        Ok((on("summaryBelow", true), on("summaryRight", true), on("applyStyles", false)))
    }

    /// Whether outlinePr is present.
    pub fn has_outline_properties(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .map(|p| p.child("outlinePr").is_some())
            .unwrap_or(false))
    }

    /// Clear outlinePr. Returns whether present.
    pub fn clear_outline_properties(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(pr) = root.child_mut("sheetPr") else {
            return Ok(false);
        };
        let before = pr.children.len();
        pr.children.retain(|c| c.local_name != "outlinePr");
        let removed = pr.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Set `sheetPr/@filterMode`.
    pub fn set_sheet_filter_mode(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let pr = self.ensure_sheet_pr_mut(&mut root);
        pr.set_attribute("filterMode", if enabled { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether sheet filterMode is enabled.
    pub fn sheet_filter_mode(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.get_attribute("filterMode"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `sheetPr/@transitionEvaluation` (Lotus 1-2-3 formula evaluation).
    /// Disable `sheet filter mode` on a sheet. Returns whether it was enabled.
    pub fn clear_sheet_filter_mode(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.sheet_filter_mode(sheet_name)?;
        if had {
            self.set_sheet_filter_mode(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_transition_evaluation(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let pr = self.ensure_sheet_pr_mut(&mut root);
        pr.set_attribute("transitionEvaluation", if enabled { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether transitionEvaluation is enabled.
    pub fn transition_evaluation(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.get_attribute("transitionEvaluation"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `sheetPr/@transitionEntry`.
    /// Disable `transition evaluation` on a sheet. Returns whether it was enabled.
    pub fn clear_transition_evaluation(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.transition_evaluation(sheet_name)?;
        if had {
            self.set_transition_evaluation(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn set_transition_entry(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let pr = self.ensure_sheet_pr_mut(&mut root);
        pr.set_attribute("transitionEntry", if enabled { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether transitionEntry is enabled.
    pub fn transition_entry(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetPr")
            .and_then(|p| p.get_attribute("transitionEntry"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Remove the worksheet tab color. Returns whether one was present.
    /// Disable `transition entry` on a sheet. Returns whether it was enabled.
    pub fn clear_transition_entry(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.transition_entry(sheet_name)?;
        if had {
            self.set_transition_entry(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn clear_sheet_tab_color(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = false;
        if let Some(pr) = root.child_mut("sheetPr") {
            let before = pr.children.len();
            pr.children.retain(|c| c.local_name != "tabColor");
            removed = pr.children.len() < before;
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Set worksheet page margins (inches) and optional page setup.
    pub fn set_page_setup(
        &mut self,
        sheet_name: &str,
        margins: (f64, f64, f64, f64, f64, f64), // l,r,t,b,header,footer
        paper_size: u32,
        orientation: &str,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children
            .retain(|c| c.local_name != "pageMargins" && c.local_name != "pageSetup");
        let (l, r, t, b, h, f) = margins;
        root.append_child(page_margins(l, r, t, b, h, f));
        root.append_child(page_setup(paper_size, orientation, false));
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read page margins as `(left, right, top, bottom, header, footer)` inches.
    pub fn get_page_margins(
        &self,
        sheet_name: &str,
    ) -> Result<Option<(f64, f64, f64, f64, f64, f64)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(pm) = root.child("pageMargins") else {
            return Ok(None);
        };
        let parse = |name: &str| {
            pm.get_attribute(name)
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0)
        };
        Ok(Some((
            parse("left"),
            parse("right"),
            parse("top"),
            parse("bottom"),
            parse("header"),
            parse("footer"),
        )))
    }

    /// Read page setup as `(paper_size, orientation)` when present.
    pub fn get_page_setup(&self, sheet_name: &str) -> Result<Option<(u32, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(ps) = root.child("pageSetup") else {
            return Ok(None);
        };
        let paper = ps
            .get_attribute("paperSize")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let orientation = ps
            .get_attribute("orientation")
            .unwrap_or("portrait")
            .to_string();
        Ok(Some((paper, orientation)))
    }

    /// Whether page margins are set on the sheet.
    pub fn has_page_margins(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.get_page_margins(sheet_name)?.is_some())
    }

    /// Set page margins in inches `(left, right, top, bottom, header, footer)`.
    pub fn set_page_margins(
        &mut self,
        sheet_name: &str,
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
        header: f64,
        footer: f64,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        root.children.retain(|c| c.local_name != "pageMargins");
        let insert_at = root
            .children
            .iter()
            .position(|c| {
                matches!(
                    c.local_name.as_str(),
                    "pageSetup"
                        | "headerFooter"
                        | "drawing"
                        | "legacyDrawing"
                        | "tableParts"
                        | "extLst"
                )
            })
            .unwrap_or(root.children.len());
        root.children.insert(
            insert_at,
            page_margins(left, right, top, bottom, header, footer),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Clear pageMargins. Returns whether present.
    pub fn clear_page_margins(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "pageMargins");
        let removed = root.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Update a single page margin attribute (e.g. `"left"`, `"header"`).
    pub fn set_page_margin_attr(
        &mut self,
        sheet_name: &str,
        attr: &str,
        inches: f64,
    ) -> Result<()> {
        self.set_sheet_child_attr(sheet_name, "pageMargins", attr, &inches.to_string())
    }

    /// Read a single page margin attribute in inches.
    pub fn page_margin_attr(&self, sheet_name: &str, attr: &str) -> Result<Option<f64>> {
        Ok(self
            .sheet_child_attr(sheet_name, "pageMargins", attr)?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether a pageMargins attribute is set.
    pub fn has_page_margin_attr(&self, sheet_name: &str, attr: &str) -> Result<bool> {
        Ok(self.sheet_child_attr(sheet_name, "pageMargins", attr)?.is_some())
    }

    /// Clear a single pageMargins attribute.
    pub fn clear_page_margin_attr(
        &mut self,
        sheet_name: &str,
        attr: &str,
    ) -> Result<bool> {
        self.clear_sheet_child_attr(sheet_name, "pageMargins", attr)
    }

    /// Set protected range security descriptor / algorithm shell attributes.
    ///
    /// Does not compute password hashes — pass precomputed values when needed.
    pub fn set_protected_range_attrs(
        &mut self,
        sheet_name: &str,
        name: &str,
        sqref: Option<&str>,
        algorithm_name: Option<&str>,
        security_descriptor: Option<&str>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("protectedRanges") else {
            return Ok(false);
        };
        let mut found = false;
        for pr in container
            .children
            .iter_mut()
            .filter(|c| c.local_name == "protectedRange")
        {
            if pr.get_attribute("name").unwrap_or("") != name {
                continue;
            }
            found = true;
            if let Some(s) = sqref {
                pr.set_attribute("sqref", s);
            }
            if let Some(a) = algorithm_name {
                pr.set_attribute("algorithmName", a);
            }
            if let Some(sd) = security_descriptor {
                pr.set_attribute("securityDescriptor", sd);
            }
            break;
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Whether page setup is set on the sheet.
    pub fn has_page_setup(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.get_page_setup(sheet_name)?.is_some())
    }

    /// Write a calculation chain part listing formula cells.
    ///
    /// `cells` are `(cell_ref, sheet_id)` pairs. Returns the part URI.
    /// Sheet names that have a pageSetup element.
    pub fn sheets_with_page_setup(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_page_setup(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet is returned by [`sheets_with_page_setup`](Self::sheets_with_page_setup).
    pub fn has_sheets_with_page_setup(&self) -> Result<bool> {
        Ok(!self.sheets_with_page_setup()?.is_empty())
    }

    pub fn set_calc_chain(&mut self, cells: &[(&str, u32)]) -> Result<PackUri> {
        let wb_uri = self.ensure_workbook()?;
        let chain_uri = PackUri::new("/xl/calcChain.xml");
        let kids: Vec<_> = cells
            .iter()
            .map(|(r, i)| calc_chain_cell(r, *i))
            .collect();
        let xml = write_element(&calc_chain(kids))?;
        self.package.set_part(
            chain_uri.clone(),
            content_type::SPREADSHEET_CALC_CHAIN,
            xml,
        );
        if self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| rels.get_by_type(rel::CALC_CHAIN))
            .is_none()
        {
            self.package.add_part_relationship(
                &wb_uri,
                rel::CALC_CHAIN,
                &chain_uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok(chain_uri)
    }

    /// Count entries currently stored in the calculation chain part, if present.
    pub fn calc_chain_entry_count(&self) -> Result<usize> {
        Ok(self.list_calc_chain()?.len())
    }

    /// Rebuild `/xl/calcChain.xml` from all formula cells across worksheets.
    ///
    /// Sheet ids are 1-based workbook order. Returns the number of chain entries written.
    pub fn rebuild_calc_chain(&mut self) -> Result<usize> {
        let mut cells: Vec<(String, u32)> = Vec::new();
        for (idx, name) in self.sheet_names().into_iter().enumerate() {
            let sheet_id = (idx as u32) + 1;
            for (r, _f) in self.list_formulas(name)? {
                cells.push((r, sheet_id));
            }
        }
        if cells.is_empty() {
            self.clear_calc_chain()?;
            return Ok(0);
        }
        let refs: Vec<(&str, u32)> = cells.iter().map(|(r, i)| (r.as_str(), *i)).collect();
        self.set_calc_chain(&refs)?;
        Ok(cells.len())
    }

    /// List calculation chain entries as `(cell_ref, sheet_id)`.
    pub fn list_calc_chain(&self) -> Result<Vec<(String, u32)>> {
        let chain_uri = PackUri::new("/xl/calcChain.xml");
        let Some(data) = self.package.opc().get_part(&chain_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("c")
            .filter_map(|c| {
                let r = c.get_attribute("r")?.to_string();
                let i = c
                    .get_attribute("i")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                Some((r, i))
            })
            .collect())
    }

    /// Whether the sheet has any data validation rules.
    pub fn has_data_validations(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root.child("dataValidations").is_some())
    }

    /// Sheet names that have data validation rules.
    pub fn sheets_with_data_validations(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_data_validations(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has data validations.
    pub fn has_sheets_with_data_validations(&self) -> Result<bool> {
        Ok(!self.sheets_with_data_validations()?.is_empty())
    }

    pub fn set_data_validations_disable_prompts(
        &mut self,
        sheet_name: &str,
        disable: bool,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        container.set_attribute("disablePrompts", if disable { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Clear dataValidations `@disablePrompts`.
    pub fn clear_data_validations_disable_prompts(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        if container.get_attribute("disablePrompts").is_none() {
            return Ok(false);
        }
        container.remove_attribute("disablePrompts");
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Whether data validation prompts are disabled.
    pub fn data_validations_disable_prompts(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("dataValidations")
            .and_then(|c| c.get_attribute("disablePrompts"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `dataValidations` window position attributes.
    pub fn set_data_validations_window(
        &mut self,
        sheet_name: &str,
        x: u32,
        y: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        container.set_attribute("xWindow", x.to_string());
        container.set_attribute("yWindow", y.to_string());
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Clear dataValidations xWindow/yWindow attributes.
    pub fn clear_data_validations_window(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let before = container.attributes.len();
        container.attributes.retain(|a| {
            a.local_name != "xWindow" && a.local_name != "yWindow"
        });
        if container.attributes.len() == before {
            return Ok(false);
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Read `dataValidations` window position as `(x, y)`.
    pub fn data_validations_window(&self, sheet_name: &str) -> Result<Option<(u32, u32)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(None);
        };
        let x = container
            .get_attribute("xWindow")
            .and_then(|s| s.parse().ok());
        let y = container
            .get_attribute("yWindow")
            .and_then(|s| s.parse().ok());
        match (x, y) {
            (Some(x), Some(y)) => Ok(Some((x, y))),
            _ => Ok(None),
        }
    }

    /// Set `sheetCalcPr/@fullCalcOnLoad` on a worksheet.
    pub fn set_sheet_full_calc_on_load(&mut self, sheet_name: &str, enabled: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("sheetCalcPr").is_none() {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, OpenXmlElement::new("x", x, "sheetCalcPr"));
        }
        if let Some(scp) = root.child_mut("sheetCalcPr") {
            scp.set_attribute("fullCalcOnLoad", if enabled { "1" } else { "0" });
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether sheetCalcPr fullCalcOnLoad is set.
    pub fn sheet_full_calc_on_load(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetCalcPr")
            .and_then(|c| c.get_attribute("fullCalcOnLoad"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Count data validation rules on a sheet.
    /// Disable `sheet full calc on load` on a sheet. Returns whether it was enabled.
    pub fn clear_sheet_full_calc_on_load(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.sheet_full_calc_on_load(sheet_name)?;
        if had {
            self.set_sheet_full_calc_on_load(sheet_name, false)?;
        }
        Ok(had)
    }

    pub fn data_validation_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_data_validations(sheet_name)?.len())
    }

    /// List data validations as `(type, sqref, formula1)` triples.
    pub fn list_data_validations(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(Vec::new());
        };
        Ok(container
            .children_by_name("dataValidation")
            .map(|dv| {
                let ty = dv.get_attribute("type").unwrap_or("none").to_string();
                let sqref = dv.get_attribute("sqref").unwrap_or("").to_string();
                let formula = dv
                    .child("formula1")
                    .map(|f| f.inner_text())
                    .unwrap_or_default();
                (ty, sqref, formula)
            })
            .collect())
    }

    /// Set prompt/error message attributes on the first data validation matching `sqref`.
    pub fn set_data_validation_messages(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        prompt_title: Option<&str>,
        prompt: Option<&str>,
        error_title: Option<&str>,
        error: Option<&str>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                if let Some(t) = prompt_title {
                    dv.set_attribute("promptTitle", t);
                    dv.set_attribute("showInputMessage", "1");
                }
                if let Some(p) = prompt {
                    dv.set_attribute("prompt", p);
                    dv.set_attribute("showInputMessage", "1");
                }
                if let Some(t) = error_title {
                    dv.set_attribute("errorTitle", t);
                    dv.set_attribute("showErrorMessage", "1");
                }
                if let Some(e) = error {
                    dv.set_attribute("error", e);
                    dv.set_attribute("showErrorMessage", "1");
                }
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear prompt/error message attributes on a dataValidation.
    pub fn clear_data_validation_messages(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                let before = dv.attributes.len();
                dv.attributes.retain(|a| {
                    !matches!(
                        a.local_name.as_str(),
                        "promptTitle" | "prompt" | "errorTitle" | "error"
                    )
                });
                found = dv.attributes.len() < before;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Read prompt/error messages for a data validation sqref as
    /// `(prompt_title, prompt, error_title, error)`.
    pub fn data_validation_messages(
        &self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<Option<(Option<String>, Option<String>, Option<String>, Option<String>)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(None);
        };
        for dv in container.children_by_name("dataValidation") {
            if dv.get_attribute("sqref") == Some(sqref) {
                return Ok(Some((
                    dv.get_attribute("promptTitle").map(|s| s.to_string()),
                    dv.get_attribute("prompt").map(|s| s.to_string()),
                    dv.get_attribute("errorTitle").map(|s| s.to_string()),
                    dv.get_attribute("error").map(|s| s.to_string()),
                )));
            }
        }
        Ok(None)
    }

    /// Set `errorStyle` on a data validation (`"stop"`, `"warning"`, `"information"`).
    pub fn set_data_validation_error_style(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        style: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                dv.set_attribute("errorStyle", style);
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear dataValidation `@errorStyle` for `sqref`.
    pub fn clear_data_validation_error_style(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        self.clear_data_validation_attr(sheet_name, sqref, "errorStyle")
    }

    /// Read errorStyle on a data validation matching `sqref`.
    pub fn data_validation_error_style(
        &self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(None);
        };
        for dv in container.children_by_name("dataValidation") {
            if dv.get_attribute("sqref") == Some(sqref) {
                return Ok(dv.get_attribute("errorStyle").map(|s| s.to_string()));
            }
        }
        Ok(None)
    }

    /// Set showInputMessage / showErrorMessage flags on a data validation.
    pub fn set_data_validation_show_messages(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        show_input: Option<bool>,
        show_error: Option<bool>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                if let Some(v) = show_input {
                    dv.set_attribute("showInputMessage", if v { "1" } else { "0" });
                }
                if let Some(v) = show_error {
                    dv.set_attribute("showErrorMessage", if v { "1" } else { "0" });
                }
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear showInputMessage/showErrorMessage on a dataValidation.
    pub fn clear_data_validation_show_messages(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                let before = dv.attributes.len();
                dv.attributes.retain(|a| {
                    a.local_name != "showInputMessage" && a.local_name != "showErrorMessage"
                });
                found = dv.attributes.len() < before;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Set individual prompt/error title and body fields on a data validation.
    pub fn set_data_validation_message_fields(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        prompt_title: Option<&str>,
        prompt: Option<&str>,
        error_title: Option<&str>,
        error: Option<&str>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                if let Some(t) = prompt_title {
                    dv.set_attribute("promptTitle", t);
                    dv.set_attribute("showInputMessage", "1");
                }
                if let Some(p) = prompt {
                    dv.set_attribute("prompt", p);
                    dv.set_attribute("showInputMessage", "1");
                }
                if let Some(t) = error_title {
                    dv.set_attribute("errorTitle", t);
                    dv.set_attribute("showErrorMessage", "1");
                }
                if let Some(e) = error {
                    dv.set_attribute("error", e);
                    dv.set_attribute("showErrorMessage", "1");
                }
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Alias for [`clear_data_validation_messages`](Self::clear_data_validation_messages).
    pub fn clear_data_validation_message_fields(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        self.clear_data_validation_messages(sheet_name, sqref)
    }

    /// Change the `sqref` of a data validation rule.
    pub fn set_data_validation_sqref(
        &mut self,
        sheet_name: &str,
        old_sqref: &str,
        new_sqref: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(old_sqref) {
                dv.set_attribute("sqref", new_sqref);
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Read showInputMessage / showErrorMessage as `(show_input, show_error)`.
    pub fn data_validation_show_messages(
        &self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<Option<(bool, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(None);
        };
        let on = |dv: &OpenXmlElement, name: &str, default: bool| {
            dv.get_attribute(name)
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(default)
        };
        for dv in container.children_by_name("dataValidation") {
            if dv.get_attribute("sqref") == Some(sqref) {
                return Ok(Some((
                    on(dv, "showInputMessage", false),
                    on(dv, "showErrorMessage", false),
                )));
            }
        }
        Ok(None)
    }

    /// Remove the first data validation matching `sqref`. Returns whether found.
    pub fn remove_data_validation(&mut self, sheet_name: &str, sqref: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let before = container.children.len();
        container.children.retain(|c| {
            !(c.local_name == "dataValidation" && c.get_attribute("sqref") == Some(sqref))
        });
        let removed = container.children.len() < before;
        if removed {
            if container.children.is_empty() {
                root.children.retain(|c| c.local_name != "dataValidations");
            } else {
                container.set_attribute("count", container.children.len().to_string());
            }
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Set operator on a data validation matching `sqref` (e.g. `"between"`, `"greaterThan"`).
    /// Whether a sheet has a data validation rule covering `sqref`.
    pub fn has_data_validation(&self, sheet_name: &str, sqref: &str) -> Result<bool> {
        Ok(self
            .list_data_validations(sheet_name)?
            .iter()
            .any(|(_ty, r, _f)| r == sqref || r.contains(sqref)))
    }

    pub fn set_data_validation_operator(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        operator: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                dv.set_attribute("operator", operator);
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear dataValidation `@operator` for `sqref`.
    pub fn clear_data_validation_operator(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        self.clear_data_validation_attr(sheet_name, sqref, "operator")
    }

    /// Read operator on a data validation matching `sqref`.
    pub fn data_validation_operator(
        &self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(None);
        };
        for dv in container.children_by_name("dataValidation") {
            if dv.get_attribute("sqref") == Some(sqref) {
                return Ok(dv.get_attribute("operator").map(|s| s.to_string()));
            }
        }
        Ok(None)
    }

    /// Set `allowBlank` on a data validation matching `sqref`.
    pub fn set_data_validation_allow_blank(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        allow: bool,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                dv.set_attribute("allowBlank", if allow { "1" } else { "0" });
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear dataValidation `@allowBlank` for `sqref`.
    pub fn clear_data_validation_allow_blank(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        self.clear_data_validation_attr(sheet_name, sqref, "allowBlank")
    }

    /// Whether allowBlank is set on a data validation matching `sqref`.
    pub fn data_validation_allow_blank(&self, sheet_name: &str, sqref: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(false);
        };
        for dv in container.children_by_name("dataValidation") {
            if dv.get_attribute("sqref") == Some(sqref) {
                return Ok(dv
                    .get_attribute("allowBlank")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false));
            }
        }
        Ok(false)
    }

    /// Set data validation type on matching `sqref` (e.g. `"list"`, `"whole"`, `"decimal"`).
    fn clear_data_validation_attr(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        attr: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                if dv.get_attribute(attr).is_some() {
                    dv.remove_attribute(attr);
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    pub fn set_data_validation_type(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        type_: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                dv.set_attribute("type", type_);
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear dataValidation `@type` for `sqref`.
    pub fn clear_data_validation_type(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        self.clear_data_validation_attr(sheet_name, sqref, "type")
    }

    /// Read data validation type for matching `sqref`.
    pub fn data_validation_type(
        &self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(None);
        };
        for dv in container.children_by_name("dataValidation") {
            if dv.get_attribute("sqref") == Some(sqref) {
                return Ok(dv.get_attribute("type").map(|s| s.to_string()));
            }
        }
        Ok(None)
    }

    /// Set formula1 (and optional formula2) on a data validation matching `sqref`.
    pub fn set_data_validation_formulas(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        formula1: &str,
        formula2: Option<&str>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                dv.children
                    .retain(|c| c.local_name != "formula1" && c.local_name != "formula2");
                dv.append_child(OpenXmlElement::new("x", x, "formula1").with_text(formula1));
                if let Some(f2) = formula2 {
                    dv.append_child(OpenXmlElement::new("x", x, "formula2").with_text(f2));
                }
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Set IME mode on a data validation (`imeMode`), e.g. `"noControl"`, `"off"`, `"on"`, `"disabled"`, `"hiragana"`, `"fullKatakana"`, `"halfKatakana"`, `"fullAlpha"`, `"halfAlpha"`, `"fullHangul"`, `"halfHangul"`.
    pub fn set_data_validation_ime_mode(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        ime_mode: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                dv.set_attribute("imeMode", ime_mode);
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear dataValidation `@imeMode` for `sqref`.
    pub fn clear_data_validation_ime_mode(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        self.clear_data_validation_attr(sheet_name, sqref, "imeMode")
    }

    /// Read IME mode on a data validation matching `sqref`.
    pub fn data_validation_ime_mode(
        &self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(container) = root.child("dataValidations") else {
            return Ok(None);
        };
        for dv in container.children_by_name("dataValidation") {
            if dv.get_attribute("sqref") == Some(sqref) {
                return Ok(dv.get_attribute("imeMode").map(|s| s.to_string()));
            }
        }
        Ok(None)
    }

    /// Remove a single autoFilter column by colId. Returns whether found.
    pub fn remove_auto_filter_column(
        &mut self,
        sheet_name: &str,
        col_id: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(af) = root.child_mut("autoFilter") else {
            return Ok(false);
        };
        let before = af.children.len();
        af.children.retain(|c| {
            !(c.local_name == "filterColumn"
                && c.get_attribute("colId") == Some(&col_id.to_string()))
        });
        let removed = af.children.len() < before;
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Set `showButton` / `hiddenButton` flags on a filter column.
    pub fn set_auto_filter_column_buttons(
        &mut self,
        sheet_name: &str,
        col_id: u32,
        show_button: Option<bool>,
        hidden_button: Option<bool>,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(af) = root.child_mut("autoFilter") else {
            return Ok(false);
        };
        let mut found = false;
        for fc in af.children.iter_mut() {
            if fc.local_name == "filterColumn"
                && fc.get_attribute("colId") == Some(&col_id.to_string())
            {
                if let Some(show) = show_button {
                    fc.set_attribute("showButton", if show { "1" } else { "0" });
                }
                if let Some(hidden) = hidden_button {
                    fc.set_attribute("hiddenButton", if hidden { "1" } else { "0" });
                }
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Read filter column button flags as `(show_button, hidden_button)`.
    pub fn auto_filter_column_buttons(
        &self,
        sheet_name: &str,
        col_id: u32,
    ) -> Result<Option<(bool, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(af) = root.child("autoFilter") else {
            return Ok(None);
        };
        for fc in af.children_by_name("filterColumn") {
            if fc.get_attribute("colId") == Some(&col_id.to_string()) {
                let show = fc
                    .get_attribute("showButton")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(true);
                let hidden = fc
                    .get_attribute("hiddenButton")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                return Ok(Some((show, hidden)));
            }
        }
        Ok(None)
    }

    /// Clear showButton/hiddenButton on a filterColumn.
    pub fn clear_auto_filter_column_buttons(
        &mut self,
        sheet_name: &str,
        col_id: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(af) = root.child_mut("autoFilter") else {
            return Ok(false);
        };
        let mut found = false;
        for fc in af.children.iter_mut() {
            if fc.local_name == "filterColumn"
                && fc.get_attribute("colId") == Some(&col_id.to_string())
            {
                let before = fc.attributes.len();
                fc.attributes.retain(|a| {
                    a.local_name != "showButton" && a.local_name != "hiddenButton"
                });
                if fc.attributes.len() < before {
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Add a color filter on an autoFilter column (`colorFilter` with `dxfId` and optional `cellColor`).
    pub fn add_auto_filter_color(
        &mut self,
        sheet_name: &str,
        col_id: u32,
        dxf_id: u32,
        cell_color: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let af = self.ensure_auto_filter_mut(&mut root);
        af.children.retain(|c| {
            !(c.local_name == "filterColumn"
                && c.get_attribute("colId") == Some(&col_id.to_string()))
        });
        let color = OpenXmlElement::new("x", x, "colorFilter")
            .with_attribute("dxfId", dxf_id.to_string())
            .with_attribute("cellColor", if cell_color { "1" } else { "0" });
        af.append_child(
            OpenXmlElement::new("x", x, "filterColumn")
                .with_attribute("colId", col_id.to_string())
                .with_child(color),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add an icon filter on an autoFilter column (`iconFilter` with `iconSet` and `iconId`).
    pub fn add_auto_filter_icon(
        &mut self,
        sheet_name: &str,
        col_id: u32,
        icon_set: &str,
        icon_id: u32,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let af = self.ensure_auto_filter_mut(&mut root);
        af.children.retain(|c| {
            !(c.local_name == "filterColumn"
                && c.get_attribute("colId") == Some(&col_id.to_string()))
        });
        let icon = OpenXmlElement::new("x", x, "iconFilter")
            .with_attribute("iconSet", icon_set)
            .with_attribute("iconId", icon_id.to_string());
        af.append_child(
            OpenXmlElement::new("x", x, "filterColumn")
                .with_attribute("colId", col_id.to_string())
                .with_child(icon),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a date-type data validation.
    pub fn add_data_validation_date(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        operator: &str,
        formula1: &str,
        formula2: Option<&str>,
        allow_blank: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut dv = OpenXmlElement::new("x", x, "dataValidation")
            .with_attribute("type", "date")
            .with_attribute("operator", operator)
            .with_attribute("allowBlank", if allow_blank { "1" } else { "0" })
            .with_attribute("showInputMessage", "1")
            .with_attribute("showErrorMessage", "1")
            .with_attribute("sqref", sqref)
            .with_child(OpenXmlElement::new("x", x, "formula1").with_text(formula1));
        if let Some(f2) = formula2 {
            dv.append_child(OpenXmlElement::new("x", x, "formula2").with_text(f2));
        }
        if let Some(container) = root.child_mut("dataValidations") {
            container.append_child(dv);
            container.set_attribute("count", container.children.len().to_string());
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, data_validations(vec![dv]));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a time-type data validation.
    pub fn add_data_validation_time(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        operator: &str,
        formula1: &str,
        formula2: Option<&str>,
        allow_blank: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut dv = OpenXmlElement::new("x", x, "dataValidation")
            .with_attribute("type", "time")
            .with_attribute("operator", operator)
            .with_attribute("allowBlank", if allow_blank { "1" } else { "0" })
            .with_attribute("showInputMessage", "1")
            .with_attribute("showErrorMessage", "1")
            .with_attribute("sqref", sqref)
            .with_child(OpenXmlElement::new("x", x, "formula1").with_text(formula1));
        if let Some(f2) = formula2 {
            dv.append_child(OpenXmlElement::new("x", x, "formula2").with_text(f2));
        }
        if let Some(container) = root.child_mut("dataValidations") {
            container.append_child(dv);
            container.set_attribute("count", container.children.len().to_string());
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, data_validations(vec![dv]));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a text-length data validation.
    pub fn add_data_validation_text_length(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        operator: &str,
        formula1: &str,
        formula2: Option<&str>,
        allow_blank: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut dv = OpenXmlElement::new("x", x, "dataValidation")
            .with_attribute("type", "textLength")
            .with_attribute("operator", operator)
            .with_attribute("allowBlank", if allow_blank { "1" } else { "0" })
            .with_attribute("showInputMessage", "1")
            .with_attribute("showErrorMessage", "1")
            .with_attribute("sqref", sqref)
            .with_child(OpenXmlElement::new("x", x, "formula1").with_text(formula1));
        if let Some(f2) = formula2 {
            dv.append_child(OpenXmlElement::new("x", x, "formula2").with_text(f2));
        }
        if let Some(container) = root.child_mut("dataValidations") {
            container.append_child(dv);
            container.set_attribute("count", container.children.len().to_string());
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, data_validations(vec![dv]));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a decimal-type data validation.
    pub fn add_data_validation_decimal(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        operator: &str,
        formula1: &str,
        formula2: Option<&str>,
        allow_blank: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let mut dv = OpenXmlElement::new("x", x, "dataValidation")
            .with_attribute("type", "decimal")
            .with_attribute("operator", operator)
            .with_attribute("allowBlank", if allow_blank { "1" } else { "0" })
            .with_attribute("showInputMessage", "1")
            .with_attribute("showErrorMessage", "1")
            .with_attribute("sqref", sqref)
            .with_child(OpenXmlElement::new("x", x, "formula1").with_text(formula1));
        if let Some(f2) = formula2 {
            dv.append_child(OpenXmlElement::new("x", x, "formula2").with_text(f2));
        }
        if let Some(container) = root.child_mut("dataValidations") {
            container.append_child(dv);
            container.set_attribute("count", container.children.len().to_string());
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, data_validations(vec![dv]));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a custom-formula data validation (`type="custom"`).
    pub fn add_data_validation_custom(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        formula: &str,
        allow_blank: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let dv = OpenXmlElement::new("x", x, "dataValidation")
            .with_attribute("type", "custom")
            .with_attribute("allowBlank", if allow_blank { "1" } else { "0" })
            .with_attribute("showInputMessage", "1")
            .with_attribute("showErrorMessage", "1")
            .with_attribute("sqref", sqref)
            .with_child(OpenXmlElement::new("x", x, "formula1").with_text(formula));
        if let Some(container) = root.child_mut("dataValidations") {
            container.append_child(dv);
            container.set_attribute("count", container.children.len().to_string());
        } else {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children
                .insert(insert_at, data_validations(vec![dv]));
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Set `showDropDown` on a data validation (inverted: true hides dropdown in Excel).
    pub fn set_data_validation_show_drop_down(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        show: bool,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(container) = root.child_mut("dataValidations") else {
            return Ok(false);
        };
        let mut found = false;
        for dv in container.children.iter_mut() {
            if dv.local_name == "dataValidation" && dv.get_attribute("sqref") == Some(sqref) {
                // OOXML: showDropDown="1" means hide the dropdown
                dv.set_attribute("showDropDown", if show { "0" } else { "1" });
                found = true;
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear dataValidation `@showDropDown` for `sqref`.
    pub fn clear_data_validation_show_drop_down(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<bool> {
        self.clear_data_validation_attr(sheet_name, sqref, "showDropDown")
    }

    /// Add a blank-values filter on an autoFilter column (`filters/@blank="1"`).
    pub fn add_auto_filter_blank(&mut self, sheet_name: &str, col_id: u32) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        let af = self.ensure_auto_filter_mut(&mut root);
        af.children.retain(|c| {
            !(c.local_name == "filterColumn"
                && c.get_attribute("colId") == Some(&col_id.to_string()))
        });
        let filters = OpenXmlElement::new("x", x, "filters").with_attribute("blank", "1");
        af.append_child(
            OpenXmlElement::new("x", x, "filterColumn")
                .with_attribute("colId", col_id.to_string())
                .with_child(filters),
        );
        self.save_sheet_root(&sheet_uri, &root)
    }

    fn sheet_uri(&self, sheet_name: &str) -> Result<PackUri> {
        self.sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))
    }

    fn load_sheet_root(&self, sheet_uri: &PackUri) -> Result<crate::element::OpenXmlElement> {
        if let Some(data) = self.package.opc().get_part(sheet_uri) {
            parse_element(data)
        } else {
            Ok(worksheet(vec![sheet_data(
                Vec::<crate::element::OpenXmlElement>::new(),
            )]))
        }
    }

    fn save_sheet_root(
        &mut self,
        sheet_uri: &PackUri,
        root: &crate::element::OpenXmlElement,
    ) -> Result<()> {
        let xml = write_element(root)?;
        self.package.set_part(
            sheet_uri.clone(),
            content_type::SPREADSHEET_WORKSHEET,
            xml,
        );
        Ok(())
    }

    /// Add a new empty worksheet and return its info.
    pub fn add_worksheet(&mut self, name: &str) -> Result<WorksheetInfo> {
        let wb_uri = self.ensure_workbook()?;
        let index = self.next_sheet_index;
        self.next_sheet_index += 1;
        let ws_uri = PackUri::new(format!("/xl/worksheets/sheet{index}.xml"));

        let ws = worksheet(vec![sheet_data(Vec::<crate::element::OpenXmlElement>::new())]);
        let ws_xml = write_element(&ws)?;
        self.package.set_part(
            ws_uri.clone(),
            content_type::SPREADSHEET_WORKSHEET,
            ws_xml,
        );

        let sheet_rel = self.package.add_part_relationship(
            &wb_uri,
            rel::WORKSHEET,
            &ws_uri,
            RelationshipTargetMode::Internal,
        );

        let info = WorksheetInfo {
            name: name.to_string(),
            sheet_id: index,
            relationship_id: sheet_rel,
            uri: ws_uri,
        };
        self.sheets.push(info.clone());
        self.rewrite_workbook()?;
        Ok(info)
    }

    /// Rename a worksheet. Updates the workbook sheet list.
    pub fn rename_sheet(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        let sheet = self
            .sheets
            .iter_mut()
            .find(|s| s.name == old_name)
            .ok_or_else(|| Error::Package(format!("sheet `{old_name}` not found")))?;
        sheet.name = new_name.to_string();
        self.rewrite_workbook()
    }

    /// Remove a worksheet by name.
    ///
    /// Deletes the worksheet part and rewrites the workbook sheet list. Does not
    /// clean up orphaned drawings/charts that only that sheet referenced.
    pub fn remove_sheet(&mut self, name: &str) -> Result<()> {
        let pos = self
            .sheets
            .iter()
            .position(|s| s.name == name)
            .ok_or_else(|| Error::Package(format!("sheet `{name}` not found")))?;
        let info = self.sheets.remove(pos);
        // Drop workbook → sheet relationship
        let wb_uri = self.ensure_workbook()?;
        let _ = self.package.delete_reference_relationship(Some(&wb_uri), &info.relationship_id);
        self.package.delete_part(&info.uri);
        self.rewrite_workbook()
    }

    /// Move a sheet from one index to another (0-based among workbook sheets).
    pub fn move_sheet(&mut self, from: usize, to: usize) -> Result<()> {
        if from >= self.sheets.len() {
            return Err(Error::Package(format!("sheet index {from} out of range")));
        }
        let item = self.sheets.remove(from);
        let insert_at = to.min(self.sheets.len());
        self.sheets.insert(insert_at, item);
        self.rewrite_workbook()
    }

    /// Sheet names in workbook order.
    pub fn sheet_names(&self) -> Vec<&str> {
        self.sheets.iter().map(|s| s.name.as_str()).collect()
    }

    /// Number of worksheets tracked in this document.
    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }

    /// Whether a worksheet with the given name is tracked.
    pub fn has_sheet(&self, name: &str) -> bool {
        self.sheets.iter().any(|s| s.name == name)
    }

    /// Whether the workbook has no worksheets.
    pub fn is_workbook_empty(&self) -> bool {
        self.sheets.is_empty()
    }

    /// Whether a sheet has no cell data under `sheetData`.
    pub fn sheet_is_empty(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(true),
        };
        let root = parse_element(data)?;
        let Some(sd) = root.child("sheetData") else {
            return Ok(true);
        };
        Ok(!sd.children.iter().any(|c| c.local_name == "row"))
    }

    /// Names of sheets with no row data.
    pub fn list_empty_sheets(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.sheet_is_empty(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has no row data.
    /// Remove all empty worksheets (no cell data). Returns how many were removed.
    ///
    /// Will not remove the last remaining sheet.
    pub fn remove_empty_sheets(&mut self) -> Result<usize> {
        let empty = self.list_empty_sheets()?;
        let mut n = 0usize;
        for name in empty {
            if self.sheet_count() <= 1 {
                break;
            }
            self.remove_sheet(&name)?;
            n += 1;
        }
        Ok(n)
    }

    pub fn has_empty_sheets(&self) -> Result<bool> {
        Ok(!self.list_empty_sheets()?.is_empty())
    }

    /// Add the workbook part and a single empty worksheet named `sheet_name`.
    ///
    /// Returns the relationship id of the worksheet.
    pub fn add_workbook_with_sheet(&mut self, sheet_name: &str) -> Result<String> {
        let info = self.add_worksheet(sheet_name)?;
        Ok(info.relationship_id)
    }

    /// Access (and create) the shared string table builder.
    pub fn shared_strings_mut(&mut self) -> &mut SharedStringTableBuilder {
        if self.sst.is_none() {
            self.sst = Some(SharedStringTableBuilder::new());
        }
        self.sst.as_mut().unwrap()
    }

    pub fn shared_strings(&self) -> Option<&SharedStringTableBuilder> {
        self.sst.as_ref()
    }

    /// Persist the shared string table into the package.
    pub fn flush_shared_strings(&mut self) -> Result<()> {
        let xml = {
            let Some(sst) = &self.sst else {
                return Ok(());
            };
            write_element(&sst.to_element())?
        };
        let wb_uri = self.ensure_workbook()?;
        let sst_uri = PackUri::new(SHARED_STRINGS_URI);
        self.package.set_part(
            sst_uri.clone(),
            content_type::SPREADSHEET_SHARED_STRINGS,
            xml,
        );

        // Ensure workbook → sharedStrings relationship
        let has = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| rels.get_by_type(rel::SHARED_STRINGS))
            .is_some();
        if !has {
            self.package.add_part_relationship(
                &wb_uri,
                rel::SHARED_STRINGS,
                &sst_uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok(())
    }

    /// Write a 2D grid of strings into a sheet by name (creates sheet if needed).
    ///
    /// When `use_shared_strings` is true, cells reference the SST (`t="s"`).
    pub fn write_sheet_strings_ex(
        &mut self,
        sheet_name: &str,
        rows: &[Vec<&str>],
        use_shared_strings: bool,
    ) -> Result<()> {
        if self.sheets.is_empty()
            && self
                .package
                .opc()
                .main_part_uri(rel::OFFICE_DOCUMENT)
                .is_err()
        {
            self.add_worksheet(sheet_name)?;
        } else if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }

        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        let mut row_elems = Vec::new();
        for (ri, row_vals) in rows.iter().enumerate() {
            let row_idx = (ri + 1) as u32;
            let mut cells = Vec::new();
            for (ci, val) in row_vals.iter().enumerate() {
                let col = column_name(ci);
                let reference = format!("{col}{row_idx}");
                if use_shared_strings {
                    let idx = self.shared_strings_mut().intern(*val);
                    cells.push(cell_shared_string(&reference, idx));
                } else {
                    cells.push(cell_inline_str(&reference, val));
                }
            }
            row_elems.push(row(row_idx, cells));
        }
        let ws = worksheet(vec![sheet_data(row_elems)]);
        let ws_xml = write_element(&ws)?;
        self.package.set_part(
            sheet_uri,
            content_type::SPREADSHEET_WORKSHEET,
            ws_xml,
        );

        if use_shared_strings {
            self.flush_shared_strings()?;
        }
        Ok(())
    }

    /// Write a simple 2D grid of inline-string cells into the named sheet.
    pub fn write_sheet_strings(&mut self, sheet_name: &str, rows: &[Vec<&str>]) -> Result<()> {
        self.write_sheet_strings_ex(sheet_name, rows, false)
    }

    /// Write using the shared string table.
    pub fn write_sheet_shared_strings(
        &mut self,
        sheet_name: &str,
        rows: &[Vec<&str>],
    ) -> Result<()> {
        self.write_sheet_strings_ex(sheet_name, rows, true)
    }

    /// Set column widths on a sheet.
    ///
    /// `widths` is a list of `(min_col, max_col, width)` where columns are 1-based.
    pub fn set_column_widths(
        &mut self,
        sheet_name: &str,
        widths: &[(u32, u32, f64)],
    ) -> Result<()> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        // Load existing worksheet or create empty
        let mut root = if let Some(data) = self.package.opc().get_part(&sheet_uri) {
            parse_element(data)?
        } else {
            worksheet(vec![sheet_data(Vec::<crate::element::OpenXmlElement>::new())])
        };

        // Remove existing cols if present
        root.children.retain(|c| c.local_name != "cols");

        let col_els: Vec<_> = widths
            .iter()
            .map(|(min, max, w)| column(*min, *max, *w, true))
            .collect();
        let cols = columns(col_els);

        // Insert cols before sheetData (OOXML order)
        if let Some(pos) = root.children.iter().position(|c| c.local_name == "sheetData") {
            root.children.insert(pos, cols);
        } else {
            root.append_child(cols);
        }

        let xml = write_element(&root)?;
        self.package
            .set_part(sheet_uri, content_type::SPREADSHEET_WORKSHEET, xml);
        Ok(())
    }

    /// Hide or show columns in the inclusive 1-based range `[min_col, max_col]`.
    pub fn set_column_hidden(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
        hidden: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if root.child("cols").is_none() {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(
                insert_at,
                columns(vec![column_with_hidden(min_col, max_col, 8.43, false, hidden)]),
            );
        } else {
            let cols = root.child_mut("cols").unwrap();
            let mut found = false;
            for col in cols.children.iter_mut().filter(|c| c.local_name == "col") {
                let min: u32 = col
                    .get_attribute("min")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let max: u32 = col
                    .get_attribute("max")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                if min == min_col && max == max_col {
                    if hidden {
                        col.set_attribute("hidden", "1");
                    } else {
                        col.attributes.retain(|a| a.local_name != "hidden");
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                cols.append_child(column_with_hidden(min_col, max_col, 8.43, false, hidden));
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Clear hidden on a column range (set hidden=false).
    pub fn clear_column_hidden(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
    ) -> Result<()> {
        self.set_column_hidden(sheet_name, min_col, max_col, false)
    }

    fn ensure_col_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
        min_col: u32,
        max_col: u32,
    ) -> &'a mut OpenXmlElement {
        let x = crate::namespace::ns::SPREADSHEETML.uri;
        if root.child("cols").is_none() {
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sheetData")
                .unwrap_or(0);
            root.children.insert(
                insert_at,
                OpenXmlElement::new("x", x, "cols").with_child(column(min_col, max_col, 8.43, false)),
            );
        }
        let cols = root.child_mut("cols").unwrap();
        let has = cols.children.iter().any(|c| {
            c.local_name == "col"
                && c.get_attribute("min") == Some(&min_col.to_string())
                && c.get_attribute("max") == Some(&max_col.to_string())
        });
        if !has {
            cols.append_child(column(min_col, max_col, 8.43, false));
        }
        cols.children
            .iter_mut()
            .find(|c| {
                c.local_name == "col"
                    && c.get_attribute("min") == Some(&min_col.to_string())
                    && c.get_attribute("max") == Some(&max_col.to_string())
            })
            .expect("col ensured")
    }

    /// Set bestFit flag on a column range.
    /// Unhide every hidden column on a sheet. Returns how many column ranges were unhidden.
    pub fn unhide_all_columns(&mut self, sheet_name: &str) -> Result<usize> {
        let hidden = self.list_hidden_columns(sheet_name)?;
        if hidden.is_empty() {
            return Ok(0);
        }
        // Collapse consecutive into ranges
        let mut n = 0usize;
        let mut start = hidden[0];
        let mut prev = hidden[0];
        for &c in &hidden[1..] {
            if c == prev + 1 {
                prev = c;
                continue;
            }
            self.set_column_hidden(sheet_name, start, prev, false)?;
            n += 1;
            start = c;
            prev = c;
        }
        self.set_column_hidden(sheet_name, start, prev, false)?;
        n += 1;
        Ok(n)
    }

    pub fn set_column_best_fit(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
        best_fit: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let col = self.ensure_col_mut(&mut root, min_col, max_col);
        col.set_attribute("bestFit", if best_fit { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether bestFit is set on a matching column definition.
    pub fn column_best_fit(&self, sheet_name: &str, min_col: u32, max_col: u32) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("cols") else {
            return Ok(false);
        };
        for col in cols.children_by_name("col") {
            if col.get_attribute("min") == Some(&min_col.to_string())
                && col.get_attribute("max") == Some(&max_col.to_string())
            {
                return Ok(col
                    .get_attribute("bestFit")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false));
            }
        }
        Ok(false)
    }

    /// Set style index on a column range (`col/@style`).
    pub fn set_column_style(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
        style: u32,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let col = self.ensure_col_mut(&mut root, min_col, max_col);
        col.set_attribute("style", style.to_string());
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read column style index when present.
    pub fn column_style(
        &self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
    ) -> Result<Option<u32>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("cols") else {
            return Ok(None);
        };
        for col in cols.children_by_name("col") {
            if col.get_attribute("min") == Some(&min_col.to_string())
                && col.get_attribute("max") == Some(&max_col.to_string())
            {
                return Ok(col.get_attribute("style").and_then(|s| s.parse().ok()));
            }
        }
        Ok(None)
    }

    /// Set outline level on a column range (0–7).
    pub fn set_column_outline_level(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
        level: u8,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let col = self.ensure_col_mut(&mut root, min_col, max_col);
        col.set_attribute("outlineLevel", level.to_string());
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read column outline level.
    pub fn column_outline_level(
        &self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
    ) -> Result<Option<u8>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("cols") else {
            return Ok(None);
        };
        for col in cols.children_by_name("col") {
            if col.get_attribute("min") == Some(&min_col.to_string())
                && col.get_attribute("max") == Some(&max_col.to_string())
            {
                return Ok(col
                    .get_attribute("outlineLevel")
                    .and_then(|s| s.parse().ok()));
            }
        }
        Ok(None)
    }

    /// Set collapsed flag on a column range.
    pub fn set_column_collapsed(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
        collapsed: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let col = self.ensure_col_mut(&mut root, min_col, max_col);
        col.set_attribute("collapsed", if collapsed { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether collapsed is set on a column range.
    pub fn column_collapsed(&self, sheet_name: &str, min_col: u32, max_col: u32) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("cols") else {
            return Ok(false);
        };
        for col in cols.children_by_name("col") {
            if col.get_attribute("min") == Some(&min_col.to_string())
                && col.get_attribute("max") == Some(&max_col.to_string())
            {
                return Ok(col
                    .get_attribute("collapsed")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false));
            }
        }
        Ok(false)
    }

    /// Clear bestFit on a column range. Returns whether found.
    pub fn clear_column_best_fit(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(cols) = root.child_mut("cols") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "col" {
                continue;
            }
            if col.get_attribute("min") == Some(&min_col.to_string())
                && col.get_attribute("max") == Some(&max_col.to_string())
            {
                if col.get_attribute("bestFit").is_some() {
                    col.remove_attribute("bestFit");
                    found = true;
                }
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear style on a column range. Returns whether found.
    pub fn clear_column_style(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(cols) = root.child_mut("cols") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "col" {
                continue;
            }
            if col.get_attribute("min") == Some(&min_col.to_string())
                && col.get_attribute("max") == Some(&max_col.to_string())
            {
                if col.get_attribute("style").is_some() {
                    col.remove_attribute("style");
                    found = true;
                }
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear collapsed on a column range. Returns whether found.
    pub fn clear_column_collapsed(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(cols) = root.child_mut("cols") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "col" {
                continue;
            }
            if col.get_attribute("min") == Some(&min_col.to_string())
                && col.get_attribute("max") == Some(&max_col.to_string())
            {
                if col.get_attribute("collapsed").is_some() {
                    col.remove_attribute("collapsed");
                    found = true;
                }
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// List column outline levels as `(min, max, outlineLevel, collapsed)`.
    ///
    /// Only columns that declare an `outlineLevel` attribute are returned.
    pub fn column_outline_levels(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(u32, u32, u8, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("cols") else {
            return Ok(Vec::new());
        };
        Ok(cols
            .children_by_name("col")
            .filter_map(|c| {
                let level = c.get_attribute("outlineLevel")?.parse::<u8>().ok()?;
                let min = c.get_attribute("min")?.parse().ok()?;
                let max = c.get_attribute("max")?.parse().ok()?;
                let collapsed = c
                    .get_attribute("collapsed")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                Some((min, max, level, collapsed))
            })
            .collect())
    }

    /// Number of column ranges with an outline level.
    pub fn column_outline_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.column_outline_levels(sheet_name)?.len())
    }

    /// Whether any column ranges have an outline level.
    pub fn has_column_outlines(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.column_outline_count(sheet_name)? > 0)
    }

    /// Clear outline level (and collapsed) on a column range. Returns whether found.
    pub fn clear_column_outline(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(cols) = root.child_mut("cols") else {
            return Ok(false);
        };
        let mut found = false;
        for col in cols.children.iter_mut() {
            if col.local_name != "col" {
                continue;
            }
            if col.get_attribute("min") == Some(&min_col.to_string())
                && col.get_attribute("max") == Some(&max_col.to_string())
            {
                col.remove_attribute("outlineLevel");
                col.remove_attribute("collapsed");
                found = true;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Alias for [`clear_column_outline`](Self::clear_column_outline).
    pub fn clear_column_outline_level(
        &mut self,
        sheet_name: &str,
        min_col: u32,
        max_col: u32,
    ) -> Result<bool> {
        self.clear_column_outline(sheet_name, min_col, max_col)
    }

    /// Clear outlineLevel/collapsed on every column entry. Returns count cleared.
    pub fn clear_all_column_outlines(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(cols) = root.child_mut("cols") else {
            return Ok(0);
        };
        let mut count = 0usize;
        for col in cols.children.iter_mut() {
            if col.local_name != "col" {
                continue;
            }
            if col.get_attribute("outlineLevel").is_some()
                || col.get_attribute("collapsed").is_some()
            {
                col.remove_attribute("outlineLevel");
                col.remove_attribute("collapsed");
                count += 1;
            }
        }
        if count > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(count)
    }

    /// Set row heights (points) and/or hide rows on a sheet.
    ///
    /// Each entry is `(row_index_1_based, height_points, hidden)`.
    /// Missing rows are created empty; existing row attributes are updated.
    pub fn set_row_heights(
        &mut self,
        sheet_name: &str,
        rows: &[(u32, f64, bool)],
    ) -> Result<()> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let sheet_data = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("worksheet has no sheetData".into()))?;

        for &(row_idx, height, hidden) in rows {
            let existing = sheet_data
                .children
                .iter_mut()
                .find(|c| {
                    c.local_name == "row"
                        && c.get_attribute("r")
                            .and_then(|s| s.parse::<u32>().ok())
                            == Some(row_idx)
                });
            if let Some(row_el) = existing {
                row_el.set_attribute("ht", height.to_string());
                row_el.set_attribute("customHeight", "1");
                if hidden {
                    row_el.set_attribute("hidden", "1");
                } else {
                    // remove hidden if present
                    row_el.attributes.retain(|a| a.local_name != "hidden");
                }
            } else {
                use crate::spreadsheet::row_with_height;
                sheet_data.append_child(row_with_height(
                    row_idx,
                    Some(height),
                    hidden,
                    Vec::<crate::element::OpenXmlElement>::new(),
                ));
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    fn ensure_row_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
        row_idx: u32,
    ) -> Result<&'a mut OpenXmlElement> {
        let sheet_data = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("worksheet has no sheetData".into()))?;
        let has = sheet_data.children.iter().any(|c| {
            c.local_name == "row"
                && c.get_attribute("r").and_then(|s| s.parse::<u32>().ok()) == Some(row_idx)
        });
        if !has {
            sheet_data.append_child(row(
                row_idx,
                Vec::<crate::element::OpenXmlElement>::new(),
            ));
        }
        Ok(sheet_data
            .children
            .iter_mut()
            .find(|c| {
                c.local_name == "row"
                    && c.get_attribute("r").and_then(|s| s.parse::<u32>().ok()) == Some(row_idx)
            })
            .expect("row ensured"))
    }

    /// Set thick top border flag on a row (`thickTop`).
    /// Clear custom row heights (`ht` / `customHeight`) on a sheet. Returns rows cleared.
    pub fn clear_row_heights(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut n = 0usize;
        if let Some(sd) = root.child_mut("sheetData") {
            for row in sd.children.iter_mut() {
                if row.local_name != "row" {
                    continue;
                }
                let had = row.get_attribute("ht").is_some()
                    || row.get_attribute("customHeight").is_some();
                if had {
                    row.attributes.retain(|a| {
                        a.local_name != "ht" && a.local_name != "customHeight"
                    });
                    n += 1;
                }
            }
        }
        if n > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    pub fn set_row_thick_top(
        &mut self,
        sheet_name: &str,
        row_idx: u32,
        enabled: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let row_el = self.ensure_row_mut(&mut root, row_idx)?;
        row_el.set_attribute("thickTop", if enabled { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether row has thickTop.
    pub fn row_thick_top(&self, sheet_name: &str, row_idx: u32) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetData")
            .and_then(|sd| {
                sd.children.iter().find(|c| {
                    c.local_name == "row"
                        && c.get_attribute("r").and_then(|s| s.parse().ok()) == Some(row_idx)
                })
            })
            .and_then(|r| r.get_attribute("thickTop"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set thick bottom border flag on a row (`thickBot`).
    pub fn set_row_thick_bottom(
        &mut self,
        sheet_name: &str,
        row_idx: u32,
        enabled: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let row_el = self.ensure_row_mut(&mut root, row_idx)?;
        row_el.set_attribute("thickBot", if enabled { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether row has thickBot.
    pub fn row_thick_bottom(&self, sheet_name: &str, row_idx: u32) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetData")
            .and_then(|sd| {
                sd.children.iter().find(|c| {
                    c.local_name == "row"
                        && c.get_attribute("r").and_then(|s| s.parse().ok()) == Some(row_idx)
                })
            })
            .and_then(|r| r.get_attribute("thickBot"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Clear thickTop on a row.
    pub fn clear_row_thick_top(
        &mut self,
        sheet_name: &str,
        row_idx: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sd) = root.child_mut("sheetData") else {
            return Ok(false);
        };
        let mut found = false;
        for row in sd.children.iter_mut() {
            if row.local_name == "row"
                && row.get_attribute("r").and_then(|x| x.parse().ok()) == Some(row_idx)
            {
                if row.get_attribute("thickTop").is_some() {
                    row.remove_attribute("thickTop");
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Clear thickBot on a row.
    pub fn clear_row_thick_bottom(
        &mut self,
        sheet_name: &str,
        row_idx: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sd) = root.child_mut("sheetData") else {
            return Ok(false);
        };
        let mut found = false;
        for row in sd.children.iter_mut() {
            if row.local_name == "row"
                && row.get_attribute("r").and_then(|x| x.parse().ok()) == Some(row_idx)
            {
                if row.get_attribute("thickBot").is_some() {
                    row.remove_attribute("thickBot");
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Set collapsed flag on a row.
    pub fn set_row_collapsed(
        &mut self,
        sheet_name: &str,
        row_idx: u32,
        collapsed: bool,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let row_el = self.ensure_row_mut(&mut root, row_idx)?;
        row_el.set_attribute("collapsed", if collapsed { "1" } else { "0" });
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Whether row is collapsed.
    pub fn row_collapsed(&self, sheet_name: &str, row_idx: u32) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetData")
            .and_then(|sd| {
                sd.children.iter().find(|c| {
                    c.local_name == "row"
                        && c.get_attribute("r").and_then(|s| s.parse().ok()) == Some(row_idx)
                })
            })
            .and_then(|r| r.get_attribute("collapsed"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Clear collapsed attribute on a row.
    pub fn clear_row_collapsed(
        &mut self,
        sheet_name: &str,
        row_idx: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sd) = root.child_mut("sheetData") else {
            return Ok(false);
        };
        let mut found = false;
        for row in sd.children.iter_mut() {
            if row.local_name == "row"
                && row.get_attribute("r").and_then(|x| x.parse().ok()) == Some(row_idx)
            {
                if row.get_attribute("collapsed").is_some() {
                    row.remove_attribute("collapsed");
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// Set style index on a row (`s` attribute).
    pub fn set_row_style(&mut self, sheet_name: &str, row_idx: u32, style: u32) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let row_el = self.ensure_row_mut(&mut root, row_idx)?;
        row_el.set_attribute("s", style.to_string());
        row_el.set_attribute("customFormat", "1");
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Read row style index.
    pub fn row_style(&self, sheet_name: &str, row_idx: u32) -> Result<Option<u32>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sheetData")
            .and_then(|sd| {
                sd.children.iter().find(|c| {
                    c.local_name == "row"
                        && c.get_attribute("r").and_then(|s| s.parse().ok()) == Some(row_idx)
                })
            })
            .and_then(|r| r.get_attribute("s"))
            .and_then(|s| s.parse().ok()))
    }

    /// Whether row has a style index.
    pub fn has_row_style(&self, sheet_name: &str, row_idx: u32) -> Result<bool> {
        Ok(self.row_style(sheet_name, row_idx)?.is_some())
    }

    /// Clear row style (`s` and `customFormat`).
    pub fn clear_row_style(
        &mut self,
        sheet_name: &str,
        row_idx: u32,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sd) = root.child_mut("sheetData") else {
            return Ok(false);
        };
        let mut found = false;
        for row in sd.children.iter_mut() {
            if row.local_name == "row"
                && row.get_attribute("r").and_then(|x| x.parse().ok()) == Some(row_idx)
            {
                let before = row.attributes.len();
                row.attributes.retain(|a| a.local_name != "s" && a.local_name != "customFormat");
                if row.attributes.len() < before {
                    found = true;
                }
                break;
            }
        }
        if found {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(found)
    }

    /// List column width definitions as `(min, max, width, hidden)` tuples.
    pub fn column_widths(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(u32, u32, f64, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("cols") else {
            return Ok(Vec::new());
        };
        Ok(cols
            .children_by_name("col")
            .map(|c| {
                let min = c
                    .get_attribute("min")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                let max = c
                    .get_attribute("max")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(min);
                let width = c
                    .get_attribute("width")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(8.43);
                let hidden = c.get_attribute("hidden").map(|s| s == "1").unwrap_or(false);
                (min, max, width, hidden)
            })
            .collect())
    }

    /// Whether any column definition covering `col_index` (1-based) is hidden.
    /// Remove custom column width definitions (`cols`) from a sheet.
    /// Returns whether a `cols` element was present.
    pub fn clear_column_widths(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "cols");
        if root.children.len() == before {
            return Ok(false);
        }
        self.save_sheet_root(&sheet_uri, &root)?;
        Ok(true)
    }

    /// Clear custom column widths on every sheet. Returns sheets modified.
    pub fn clear_all_column_widths(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_column_widths(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn is_column_hidden(&self, sheet_name: &str, col_index: u32) -> Result<bool> {
        Ok(self
            .column_widths(sheet_name)?
            .into_iter()
            .any(|(min, max, _, hidden)| hidden && min <= col_index && col_index <= max))
    }

    /// List 1-based column indices that fall under a hidden column definition.
    pub fn list_hidden_columns(&self, sheet_name: &str) -> Result<Vec<u32>> {
        let mut cols = Vec::new();
        for (min, max, _, hidden) in self.column_widths(sheet_name)? {
            if hidden {
                for c in min..=max {
                    if !cols.contains(&c) {
                        cols.push(c);
                    }
                }
            }
        }
        cols.sort_unstable();
        Ok(cols)
    }


    /// Whether any columns are hidden on a sheet.
    pub fn has_hidden_columns(&self, sheet_name: &str) -> Result<bool> {
        Ok(!self.list_hidden_columns(sheet_name)?.is_empty())
    }

    /// Count of hidden columns on a sheet (expanded min..=max ranges).
    pub fn hidden_column_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_hidden_columns(sheet_name)?.len())
    }

    /// Sheet names that have at least one hidden column.
    pub fn sheets_with_hidden_columns(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_hidden_columns(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has hidden columns.
    pub fn has_sheets_with_hidden_columns(&self) -> Result<bool> {
        Ok(!self.sheets_with_hidden_columns()?.is_empty())
    }

    /// List custom row heights as `(row_index, height, hidden)` for rows with `ht` set.
    pub fn row_heights(&self, sheet_name: &str) -> Result<Vec<(u32, f64, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let Some(sd) = root.child("sheetData") else {
            return Ok(Vec::new());
        };
        Ok(sd
            .children_by_name("row")
            .filter_map(|r| {
                let ht = r.get_attribute("ht")?.parse::<f64>().ok()?;
                let idx = r
                    .get_attribute("r")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let hidden = r.get_attribute("hidden").map(|s| s == "1").unwrap_or(false);
                Some((idx, ht, hidden))
            })
            .collect())
    }

    /// Clone into a new in-memory spreadsheet (C# `CloneableExtensions.Clone()`).
    pub fn clone_document(&mut self) -> Result<Self> {
        let bytes = self.to_bytes()?;
        Self::open_bytes(bytes)
    }

    /// Clone to a new file path (C# `Clone(string path)`).
    pub fn clone_to_path(&mut self, path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = self.to_bytes()?;
        let mut cloned = Self::open_bytes(bytes)?;
        *cloned.settings_mut() = self.settings().clone();
        cloned.save_as(path)?;
        let settings = cloned.settings().clone();
        drop(cloned);
        Self::open_with_settings(path, true, settings)
    }

    /// Clone package ZIP bytes.
    pub fn clone_to_bytes(&mut self) -> Result<Vec<u8>> {
        self.to_bytes()
    }

    /// Clone and write ZIP bytes to a writer.
    pub fn clone_to_writer<W: std::io::Write>(&mut self, mut writer: W) -> Result<()> {
        let bytes = self.clone_to_bytes()?;
        writer.write_all(&bytes)?;
        Ok(())
    }

    /// Write a formula into a cell on the given sheet.
    ///
    /// `formula` should not include a leading `=`. Creates the sheet if needed.
    /// Preserves existing sheet content and overwrites only the target cell.
    pub fn set_cell_formula(
        &mut self,
        sheet_name: &str,
        reference: &str,
        formula: &str,
        cached_value: Option<&str>,
    ) -> Result<()> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        let mut root = if let Some(data) = self.package.opc().get_part(&sheet_uri) {
            parse_element(data)?
        } else {
            worksheet(vec![sheet_data(Vec::<crate::element::OpenXmlElement>::new())])
        };

        // Ensure sheetData exists
        if root.child("sheetData").is_none() {
            root.append_child(sheet_data(Vec::<crate::element::OpenXmlElement>::new()));
        }
        let sheet_data_el = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("no sheetData".into()))?;

        // Parse row number from reference (e.g. "B12" → 12)
        let row_num: u32 = reference
            .chars()
            .skip_while(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .parse()
            .unwrap_or(1);

        // Find or create the row
        let row_pos = sheet_data_el
            .children
            .iter()
            .position(|c| c.local_name == "row" && c.get_attribute("r") == Some(&row_num.to_string()));
        if row_pos.is_none() {
            sheet_data_el.append_child(row(row_num, Vec::<crate::element::OpenXmlElement>::new()));
        }
        let row_el = sheet_data_el
            .children
            .iter_mut()
            .find(|c| c.local_name == "row" && c.get_attribute("r") == Some(&row_num.to_string()))
            .unwrap();

        // Remove existing cell at reference
        row_el
            .children
            .retain(|c| !(c.local_name == "c" && c.get_attribute("r") == Some(reference)));
        row_el.append_child(cell_formula(reference, formula, cached_value));

        let xml = write_element(&root)?;
        self.package
            .set_part(sheet_uri, content_type::SPREADSHEET_WORKSHEET, xml);
        Ok(())
    }

    /// Write an inline-string value into a cell (creates sheet/row/cell if needed).
    pub fn set_cell_value(
        &mut self,
        sheet_name: &str,
        reference: &str,
        value: &str,
    ) -> Result<()> {
        self.upsert_cell(sheet_name, reference, cell_inline_str(reference, value))
    }

    /// Write a numeric value into a cell (creates sheet/row/cell if needed).
    pub fn set_cell_number(
        &mut self,
        sheet_name: &str,
        reference: &str,
        value: f64,
    ) -> Result<()> {
        self.upsert_cell(sheet_name, reference, cell_number(reference, value))
    }

    /// Write a numeric value with a style index into a cell.
    pub fn set_cell_number_styled(
        &mut self,
        sheet_name: &str,
        reference: &str,
        value: f64,
        style_index: u32,
    ) -> Result<()> {
        self.upsert_cell(
            sheet_name,
            reference,
            cell_number_styled(reference, value, style_index),
        )
    }

    fn upsert_cell(
        &mut self,
        sheet_name: &str,
        reference: &str,
        cell: crate::element::OpenXmlElement,
    ) -> Result<()> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if root.child("sheetData").is_none() {
            root.append_child(sheet_data(Vec::<crate::element::OpenXmlElement>::new()));
        }
        let row_num: u32 = reference
            .chars()
            .skip_while(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .parse()
            .unwrap_or(1);
        let sheet_data_el = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("no sheetData".into()))?;
        if !sheet_data_el.children.iter().any(|c| {
            c.local_name == "row" && c.get_attribute("r") == Some(&row_num.to_string())
        }) {
            sheet_data_el.append_child(row(row_num, Vec::<crate::element::OpenXmlElement>::new()));
        }
        let row_el = sheet_data_el
            .children
            .iter_mut()
            .find(|c| c.local_name == "row" && c.get_attribute("r") == Some(&row_num.to_string()))
            .unwrap();
        row_el
            .children
            .retain(|c| !(c.local_name == "c" && c.get_attribute("r") == Some(reference)));
        row_el.append_child(cell);
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Insert `count` empty rows starting at 1-based `start_row`, shifting existing rows down.
    ///
    /// Only rewrites `r` attributes on rows and cells; does not update formulas or merges.
    pub fn insert_rows(
        &mut self,
        sheet_name: &str,
        start_row: u32,
        count: u32,
    ) -> Result<()> {
        if count == 0 {
            return Ok(());
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sheet_data_el) = root.child_mut("sheetData") else {
            return Ok(());
        };
        // Process rows from bottom to top so renumbering doesn't collide
        let mut rows: Vec<&mut OpenXmlElement> = sheet_data_el
            .children
            .iter_mut()
            .filter(|c| c.local_name == "row")
            .collect();
        rows.sort_by_key(|r| {
            r.get_attribute("r")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0)
        });
        for row_el in rows.into_iter().rev() {
            let Some(r_str) = row_el.get_attribute("r") else {
                continue;
            };
            let r: u32 = r_str.parse().unwrap_or(0);
            if r >= start_row {
                let new_r = r + count;
                row_el.set_attribute("r", new_r.to_string());
                // Rewrite cell refs
                for cell in row_el.children.iter_mut().filter(|c| c.local_name == "c") {
                    if let Some(cref) = cell.get_attribute("r") {
                        if let Some((_, col)) = cell_ref_to_row_col(cref) {
                            let col_letters = column_name(col as usize);
                            cell.set_attribute("r", format!("{col_letters}{new_r}"));
                        }
                    }
                }
            }
        }
        // Insert empty rows
        for i in 0..count {
            let r = start_row + i;
            sheet_data_el.append_child(row(r, Vec::<crate::element::OpenXmlElement>::new()));
        }
        // Sort rows by r for neatness
        if let Some(sd) = root.child_mut("sheetData") {
            sd.children.sort_by_key(|c| {
                if c.local_name == "row" {
                    c.get_attribute("r")
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0)
                } else {
                    0
                }
            });
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Delete a single 1-based row, shifting subsequent rows up.
    pub fn delete_row(&mut self, sheet_name: &str, row_num: u32) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sheet_data_el) = root.child_mut("sheetData") else {
            return Ok(());
        };
        sheet_data_el
            .children
            .retain(|c| !(c.local_name == "row" && c.get_attribute("r") == Some(&row_num.to_string())));
        for row_el in sheet_data_el
            .children
            .iter_mut()
            .filter(|c| c.local_name == "row")
        {
            let Some(r_str) = row_el.get_attribute("r") else {
                continue;
            };
            let r: u32 = r_str.parse().unwrap_or(0);
            if r > row_num {
                let new_r = r - 1;
                row_el.set_attribute("r", new_r.to_string());
                for cell in row_el.children.iter_mut().filter(|c| c.local_name == "c") {
                    if let Some(cref) = cell.get_attribute("r") {
                        if let Some((_, col)) = cell_ref_to_row_col(cref) {
                            let col_letters = column_name(col as usize);
                            cell.set_attribute("r", format!("{col_letters}{new_r}"));
                        }
                    }
                }
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Clear a cell's value/formula while preserving the row structure.
    ///
    /// Removes the cell element entirely when present. Returns `true` if a cell was removed.
    pub fn clear_cell(&mut self, sheet_name: &str, reference: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sheet_data_el) = root.child_mut("sheetData") else {
            return Ok(false);
        };
        let mut removed = false;
        for row_el in sheet_data_el
            .children
            .iter_mut()
            .filter(|c| c.local_name == "row")
        {
            let before = row_el.children.len();
            row_el
                .children
                .retain(|c| !(c.local_name == "c" && c.get_attribute("r") == Some(reference)));
            if row_el.children.len() < before {
                removed = true;
            }
        }
        if removed {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear all cells in an inclusive A1-style range (e.g. `"A1:C3"`).
    ///
    /// Returns the number of cells removed.
    pub fn clear_range(&mut self, sheet_name: &str, range: &str) -> Result<usize> {
        let (from, to) = range.split_once(':').unwrap_or((range, range));
        let (r1, c1) = cell_ref_to_row_col(from)
            .ok_or_else(|| Error::Package(format!("bad range start `{from}`")))?;
        let (r2, c2) = cell_ref_to_row_col(to)
            .ok_or_else(|| Error::Package(format!("bad range end `{to}`")))?;
        let (min_r, max_r) = (r1.min(r2), r1.max(r2));
        let (min_c, max_c) = (c1.min(c2), c1.max(c2));
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sheet_data_el) = root.child_mut("sheetData") else {
            return Ok(0);
        };
        let mut removed = 0usize;
        for row_el in sheet_data_el
            .children
            .iter_mut()
            .filter(|c| c.local_name == "row")
        {
            let before = row_el.children.len();
            row_el.children.retain(|c| {
                if c.local_name != "c" {
                    return true;
                }
                let Some(r) = c.get_attribute("r") else {
                    return true;
                };
                match cell_ref_to_row_col(r) {
                    Some((rr, cc)) if rr >= min_r && rr <= max_r && cc >= min_c && cc <= max_c => {
                        false
                    }
                    _ => true,
                }
            });
            removed += before - row_el.children.len();
        }
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Copy a worksheet to a new sheet name (deep-copies the part XML; does not
    /// duplicate drawings/charts relationships targets).
    pub fn copy_sheet(&mut self, source_name: &str, new_name: &str) -> Result<WorksheetInfo> {
        let src_uri = self.sheet_uri(source_name)?;
        let data = self
            .package
            .opc()
            .get_part(&src_uri)
            .ok_or_else(|| Error::PartNotFound(src_uri.to_string()))?
            .to_vec();
        let info = self.add_worksheet(new_name)?;
        self.package.set_part(
            info.uri.clone(),
            content_type::SPREADSHEET_WORKSHEET,
            data,
        );
        Ok(info)
    }

    /// Read a formula from a cell, if present. Returns `(formula, cached_value)`.
    pub fn cell_formula(
        &self,
        sheet_name: &str,
        reference: &str,
    ) -> Result<Option<(String, Option<String>)>> {
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;
        let data = self
            .package
            .opc()
            .get_part(&sheet_uri)
            .ok_or_else(|| Error::PartNotFound(sheet_uri.to_string()))?;
        let root = parse_element(data)?;
        for cell in root.descendants().filter(|e| e.local_name == "c") {
            if cell.get_attribute("r") == Some(reference) {
                if let Some(f) = cell.child("f") {
                    let formula = f.inner_text();
                    let cached = cell.child("v").map(|v| v.inner_text());
                    return Ok(Some((formula, cached)));
                }
                return Ok(None);
            }
        }
        Ok(None)
    }

    /// Add a chart part containing a simple bar/column chart and return its URI + rel id.
    ///

    /// Convenience: add a bar chart part (same as [`add_bar_chart`](Self::add_bar_chart)).
    ///
    /// Returns `(chart_uri, relationship_id)`.
    pub fn add_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_bar_chart(title, categories, values)
    }

    /// The chart is related from the workbook. For an on-sheet anchor, use
    /// [`add_bar_chart_on_sheet`](Self::add_bar_chart_on_sheet).
    pub fn add_bar_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_chart_part(&bar_chart_space(title, categories, values))
    }

    /// Add a line chart part related from the workbook.
    pub fn add_line_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_chart_part(&line_chart_space(title, categories, values))
    }

    /// Add a pie chart part related from the workbook.
    pub fn add_pie_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_chart_part(&pie_chart_space(title, categories, values))
    }

    /// Add a scatter chart part related from the workbook.
    pub fn add_scatter_chart(
        &mut self,
        title: &str,
        x_values: &[f64],
        y_values: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_chart_part(&scatter_chart_space(title, x_values, y_values))
    }

    /// Add an area chart part related from the workbook.
    pub fn add_area_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_chart_part(&area_chart_space(title, categories, values))
    }

    /// Add a doughnut chart part related from the workbook.
    pub fn add_doughnut_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_chart_part(&doughnut_chart_space(title, categories, values))
    }

    /// Add a radar chart part related from the workbook.
    pub fn add_radar_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_chart_part(&radar_chart_space(title, categories, values))
    }

    /// Add a bubble chart part related from the workbook.
    pub fn add_bubble_chart(
        &mut self,
        title: &str,
        x_values: &[f64],
        y_values: &[f64],
        sizes: &[f64],
    ) -> Result<(PackUri, String)> {
        self.add_chart_part(&bubble_chart_space(title, x_values, y_values, sizes))
    }

    /// Add an arbitrary extended part related from the workbook.
    ///
    /// Corresponds to C# `ExtendedPart` — any content type/relationship not covered
    /// by a typed part API. Returns `(uri, relationship_id)`.
    pub fn add_extended_part(
        &mut self,
        uri: &str,
        content_type_str: &str,
        relationship_type: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let part_uri = PackUri::new(if uri.starts_with('/') {
            uri.to_string()
        } else {
            format!("/{uri}")
        });
        self.package
            .set_part(part_uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &wb_uri,
            relationship_type,
            &part_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((part_uri, rid))
    }

    fn add_chart_part(
        &mut self,
        chart_root: &crate::element::OpenXmlElement,
    ) -> Result<(PackUri, String)> {
        let wb_uri = self.ensure_workbook()?;
        let mut index = 1u32;
        let chart_uri = loop {
            let candidate = PackUri::new(format!("/xl/charts/chart{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let xml = write_element(chart_root)?;
        self.package.set_part(
            chart_uri.clone(),
            content_type::DRAWINGML_CHART,
            xml,
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::CHART,
            &chart_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((chart_uri, rid))
    }

    /// Add a bar chart and anchor it on a worksheet via a drawings part.
    ///
    /// Cell coordinates are **0-based** (as in the OOXML drawing schema).
    /// Creates the sheet if needed. Returns `(chart_uri, drawing_uri)`.
    pub fn add_bar_chart_on_sheet(
        &mut self,
        sheet_name: &str,
        title: &str,
        categories: &[&str],
        values: &[f64],
        from_col: u32,
        from_row: u32,
        to_col: u32,
        to_row: u32,
    ) -> Result<(PackUri, PackUri)> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        // Chart part
        let mut index = 1u32;
        let chart_uri = loop {
            let candidate = PackUri::new(format!("/xl/charts/chart{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let chart_xml = write_element(&bar_chart_space(title, categories, values))?;
        self.package.set_part(
            chart_uri.clone(),
            content_type::DRAWINGML_CHART,
            chart_xml,
        );

        // Drawing part
        let mut dindex = 1u32;
        let drawing_uri = loop {
            let candidate = PackUri::new(format!("/xl/drawings/drawing{dindex}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            dindex += 1;
        };

        // Placeholder drawing so we can create the chart relationship from it
        self.package.set_part(
            drawing_uri.clone(),
            content_type::SPREADSHEET_DRAWING,
            b"<?xml version=\"1.0\"?><xdr:wsDr xmlns:xdr=\"http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing\"/>".to_vec(),
        );

        let chart_rel = self.package.add_part_relationship(
            &drawing_uri,
            rel::CHART,
            &chart_uri,
            RelationshipTargetMode::Internal,
        );

        let anchor = two_cell_anchor_chart(
            from_col,
            from_row,
            to_col,
            to_row,
            &chart_rel,
            title,
        );
        let drawing_xml = write_element(&worksheet_drawing(vec![anchor]))?;
        self.package.set_part(
            drawing_uri.clone(),
            content_type::SPREADSHEET_DRAWING,
            drawing_xml,
        );

        // Worksheet → drawing relationship
        let drawing_rel = self.package.add_part_relationship(
            &sheet_uri,
            rel::DRAWING,
            &drawing_uri,
            RelationshipTargetMode::Internal,
        );

        // Ensure worksheet has <drawing r:id="…"/>
        let mut root = if let Some(data) = self.package.opc().get_part(&sheet_uri) {
            parse_element(data)?
        } else {
            worksheet(vec![sheet_data(Vec::<crate::element::OpenXmlElement>::new())])
        };
        // Remove existing drawing refs (we replace with our single drawing for simplicity)
        root.children.retain(|c| c.local_name != "drawing");
        root.append_child(worksheet_drawing_ref(&drawing_rel));
        let sheet_xml = write_element(&root)?;
        self.package.set_part(
            sheet_uri,
            content_type::SPREADSHEET_WORKSHEET,
            sheet_xml,
        );

        Ok((chart_uri, drawing_uri))
    }

    /// Embed a PNG/JPEG/etc. image on a worksheet using a one-cell anchor.
    ///
    /// `cx`/`cy` are extents in EMUs. Columns/rows are **0-based**.
    /// Returns `(image_uri, drawing_uri)`.
    pub fn add_image_on_sheet(
        &mut self,
        sheet_name: &str,
        image_bytes: &[u8],
        content_type: &str,
        extension: &str,
        from_col: u32,
        from_row: u32,
        cx: i64,
        cy: i64,
        name: &str,
    ) -> Result<(PackUri, PackUri)> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        // Image media part
        let mut index = 1u32;
        let image_uri = loop {
            let candidate = PackUri::new(format!("/xl/media/image{index}.{extension}"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_content_type_default(extension, content_type);
        self.package
            .set_part(image_uri.clone(), content_type, image_bytes.to_vec());

        // Drawing part
        let mut dindex = 1u32;
        let drawing_uri = loop {
            let candidate = PackUri::new(format!("/xl/drawings/drawing{dindex}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            dindex += 1;
        };
        self.package.set_part(
            drawing_uri.clone(),
            content_type::SPREADSHEET_DRAWING,
            b"<?xml version=\"1.0\"?><xdr:wsDr xmlns:xdr=\"http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing\"/>".to_vec(),
        );

        let image_rel = self.package.add_part_relationship(
            &drawing_uri,
            rel::IMAGE,
            &image_uri,
            RelationshipTargetMode::Internal,
        );

        let anchor =
            one_cell_anchor_picture(from_col, from_row, cx, cy, &image_rel, name);
        let drawing_xml = write_element(&worksheet_drawing(vec![anchor]))?;
        self.package.set_part(
            drawing_uri.clone(),
            content_type::SPREADSHEET_DRAWING,
            drawing_xml,
        );

        let drawing_rel = self.package.add_part_relationship(
            &sheet_uri,
            rel::DRAWING,
            &drawing_uri,
            RelationshipTargetMode::Internal,
        );

        let mut root = if let Some(data) = self.package.opc().get_part(&sheet_uri) {
            parse_element(data)?
        } else {
            worksheet(vec![sheet_data(Vec::<crate::element::OpenXmlElement>::new())])
        };
        // Append drawing ref if not present
        if root.child("drawing").is_none() {
            root.append_child(worksheet_drawing_ref(&drawing_rel));
        }
        let sheet_xml = write_element(&root)?;
        self.package.set_part(
            sheet_uri,
            content_type::SPREADSHEET_WORKSHEET,
            sheet_xml,
        );

        Ok((image_uri, drawing_uri))
    }

    /// Add a comments part for a worksheet with a single author and cell notes.
    ///

    /// Whether any classic comments parts exist under `/xl/comments`.
    pub fn has_comments(&self) -> bool {
        self.comments_part_count() > 0
    }

    /// List classic comments part URIs.
    pub fn list_comment_parts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/comments"))
            
            .collect()
    }


    /// List classic comments across all `/xl/comments*.xml` parts as
    /// `(part_uri, cell_ref, author, text)`.
    pub fn list_comments(&self) -> Result<Vec<(String, String, String, String)>> {
        let mut out = Vec::new();
        for uri in self.list_comment_parts() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            let authors: Vec<String> = root
                .child("authors")
                .map(|a| {
                    a.children_by_name("author")
                        .filter_map(|e| e.text.clone())
                        .collect()
                })
                .unwrap_or_default();
            let Some(list) = root.child("commentList") else {
                continue;
            };
            for c in list.children_by_name("comment") {
                let cell = c
                    .get_attribute("ref")
                    .unwrap_or("")
                    .to_string();
                let author_id = c
                    .get_attribute("authorId")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0);
                let author = authors
                    .get(author_id)
                    .cloned()
                    .unwrap_or_default();
                let text = c
                    .descendants()
                    .filter(|e| e.local_name == "t")
                    .filter_map(|e| e.text.as_deref())
                    .collect::<Vec<_>>()
                    .join("");
                out.push((uri.to_string(), cell, author, text));
            }
        }
        Ok(out)
    }

    /// Total classic comment entries across all comments parts.
    pub fn comment_count(&self) -> Result<usize> {
        Ok(self.list_comments()?.len())
    }

    /// Remove all classic comments parts and comments relationships from sheets.
    pub fn clear_comments(&mut self) -> Result<usize> {
        let uris = self.list_comment_parts();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        // Drop comments relationships from worksheets
        let sheet_uris: Vec<PackUri> = self.sheets.iter().map(|s| s.uri.clone()).collect();
        for sheet_uri in sheet_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&sheet_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type.contains("/comments"))
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                self.package
                    .delete_reference_relationships(Some(&sheet_uri), &ids);
            }
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// `notes` is a list of `(cell_ref, text)` e.g. `[("B2", "Check this")]`.
    /// Returns the comments part URI.
    pub fn add_sheet_comments(
        &mut self,
        sheet_name: &str,
        author: &str,
        notes: &[(&str, &str)],
    ) -> Result<PackUri> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        // commentsN.xml next to worksheets: /xl/worksheets/commentsN.xml is common;
        // SDK uses Paths General ".." so /xl/commentsN.xml also works. Use /xl/comments{N}.xml.
        let mut index = 1u32;
        let comments_uri = loop {
            let candidate = PackUri::new(format!("/xl/comments{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };

        let xml = write_element(&comments_for_author(author, notes))?;
        self.package.set_part(
            comments_uri.clone(),
            content_type::SPREADSHEET_COMMENTS,
            xml,
        );

        // Avoid duplicate comments relationships
        let has = self
            .package
            .opc()
            .part_relationships(&sheet_uri)
            .map(|rels| {
                rels.iter()
                    .any(|r| r.relationship_type.contains("/comments"))
            })
            .unwrap_or(false);
        if !has {
            self.package.add_part_relationship(
                &sheet_uri,
                rel::COMMENTS,
                &comments_uri,
                RelationshipTargetMode::Internal,
            );
        }

        // Optional VML drawing so legacy Excel shows comment indicators
        let coords: Vec<(u32, u32)> = notes
            .iter()
            .filter_map(|(cell, _)| cell_ref_to_row_col(cell))
            .collect();
        if !coords.is_empty() {
            let mut vindex = 1u32;
            let vml_uri = loop {
                let candidate = PackUri::new(format!("/xl/drawings/vmlDrawing{vindex}.vml"));
                if !self.package.opc().has_part(&candidate) {
                    break candidate;
                }
                vindex += 1;
            };
            let vml_xml = write_element(&vml_comments_drawing(&coords))?;
            self.package.set_part(
                vml_uri.clone(),
                content_type::VML_DRAWING,
                vml_xml,
            );
            let has_vml = self
                .package
                .opc()
                .part_relationships(&sheet_uri)
                .map(|rels| {
                    rels.iter()
                        .any(|r| r.relationship_type.contains("vmlDrawing"))
                })
                .unwrap_or(false);
            if !has_vml {
                self.package.add_part_relationship(
                    &sheet_uri,
                    rel::VML_DRAWING,
                    &vml_uri,
                    RelationshipTargetMode::Internal,
                );
            }
        }

        Ok(comments_uri)
    }

    /// Read comments from a sheet's comments part (if present).
    ///
    /// Returns `(cell_ref, author, text)` triples.
    pub fn sheet_comments(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String, String)>> {
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        let rel = self
            .package
            .opc()
            .part_relationships(&sheet_uri)
            .and_then(|rels| {
                rels.iter()
                    .find(|r| r.relationship_type.contains("/comments"))
                    .cloned()
            });
        let Some(rel) = rel else {
            return Ok(Vec::new());
        };
        let comments_uri = self
            .package
            .opc()
            .resolve_relationship(Some(&sheet_uri), &rel)?;
        let data = self
            .package
            .opc()
            .get_part(&comments_uri)
            .ok_or_else(|| Error::PartNotFound(comments_uri.to_string()))?;
        let root = parse_element(data)?;

        let authors: Vec<String> = root
            .child("authors")
            .map(|a| {
                a.children_by_name("author")
                    .map(|el| el.inner_text())
                    .collect()
            })
            .unwrap_or_default();

        let mut out = Vec::new();
        if let Some(list) = root.child("commentList") {
            for c in list.children_by_name("comment") {
                let cell_ref = c.get_attribute("ref").unwrap_or("").to_string();
                let author_id: usize = c
                    .get_attribute("authorId")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let author = authors
                    .get(author_id)
                    .cloned()
                    .unwrap_or_default();
                let text = c
                    .child("text")
                    .map(|t| t.inner_text())
                    .unwrap_or_default();
                out.push((cell_ref, author, text));
            }
        }
        Ok(out)
    }

    /// Update the text of a sheet comment at `cell_ref`. Returns whether found.
    pub fn set_sheet_comment_text(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
        text: &str,
    ) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let rel = self
            .package
            .opc()
            .part_relationships(&sheet_uri)
            .and_then(|rels| {
                rels.iter()
                    .find(|r| r.relationship_type.contains("/comments"))
                    .cloned()
            });
        let Some(rel) = rel else {
            return Ok(false);
        };
        let comments_uri = self
            .package
            .opc()
            .resolve_relationship(Some(&sheet_uri), &rel)?;
        let Some(data) = self.package.opc().get_part(&comments_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        if let Some(list) = root.child_mut("commentList") {
            for c in list.children.iter_mut().filter(|c| c.local_name == "comment") {
                if c.get_attribute("ref") != Some(cell_ref) {
                    continue;
                }
                found = true;
                if let Some(t) = c.child_mut("text") {
                    // replace all t descendants text
                    fn set_t(el: &mut OpenXmlElement, text: &str) -> bool {
                        if el.local_name == "t" {
                            el.set_text(text);
                            return true;
                        }
                        for ch in el.children.iter_mut() {
                            if set_t(ch, text) {
                                return true;
                            }
                        }
                        false
                    }
                    if !set_t(t, text) {
                        // fallback: replace children with single t
                        t.children.clear();
                        t.append_child(
                            OpenXmlElement::new(
                                "x",
                                crate::namespace::ns::SPREADSHEETML.uri,
                                "t",
                            )
                            .with_text(text),
                        );
                    }
                }
                break;
            }
        }
        if found {
            self.package.set_part(
                comments_uri,
                content_type::SPREADSHEET_COMMENTS,
                write_element(&root)?,
            );
        }
        Ok(found)
    }

    /// Clear comment text for a cell (sets empty text).
    pub fn clear_sheet_comment_text(
        &mut self,
        sheet_name: &str,
        cell_ref: &str,
    ) -> Result<bool> {
        self.set_sheet_comment_text(sheet_name, cell_ref, "")
    }

    /// Remove a single sheet comment by cell ref. Returns whether found.
    pub fn remove_sheet_comment(&mut self, sheet_name: &str, cell_ref: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let rel = self
            .package
            .opc()
            .part_relationships(&sheet_uri)
            .and_then(|rels| {
                rels.iter()
                    .find(|r| r.relationship_type.contains("/comments"))
                    .cloned()
            });
        let Some(rel) = rel else {
            return Ok(false);
        };
        let comments_uri = self
            .package
            .opc()
            .resolve_relationship(Some(&sheet_uri), &rel)?;
        let Some(data) = self.package.opc().get_part(&comments_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(list) = root.child_mut("commentList") else {
            return Ok(false);
        };
        let before = list.children.len();
        list.children.retain(|c| {
            !(c.local_name == "comment" && c.get_attribute("ref") == Some(cell_ref))
        });
        let removed = list.children.len() < before;
        if removed {
            if list.children.is_empty() {
                // clear whole comments part
                return self.clear_sheet_comments(sheet_name);
            }
            self.package.set_part(
                comments_uri,
                content_type::SPREADSHEET_COMMENTS,
                write_element(&root)?,
            );
        }
        Ok(removed)
    }

    /// Remove comments (and related VML drawing) from a sheet. Returns whether comments existed.
    pub fn clear_sheet_comments(&mut self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let targets: Vec<(String, PackUri)> = {
            let Some(rels) = self.package.opc().part_relationships(&sheet_uri) else {
                return Ok(false);
            };
            let mut out = Vec::new();
            for r in rels.iter() {
                if r.relationship_type.contains("/comments")
                    || r.relationship_type.contains("vmlDrawing")
                {
                    if let Ok(uri) = self
                        .package
                        .opc()
                        .resolve_relationship(Some(&sheet_uri), r)
                    {
                        out.push((r.id.clone(), uri));
                    }
                }
            }
            out
        };
        if targets.is_empty() {
            return Ok(false);
        }
        for (id, uri) in targets {
            self.package.delete_part(&uri);
            let _ = self.package.delete_reference_relationship(Some(&sheet_uri), &id);
        }
        Ok(true)
    }

    /// Add a cell-is conditional formatting rule on a sheet.
    ///
    /// Example: highlight values greater than 50 in `B2:B100` with a red fill dxf.
    /// Ensures a stylesheet with a matching `dxfs` entry exists.
    /// Clear comments on every sheet. Returns sheets modified.
    pub fn clear_all_sheet_comments(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_sheet_comments(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn add_conditional_formatting_cell_is(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        operator: &str,
        formula: &str,
        fill_rgb: &str,
        priority: u32,
    ) -> Result<()> {
        // Ensure styles with dxf
        let dxf_id = self.ensure_dxf_fill(fill_rgb)?;

        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        let mut root = if let Some(data) = self.package.opc().get_part(&sheet_uri) {
            parse_element(data)?
        } else {
            worksheet(vec![sheet_data(Vec::<crate::element::OpenXmlElement>::new())])
        };

        let rule = cf_rule_cell_is(operator, priority, Some(dxf_id), &[formula]);
        let cf = conditional_formatting(sqref, vec![rule]);
        // Insert after sheetData
        if let Some(pos) = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
        {
            root.children.insert(pos + 1, cf);
        } else {
            root.append_child(cf);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(sheet_uri, content_type::SPREADSHEET_WORKSHEET, xml);
        Ok(())
    }

    /// Whether the sheet has any conditional formatting rules.
    pub fn has_conditional_formatting(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root.child("conditionalFormatting").is_some()
            || root
                .children
                .iter()
                .any(|c| c.local_name == "conditionalFormatting"))
    }

    /// Count `conditionalFormatting` containers on a sheet.
    /// Sheet names that have conditional formatting.
    pub fn sheets_with_conditional_formatting(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_conditional_formatting(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has conditional formatting.
    pub fn has_sheets_with_conditional_formatting(&self) -> Result<bool> {
        Ok(!self.sheets_with_conditional_formatting()?.is_empty())
    }

    pub fn conditional_formatting_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_conditional_formatting(sheet_name)?.len())
    }

    /// List conditional formatting as `(sqref, rule_type, formula)` triples (one per rule).
    pub fn list_conditional_formatting(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let mut out = Vec::new();
        for cf in root
            .children
            .iter()
            .filter(|c| c.local_name == "conditionalFormatting")
        {
            let sqref = cf.get_attribute("sqref").unwrap_or("").to_string();
            for rule in cf.children_by_name("cfRule") {
                let ty = rule.get_attribute("type").unwrap_or("").to_string();
                let formula = rule
                    .child("formula")
                    .map(|f| f.inner_text())
                    .unwrap_or_default();
                out.push((sqref.clone(), ty, formula));
            }
        }
        Ok(out)
    }

    /// Remove all conditional formatting from a sheet. Returns how many containers were removed.
    pub fn clear_conditional_formatting(&mut self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children
            .retain(|c| c.local_name != "conditionalFormatting");
        let removed = before - root.children.len();
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Remove conditionalFormatting containers matching `sqref`. Returns count removed.
    /// Clear conditional formatting on every sheet. Returns sheets modified.
    pub fn clear_all_conditional_formatting(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_conditional_formatting(&name)? > 0 {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn remove_conditional_formatting_sqref(
        &mut self,
        sheet_name: &str,
        sqref: &str,
    ) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let before = root.children.len();
        root.children.retain(|c| {
            !(c.local_name == "conditionalFormatting" && c.get_attribute("sqref") == Some(sqref))
        });
        let removed = before - root.children.len();
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Update attributes on all `cfRule` elements under a matching `sqref` container.
    pub fn set_cf_rule_attrs(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        priority: Option<u32>,
        stop_if_true: Option<bool>,
        operator: Option<&str>,
        dxf_id: Option<u32>,
    ) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut n = 0;
        for cf in root
            .children
            .iter_mut()
            .filter(|c| c.local_name == "conditionalFormatting")
        {
            if cf.get_attribute("sqref").unwrap_or("") != sqref {
                continue;
            }
            for rule in cf
                .children
                .iter_mut()
                .filter(|c| c.local_name == "cfRule")
            {
                n += 1;
                if let Some(p) = priority {
                    rule.set_attribute("priority", p.to_string());
                }
                if let Some(s) = stop_if_true {
                    rule.set_attribute("stopIfTrue", if s { "1" } else { "0" });
                }
                if let Some(op) = operator {
                    rule.set_attribute("operator", op);
                }
                if let Some(d) = dxf_id {
                    rule.set_attribute("dxfId", d.to_string());
                }
            }
        }
        if n > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(n)
    }

    /// List detailed CF rules as `(sqref, type, operator?, formula, priority?, stop_if_true)`.
    pub fn list_cf_rules(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String, Option<String>, String, Option<u32>, bool)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let Some(data) = self.package.opc().get_part(&sheet_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let mut out = Vec::new();
        for cf in root
            .children
            .iter()
            .filter(|c| c.local_name == "conditionalFormatting")
        {
            let sqref = cf.get_attribute("sqref").unwrap_or("").to_string();
            for rule in cf.children_by_name("cfRule") {
                let ty = rule.get_attribute("type").unwrap_or("").to_string();
                let op = rule.get_attribute("operator").map(|s| s.to_string());
                let formula = rule
                    .child("formula")
                    .map(|f| f.inner_text())
                    .unwrap_or_default();
                let priority = rule
                    .get_attribute("priority")
                    .and_then(|s| s.parse().ok());
                let stop = rule
                    .get_attribute("stopIfTrue")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                out.push((sqref.clone(), ty, op, formula, priority, stop));
            }
        }
        Ok(out)
    }

    /// Add a 3-color scale on a range (min / midpoint / max).
    /// Whether a sheet has any conditional formatting rules (via `list_cf_rules`).
    pub fn has_cf_rules(&self, sheet_name: &str) -> Result<bool> {
        Ok(!self.list_cf_rules(sheet_name)?.is_empty())
    }

    /// Count conditional formatting rules on a sheet.
    pub fn cf_rule_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_cf_rules(sheet_name)?.len())
    }

    /// Remove `cfRule` elements matching optional `rule_type` and/or `priority`
    /// under a given `sqref`. Returns how many rules were removed.
    ///
    /// Empty conditionalFormatting containers are dropped after rule removal.
    pub fn remove_cf_rule(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        rule_type: Option<&str>,
        priority: Option<u32>,
    ) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = 0usize;
        let mut empty_containers = Vec::new();
        for (i, cf) in root.children.iter_mut().enumerate() {
            if cf.local_name != "conditionalFormatting" {
                continue;
            }
            if cf.get_attribute("sqref").unwrap_or("") != sqref {
                continue;
            }
            let before = cf.children.len();
            cf.children.retain(|c| {
                if c.local_name != "cfRule" {
                    return true;
                }
                if let Some(t) = rule_type {
                    if c.get_attribute("type") != Some(t) {
                        return true;
                    }
                }
                if let Some(p) = priority {
                    let cp = c
                        .get_attribute("priority")
                        .and_then(|s| s.parse::<u32>().ok());
                    if cp != Some(p) {
                        return true;
                    }
                }
                false
            });
            removed += before - cf.children.len();
            if !cf.children.iter().any(|c| c.local_name == "cfRule") {
                empty_containers.push(i);
            }
        }
        // drop empty CF containers from the end
        for i in empty_containers.into_iter().rev() {
            if root.children.get(i).map(|c| c.local_name.as_str()) == Some("conditionalFormatting")
                && !root.children[i]
                    .children
                    .iter()
                    .any(|c| c.local_name == "cfRule")
            {
                root.children.remove(i);
            }
        }
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }

    /// Remove all CF rules of a given type (e.g. `"dataBar"`, `"iconSet"`, `"colorScale"`, `"cellIs"`).
    /// Returns how many rules were removed across the sheet.
    pub fn remove_cf_rules_by_type(&mut self, sheet_name: &str, rule_type: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut removed = 0usize;
        let mut empty = Vec::new();
        for (i, cf) in root.children.iter_mut().enumerate() {
            if cf.local_name != "conditionalFormatting" {
                continue;
            }
            let before = cf.children.len();
            cf.children.retain(|c| {
                !(c.local_name == "cfRule" && c.get_attribute("type") == Some(rule_type))
            });
            removed += before - cf.children.len();
            if !cf.children.iter().any(|c| c.local_name == "cfRule") {
                empty.push(i);
            }
        }
        for i in empty.into_iter().rev() {
            if root.children.get(i).map(|c| c.local_name.as_str()) == Some("conditionalFormatting")
                && !root.children[i]
                    .children
                    .iter()
                    .any(|c| c.local_name == "cfRule")
            {
                root.children.remove(i);
            }
        }
        if removed > 0 {
            self.save_sheet_root(&sheet_uri, &root)?;
        }
        Ok(removed)
    }


    /// Clear all cfRule elements on a sheet (drops empty containers). Alias depth for `has_cf_rules`.
    pub fn clear_cf_rules(&mut self, sheet_name: &str) -> Result<usize> {
        self.clear_conditional_formatting(sheet_name)
    }

    pub fn add_conditional_formatting_color_scale(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        priority: u32,
    ) -> Result<()> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        let mut root = if let Some(data) = self.package.opc().get_part(&sheet_uri) {
            parse_element(data)?
        } else {
            worksheet(vec![sheet_data(Vec::<crate::element::OpenXmlElement>::new())])
        };

        let rule = cf_rule_color_scale(
            priority,
            &[
                ("min", None, "FFF8696B"),
                ("percentile", Some("50"), "FFFFEB84"),
                ("max", None, "FF63BE7B"),
            ],
        );
        let cf = conditional_formatting(sqref, vec![rule]);
        if let Some(pos) = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
        {
            root.children.insert(pos + 1, cf);
        } else {
            root.append_child(cf);
        }
        let xml = write_element(&root)?;
        self.package
            .set_part(sheet_uri, content_type::SPREADSHEET_WORKSHEET, xml);
        Ok(())
    }

    /// Add a data-bar conditional formatting rule on a range.
    pub fn add_conditional_formatting_data_bar(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        rgb: &str,
        priority: u32,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let rule = cf_rule_data_bar(priority, rgb);
        let cf = conditional_formatting(sqref, vec![rule]);
        if let Some(pos) = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
        {
            root.children.insert(pos + 1, cf);
        } else {
            root.append_child(cf);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add an icon-set conditional formatting rule on a range.
    /// Whether any dataBar CF rules exist on a sheet.
    pub fn has_data_bars(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .list_cf_rules(sheet_name)?
            .iter()
            .any(|(_, ty, _, _, _, _)| ty == "dataBar"))
    }

    /// Whether any iconSet CF rules exist on a sheet.
    pub fn has_icon_sets(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .list_cf_rules(sheet_name)?
            .iter()
            .any(|(_, ty, _, _, _, _)| ty == "iconSet"))
    }

    /// Whether any colorScale CF rules exist on a sheet.
    pub fn has_color_scales(&self, sheet_name: &str) -> Result<bool> {
        Ok(self
            .list_cf_rules(sheet_name)?
            .iter()
            .any(|(_, ty, _, _, _, _)| ty == "colorScale"))
    }


    pub fn add_conditional_formatting_icon_set(
        &mut self,
        sheet_name: &str,
        sqref: &str,
        icon_set: &str,
        priority: u32,
    ) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let rule = cf_rule_icon_set(priority, icon_set);
        let cf = conditional_formatting(sqref, vec![rule]);
        if let Some(pos) = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
        {
            root.children.insert(pos + 1, cf);
        } else {
            root.append_child(cf);
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Ensure stylesheet has a dxf with solid fill `rgb`; returns its dxfId.
    fn ensure_dxf_fill(&mut self, rgb: &str) -> Result<u32> {
        let styles_uri = PackUri::new("/xl/styles.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&styles_uri) {
            parse_element(data)?
        } else {
            // Create minimal styles first
            self.add_minimal_styles(false)?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&styles_uri)
                    .ok_or_else(|| Error::PartNotFound(styles_uri.to_string()))?,
            )?
        };

        // Find or create dxfs
        let dxf_id = if let Some(dxfs_el) = root.child("dxfs") {
            dxfs_el.children_by_name("dxf").count() as u32
        } else {
            0
        };

        if let Some(dxfs_el) = root.child_mut("dxfs") {
            dxfs_el.append_child(dxf_fill(rgb));
            let count = dxfs_el.children_by_name("dxf").count();
            dxfs_el.set_attribute("count", count.to_string());
        } else {
            root.append_child(dxfs(vec![dxf_fill(rgb)]));
        }

        let xml = write_element(&root)?;
        self.package.set_part(
            styles_uri.clone(),
            content_type::SPREADSHEET_STYLES,
            xml,
        );
        // Ensure workbook → styles relationship exists (without rewriting content)
        let wb_uri = self.ensure_workbook()?;
        let has = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| rels.get_by_type(rel::STYLES))
            .is_some();
        if !has {
            self.package.add_part_relationship(
                &wb_uri,
                rel::STYLES,
                &styles_uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok(dxf_id)
    }

    /// Add a minimal pivot table over a source range.
    ///
    /// Creates pivot cache definition/records and a pivot table definition on
    /// `target_sheet` at `location_ref` (e.g. `"E3"`). Field names must match
    /// the header row of `source_ref` on `source_sheet`.
    ///
    /// Returns `(pivot_table_uri, cache_definition_uri)`.
    pub fn add_pivot_table(
        &mut self,
        source_sheet: &str,
        source_ref: &str,
        target_sheet: &str,
        location_ref: &str,
        field_names: &[&str],
        row_field: u32,
        data_field: u32,
        record_count: u32,
    ) -> Result<(PackUri, PackUri)> {
        self.add_pivot_table_with_rows(
            source_sheet,
            source_ref,
            target_sheet,
            location_ref,
            field_names,
            row_field,
            data_field,
            record_count,
            None,
        )
    }

    /// Like [`add_pivot_table`] but optionally embeds real cache record rows.
    ///
    /// `rows` is data without the header (each inner slice matches `field_names` length).
    pub fn add_pivot_table_with_rows(
        &mut self,
        source_sheet: &str,
        source_ref: &str,
        target_sheet: &str,
        location_ref: &str,
        field_names: &[&str],
        row_field: u32,
        data_field: u32,
        record_count: u32,
        rows: Option<&[Vec<&str>]>,
    ) -> Result<(PackUri, PackUri)> {
        if field_names.is_empty() {
            return Err(Error::Package("pivot table requires field names".into()));
        }
        if row_field as usize >= field_names.len() || data_field as usize >= field_names.len() {
            return Err(Error::Package("pivot field index out of range".into()));
        }

        // Ensure sheets exist
        if !self.sheets.iter().any(|s| s.name == source_sheet) {
            self.add_worksheet(source_sheet)?;
        }
        if !self.sheets.iter().any(|s| s.name == target_sheet) {
            self.add_worksheet(target_sheet)?;
        }
        let target_uri = self
            .sheets
            .iter()
            .find(|s| s.name == target_sheet)
            .map(|s| s.uri.clone())
            .unwrap();

        let wb_uri = self.ensure_workbook()?;

        // Cache definition + records
        let mut cindex = 1u32;
        let cache_def_uri = loop {
            let c = PackUri::new(format!(
                "/xl/pivotCache/pivotCacheDefinition{cindex}.xml"
            ));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            cindex += 1;
        };
        let cache_rec_uri = PackUri::new(format!(
            "/xl/pivotCache/pivotCacheRecords{cindex}.xml"
        ));

        // Placeholder cache def to create relationship
        self.package.set_part(
            cache_def_uri.clone(),
            content_type::SPREADSHEET_PIVOT_CACHE_DEFINITION,
            b"<?xml version=\"1.0\"?><x:pivotCacheDefinition xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"/>".to_vec(),
        );
        let records_el = if let Some(rows) = rows {
            pivot_cache_records_from_rows(rows)
        } else {
            pivot_cache_records(record_count)
        };
        let actual_count = if let Some(rows) = rows {
            rows.len() as u32
        } else {
            record_count
        };
        self.package.set_part(
            cache_rec_uri.clone(),
            content_type::SPREADSHEET_PIVOT_CACHE_RECORDS,
            write_element(&records_el)?,
        );
        let records_rel = self.package.add_part_relationship(
            &cache_def_uri,
            rel::PIVOT_CACHE_RECORDS,
            &cache_rec_uri,
            RelationshipTargetMode::Internal,
        );
        let cache_def_xml = write_element(&pivot_cache_definition(
            &records_rel,
            source_sheet,
            source_ref,
            field_names,
            actual_count,
        ))?;
        self.package.set_part(
            cache_def_uri.clone(),
            content_type::SPREADSHEET_PIVOT_CACHE_DEFINITION,
            cache_def_xml,
        );

        // Workbook → cache definition
        let cache_id = cindex;
        let cache_rel = self.package.add_part_relationship(
            &wb_uri,
            rel::PIVOT_CACHE_DEFINITION,
            &cache_def_uri,
            RelationshipTargetMode::Internal,
        );

        // Ensure workbook has pivotCaches
        {
            let mut wb_root = parse_element(
                self.package
                    .opc()
                    .get_part(&wb_uri)
                    .ok_or_else(|| Error::PartNotFound(wb_uri.to_string()))?,
            )?;
            // Remove existing pivotCaches and rebuild with all caches we know
            // For simplicity append a single-entry pivotCaches (or merge)
            let entry = workbook_pivot_cache(cache_id, &cache_rel);
            if let Some(pc) = wb_root.child_mut("pivotCaches") {
                pc.append_child(entry);
            } else {
                // Insert before calcPr/extLst if present, else append
                wb_root.append_child(workbook_pivot_caches(vec![entry]));
            }
            let xml = write_element(&wb_root)?;
            self.package.set_part(
                wb_uri,
                self.document_type.content_type(),
                xml,
            );
        }

        // Pivot table part
        let mut tindex = 1u32;
        let pivot_uri = loop {
            let c = PackUri::new(format!("/xl/pivotTables/pivotTable{tindex}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            tindex += 1;
        };
        let pivot_xml = write_element(&pivot_table_definition(
            &format!("PivotTable{tindex}"),
            cache_id,
            location_ref,
            field_names,
            row_field,
            data_field,
        ))?;
        self.package.set_part(
            pivot_uri.clone(),
            content_type::SPREADSHEET_PIVOT_TABLE,
            pivot_xml,
        );

        // Target sheet → pivot table
        self.package.add_part_relationship(
            &target_uri,
            rel::PIVOT_TABLE,
            &pivot_uri,
            RelationshipTargetMode::Internal,
        );

        // Pivot table → cache definition
        self.package.add_part_relationship(
            &pivot_uri,
            rel::PIVOT_CACHE_DEFINITION,
            &cache_def_uri,
            RelationshipTargetMode::Internal,
        );

        Ok((pivot_uri, cache_def_uri))
    }

    /// Set merged cell ranges on a sheet.
    ///
    /// Each entry is an A1-style range like `"A1:B2"`. Replaces any existing `mergeCells`.
    pub fn set_merge_cells(
        &mut self,
        sheet_name: &str,
        ranges: &[&str],
    ) -> Result<()> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;

        let mut root = if let Some(data) = self.package.opc().get_part(&sheet_uri) {
            parse_element(data)?
        } else {
            worksheet(vec![sheet_data(Vec::<crate::element::OpenXmlElement>::new())])
        };

        root.children.retain(|c| c.local_name != "mergeCells");
        let mc = merge_cells(ranges.iter().copied());
        // OOXML order: mergeCells comes after sheetData
        if let Some(pos) = root
            .children
            .iter()
            .position(|c| c.local_name == "sheetData")
        {
            root.children.insert(pos + 1, mc);
        } else {
            root.append_child(mc);
        }

        let xml = write_element(&root)?;
        self.package
            .set_part(sheet_uri, content_type::SPREADSHEET_WORKSHEET, xml);
        Ok(())
    }

    /// Read merge cell refs from a sheet (e.g. `["A1:B2"]`).
    pub fn merge_cells(&self, sheet_name: &str) -> Result<Vec<String>> {
        let sheet_uri = self
            .sheets
            .iter()
            .find(|s| s.name == sheet_name)
            .map(|s| s.uri.clone())
            .ok_or_else(|| Error::Package(format!("sheet `{sheet_name}` not found")))?;
        let data = self
            .package
            .opc()
            .get_part(&sheet_uri)
            .ok_or_else(|| Error::PartNotFound(sheet_uri.to_string()))?;
        let root = parse_element(data)?;
        let Some(mc) = root.child("mergeCells") else {
            return Ok(Vec::new());
        };
        Ok(mc
            .children_by_name("mergeCell")
            .filter_map(|c| c.get_attribute("ref").map(|s| s.to_string()))
            .collect())
    }


    /// Alias for [`merge_cells`](Self::merge_cells).
    pub fn list_merged_cells(&self, sheet_name: &str) -> Result<Vec<String>> {
        self.merge_cells(sheet_name)
    }

    /// Whether the sheet has any merged cell ranges.
    pub fn has_merged_cells(&self, sheet_name: &str) -> Result<bool> {
        Ok(!self.merge_cells(sheet_name)?.is_empty())
    }

    /// Clear all merged cell ranges on a sheet.
    /// Sheet names that have merged cells.
    pub fn sheets_with_merged_cells(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_merged_cells(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has merged cells.
    pub fn has_sheets_with_merged_cells(&self) -> Result<bool> {
        Ok(!self.sheets_with_merged_cells()?.is_empty())
    }

    pub fn clear_merged_cells(&mut self, sheet_name: &str) -> Result<bool> {
        let had = self.has_merged_cells(sheet_name)?;
        if had {
            self.set_merge_cells(sheet_name, &[])?;
        }
        Ok(had)
    }


    /// Whether any rows are marked hidden on the sheet.
    /// Clear merged cells on every sheet. Returns sheets modified.
    pub fn clear_all_merged_cells(&mut self) -> Result<usize> {
        let names: Vec<String> = self.sheet_names().into_iter().map(|s| s.to_string()).collect();
        let mut n = 0usize;
        for name in names {
            if self.clear_merged_cells(&name)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn has_hidden_rows(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        let Some(sd) = root.child("sheetData") else {
            return Ok(false);
        };
        let hidden = sd.children_by_name("row").any(|r| {
            r.get_attribute("hidden")
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        });
        Ok(hidden)
    }

    /// Sheet names that have at least one hidden row.
    pub fn sheets_with_hidden_rows(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for name in self.sheet_names() {
            if self.has_hidden_rows(name)? {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }

    /// Whether any sheet has hidden rows.
    pub fn has_sheets_with_hidden_rows(&self) -> Result<bool> {
        Ok(!self.sheets_with_hidden_rows()?.is_empty())
    }

    /// Whether any columns are marked hidden on the sheet.
    pub fn has_hidden_cols(&self, sheet_name: &str) -> Result<bool> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        let Some(cols) = root.child("cols") else {
            return Ok(false);
        };
        let hidden = cols.children_by_name("col").any(|c| {
            c.get_attribute("hidden")
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        });
        Ok(hidden)
    }

    /// Number of merge cell ranges on a sheet.
    pub fn merge_cell_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.merge_cells(sheet_name)?.len())
    }

    /// Whether the sheet has any merge cell ranges.
    pub fn has_merge_cells(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.merge_cell_count(sheet_name)? > 0)
    }

    /// Add a single merge range without replacing existing merges.
    pub fn merge_range(&mut self, sheet_name: &str, range: &str) -> Result<()> {
        let mut existing = self.merge_cells(sheet_name)?;
        if !existing.iter().any(|r| r == range) {
            existing.push(range.to_string());
        }
        let refs: Vec<&str> = existing.iter().map(|s| s.as_str()).collect();
        self.set_merge_cells(sheet_name, &refs)
    }

    /// Remove a single merge range. Returns whether it was present.
    pub fn unmerge_range(&mut self, sheet_name: &str, range: &str) -> Result<bool> {
        let existing = self.merge_cells(sheet_name)?;
        let filtered: Vec<&str> = existing
            .iter()
            .filter(|r| r.as_str() != range)
            .map(|s| s.as_str())
            .collect();
        let removed = filtered.len() < existing.len();
        if removed {
            self.set_merge_cells(sheet_name, &filtered)?;
        }
        Ok(removed)
    }

    /// Whether a specific range is merged.
    pub fn is_merged_range(&self, sheet_name: &str, range: &str) -> Result<bool> {
        Ok(self.merge_cells(sheet_name)?.iter().any(|r| r == range))
    }

    /// Clear all cells (and optional merges) on a sheet, keeping the sheet part.
    pub fn clear_sheet(&mut self, sheet_name: &str, clear_merges: bool) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if let Some(sd) = root.child_mut("sheetData") {
            sd.children.clear();
        }
        if clear_merges {
            root.children.retain(|c| c.local_name != "mergeCells");
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Count non-empty cells on a sheet (cells present in sheetData).
    pub fn cell_count(&self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(0),
        };
        let root = parse_element(data)?;
        Ok(root.descendants().filter(|e| e.local_name == "c").count())
    }

    /// Whether the sheet has no cell elements.
    pub fn is_sheet_empty(&self, sheet_name: &str) -> Result<bool> {
        Ok(self.cell_count(sheet_name)? == 0)
    }

    /// Count data rows (row elements) on a sheet.
    pub fn row_count(&self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(0),
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "row")
            .count())
    }

    /// Estimate column span from cells present (max 0-based column index + 1), or 0 if empty.
    pub fn column_count(&self, sheet_name: &str) -> Result<usize> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(0),
        };
        let root = parse_element(data)?;
        let mut max_col = 0usize;
        for cell in root.descendants().filter(|e| e.local_name == "c") {
            if let Some(r) = cell.get_attribute("r") {
                if let Some((_, c)) = cell_ref_to_row_col(r) {
                    max_col = max_col.max((c + 1) as usize);
                }
            }
        }
        Ok(max_col)
    }

    /// Delete a 0-based column, shifting cells to the left.
    ///
    /// Only rewrites cell `r` attributes; does not update formulas, merges, or col widths.
    pub fn delete_column(&mut self, sheet_name: &str, col_index: u32) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sd) = root.child_mut("sheetData") else {
            return Ok(());
        };
        for row_el in sd.children.iter_mut().filter(|c| c.local_name == "row") {
            row_el.children.retain(|c| {
                if c.local_name != "c" {
                    return true;
                }
                match c.get_attribute("r").and_then(cell_ref_to_row_col) {
                    Some((_, cc)) if cc == col_index => false,
                    _ => true,
                }
            });
            for cell in row_el.children.iter_mut().filter(|c| c.local_name == "c") {
                if let Some(cref) = cell.get_attribute("r") {
                    if let Some((rr, cc)) = cell_ref_to_row_col(cref) {
                        if cc > col_index {
                            let new_c = cc - 1;
                            cell.set_attribute(
                                "r",
                                format!("{}{}", column_name(new_c as usize), rr + 1),
                            );
                        }
                    }
                }
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Insert an empty column at 0-based `col_index`, shifting existing cells right.
    pub fn insert_column(&mut self, sheet_name: &str, col_index: u32) -> Result<()> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let Some(sd) = root.child_mut("sheetData") else {
            return Ok(());
        };
        for row_el in sd.children.iter_mut().filter(|c| c.local_name == "row") {
            for cell in row_el.children.iter_mut().filter(|c| c.local_name == "c") {
                if let Some(cref) = cell.get_attribute("r") {
                    if let Some((rr, cc)) = cell_ref_to_row_col(cref) {
                        if cc >= col_index {
                            let new_c = cc + 1;
                            cell.set_attribute(
                                "r",
                                format!("{}{}", column_name(new_c as usize), rr + 1),
                            );
                        }
                    }
                }
            }
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    /// Add a minimal stylesheet (with optional bold style at index 1).
    ///
    /// Returns the relationship id from the workbook.
    pub fn add_minimal_styles(&mut self, include_bold: bool) -> Result<String> {
        let wb_uri = self.ensure_workbook()?;
        let styles_uri = PackUri::new("/xl/styles.xml");
        let xml = write_element(&minimal_stylesheet(include_bold))?;
        self.package.set_part(
            styles_uri.clone(),
            content_type::SPREADSHEET_STYLES,
            xml,
        );

        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| rels.get_by_type(rel::STYLES).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.package.add_part_relationship(
            &wb_uri,
            rel::STYLES,
            &styles_uri,
            RelationshipTargetMode::Internal,
        ))
    }

    /// Write a stylesheet with a solid fill style (index [`crate::spreadsheet::STYLE_FILL`]).
    pub fn add_styles_with_fill(&mut self, rgb: &str) -> Result<String> {
        self.write_styles_part(&stylesheet_with_fill(rgb))
    }

    /// Write a stylesheet with a named "Title" style (index [`crate::spreadsheet::STYLE_NAMED_TITLE`]).
    pub fn add_styles_with_named_title(&mut self) -> Result<String> {
        self.write_styles_part(&stylesheet_with_named_styles())
    }

    /// Write a stylesheet with a custom number format.
    ///
    /// Returns `(relationship_id, style_index)`.
    pub fn add_styles_with_num_fmt(&mut self, num_fmt_code: &str) -> Result<(String, u32)> {
        let (sheet, style_index) = stylesheet_with_num_fmt(num_fmt_code);
        let rid = self.write_styles_part(&sheet)?;
        Ok((rid, style_index))
    }

    /// Write a stylesheet with a thin-border style (index [`crate::spreadsheet::STYLE_BORDER`]).
    pub fn add_styles_with_border(&mut self) -> Result<String> {
        self.write_styles_part(&stylesheet_with_border())
    }

    /// Set the `s` (style index) attribute on a cell.
    ///
    /// Creates the sheet/row/cell if needed. The stylesheet must already define
    /// the given style index (e.g. via [`add_styles_with_border`](Self::add_styles_with_border)).
    pub fn set_cell_style(
        &mut self,
        sheet_name: &str,
        reference: &str,
        style_index: u32,
    ) -> Result<()> {
        if !self.sheets.iter().any(|s| s.name == sheet_name) {
            self.add_worksheet(sheet_name)?;
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        if root.child("sheetData").is_none() {
            root.append_child(sheet_data(Vec::<crate::element::OpenXmlElement>::new()));
        }
        let row_num: u32 = reference
            .chars()
            .skip_while(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .parse()
            .map_err(|_| Error::Package(format!("bad cell ref `{reference}`")))?;
        let sheet_data_el = root
            .child_mut("sheetData")
            .ok_or_else(|| Error::Package("no sheetData".into()))?;
        // Find or create row
        let row_pos = sheet_data_el
            .children
            .iter()
            .position(|c| c.local_name == "row" && c.get_attribute("r") == Some(&row_num.to_string()));
        if row_pos.is_none() {
            sheet_data_el.append_child(
                OpenXmlElement::new("x", crate::namespace::ns::SPREADSHEETML.uri, "row")
                    .with_attribute("r", row_num.to_string()),
            );
        }
        let row_el = sheet_data_el
            .children
            .iter_mut()
            .find(|c| c.local_name == "row" && c.get_attribute("r") == Some(&row_num.to_string()))
            .ok_or_else(|| Error::Package("row missing after insert".into()))?;
        // Find or create cell
        if let Some(cell) = row_el
            .children
            .iter_mut()
            .find(|c| c.local_name == "c" && c.get_attribute("r") == Some(reference))
        {
            cell.set_attribute("s", style_index.to_string());
        } else {
            row_el.append_child(
                OpenXmlElement::new("x", crate::namespace::ns::SPREADSHEETML.uri, "c")
                    .with_attribute("r", reference)
                    .with_attribute("s", style_index.to_string()),
            );
        }
        self.save_sheet_root(&sheet_uri, &root)
    }

    fn write_styles_part(
        &mut self,
        styles: &crate::element::OpenXmlElement,
    ) -> Result<String> {
        let wb_uri = self.ensure_workbook()?;
        let styles_uri = PackUri::new("/xl/styles.xml");
        let xml = write_element(styles)?;
        self.package.set_part(
            styles_uri.clone(),
            content_type::SPREADSHEET_STYLES,
            xml,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| rels.get_by_type(rel::STYLES).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.package.add_part_relationship(
            &wb_uri,
            rel::STYLES,
            &styles_uri,
            RelationshipTargetMode::Internal,
        ))
    }

    /// Read package core properties.
    pub fn package_properties(&self) -> Result<PackageProperties> {
        PackageProperties::load_from(self.package.opc())
    }

    /// Write package core properties.
    pub fn set_package_properties(&mut self, props: &PackageProperties) -> Result<()> {
        self.package.set_package_properties(props)
    }

    /// Read extended properties (`docProps/app.xml`).
    pub fn extended_properties(&self) -> Result<ExtendedProperties> {
        ExtendedProperties::load_from(self.package.opc())
    }

    /// Write extended properties.
    pub fn set_extended_properties(&mut self, props: &ExtendedProperties) -> Result<()> {
        self.package.set_extended_properties(props)
    }

    /// Read custom properties.
    pub fn custom_properties(&self) -> Result<CustomProperties> {
        CustomProperties::load_from(self.package.opc())
    }

    /// Write custom properties.
    pub fn set_custom_properties(&mut self, props: &CustomProperties) -> Result<()> {
        self.package.set_custom_properties(props)
    }

    /// Whether a core properties part exists.
    pub fn has_package_properties(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docProps/core.xml"))
    }

    /// Whether an extended properties part exists.
    pub fn has_extended_properties(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docProps/app.xml"))
    }

    /// Whether a custom properties part exists.
    pub fn has_custom_properties(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docProps/custom.xml"))
    }

    /// Number of custom properties.
    pub fn custom_property_count(&self) -> Result<usize> {
        Ok(self.custom_properties()?.properties.len())
    }

    /// Convenience: set the document title in core properties.
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.title = Some(title.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read the document title from core properties.
    pub fn title(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.title)
    }

    /// Convenience: set the document creator in core properties.
    /// Whether core `title` is set.
    pub fn has_title(&self) -> Result<bool> {
        Ok(self.title()?.is_some())
    }

    /// Clear core `title`. Returns whether it was present.
    pub fn clear_title(&mut self) -> Result<bool> {
        let had = self.title()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.title = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_creator(&mut self, creator: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.creator = Some(creator.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read the document creator from core properties.
    pub fn creator(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.creator)
    }

    /// Convenience: set core subject.
    /// Whether core `creator` is set.
    pub fn has_creator(&self) -> Result<bool> {
        Ok(self.creator()?.is_some())
    }

    /// Clear core `creator`. Returns whether it was present.
    pub fn clear_creator(&mut self) -> Result<bool> {
        let had = self.creator()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.creator = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_subject(&mut self, subject: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.subject = Some(subject.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core subject.
    pub fn subject(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.subject)
    }

    /// Convenience: set core keywords.
    /// Whether core `subject` is set.
    pub fn has_subject(&self) -> Result<bool> {
        Ok(self.subject()?.is_some())
    }

    /// Clear core `subject`. Returns whether it was present.
    pub fn clear_subject(&mut self) -> Result<bool> {
        let had = self.subject()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.subject = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_keywords(&mut self, keywords: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.keywords = Some(keywords.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core keywords.
    pub fn keywords(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.keywords)
    }

    /// Convenience: set core description.
    /// Whether core `keywords` is set.
    pub fn has_keywords(&self) -> Result<bool> {
        Ok(self.keywords()?.is_some())
    }

    /// Clear core `keywords`. Returns whether it was present.
    pub fn clear_keywords(&mut self) -> Result<bool> {
        let had = self.keywords()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.keywords = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.description = Some(description.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core description.
    pub fn description(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.description)
    }

    /// Convenience: set core category.
    /// Whether core `description` is set.
    pub fn has_description(&self) -> Result<bool> {
        Ok(self.description()?.is_some())
    }

    /// Clear core `description`. Returns whether it was present.
    pub fn clear_description(&mut self) -> Result<bool> {
        let had = self.description()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.description = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_category(&mut self, category: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.category = Some(category.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core category.
    pub fn category(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.category)
    }

    /// Convenience: set extended Application name.
    /// Whether core `category` is set.
    pub fn has_category(&self) -> Result<bool> {
        Ok(self.category()?.is_some())
    }

    /// Clear core `category`. Returns whether it was present.
    pub fn clear_category(&mut self) -> Result<bool> {
        let had = self.category()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.category = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_application(&mut self, application: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.application = Some(application.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Application name.
    pub fn application(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.application)
    }

    /// Convenience: set extended Company.
    /// Whether extended `application` is set.
    pub fn has_application(&self) -> Result<bool> {
        Ok(self.application()?.is_some())
    }

    /// Clear extended `application`. Returns whether it was present.
    pub fn clear_application(&mut self) -> Result<bool> {
        let had = self.application()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.application = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_company(&mut self, company: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.company = Some(company.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Company.
    pub fn company(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.company)
    }

    /// Convenience: set core lastModifiedBy.
    /// Whether extended `company` is set.
    pub fn has_company(&self) -> Result<bool> {
        Ok(self.company()?.is_some())
    }

    /// Clear extended `company`. Returns whether it was present.
    pub fn clear_company(&mut self) -> Result<bool> {
        let had = self.company()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.company = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_last_modified_by(&mut self, name: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.last_modified_by = Some(name.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core lastModifiedBy.
    pub fn last_modified_by(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.last_modified_by)
    }

    /// Convenience: set core revision.
    /// Whether core `last_modified_by` is set.
    pub fn has_last_modified_by(&self) -> Result<bool> {
        Ok(self.last_modified_by()?.is_some())
    }

    /// Clear core `last_modified_by`. Returns whether it was present.
    pub fn clear_last_modified_by(&mut self) -> Result<bool> {
        let had = self.last_modified_by()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.last_modified_by = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_revision(&mut self, revision: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.revision = Some(revision.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core revision.
    pub fn revision(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.revision)
    }

    /// Convenience: set core language.
    /// Whether core `revision` is set.
    pub fn has_revision(&self) -> Result<bool> {
        Ok(self.revision()?.is_some())
    }

    /// Clear core `revision`. Returns whether it was present.
    pub fn clear_revision(&mut self) -> Result<bool> {
        let had = self.revision()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.revision = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_language(&mut self, language: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.language = Some(language.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core language.
    pub fn language(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.language)
    }

    /// Whether core language is set.
    pub fn has_language(&self) -> Result<bool> {
        Ok(self.language()?.is_some())
    }

    /// Clear core language.
    pub fn clear_language(&mut self) -> Result<bool> {
        let had = self.language()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.language = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    /// Convenience: set core version.
    pub fn set_version(&mut self, version: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.version = Some(version.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core version.
    pub fn version(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.version)
    }

    /// Convenience: set core contentStatus.
    /// Whether core `version` is set.
    pub fn has_version(&self) -> Result<bool> {
        Ok(self.version()?.is_some())
    }

    /// Clear core `version`. Returns whether it was present.
    pub fn clear_version(&mut self) -> Result<bool> {
        let had = self.version()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.version = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_content_status(&mut self, status: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.content_status = Some(status.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core contentStatus.
    pub fn content_status(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.content_status)
    }

    /// Convenience: set extended Manager.
    /// Whether core `content_status` is set.
    pub fn has_content_status(&self) -> Result<bool> {
        Ok(self.content_status()?.is_some())
    }

    /// Clear core `content_status`. Returns whether it was present.
    pub fn clear_content_status(&mut self) -> Result<bool> {
        let had = self.content_status()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.content_status = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_manager(&mut self, manager: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.manager = Some(manager.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Manager.
    pub fn manager(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.manager)
    }

    /// Convenience: set extended Template.
    /// Whether extended `manager` is set.
    pub fn has_manager(&self) -> Result<bool> {
        Ok(self.manager()?.is_some())
    }

    /// Clear extended `manager`. Returns whether it was present.
    pub fn clear_manager(&mut self) -> Result<bool> {
        let had = self.manager()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.manager = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_template(&mut self, template: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.template = Some(template.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Template.
    pub fn template(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.template)
    }

    /// Convenience: set extended HyperlinkBase.
    /// Whether extended `template` is set.
    pub fn has_template(&self) -> Result<bool> {
        Ok(self.template()?.is_some())
    }

    /// Clear extended `template`. Returns whether it was present.
    pub fn clear_template(&mut self) -> Result<bool> {
        let had = self.template()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.template = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_hyperlink_base(&mut self, base: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.hyperlink_base = Some(base.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended HyperlinkBase.
    pub fn hyperlink_base(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.hyperlink_base)
    }

    /// Convenience: set core `dcterms:created` timestamp (ISO-8601 string).
    /// Whether a hyperlink base is set.
    pub fn has_hyperlink_base(&self) -> Result<bool> {
        Ok(self.hyperlink_base()?.is_some())
    }

    /// Clear hyperlink base. Returns whether it was present.
    pub fn clear_hyperlink_base(&mut self) -> Result<bool> {
        let had = self.hyperlink_base()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.hyperlink_base = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_created(&mut self, created: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.created = Some(created.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core `dcterms:created`.
    pub fn created(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.created)
    }

    /// Convenience: set core `dcterms:modified` timestamp (ISO-8601 string).
    /// Whether core `created` is set.
    pub fn has_created(&self) -> Result<bool> {
        Ok(self.created()?.is_some())
    }

    /// Clear core `created`. Returns whether it was present.
    pub fn clear_created(&mut self) -> Result<bool> {
        let had = self.created()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.created = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_modified(&mut self, modified: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.modified = Some(modified.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core `dcterms:modified`.
    pub fn modified(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.modified)
    }

    /// Resolve the content type of a part URI (override or default by extension).
    /// Whether core `modified` is set.
    pub fn has_modified(&self) -> Result<bool> {
        Ok(self.modified()?.is_some())
    }

    /// Clear core `modified`. Returns whether it was present.
    pub fn clear_modified(&mut self) -> Result<bool> {
        let had = self.modified()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.modified = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn part_content_type(&self, uri: &str) -> Option<String> {
        self.package
            .opc()
            .content_types()
            .content_type_for(uri)
            .map(|s| s.to_string())
    }

    /// List package-level relationships as `(id, type, target)`.
    pub fn list_package_relationships(&self) -> Vec<(String, String, String)> {
        self.package
            .opc()
            .package_relationships()
            .iter()
            .map(|r| {
                (
                    r.id.clone(),
                    r.relationship_type.clone(),
                    r.target.clone(),
                )
            })
            .collect()
    }

    /// Count package-level relationships.
    pub fn package_relationship_count(&self) -> usize {
        self.package.opc().package_relationships().len()
    }

    /// Count relationships from the workbook part.
    pub fn workbook_relationship_count(&self) -> usize {
        let wb = PackUri::new(WORKBOOK_URI);
        self.package
            .opc()
            .part_relationships(&wb)
            .map(|r| r.len())
            .unwrap_or(0)
    }

    /// List workbook relationships as `(id, type, target)`.
    pub fn list_workbook_relationships(&self) -> Vec<(String, String, String)> {
        let wb = PackUri::new(WORKBOOK_URI);
        self.package
            .opc()
            .part_relationships(&wb)
            .map(|rels| {
                rels.iter()
                    .map(|r| {
                        (
                            r.id.clone(),
                            r.relationship_type.clone(),
                            r.target.clone(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }


    /// Alias for [`list_workbook_relationships`](Self::list_workbook_relationships).
    pub fn list_main_relationships(&self) -> Vec<(String, String, String)> {
        self.list_workbook_relationships()
    }

    /// Alias for [`workbook_relationship_count`](Self::workbook_relationship_count).
    pub fn main_relationship_count(&self) -> usize {
        self.workbook_relationship_count()
    }

    /// List relationships from a worksheet as `(id, type, target)`.
    pub fn list_sheet_relationships(
        &self,
        sheet_name: &str,
    ) -> Result<Vec<(String, String, String)>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        Ok(self
            .package
            .opc()
            .part_relationships(&sheet_uri)
            .map(|rels| {
                rels.iter()
                    .map(|r| {
                        (
                            r.id.clone(),
                            r.relationship_type.clone(),
                            r.target.clone(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Count relationships from a worksheet.
    pub fn sheet_relationship_count(&self, sheet_name: &str) -> Result<usize> {
        Ok(self.list_sheet_relationships(sheet_name)?.len())
    }

    /// Whether a part exists at `uri`.
    pub fn has_part(&self, uri: &str) -> bool {
        self.package.opc().has_part(&PackUri::new(uri))
    }

    /// Read raw part bytes by URI.
    pub fn get_part_bytes(&self, uri: &str) -> Option<Vec<u8>> {
        self.package
            .opc()
            .get_part(&PackUri::new(uri))
            .map(|b| b.to_vec())
    }

    /// Write/replace raw part bytes and content type.
    pub fn set_part_bytes(
        &mut self,
        uri: &str,
        content_type: &str,
        data: impl Into<Vec<u8>>,
    ) {
        self.package
            .set_part(PackUri::new(uri), content_type, data);
    }

    /// Byte length of a part, if present.
    pub fn part_size(&self, uri: &str) -> Option<usize> {
        self.package
            .opc()
            .get_part(&PackUri::new(uri))
            .map(|b| b.len())
    }

    /// Resolve a package-level relationship target by id.
    pub fn package_relationship_target(&self, id: &str) -> Option<String> {
        self.package
            .opc()
            .package_relationships()
            .get(id)
            .map(|r| r.target.clone())
    }

    /// Number of parts in the package.
    pub fn package_part_count(&self) -> usize {
        self.package.opc().part_uris().len()
    }

    /// Add a package thumbnail image (`docProps/thumbnail.{ext}`).
    ///
    /// Returns the relationship id. `content_type` e.g. `image/jpeg`, `image/png`.
    pub fn add_thumbnail(
        &mut self,
        image_bytes: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<String> {
        let uri = PackUri::new(format!("/docProps/thumbnail.{extension}"));
        self.package.set_part(
            uri.clone(),
            content_type_str,
            image_bytes.into(),
        );
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .map(|r| r.id.clone())
        {
            return Ok(existing);
        }
        Ok(self.package.add_package_relationship(
            rel::THUMBNAIL,
            &uri,
            RelationshipTargetMode::Internal,
        ))
    }

    /// Whether a package thumbnail relationship or part exists.
    pub fn has_thumbnail(&self) -> bool {
        self.package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .is_some()
            || self
                .package
                .opc()
                .part_uris().into_iter().any(|u| u.as_str().starts_with("/docProps/thumbnail."))
    }

    /// Remove the package thumbnail part and relationship.
    pub fn clear_thumbnail(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/docProps/thumbnail."))
            
            .collect();
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .is_some();
        if uris.is_empty() && !had_rel {
            return Ok(false);
        }
        if let Some(rel_id) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .map(|r| r.id.clone())
        {
            let _ = self.package.delete_reference_relationship(None, &rel_id);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(true)
    }

    /// Add a digital signature origin part shell (no crypto).
    pub fn add_digital_signature_origin(&mut self) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/_xmlsignatures/origin.sigs");
        if !self.package.opc().has_part(&uri) {
            self.package.set_part(
                uri.clone(),
                content_type::DIGITAL_SIGNATURE_ORIGIN,
                Vec::new(),
            );
        }
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
            .map(|r| r.id.clone())
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_package_relationship(
            rel::DIGITAL_SIGNATURE_ORIGIN,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Attach an XML signature part under the origin (shell only — stores raw XML).
    pub fn add_xml_signature_part(
        &mut self,
        signature_xml: impl AsRef<[u8]>,
    ) -> Result<(String, PackUri)> {
        let (_origin_rid, origin_uri) = self.add_digital_signature_origin()?;
        let mut index = 1u32;
        let sig_uri = loop {
            let candidate = PackUri::new(format!("/_xmlsignatures/sig{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_part(
            sig_uri.clone(),
            content_type::DIGITAL_SIGNATURE_XML,
            signature_xml.as_ref().to_vec(),
        );
        let rid = self.package.add_part_relationship(
            &origin_uri,
            rel::DIGITAL_SIGNATURE,
            &sig_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, sig_uri))
    }

    /// Whether a digital signature origin part is present.
    pub fn has_digital_signature_origin(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/_xmlsignatures/origin.sigs"))
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
                .is_some()
    }

    /// Count XML signature parts under `/_xmlsignatures/`.
    pub fn digital_signature_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.starts_with("/_xmlsignatures/") && s.ends_with(".xml")
            })
            .count()
    }

    /// Remove all digital signature parts and package origin relationship.
    pub fn clear_digital_signatures(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/_xmlsignatures/"))
            
            .collect();
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
            .is_some();
        if uris.is_empty() && !had_rel {
            return Ok(false);
        }
        if let Some(id) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
            .map(|r| r.id.clone())
        {
            let _ = self.package.delete_reference_relationship(None, &id);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(true)
    }

    /// List custom XML parts related from the workbook as `(id, uri, bytes)`.
    pub fn custom_xml_parts(&self) -> Result<Vec<(String, PackUri, Vec<u8>)>> {
        let wb_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let Some(rels) = self.package.opc().part_relationships(&wb_uri) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for r in rels.find_all_by_type(rel::CUSTOM_XML) {
            let target = crate::opc::resolve_uri(&wb_uri, &r.target)?;
            if let Some(data) = self.package.opc().get_part(&target) {
                out.push((r.id.clone(), target, data.to_vec()));
            }
        }
        Ok(out)
    }

    /// Number of custom XML parts.
    pub fn custom_xml_part_count(&self) -> Result<usize> {
        Ok(self.custom_xml_parts()?.len())
    }

    /// Whether any custom XML parts are present.
    pub fn has_custom_xml_parts(&self) -> Result<bool> {
        Ok(!self.custom_xml_parts()?.is_empty())
    }

    /// Add a custom XML part related from the workbook.
    pub fn add_custom_xml_part(
        &mut self,
        xml_bytes: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let wb_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let part_uri = loop {
            let candidate = PackUri::new(format!("/customXml/item{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_part(
            part_uri.clone(),
            content_type::CUSTOM_XML,
            xml_bytes.into(),
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::CUSTOM_XML,
            &part_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, part_uri))
    }


    /// Attach a custom XML properties part to an existing custom XML item part.
    pub fn add_custom_xml_properties(
        &mut self,
        custom_xml_uri: &PackUri,
        item_id: &str,
    ) -> Result<(String, PackUri)> {
        let mut index = 1u32;
        let props_uri = loop {
            let candidate = PackUri::new(format!("/customXml/itemProps{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let ds = "http://schemas.openxmlformats.org/officeDocument/2006/customXml";
        let root = OpenXmlElement::new("ds", ds, "datastoreItem")
            .with_ns_decl("ds", ds)
            .with_attribute_qname("ds:itemID", item_id)
            .with_child(OpenXmlElement::new("ds", ds, "schemaRefs"));
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            props_uri.clone(),
            content_type::CUSTOM_XML_PROPERTIES,
            xml,
        );
        let rid = self.package.add_part_relationship(
            custom_xml_uri,
            rel::CUSTOM_XML_PROPS,
            &props_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, props_uri))
    }

    /// Remove a custom XML part by relationship id.
    pub fn remove_custom_xml_part(&mut self, relationship_id: &str) -> Result<bool> {
        let wb_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let target = {
            let Some(rels) = self.package.opc().part_relationships(&wb_uri) else {
                return Ok(false);
            };
            let Some(rel_entry) = rels.get(relationship_id) else {
                return Ok(false);
            };
            if rel_entry.relationship_type != rel::CUSTOM_XML {
                return Ok(false);
            }
            crate::opc::resolve_uri(&wb_uri, &rel_entry.target)?
        };
        let _ = self.package.delete_reference_relationship(Some(&wb_uri), relationship_id);
        self.package.delete_part(&target);
        Ok(true)
    }

    /// Remove all custom XML parts related from the workbook.
    pub fn clear_custom_xml_parts(&mut self) -> Result<usize> {
        let parts = self.custom_xml_parts()?;
        let n = parts.len();
        for (rid, _, _) in parts {
            let _ = self.remove_custom_xml_part(&rid)?;
        }
        Ok(n)
    }

    /// Embed an arbitrary package/object under `/xl/embeddings/`.
    ///
    /// Returns `(relationship_id, part_uri)`. Does not insert a drawing into the sheet.
    pub fn add_embedded_package(
        &mut self,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<(String, PackUri)> {
        let wb_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let uri = loop {
            let candidate =
                PackUri::new(format!("/xl/embeddings/Microsoft_Object{index}.{extension}"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package
            .set_part(uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::PACKAGE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }



    /// Alias for [`add_embedded_package`](Self::add_embedded_package) using package content type.
    pub fn add_embedded_package_part(
        &mut self,
        data: impl Into<Vec<u8>>,
        extension: &str,
    ) -> Result<(String, PackUri)> {
        self.add_embedded_package(data, content_type::PACKAGE_EMBEDDED, extension)
    }

    /// Embed an OLE object binary part shell under `/xl/embeddings/`.
    ///
    /// `prog_id` is accepted for API compatibility and currently unused.
    pub fn add_embedded_object(
        &mut self,
        data: impl Into<Vec<u8>>,
        prog_id: &str,
    ) -> Result<(String, PackUri)> {
        let _ = prog_id;
        let main_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/embeddings/oleObject{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            "application/vnd.openxmlformats-officedocument.oleObject",
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::OLE_OBJECT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Whether any embedded package/object parts exist under `/xl/embeddings/`.
    pub fn has_embeddings(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/embeddings/"))
    }

    /// Count embedding parts under `/xl/embeddings/`.
    pub fn embedding_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/embeddings/"))
            .count()
    }

    /// List embedding part URIs.
    pub fn list_embeddings(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/embeddings/"))
            
            .collect()
    }

    /// Remove a single embedding part by URI and drop workbook relationships that target it.
    pub fn remove_embedding(&mut self, uri: &PackUri) -> Result<bool> {
        if !uri.as_str().starts_with("/xl/embeddings/") {
            return Ok(false);
        }
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        let target = uri.as_str().to_string();
        let part_uris: Vec<PackUri> = self.package.opc().part_uris();
        for src in part_uris {
            let Some(rels) = self.package.opc().part_relationships(&src) else {
                continue;
            };
            let ids: Vec<String> = rels
                .iter()
                .filter(|r| relationship_targets_uri(&src, r.target.as_str(), &target))
                .map(|r| r.id.clone())
                .collect();
            if ids.is_empty() {
                continue;
            }
            self.package
                .delete_reference_relationships(Some(&src), &ids);
        }
        self.package.delete_part(&uri);
        Ok(true)
    }

    /// Remove all embedding parts and related workbook relationships.
    pub fn clear_embeddings(&mut self) -> Result<usize> {
        let uris = self.list_embeddings();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.target.contains("embeddings/")
                                || r.relationship_type.contains("package")
                                || r.relationship_type.contains("oleObject")
                                || r.relationship_type.contains("embedded")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Add a VBA project binary part shell (no macro execution).
    pub fn add_vba_project(&mut self, data: impl Into<Vec<u8>>) -> Result<(String, PackUri)> {
        let wb_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let uri = PackUri::new("/xl/vbaProject.bin");
        self.package
            .set_part(uri.clone(), content_type::VBA_PROJECT, data.into());
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&wb_uri)
            .and_then(|rels| rels.get_by_type(rel::VBA_PROJECT).map(|r| r.id.clone()))
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::VBA_PROJECT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// True if a VBA project part is present.
    pub fn has_vba_project(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("vbaProject") || u.as_str().ends_with("vbaProject.bin"))
    }

    /// Remove VBA project parts and their relationships.
    /// Read raw VBA project bytes if present.
    pub fn vba_project_bytes(&self) -> Option<Vec<u8>> {
        for uri in self.package.opc().part_uris() {
            let s = uri.as_str();
            if s.contains("vbaProject") || s.ends_with("vbaProject.bin") {
                return self.package.opc().get_part_cloned(&uri).ok().flatten();
            }
        }
        None
    }

    /// List URIs of VBA-related parts.
    pub fn list_vba_parts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str().to_ascii_lowercase();
                s.contains("vbaproject") || s.contains("vbadata") || s.contains("vbasignature")
            })
            .collect()
    }

    pub fn vba_part_count(&self) -> usize {
        self.list_vba_parts().len()
    }

    /// Parse `vbaProject.bin` CFB structure (streams/storages inventory; no macro execution).
    pub fn inspect_vba_project(&self) -> crate::Result<Option<crate::opc::CfbFile>> {
        let Some(bytes) = self.vba_project_bytes() else { return Ok(None); };
        Ok(Some(crate::opc::inspect_vba_project(&bytes)?))
    }


    pub fn clear_vba_project(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.contains("vbaProject") || s.contains("vbaData")
            })
            
            .collect();
        if uris.is_empty() {
            return Ok(false);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type == rel::VBA_PROJECT)
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(true)
    }

    /// Add a Custom UI part (`/customUI/customUI.xml`) at package level.
    pub fn add_custom_ui(
        &mut self,
        custom_ui_xml: impl AsRef<[u8]>,
    ) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/customUI/customUI.xml");
        self.package.set_part(
            uri.clone(),
            content_type::CUSTOM_UI,
            custom_ui_xml.as_ref().to_vec(),
        );
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::CUSTOM_UI_2007)
            .or_else(|| {
                self.package
                    .opc()
                    .package_relationships()
                    .get_by_type(rel::CUSTOM_UI)
            })
            .map(|r| r.id.clone())
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_package_relationship(
            rel::CUSTOM_UI_2007,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Whether a Custom UI part/relationship exists.
    pub fn has_custom_ui(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/customUI/customUI.xml"))
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::CUSTOM_UI_2007)
                .is_some()
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::CUSTOM_UI)
                .is_some()
    }

    /// Remove Custom UI part and package relationship.
    pub fn clear_custom_ui(&mut self) -> Result<bool> {
        let uri = PackUri::new("/customUI/customUI.xml");
        let had_part = self.package.opc().has_part(&uri);
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::CUSTOM_UI_2007)
            .is_some()
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::CUSTOM_UI)
                .is_some();
        if !had_part && !had_rel {
            return Ok(false);
        }
        for ty in [rel::CUSTOM_UI_2007, rel::CUSTOM_UI] {
            if let Some(id) = self
                .package
                .opc()
                .package_relationships()
                .get_by_type(ty)
                .map(|r| r.id.clone())
            {
                let _ = self.package.delete_reference_relationship(None, &id);
            }
        }
        if had_part {
            self.package.delete_part(&uri);
        }
        Ok(true)
    }

    /// Add a printer settings binary part shell related from the workbook.
    pub fn add_printer_settings(
        &mut self,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let wb_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let uri = loop {
            let candidate =
                PackUri::new(format!("/xl/printerSettings/printerSettings{index}.bin"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            content_type::SPREADSHEET_PRINTER_SETTINGS,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &wb_uri,
            rel::PRINTER_SETTINGS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Whether any printer settings parts exist.
    pub fn has_printer_settings(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
        })
    }

    /// Count printer settings parts.
    pub fn printer_settings_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| {
                u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
            })
            .count()
    }

    /// Remove all printer settings parts and related workbook relationships.
    pub fn clear_printer_settings(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| {
                u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
            })
            
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::PRINTER_SETTINGS)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Add a Quick Access Toolbar customizations part (package-level).
    pub fn add_quick_access_toolbar(&mut self) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/customUI/qatCustomizations.xml");
        let mso = "http://schemas.microsoft.com/office/2006/01/customui";
        let root = OpenXmlElement::new("mso", mso, "customUI")
            .with_ns_decl("mso", mso)
            .with_child(
                OpenXmlElement::new("mso", mso, "ribbon").with_child(
                    OpenXmlElement::new("mso", mso, "qat").with_child(
                        OpenXmlElement::new("mso", mso, "sharedControls"),
                    ),
                ),
            );
        self.package.set_part(
            uri.clone(),
            content_type::QAT,
            crate::element::write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::QAT)
            .map(|r| r.id.clone())
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_package_relationship(
            rel::QAT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Whether a QAT customizations part exists.
    pub fn has_quick_access_toolbar(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/customUI/qatCustomizations.xml"))
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::QAT)
                .is_some()
    }

    /// Remove QAT customizations part and package relationship.
    pub fn clear_quick_access_toolbar(&mut self) -> Result<bool> {
        let uri = PackUri::new("/customUI/qatCustomizations.xml");
        let had_part = self.package.opc().has_part(&uri);
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::QAT)
            .is_some();
        if !had_part && !had_rel {
            return Ok(false);
        }
        if let Some(id) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::QAT)
            .map(|r| r.id.clone())
        {
            let _ = self.package.delete_reference_relationship(None, &id);
        }
        if had_part {
            self.package.delete_part(&uri);
        }
        Ok(true)
    }

    /// Add a sensitivity / classification label info part shell.
    pub fn add_label_info(&mut self, label_id: &str, name: &str) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/docMetadata/LabelInfo.xml");
        let clbl = "http://schemas.microsoft.com/office/2020/mipLabelMetadata";
        let root = OpenXmlElement::new("clbl", clbl, "labelList")
            .with_ns_decl("clbl", clbl)
            .with_child(
                OpenXmlElement::new("clbl", clbl, "label")
                    .with_attribute("id", label_id)
                    .with_attribute("name", name)
                    .with_attribute("enabled", "1"),
            );
        self.package.set_part(
            uri.clone(),
            content_type::LABEL_INFO,
            crate::element::write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::LABEL_INFO)
            .map(|r| r.id.clone())
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_package_relationship(
            rel::LABEL_INFO,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Whether a sensitivity label info part exists.
    pub fn has_label_info(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docMetadata/LabelInfo.xml"))
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::LABEL_INFO)
                .is_some()
    }

    /// Remove label info part and package relationship.
    pub fn clear_label_info(&mut self) -> Result<bool> {
        let uri = PackUri::new("/docMetadata/LabelInfo.xml");
        let had_part = self.package.opc().has_part(&uri);
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::LABEL_INFO)
            .is_some();
        if !had_part && !had_rel {
            return Ok(false);
        }
        if let Some(id) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::LABEL_INFO)
            .map(|r| r.id.clone())
        {
            let _ = self.package.delete_reference_relationship(None, &id);
        }
        if had_part {
            self.package.delete_part(&uri);
        }
        Ok(true)
    }

    /// Add Office web extension + taskpanes shells under `/xl/webextensions/`.
    pub fn add_web_extension_shell(
        &mut self,
        store_id: &str,
        version: &str,
    ) -> Result<(PackUri, PackUri)> {
        let we_uri = PackUri::new("/xl/webextensions/webextension1.xml");
        let tp_uri = PackUri::new("/xl/webextensions/taskpanes.xml");
        let we = "http://schemas.microsoft.com/office/webextensions/webextension/2010/11";
        let wetp =
            "http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11";
        let ext = OpenXmlElement::new("we", we, "webextension")
            .with_ns_decl("we", we)
            .with_attribute("id", format!("{{{}-0000-0000-0000-000000000000}}", store_id))
            .with_child(
                OpenXmlElement::new("we", we, "reference")
                    .with_attribute("id", store_id)
                    .with_attribute("version", version)
                    .with_attribute("store", "developer")
                    .with_attribute("storeType", "Registry"),
            )
            .with_child(OpenXmlElement::new("we", we, "alternateReferences"))
            .with_child(
                OpenXmlElement::new("we", we, "properties").with_child(
                    OpenXmlElement::new("we", we, "property")
                        .with_attribute("name", "Office.AutoShowTaskpaneWithDocument")
                        .with_attribute("value", "true"),
                ),
            )
            .with_child(OpenXmlElement::new("we", we, "bindings"))
            .with_child(OpenXmlElement::new("we", we, "snapshot"));
        let taskpanes = OpenXmlElement::new("wetp", wetp, "taskpanes")
            .with_ns_decl("wetp", wetp)
            .with_ns_decl(
                "r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            )
            .with_child(
                OpenXmlElement::new("wetp", wetp, "taskpane")
                    .with_attribute("dockstate", "right")
                    .with_attribute("visibility", "1")
                    .with_attribute("width", "350")
                    .with_attribute("row", "1")
                    .with_child(
                        OpenXmlElement::new("wetp", wetp, "webextensionref")
                            .with_attribute_qname("r:id", "rId1"),
                    ),
            );
        self.package.set_part(
            we_uri.clone(),
            content_type::WEB_EXTENSION,
            crate::element::write_element(&ext)?,
        );
        self.package.set_part(
            tp_uri.clone(),
            content_type::WEB_EXTENSION_TASKPANES,
            crate::element::write_element(&taskpanes)?,
        );
        self.package.add_part_relationship(
            &tp_uri,
            rel::WEB_EXTENSION,
            &we_uri,
            RelationshipTargetMode::Internal,
        );
        if self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::WEB_EXTENSION_TASKPANES)
            .is_none()
        {
            self.package.add_package_relationship(
                rel::WEB_EXTENSION_TASKPANES,
                &tp_uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok((we_uri, tp_uri))
    }

    /// Whether any web extension parts exist under `/xl/webextensions/`.
    pub fn has_web_extensions(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/xl/webextensions/"))
    }

    /// Count web extension parts under `/xl/webextensions/`.
    pub fn web_extension_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/xl/webextensions/"))
            .count()
    }

    /// Remove web extension + taskpanes parts and package relationships.
    pub fn clear_web_extensions(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/xl/webextensions/"))
            
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        for ty in [rel::WEB_EXTENSION, rel::WEB_EXTENSION_TASKPANES] {
            if let Some(id) = self
                .package
                .opc()
                .package_relationships()
                .get_by_type(ty)
                .map(|r| r.id.clone())
            {
                let _ = self.package.delete_reference_relationship(None, &id);
            }
        }
        if let Ok(wb_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&wb_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::WEB_EXTENSION
                                || r.relationship_type == rel::WEB_EXTENSION_TASKPANES
                                || r.target.contains("webextension")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&wb_uri), &ids);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }


    /// Whether any SmartArt/diagram parts exist under `/xl/diagrams/`.
    pub fn has_diagrams(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/xl/diagrams/"))
    }

    /// Count diagram parts under `/xl/diagrams/`.
    pub fn diagram_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/xl/diagrams/"))
            .count()
    }

    /// List diagram part URIs.
    pub fn list_diagrams(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/xl/diagrams/"))
            
            .collect()
    }

    /// Remove diagram parts and related main-part diagram relationships.
    pub fn clear_diagrams(&mut self) -> Result<usize> {
        let uris = self.list_diagrams();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(main_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::DIAGRAM_DATA
                                || r.relationship_type == rel::DIAGRAM_LAYOUT
                                || r.relationship_type == rel::DIAGRAM_COLORS
                                || r.relationship_type == rel::DIAGRAM_STYLE
                                || r.relationship_type == rel::DIAGRAM_PERSIST_LAYOUT
                                || r.target.contains("diagrams/")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&main_uri), &ids);
        }
        for uri in uris {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }


    /// Add a SmartArt / diagram parts shell (data, layout, colors, style, persist layout).
    ///
    /// Creates minimal diagram parts under `/xl/diagrams/` related from the main
    /// document (persist layout is related from the data part). Returns the data part URI.
    pub fn add_diagram_shell(&mut self, unique_id: &str) -> Result<PackUri> {
        let main_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let data_uri = loop {
            let c = PackUri::new(format!("/xl/diagrams/data{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let layout_uri = PackUri::new(format!("/xl/diagrams/layout{index}.xml"));
        let colors_uri = PackUri::new(format!("/xl/diagrams/colors{index}.xml"));
        let style_uri = PackUri::new(format!("/xl/diagrams/quickStyle{index}.xml"));
        let drawing_uri = PackUri::new(format!("/xl/diagrams/drawing{index}.xml"));
        let dgm = "http://schemas.openxmlformats.org/drawingml/2006/diagram";
        let a = crate::namespace::ns::DRAWINGML.uri;
        let dsp = "http://schemas.microsoft.com/office/drawing/2008/diagram";
        // Minimal data model
        let data = OpenXmlElement::new("dgm", dgm, "dataModel")
            .with_ns_decl("dgm", dgm)
            .with_ns_decl("a", a)
            .with_child(
                OpenXmlElement::new("dgm", dgm, "ptLst")
                    .with_child(
                        OpenXmlElement::new("dgm", dgm, "pt")
                            .with_attribute("modelId", unique_id)
                            .with_attribute("type", "doc"),
                    )
                    .with_child(
                        OpenXmlElement::new("dgm", dgm, "pt")
                            .with_attribute("modelId", format!("{unique_id}-1"))
                            .with_child(
                                OpenXmlElement::new("dgm", dgm, "prSet")
                                    .with_attribute("phldr", "1"),
                            )
                            .with_child(
                                OpenXmlElement::new("a", a, "t").with_child(
                                    OpenXmlElement::new("a", a, "p").with_child(
                                        OpenXmlElement::new("a", a, "r").with_child(
                                            OpenXmlElement::new("a", a, "t").with_text("Node"),
                                        ),
                                    ),
                                ),
                            ),
                    ),
            )
            .with_child(OpenXmlElement::new("dgm", dgm, "cxnLst"));
        let layout = OpenXmlElement::new("dgm", dgm, "layoutDef")
            .with_ns_decl("dgm", dgm)
            .with_attribute("uniqueId", format!("layout-{unique_id}"))
            .with_child(OpenXmlElement::new("dgm", dgm, "title").with_attribute("val", ""))
            .with_child(OpenXmlElement::new("dgm", dgm, "desc").with_attribute("val", ""));
        let colors = OpenXmlElement::new("dgm", dgm, "colorsDef")
            .with_ns_decl("dgm", dgm)
            .with_attribute("uniqueId", format!("colors-{unique_id}"))
            .with_child(OpenXmlElement::new("dgm", dgm, "title").with_attribute("val", ""))
            .with_child(OpenXmlElement::new("dgm", dgm, "desc").with_attribute("val", ""));
        let style = OpenXmlElement::new("dgm", dgm, "styleDef")
            .with_ns_decl("dgm", dgm)
            .with_attribute("uniqueId", format!("style-{unique_id}"))
            .with_child(OpenXmlElement::new("dgm", dgm, "title").with_attribute("val", ""))
            .with_child(OpenXmlElement::new("dgm", dgm, "desc").with_attribute("val", ""));
        // Persist layout / drawing shell (dsp:drawing)
        let drawing = OpenXmlElement::new("dsp", dsp, "drawing")
            .with_ns_decl("dsp", dsp)
            .with_ns_decl("a", a)
            .with_child(OpenXmlElement::new("dsp", dsp, "spTree"));
        for (uri, ct, el) in [
            (&data_uri, content_type::DIAGRAM_DATA, data),
            (&layout_uri, content_type::DIAGRAM_LAYOUT, layout),
            (&colors_uri, content_type::DIAGRAM_COLORS, colors),
            (&style_uri, content_type::DIAGRAM_STYLE, style),
            (&drawing_uri, content_type::DIAGRAM_PERSIST_LAYOUT, drawing),
        ] {
            self.package.set_part(
                uri.clone(),
                ct,
                crate::element::write_element(&el)?,
            );
        }
        // Relate four core diagram parts from main document
        for (uri, rel_ty) in [
            (&data_uri, rel::DIAGRAM_DATA),
            (&layout_uri, rel::DIAGRAM_LAYOUT),
            (&colors_uri, rel::DIAGRAM_COLORS),
            (&style_uri, rel::DIAGRAM_STYLE),
        ] {
            self.package.add_part_relationship(
                &main_uri,
                rel_ty,
                uri,
                RelationshipTargetMode::Internal,
            );
        }
        // Persist layout is a child of the data part
        self.package.add_part_relationship(
            &data_uri,
            rel::DIAGRAM_PERSIST_LAYOUT,
            &drawing_uri,
            RelationshipTargetMode::Internal,
        );
        Ok(data_uri)
    }


    /// Add legacy diagram text parts shell (VML diagram text).
    pub fn add_legacy_diagram_text(
        &mut self,
        text_data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, PackUri)> {
        let main_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let text_uri = loop {
            let c = PackUri::new(format!("/xl/diagrams/legacy/text{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let info_uri = PackUri::new(format!("/xl/diagrams/legacy/textInfo{index}.xml"));
        self.package.set_part(
            text_uri.clone(),
            content_type::LEGACY_DIAGRAM_TEXT,
            text_data.into(),
        );
        let dgm = "http://schemas.microsoft.com/office/drawing/2008/diagram";
        let info = OpenXmlElement::new("dgm", dgm, "textInfo").with_ns_decl("dgm", dgm);
        self.package.set_part(
            info_uri.clone(),
            content_type::LEGACY_DIAGRAM_TEXT_INFO,
            crate::element::write_element(&info)?,
        );
        self.package.add_part_relationship(
            &main_uri,
            rel::LEGACY_DIAGRAM_TEXT,
            &text_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            &text_uri,
            rel::LEGACY_DIAGRAM_TEXT_INFO,
            &info_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((text_uri, info_uri))
    }

    /// Store an image media part under `/xl/media/` without anchoring it.
    ///
    /// Returns `(relationship_id, part_uri)` related from the main document part.
    /// Prefer [`add_image_on_sheet`] / [`add_image_on_slide`] when a drawing anchor is needed.
    pub fn add_image(
        &mut self,
        format: crate::packaging::ImageFormat,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let main_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let ext = match format {
            crate::packaging::ImageFormat::Png => "png",
            crate::packaging::ImageFormat::Jpeg => "jpeg",
            crate::packaging::ImageFormat::Gif => "gif",
            crate::packaging::ImageFormat::Bmp => "bmp",
            crate::packaging::ImageFormat::Tiff => "tiff",
            crate::packaging::ImageFormat::Emf => "emf",
            crate::packaging::ImageFormat::Wmf => "wmf",
            crate::packaging::ImageFormat::Svg => "svg",
        };
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/xl/media/image{index}.{ext}"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let ct = format.content_type();
        self.package.set_content_type_default(ext, ct);
        self.package
            .set_part(uri.clone(), ct, data.into());
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::IMAGE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }


    /// Whether any media/image parts exist under `/xl/media/`.
    pub fn has_images(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/xl/media/"))
    }

    /// Count media/image parts under `/xl/media/`.
    pub fn image_count(&self) -> usize {
        self.list_images().len()
    }

    /// List media/image part URIs under `/xl/media/`.
    pub fn list_images(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/xl/media/"))
            
            .collect()
    }

    /// Remove media/image parts under `/xl/media/` and related image relationships.
    pub fn clear_images(&mut self) -> Result<usize> {
        let images = self.list_images();
        let n = images.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(main_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::IMAGE
                                || r.relationship_type.contains("image")
                                || r.target.contains("media/")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            self.package
                .delete_reference_relationships(Some(&main_uri), &ids);
        }
        // also drop sheet/slide-level image rels pointing at media
        let part_uris: Vec<PackUri> = self.package.opc().part_uris();
        for part_uri in part_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&part_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::IMAGE
                                || r.target.contains("media/")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if ids.is_empty() {
                continue;
            }
            self.package
                .delete_reference_relationships(Some(&part_uri), &ids);
        }
        for uri in images {
            self.package.delete_part(&uri);
        }
        Ok(n)
    }

    /// Flat OPC XML string for this workbook.
    pub fn to_flat_opc_string(&mut self) -> Result<String> {
        use crate::opc::{progid, to_flat_opc};
        let bytes = to_flat_opc(self.package.opc(), Some(progid::EXCEL))?;
        Ok(String::from_utf8(bytes).map_err(|e| Error::Xml(e.to_string()))?)
    }

    /// Open a workbook from Flat OPC XML.
    pub fn from_flat_opc(xml: impl AsRef<[u8]>) -> Result<Self> {
        use crate::opc::from_flat_opc;
        let opc = from_flat_opc(xml)?;
        let mut settings = OpenSettings::default();
        settings.auto_save = false;
        Self::from_opc(opc, settings)
    }

    /// Normalize Strict OOXML namespaces/relationships to Transitional.
    ///
    /// Returns `(xml_replacements, relationship_replacements)`.
    pub fn rewrite_strict_to_transitional(&mut self) -> Result<(usize, usize)> {
        crate::namespace_rewrite::rewrite_package_to_transitional(self.package.opc_mut())
    }

    /// Normalize Transitional OOXML namespaces/relationships to Strict.
    pub fn rewrite_transitional_to_strict(&mut self) -> Result<(usize, usize)> {
        crate::namespace_rewrite::rewrite_package_to_strict(self.package.opc_mut())
    }

    /// Access open settings.
    pub fn settings(&self) -> &OpenSettings {
        self.package.settings()
    }

    /// Mutable access to open settings.
    pub fn settings_mut(&mut self) -> &mut OpenSettings {
        self.package.settings_mut()
    }

    /// Whether auto-save is enabled.
    pub fn auto_save(&self) -> bool {
        self.package.auto_save()
    }

    /// Detect an encrypted Office compound file at `path`.
    pub fn is_encrypted_office_file(path: impl AsRef<std::path::Path>) -> Result<bool> {
        OpcPackage::is_encrypted_office_file(path)
    }

    /// List content-type overrides as `(part_name, content_type)`.
    pub fn list_content_type_overrides(&self) -> Vec<(String, String)> {
        self.package
            .opc()
            .content_types()
            .overrides
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }


    /// Whether any content-type overrides are registered.
    pub fn has_content_type_overrides(&self) -> bool {
        !self.list_content_type_overrides().is_empty()
    }

    /// Count content-type overrides.
    pub fn content_type_override_count(&self) -> usize {
        self.list_content_type_overrides().len()
    }

    /// Convenience: set extended AppVersion.
    pub fn set_application_version(&mut self, version: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.application_version = Some(version.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended AppVersion.
    pub fn application_version(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.application_version)
    }

    /// Convenience: set extended DocSecurity.
    /// Whether extended `application_version` is set.
    pub fn has_application_version(&self) -> Result<bool> {
        Ok(self.application_version()?.is_some())
    }

    /// Clear extended `application_version`. Returns whether it was present.
    pub fn clear_application_version(&mut self) -> Result<bool> {
        let had = self.application_version()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.application_version = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_doc_security(&mut self, security: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.doc_security = Some(security);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended DocSecurity.
    pub fn doc_security(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.doc_security)
    }

    /// Convenience: set extended SharedDoc.
    pub fn set_shared_doc(&mut self, shared: bool) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.shared_doc = Some(shared);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended SharedDoc.
    pub fn shared_doc(&self) -> Result<Option<bool>> {
        Ok(self.extended_properties()?.shared_doc)
    }

    /// Convenience: set extended LinksUpToDate.
    pub fn set_links_up_to_date(&mut self, up_to_date: bool) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.links_up_to_date = Some(up_to_date);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended LinksUpToDate.
    pub fn links_up_to_date(&self) -> Result<Option<bool>> {
        Ok(self.extended_properties()?.links_up_to_date)
    }

    /// Convenience: set extended HyperlinksChanged.
    pub fn set_hyperlinks_changed(&mut self, changed: bool) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.hyperlinks_changed = Some(changed);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended HyperlinksChanged.
    pub fn hyperlinks_changed(&self) -> Result<Option<bool>> {
        Ok(self.extended_properties()?.hyperlinks_changed)
    }

    /// Convenience: set extended ScaleCrop.
    pub fn set_scale_crop(&mut self, scale_crop: bool) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.scale_crop = Some(scale_crop);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended ScaleCrop.
    pub fn scale_crop(&self) -> Result<Option<bool>> {
        Ok(self.extended_properties()?.scale_crop)
    }

    /// Whether DocSecurity is set.
    pub fn has_doc_security(&self) -> Result<bool> {
        Ok(self.doc_security()?.is_some())
    }

    /// Clear DocSecurity.
    pub fn clear_doc_security(&mut self) -> Result<bool> {
        let had = self.doc_security()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.doc_security = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Whether SharedDoc is set.
    pub fn has_shared_doc(&self) -> Result<bool> {
        Ok(self.shared_doc()?.is_some())
    }

    /// Clear SharedDoc.
    pub fn clear_shared_doc(&mut self) -> Result<bool> {
        let had = self.shared_doc()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.shared_doc = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Whether LinksUpToDate is set.
    pub fn has_links_up_to_date(&self) -> Result<bool> {
        Ok(self.links_up_to_date()?.is_some())
    }

    /// Clear LinksUpToDate.
    pub fn clear_links_up_to_date(&mut self) -> Result<bool> {
        let had = self.links_up_to_date()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.links_up_to_date = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Whether HyperlinksChanged is set.
    pub fn has_hyperlinks_changed(&self) -> Result<bool> {
        Ok(self.hyperlinks_changed()?.is_some())
    }

    /// Clear HyperlinksChanged.
    pub fn clear_hyperlinks_changed(&mut self) -> Result<bool> {
        let had = self.hyperlinks_changed()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.hyperlinks_changed = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Whether ScaleCrop is set.
    pub fn has_scale_crop(&self) -> Result<bool> {
        Ok(self.scale_crop()?.is_some())
    }

    /// Clear ScaleCrop.
    pub fn clear_scale_crop(&mut self) -> Result<bool> {
        let had = self.scale_crop()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.scale_crop = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Convenience: set extended TotalTime (minutes).
    pub fn set_total_time(&mut self, minutes: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.total_time = Some(minutes);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended TotalTime.
    pub fn total_time(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.total_time)
    }

    /// Whether TotalTime is set.
    pub fn has_total_time(&self) -> Result<bool> {
        Ok(self.total_time()?.is_some())
    }

    /// Clear TotalTime.
    pub fn clear_total_time(&mut self) -> Result<bool> {
        let had = self.total_time()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.total_time = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Set an integer custom property by name.
    pub fn set_custom_property_i4(&mut self, name: &str, value: i32) -> Result<()> {
        let mut props = self.custom_properties()?;
        props.set_i4(name, value);
        self.set_custom_properties(&props)
    }

    /// Read an integer custom property by name.
    pub fn get_custom_property_i4(&self, name: &str) -> Result<Option<i32>> {
        Ok(self.custom_properties()?.get(name).and_then(|p| match &p.value {
            crate::opc::CustomPropertyValue::I4(v) => Some(*v),
            _ => None,
        }))
    }

    /// Set a boolean custom property by name.
    pub fn set_custom_property_bool(&mut self, name: &str, value: bool) -> Result<()> {
        let mut props = self.custom_properties()?;
        props.set_bool(name, value);
        self.set_custom_properties(&props)
    }

    /// Read a boolean custom property by name.
    pub fn get_custom_property_bool(&self, name: &str) -> Result<Option<bool>> {
        Ok(self.custom_properties()?.get(name).and_then(|p| match &p.value {
            crate::opc::CustomPropertyValue::Bool(v) => Some(*v),
            _ => None,
        }))
    }

    /// List custom property names.
    pub fn list_custom_property_names(&self) -> Result<Vec<String>> {
        Ok(self
            .custom_properties()?
            .names()
            .into_iter()
            .map(|s| s.to_string())
            .collect())
    }

    /// Set a string custom property by name.
    pub fn set_custom_property_string(&mut self, name: &str, value: &str) -> Result<()> {
        let mut props = self.custom_properties()?;
        props.set_string(name, value);
        self.set_custom_properties(&props)
    }

    /// Read a custom property string value by name.
    pub fn get_custom_property_string(&self, name: &str) -> Result<Option<String>> {
        Ok(self
            .custom_properties()?
            .get(name)
            .and_then(|p| p.value.as_str().map(|s| s.to_string())))
    }

    /// Remove one custom property by name. Returns whether it was present.
    pub fn remove_custom_property(&mut self, name: &str) -> Result<bool> {
        let mut props = self.custom_properties()?;
        let removed = props.remove(name);
        if removed {
            if props.is_empty() {
                // keep empty part for round-trip consistency
            }
            self.set_custom_properties(&props)?;
        }
        Ok(removed)
    }

    /// Remove all custom properties (writes empty collection).
    pub fn clear_custom_properties(&mut self) -> Result<bool> {
        if !self.has_custom_properties() {
            return Ok(false);
        }
        let mut props = self.custom_properties()?;
        if props.is_empty() {
            return Ok(false);
        }
        props.clear();
        self.set_custom_properties(&props)?;
        Ok(true)
    }

    /// Convenience: package structure + workbook relationship/uniqueness checks.
    ///
    /// Excel has no Word-style lightweight body particle tree; this combines
    /// [`validate_package`](Self::validate_package) and
    /// [`validate_relationships`](Self::validate_relationships).
    pub fn validate(&self) -> Result<Vec<crate::validation::ValidationError>> {
        let mut errs = self.validate_package()?;
        errs.extend(self.validate_relationships()?);
        Ok(errs)
    }

    /// Same as [`validate`](Self::validate) for Excel (no separate particle pass).
    pub fn validate_full(&self) -> Result<Vec<crate::validation::ValidationError>> {
        self.validate()
    }


    /// Validate OPC package structure.
    pub fn validate_package(&self) -> Result<Vec<crate::validation::ValidationError>> {
        Ok(crate::validation::validate_package(
            self.package.opc(),
            true,
        ))
    }

    /// Validate part relationship constraints (C# `PackageValidator`).
    pub fn validate_package_constraints(&self) -> Result<Vec<crate::validation::ValidationError>> {
        Ok(crate::validation::validate_package_constraints(
            self.package.opc(),
        ))
    }

    /// Validate relationship-id attributes and unique-attribute rules in the workbook.
    pub fn validate_relationships(&self) -> Result<Vec<crate::validation::ValidationError>> {
        let wb_uri = PackUri::new("/xl/workbook.xml");
        if !self.package.opc().has_part(&wb_uri) {
            return Ok(Vec::new());
        }
        let xml = self
            .package
            .opc()
            .get_part(&wb_uri)
            .ok_or_else(|| Error::Package("workbook missing".into()))?;
        let root = parse_element(xml)?;
        let rel_rules = crate::validation::merged_relationship_rules(
            crate::validation::spreadsheet_relationship_rules(),
        );
        let unique_rules = crate::validation::merged_unique_attribute_rules(
            crate::validation::spreadsheet_unique_attribute_rules(),
        );
        Ok(crate::validation::validate_semantic(
            self.package.opc(),
            &wb_uri,
            &root,
            &rel_rules,
            &unique_rules,
        ))
    }

    /// Validate the workbook with the full extractable Schematron subset.
    pub fn validate_schematron(&self) -> Result<Vec<crate::validation::ValidationError>> {
        let wb_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        if !self.package.opc().has_part(&wb_uri) {
            return Ok(Vec::new());
        }
        let xml = self
            .package
            .opc()
            .get_part(&wb_uri)
            .ok_or_else(|| Error::Package("workbook missing".into()))?;
        let root = parse_element(xml)?;
        Ok(crate::validation::validate_schematron_subset(
            self.package.opc(),
            &wb_uri,
            &root,
        ))
    }


    /// Validate Schematron attribute constraints on the main document part root.
    pub fn validate_schematron_attributes(
        &self,
    ) -> Result<Vec<crate::validation::ValidationError>> {
        let main_uri = match self.package.opc().main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        let Some(data) = self.package.opc().get_part(&main_uri) else {
            return Ok(Vec::new());
        };
        let root = crate::element::parse_element(data)?;
        Ok(crate::validation::validate_schematron_attributes(&root))
    }

    /// Remove a part from the package (content-type, child rels, inbound rels).
    ///
    /// Does not rewrite workbook sheet lists; callers that remove a worksheet
    /// should update the workbook separately.
    pub fn delete_part(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.package.delete_part(uri)
    }

    /// Alias for [`delete_part`](Self::delete_part).
    pub fn remove_part(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.delete_part(uri)
    }

    /// Delete a part and cascade to parts that become unreachable.
    pub fn delete_part_and_orphans(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.package.delete_part_and_orphans(uri)
    }

    /// Delete the part identified by relationship id on the workbook part.
    pub fn delete_part_by_id(&mut self, id: &str) -> bool {
        let source = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok();
        self.package
            .delete_part_by_id(source.as_ref(), id)
    }

    /// Delete every part with the given content type, cascading orphans.
    pub fn delete_parts_of_content_type(&mut self, content_type: &str) -> usize {
        self.package
            .delete_parts_of_content_type(content_type)
    }

    /// Recursively delete parts of a relationship type (C# `DeletePartsRecursivelyOfType` stand-in).
    pub fn delete_parts_recursively_of_relationship_type(
        &mut self,
        relationship_type: &str,
    ) -> usize {
        self.package
            .delete_parts_recursively_of_relationship_type(relationship_type)
    }

    /// Add an external relationship from the workbook part.
    pub fn add_external_relationship(
        &mut self,
        relationship_type: &str,
        external_uri: &str,
    ) -> Result<String> {
        let wb = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no workbook part".into()))?;
        Ok(self.package.add_external_relationship(
            Some(&wb),
            relationship_type,
            external_uri,
        ))
    }

    /// External relationships on the workbook part.
    pub fn external_relationships(&self) -> Vec<&crate::opc::Relationship> {
        let Ok(wb) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package.opc().external_relationships(Some(&wb))
    }

    /// Ensure [`PackageEvents`](crate::features::PackageEvents) is registered.
    pub fn package_events(&mut self) -> &crate::features::PackageEvents {
        self.package.package_events()
    }

    /// Part-container events (C# `IPartEventsFeature`).
    pub fn part_events(&mut self) -> &crate::features::PartEvents {
        self.package.part_events()
    }

    /// Child parts related from the main part (C# GetPartsOfType / Parts).
    pub fn related_parts(
        &self,
        relationship_type: Option<&str>,
    ) -> Vec<crate::opc::RelatedPart> {
        let Ok(main) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package
            .opc()
            .related_parts(Some(&main), relationship_type)
    }

    /// Allocate a unique part URI under the main part.
    pub fn create_unique_part_uri(
        &self,
        content_type: &str,
        target_path: &str,
        target_name: &str,
        target_ext: &str,
    ) -> Result<PackUri> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        self.package.opc().create_unique_part_uri(
            content_type,
            &main,
            target_path,
            target_name,
            target_ext,
        )
    }



    /// Delete multiple parts by URI (C# `DeleteParts`).
    pub fn delete_parts(&mut self, uris: &[PackUri]) -> usize {
        self.package.delete_parts(uris)
    }

    /// C# `StrictRelationshipFound`.
    pub fn strict_relationship_found(&self) -> bool {
        self.package.strict_relationship_found()
    }

    /// Hyperlink relationships on the main part (C# `HyperlinkRelationships`).
    pub fn hyperlink_relationships(&self) -> Vec<crate::opc::HyperlinkRelationship> {
        let Ok(main) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package
            .hyperlink_relationships(Some(&main))
    }

    /// Relationship id of a part under the main part (C# `GetIdOfPart`).
    pub fn get_id_of_part(&self, part_uri: &PackUri) -> Option<String> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok()?;
        self.package.get_id_of_part(Some(&main), part_uri)
    }

    /// Part URI for relationship id on the main part (C# `GetPartById`).
    pub fn get_part_by_id(&self, id: &str) -> Option<PackUri> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok()?;
        self.package.get_part_by_id(Some(&main), id)
    }

    /// Change the relationship id of a child part (C# `ChangeIdOfPart`).
    pub fn change_id_of_part(&mut self, part_uri: &PackUri, new_id: &str) -> Result<String> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        self.package
            .change_id_of_part(Some(&main), part_uri, new_id)
    }

    /// Child parts as IdPartPair under the main part (C# `Parts`).
    pub fn id_part_pairs(&self) -> Vec<crate::opc::IdPartPair> {
        let Ok(main) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package.id_part_pairs(Some(&main))
    }

    /// Create a media data part in the package (C# `CreateMediaDataPart`).
    pub fn create_media_data_part(
        &mut self,
        content_type: &str,
        extension: Option<&str>,
    ) -> Result<crate::opc::DataPart> {
        self.package
            .create_media_data_part(content_type, extension)
    }

    /// Delete a package data part if unreferenced (C# `DeletePart(DataPart)`).
    pub fn delete_data_part(&mut self, uri: &PackUri) -> Result<bool> {
        self.package.delete_data_part(uri)
    }


    /// Add a data-part reference from the main part (C# `AddDataPartReferenceRelationship`).
    pub fn add_data_part_reference_relationship(
        &mut self,
        data_part: &crate::opc::DataPart,
        relationship_type: &str,
        id: Option<&str>,
    ) -> Result<crate::opc::DataPartReferenceRelationship> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        self.package.add_data_part_reference_relationship(
            &main,
            data_part,
            relationship_type,
            id,
        )
    }

    /// Data-part references on the main part.
    pub fn data_part_reference_relationships(
        &self,
    ) -> Vec<crate::opc::DataPartReferenceRelationship> {
        let Ok(main) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package
            .data_part_reference_relationships(Some(&main))
    }

    /// Delete a reference relationship by id on the main part
    /// (C# `DeleteReferenceRelationship`).
    pub fn delete_reference_relationship(&mut self, id: &str) -> Option<crate::opc::Relationship> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok()?;
        self.package
            .delete_reference_relationship(Some(&main), id)
    }

    /// Get a reference relationship by id on the main part.
    pub fn get_reference_relationship(&self, id: &str) -> Option<crate::opc::ReferenceRelationship> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok()?;
        self.package
            .get_reference_relationship(Some(&main), id)
    }

    /// Create a relationship from the main part to an existing part
    /// (C# `CreateRelationshipToPart` same-package).
    pub fn create_relationship_to_part(
        &mut self,
        target: &PackUri,
        relationship_type: &str,
        id: Option<&str>,
    ) -> Result<String> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        self.package
            .create_relationship_to_part(&main, target, relationship_type, id)
    }

    /// Create an [`ExtendedPart`] under `xl/udata/` with auto URI.
    pub fn create_extended_part(
        &mut self,
        content_type_str: &str,
        relationship_type: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, crate::packaging::ExtendedPart)> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        let mut index = 1u32;
        let part_uri = loop {
            let candidate = PackUri::new(format!("/xl/udata/data{index}.dat"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package
            .set_part(part_uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &main,
            relationship_type,
            &part_uri,
            RelationshipTargetMode::Internal,
        );
        let part = crate::packaging::ExtendedPart::new(
            part_uri,
            content_type_str,
            relationship_type,
        );
        Ok((rid, part))
    }

    /// Add a new typed child part under the workbook via generated PartInfo
    /// (C# `AddNewPart<T>` shell).
    pub fn add_typed_child_part(
        &mut self,
        part_name: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<crate::packaging::TypedPart> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        crate::packaging::add_typed_part(
            &mut self.package,
            &main,
            Some("WorkbookPart"),
            part_name,
            data,
        )
    }

    /// Create a workbook by cloning an existing package (template).
    pub fn create_from_template(template_path: impl AsRef<Path>) -> Result<Self> {
        Self::create_from_template_as(template_path, None)
    }

    /// Create a workbook by cloning a template, optionally changing document type.
    pub fn create_from_template_as(
        template_path: impl AsRef<Path>,
        document_type: Option<SpreadsheetDocumentType>,
    ) -> Result<Self> {
        let mut src = Self::open(template_path, false)?;
        let mut cloned = src.clone_document()?;
        if let Some(dt) = document_type {
            cloned.change_document_type(dt)?;
        }
        Ok(cloned)
    }

    /// Read cell values from a worksheet by name (or first sheet if `None`).
    ///
    /// Resolves shared-string indices when SST is present.
    pub fn read_sheet_strings(&self) -> Result<Vec<Vec<String>>> {
        self.read_sheet_strings_by_name(None)
    }

    /// Read a single cell's display value (shared-string resolved when possible).
    ///
    /// Returns `None` if the cell does not exist.
    pub fn get_cell_value(&self, sheet_name: &str, reference: &str) -> Result<Option<String>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        let cell = root
            .descendants()
            .find(|e| e.local_name == "c" && e.get_attribute("r") == Some(reference));
        let Some(cell) = cell else {
            return Ok(None);
        };
        // Inline string
        if cell.get_attribute("t") == Some("inlineStr") {
            if let Some(is) = cell.child("is") {
                return Ok(Some(is.inner_text()));
            }
        }
        // Shared string
        if cell.get_attribute("t") == Some("s") {
            if let Some(v) = cell.child("v") {
                let idx: usize = v.inner_text().parse().unwrap_or(0);
                if let Some(sst) = &self.sst {
                    if let Some(s) = sst.get(idx as u32) {
                        return Ok(Some(s.to_string()));
                    }
                }
                // Fall back to package SST
                let sst_uri = PackUri::new(SHARED_STRINGS_URI);
                if let Some(sst_data) = self.package.opc().get_part(&sst_uri) {
                    if let Ok(sst_root) = parse_element(sst_data) {
                        let strings: Vec<String> = sst_root
                            .children_by_name("si")
                            .map(|si| si.inner_text())
                            .collect();
                        if let Some(s) = strings.get(idx) {
                            return Ok(Some(s.clone()));
                        }
                    }
                }
                return Ok(Some(v.inner_text()));
            }
        }
        // Number / general
        if let Some(v) = cell.child("v") {
            return Ok(Some(v.inner_text()));
        }
        Ok(Some(String::new()))
    }

    /// Read the cell style index (`c/@s`) if present.
    pub fn get_cell_style_index(
        &self,
        sheet_name: &str,
        reference: &str,
    ) -> Result<Option<u32>> {
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .find(|e| e.local_name == "c" && e.get_attribute("r") == Some(reference))
            .and_then(|c| c.get_attribute("s"))
            .and_then(|s| s.parse().ok()))
    }

    /// Read a rectangular range as a 2D string grid (empty cells become `""`).
    ///
    /// `range` is A1-style inclusive, e.g. `"A1:C3"`.
    pub fn read_range(&self, sheet_name: &str, range: &str) -> Result<Vec<Vec<String>>> {
        let (from, to) = range.split_once(':').unwrap_or((range, range));
        let (r1, c1) = cell_ref_to_row_col(from)
            .ok_or_else(|| Error::Package(format!("bad range start `{from}`")))?;
        let (r2, c2) = cell_ref_to_row_col(to)
            .ok_or_else(|| Error::Package(format!("bad range end `{to}`")))?;
        let (min_r, max_r) = (r1.min(r2), r1.max(r2));
        let (min_c, max_c) = (c1.min(c2), c1.max(c2));
        let mut grid = Vec::new();
        for r in min_r..=max_r {
            let mut row_vals = Vec::new();
            for c in min_c..=max_c {
                let ref_str = format!("{}{}", column_name(c as usize), r + 1);
                row_vals.push(
                    self.get_cell_value(sheet_name, &ref_str)?
                        .unwrap_or_default(),
                );
            }
            grid.push(row_vals);
        }
        Ok(grid)
    }

    /// Write a 2D string grid into a sheet starting at `start_cell` (A1-style).
    pub fn write_range(
        &mut self,
        sheet_name: &str,
        start_cell: &str,
        values: &[Vec<&str>],
    ) -> Result<()> {
        let (start_r, start_c) = cell_ref_to_row_col(start_cell)
            .ok_or_else(|| Error::Package(format!("bad start cell `{start_cell}`")))?;
        for (ri, row_vals) in values.iter().enumerate() {
            for (ci, val) in row_vals.iter().enumerate() {
                let r = start_r + ri as u32;
                let c = start_c + ci as u32;
                let ref_str = format!("{}{}", column_name(c as usize), r + 1);
                self.set_cell_value(sheet_name, &ref_str, val)?;
            }
        }
        Ok(())
    }

    /// Find cells whose display value contains `needle`.
    ///
    /// Returns A1 references (sheet is not encoded in the reference).
    pub fn find_cells(&self, sheet_name: &str, needle: &str) -> Result<Vec<String>> {
        if needle.is_empty() {
            return Ok(Vec::new());
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let data = match self.package.opc().get_part(&sheet_uri) {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };
        let root = parse_element(data)?;
        let mut hits = Vec::new();
        for cell in root.descendants().filter(|e| e.local_name == "c") {
            let Some(r) = cell.get_attribute("r") else {
                continue;
            };
            if let Some(val) = self.get_cell_value(sheet_name, r)? {
                if val.contains(needle) {
                    hits.push(r.to_string());
                }
            }
        }
        Ok(hits)
    }

    /// Replace display text in inline-string / shared-string cells on a sheet.
    ///
    /// Only mutates cells that currently store string values (not formulas/numbers).
    /// Shared-string replacements update the SST entry (affects all cells sharing it).
    /// Returns the number of cells whose displayed value changed path was touched.
    pub fn replace_in_sheet(
        &mut self,
        sheet_name: &str,
        from: &str,
        to: &str,
    ) -> Result<usize> {
        if from.is_empty() {
            return Ok(0);
        }
        let sheet_uri = self.sheet_uri(sheet_name)?;
        let mut root = self.load_sheet_root(&sheet_uri)?;
        let mut count = 0usize;
        // Walk mutably via recursive helper on sheetData (inline strings)
        if let Some(sd) = root.child_mut("sheetData") {
            count += replace_inline_strings(sd, from, to);
        }
        self.save_sheet_root(&sheet_uri, &root)?;

        // Shared strings table
        let sst_uri = PackUri::new(SHARED_STRINGS_URI);
        if let Some(data) = self.package.opc().get_part(&sst_uri).map(|d| d.to_vec()) {
            if let Ok(mut sst) = parse_element(&data) {
                let n = replace_inline_strings(&mut sst, from, to);
                if n > 0 {
                    count += n;
                    let xml = write_element(&sst)?;
                    self.package.set_part(
                        sst_uri,
                        content_type::SPREADSHEET_SHARED_STRINGS,
                        xml,
                    );
                }
            }
        }
        // Keep in-memory SST builder in sync if present
        if let Some(sst) = self.sst.as_mut() {
            sst.replace_all(from, to);
        }
        Ok(count)
    }

    pub fn read_sheet_strings_by_name(&self, name: Option<&str>) -> Result<Vec<Vec<String>>> {
        let sheet_uri = if let Some(name) = name {
            self.sheets
                .iter()
                .find(|s| s.name == name)
                .map(|s| s.uri.clone())
                .ok_or_else(|| Error::Package(format!("sheet `{name}` not found")))?
        } else if let Some(first) = self.sheets.first() {
            first.uri.clone()
        } else {
            PackUri::new(WORKSHEET1_URI)
        };

        let data = self
            .package
            .opc()
            .get_part(&sheet_uri)
            .ok_or_else(|| Error::PartNotFound(sheet_uri.to_string()))?;
        let root = parse_element(data)?;
        let sheet_data = root
            .descendants()
            .find(|e| e.local_name == "sheetData")
            .ok_or_else(|| Error::Package("no sheetData".into()))?;

        let mut result = Vec::new();
        for row_el in sheet_data.children_by_name("row") {
            let mut row_vals = Vec::new();
            for cell in row_el.children_by_name("c") {
                let cell_type = cell.get_attribute("t").unwrap_or("");
                if cell_type == "inlineStr" {
                    if let Some(is) = cell.child("is") {
                        row_vals.push(is.inner_text());
                    } else {
                        row_vals.push(String::new());
                    }
                } else if cell_type == "s" {
                    let idx = cell
                        .child("v")
                        .map(|v| v.inner_text())
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0);
                    let text = self
                        .sst
                        .as_ref()
                        .and_then(|sst| sst.get(idx))
                        .unwrap_or("")
                        .to_string();
                    row_vals.push(text);
                } else if let Some(v) = cell.child("v") {
                    row_vals.push(v.inner_text());
                } else {
                    row_vals.push(String::new());
                }
            }
            result.push(row_vals);
        }
        Ok(result)
    }

    pub fn save(&mut self) -> Result<()> {
        self.flush_shared_strings()?;
        self.package.save()
    }

    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.flush_shared_strings()?;
        self.package.save_as(path)
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        self.flush_shared_strings()?;
        self.package.to_bytes()
    }

    /// Change the workbook content type (e.g. `.xltx` → `.xlsx`).
    pub fn change_document_type(&mut self, new_type: SpreadsheetDocumentType) -> Result<()> {
        let wb_uri = PackUri::new(WORKBOOK_URI);
        let ct = new_type.content_type();
        let data = self
            .package
            .opc()
            .get_part(&wb_uri)
            .map(|b| b.to_vec())
            .unwrap_or_default();
        self.package.set_part(wb_uri, ct, data);
        self.document_type = new_type;
        Ok(())
    }

    /// Close the workbook, saving if `auto_save` is enabled and a path is set.
    pub fn close(mut self) -> Result<()> {
        if self.package.auto_save()
            && matches!(
                self.package.opc().mode(),
                PackageMode::Create | PackageMode::ReadWrite
            )
            && self.package.path().is_some()
        {
            self.save()?;
        }
        self.package.close_without_save();
        Ok(())
    }
}

impl Drop for SpreadsheetDocument {
    fn drop(&mut self) {
        if self.package.is_closed() {
            return;
        }
        if self.package.auto_save()
            && matches!(
                self.package.opc().mode(),
                PackageMode::Create | PackageMode::ReadWrite
            )
            && self.package.path().is_some()
        {
            let _ = self.flush_shared_strings();
            let _ = self.package.save();
        }
    }
}

/// Replace text content in all `t` nodes under `root` (inline strings / SST).
fn replace_inline_strings(root: &mut OpenXmlElement, from: &str, to: &str) -> usize {
    let mut count = 0usize;
    replace_t_nodes(root, from, to, &mut count);
    count
}

fn replace_t_nodes(elem: &mut OpenXmlElement, from: &str, to: &str, count: &mut usize) {
    if elem.local_name == "t" {
        if let Some(text) = &mut elem.text {
            if text.contains(from) {
                *count += text.matches(from).count();
                *text = text.replace(from, to);
            }
        }
    }
    for child in &mut elem.children {
        replace_t_nodes(child, from, to, count);
    }
}

/// Rewrite cells with `t="s"` under `sheetData` to `inlineStr` using `strings`.
/// Returns whether any cell was changed.
fn count_shared_string_cells(sheet_data: &OpenXmlElement) -> usize {
    let mut n = 0usize;
    for row in &sheet_data.children {
        if row.local_name != "row" {
            continue;
        }
        for cell in &row.children {
            if cell.local_name == "c" && cell.get_attribute("t") == Some("s") {
                n += 1;
            }
        }
    }
    n
}

fn rewrite_shared_string_cells(sheet_data: &mut OpenXmlElement, strings: &[String]) -> bool {
    let mut changed = false;
    for row in sheet_data.children.iter_mut() {
        if row.local_name != "row" {
            continue;
        }
        for cell in row.children.iter_mut() {
            if cell.local_name != "c" {
                continue;
            }
            if cell.get_attribute("t") != Some("s") {
                continue;
            }
            let idx = cell
                .child("v")
                .map(|v| v.inner_text())
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            let text = strings.get(idx).map(|s| s.as_str()).unwrap_or("");
            // Preserve style index if present.
            let style = cell.get_attribute("s").map(|s| s.to_string());
            let reference = cell.get_attribute("r").unwrap_or("").to_string();
            cell.children.clear();
            cell.attributes.retain(|a| a.local_name != "t" && a.local_name != "s");
            // rebuild as inlineStr
            cell.set_attribute("t", "inlineStr");
            if !reference.is_empty() {
                cell.set_attribute("r", &reference);
            }
            if let Some(s) = style {
                cell.set_attribute("s", s);
            }
            let x = crate::namespace::ns::SPREADSHEETML.uri;
            cell.append_child(
                OpenXmlElement::new("x", x, "is")
                    .with_child(OpenXmlElement::new("x", x, "t").with_text(text)),
            );
            changed = true;
        }
    }
    changed
}

/// Shrink a table `ref` like `A1:C10` by one column on the right → `A1:B10`.
/// Returns `None` if the range cannot be parsed or is a single column.
fn shrink_table_ref_last_col(reference: &str) -> Option<String> {
    let (start, end) = reference.split_once(':')?;
    let (end_row, end_col) = cell_ref_to_row_col(end)?;
    if end_col == 0 {
        return None;
    }
    let new_end = format!("{}{}", column_name(end_col as usize - 1), end_row + 1);
    // Validate start still parses (best-effort).
    let _ = cell_ref_to_row_col(start)?;
    Some(format!("{start}:{new_end}"))
}



/// Whether a relationship `target` (possibly relative to `source`) resolves to `absolute_uri`.
fn relationship_targets_uri(source: &PackUri, target: &str, absolute_uri: &str) -> bool {
    if target == absolute_uri {
        return true;
    }
    if target.ends_with(absolute_uri.trim_start_matches('/')) {
        return true;
    }
    if absolute_uri.ends_with(target.trim_start_matches("./")) {
        return true;
    }
    if let Ok(resolved) = crate::opc::resolve_uri(source, target) {
        if resolved.as_str() == absolute_uri {
            return true;
        }
    }
    false
}

/// Whether a drawing anchor subtree references relationship id `rid` (e.g. on `c:chart/@r:id`).
fn anchor_references_rid(anchor: &OpenXmlElement, rid: &str) -> bool {
    for e in anchor.descendants() {
        if e.local_name == "chart" || e.local_name == "blip" || e.local_name == "hlinkClick" {
            let id = e
                .get_attribute_qname("r:id")
                .or_else(|| e.get_attribute("id"));
            if id == Some(rid) {
                return true;
            }
        }
        // graphicFrame may carry r:id on nested elements only — also check any r:id attr.
        for a in &e.attributes {
            if a.local_name == "id" && (a.prefix.as_deref() == Some("r") || a.prefix.is_none()) {
                if a.value == rid && e.local_name == "chart" {
                    return true;
                }
            }
        }
    }
    // Broader: any descendant attribute r:id == rid under this anchor.
    for e in anchor.descendants() {
        if e.get_attribute_qname("r:id") == Some(rid) {
            return true;
        }
    }
    false
}

/// Convert 0-based column index to Excel column name (A, B, …, Z, AA, …).
pub fn column_name(mut index: usize) -> String {
    let mut s = String::new();
    loop {
        s.insert(0, (b'A' + (index % 26) as u8) as char);
        if index < 26 {
            break;
        }
        index = index / 26 - 1;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_names() {
        assert_eq!(column_name(0), "A");
        assert_eq!(column_name(25), "Z");
        assert_eq!(column_name(26), "AA");
    }
}
