//! Lightweight strong-typed views over [`OpenXmlElement`].
//!
//! Full C# parity would generate thousands of element classes. This module provides
//! thin typed wrappers for the highest-traffic Word elements with property accessors,
//! without abandoning the owned-DOM model.

use super::OpenXmlElement;
use crate::namespace::ns;

/// Typed view of a WordprocessingML paragraph (`w:p`).
#[derive(Debug, Clone)]
pub struct Paragraph {
    pub(crate) inner: OpenXmlElement,
}

impl Paragraph {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::w("p"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "p" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    /// Paragraph style id (`w:pPr/w:pStyle/@w:val`), if any.
    pub fn style_id(&self) -> Option<&str> {
        self.inner
            .child("pPr")
            .and_then(|p| p.child("pStyle"))
            .and_then(|s| {
                s.get_attribute("val")
                    .or_else(|| s.get_attribute_qname("w:val"))
            })
    }

    pub fn set_style_id(&mut self, style_id: &str) {
        let ppr = if let Some(p) = self.inner.child_mut("pPr") {
            p
        } else {
            self.inner.children.insert(0, OpenXmlElement::w("pPr"));
            self.inner.child_mut("pPr").unwrap()
        };
        if let Some(ps) = ppr.child_mut("pStyle") {
            ps.set_attribute_qname("w:val", style_id);
        } else {
            ppr.append_child(
                OpenXmlElement::w("pStyle").with_attribute_qname("w:val", style_id),
            );
        }
    }

    /// Runs (`w:r`) under this paragraph.
    pub fn runs(&self) -> impl Iterator<Item = Run> + '_ {
        self.inner
            .children_by_name("r")
            .filter_map(|c| Run::from_element(c.clone()))
    }

    pub fn append_run(&mut self, run: Run) {
        self.inner.append_child(run.into_inner());
    }

    /// Concatenated text of all runs.
    pub fn text(&self) -> String {
        self.inner.inner_text()
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        let mut p = Self::new();
        p.append_run(Run::with_text(text));
        p
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Paragraph> for OpenXmlElement {
    fn from(p: Paragraph) -> Self {
        p.inner
    }
}

/// Typed view of a WordprocessingML run (`w:r`).
#[derive(Debug, Clone)]
pub struct Run {
    pub(crate) inner: OpenXmlElement,
}

impl Run {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::w("r"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "r" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        let mut r = Self::new();
        r.set_text(text);
        r
    }

    pub fn text(&self) -> String {
        self.inner
            .children_by_name("t")
            .map(|t| t.inner_text())
            .collect()
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        let text = text.into();
        // replace all w:t or add one
        self.inner.children.retain(|c| c.local_name != "t");
        self.inner.append_child(OpenXmlElement::w("t").with_text(text));
    }

    pub fn bold(&self) -> bool {
        self.inner
            .child("rPr")
            .and_then(|r| r.child("b"))
            .is_some()
    }

    pub fn set_bold(&mut self, enabled: bool) {
        let rpr = self.ensure_rpr();
        rpr.children.retain(|c| c.local_name != "b");
        if enabled {
            rpr.append_child(OpenXmlElement::w("b"));
        }
    }

    pub fn italic(&self) -> bool {
        self.inner
            .child("rPr")
            .and_then(|r| r.child("i"))
            .is_some()
    }

    pub fn set_italic(&mut self, enabled: bool) {
        let rpr = self.ensure_rpr();
        rpr.children.retain(|c| c.local_name != "i");
        if enabled {
            rpr.append_child(OpenXmlElement::w("i"));
        }
    }

    /// Font size in half-points (`w:sz/@w:val`).
    pub fn font_size_half_points(&self) -> Option<u32> {
        self.inner
            .child("rPr")
            .and_then(|r| r.child("sz"))
            .and_then(|s| {
                s.get_attribute("val")
                    .or_else(|| s.get_attribute_qname("w:val"))
            })
            .and_then(|v| v.parse().ok())
    }

    pub fn set_font_size_half_points(&mut self, half_points: u32) {
        let rpr = self.ensure_rpr();
        if let Some(sz) = rpr.child_mut("sz") {
            sz.set_attribute_qname("w:val", half_points.to_string());
        } else {
            rpr.append_child(
                OpenXmlElement::w("sz").with_attribute_qname("w:val", half_points.to_string()),
            );
        }
    }

    fn ensure_rpr(&mut self) -> &mut OpenXmlElement {
        if self.inner.child("rPr").is_none() {
            self.inner.children.insert(0, OpenXmlElement::w("rPr"));
        }
        self.inner.child_mut("rPr").unwrap()
    }
}

impl Default for Run {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Run> for OpenXmlElement {
    fn from(r: Run) -> Self {
        r.inner
    }
}

/// Typed view of a WordprocessingML body (`w:body`).
#[derive(Debug, Clone)]
pub struct Body {
    pub(crate) inner: OpenXmlElement,
}

impl Body {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::w("body"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "body" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = Paragraph> + '_ {
        self.inner
            .children_by_name("p")
            .filter_map(|c| Paragraph::from_element(c.clone()))
    }

    pub fn append_paragraph(&mut self, p: Paragraph) {
        // insert before sectPr if present
        if let Some(idx) = self.inner.children.iter().position(|c| c.local_name == "sectPr") {
            self.inner.children.insert(idx, p.into_inner());
        } else {
            self.inner.append_child(p.into_inner());
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Body> for OpenXmlElement {
    fn from(b: Body) -> Self {
        b.inner
    }
}

/// Typed view of a WordprocessingML document (`w:document`).
#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) inner: OpenXmlElement,
}

impl Document {
    pub fn new() -> Self {
        let mut el = OpenXmlElement::w("document");
        el.namespace_declarations
            .push(("w".into(), ns::WORDPROCESSINGML.uri.into()));
        el.append_child(OpenXmlElement::w("body"));
        Self { inner: el }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "document" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn body(&self) -> Option<Body> {
        self.inner
            .child("body")
            .and_then(|b| Body::from_element(b.clone()))
    }

    pub fn body_mut(&mut self) -> Option<&mut OpenXmlElement> {
        self.inner.child_mut("body")
    }

    pub fn set_body(&mut self, body: Body) {
        self.inner.children.retain(|c| c.local_name != "body");
        self.inner.append_child(body.into_inner());
    }

    pub fn with_paragraphs(paragraphs: impl IntoIterator<Item = Paragraph>) -> Self {
        let mut doc = Self::new();
        let mut body = Body::new();
        for p in paragraphs {
            body.append_paragraph(p);
        }
        doc.set_body(body);
        doc
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Document> for OpenXmlElement {
    fn from(d: Document) -> Self {
        d.inner
    }
}

/// Typed text run content (`w:t`).
#[derive(Debug, Clone)]
pub struct Text {
    pub(crate) inner: OpenXmlElement,
}

impl Text {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            inner: OpenXmlElement::w("t").with_text(value),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "t" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn value(&self) -> String {
        self.inner.inner_text()
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.inner.text = Some(value.into());
    }
}

impl From<Text> for OpenXmlElement {
    fn from(t: Text) -> Self {
        t.inner
    }
}

/// Word table cell (`w:tc`).
#[derive(Debug, Clone)]
pub struct TableCell {
    pub(crate) inner: OpenXmlElement,
}

impl TableCell {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::w("tc"),
        }
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        let mut cell = Self::new();
        cell.append_paragraph(Paragraph::with_text(text));
        cell
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "tc" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = Paragraph> + '_ {
        self.inner
            .children_by_name("p")
            .filter_map(|c| Paragraph::from_element(c.clone()))
    }

    pub fn append_paragraph(&mut self, p: Paragraph) {
        self.inner.append_child(p.into_inner());
    }

    pub fn text(&self) -> String {
        self.inner.inner_text()
    }
}

impl Default for TableCell {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TableCell> for OpenXmlElement {
    fn from(c: TableCell) -> Self {
        c.inner
    }
}

/// Word table row (`w:tr`).
#[derive(Debug, Clone)]
pub struct TableRow {
    pub(crate) inner: OpenXmlElement,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::w("tr"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "tr" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn cells(&self) -> impl Iterator<Item = TableCell> + '_ {
        self.inner
            .children_by_name("tc")
            .filter_map(|c| TableCell::from_element(c.clone()))
    }

    pub fn append_cell(&mut self, cell: TableCell) {
        self.inner.append_child(cell.into_inner());
    }

    pub fn with_strings(cells: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut row = Self::new();
        for s in cells {
            row.append_cell(TableCell::with_text(s));
        }
        row
    }
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TableRow> for OpenXmlElement {
    fn from(r: TableRow) -> Self {
        r.inner
    }
}

/// Word table (`w:tbl`).
#[derive(Debug, Clone)]
pub struct Table {
    pub(crate) inner: OpenXmlElement,
}

impl Table {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::w("tbl"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "tbl" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn rows(&self) -> impl Iterator<Item = TableRow> + '_ {
        self.inner
            .children_by_name("tr")
            .filter_map(|c| TableRow::from_element(c.clone()))
    }

    pub fn append_row(&mut self, row: TableRow) {
        self.inner.append_child(row.into_inner());
    }

    /// Build a table from a rectangular grid of strings.
    pub fn from_strings(grid: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<String>>>) -> Self {
        let mut table = Self::new();
        for row in grid {
            table.append_row(TableRow::with_strings(row));
        }
        table
    }

    /// Flatten to row/cell strings.
    pub fn to_strings(&self) -> Vec<Vec<String>> {
        self.rows()
            .map(|r| r.cells().map(|c| c.text()).collect())
            .collect()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Table> for OpenXmlElement {
    fn from(t: Table) -> Self {
        t.inner
    }
}

/// Spreadsheet cell (`x:c`).
#[derive(Debug, Clone)]
pub struct Cell {
    pub(crate) inner: OpenXmlElement,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::x("c"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "c" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn reference(&self) -> Option<&str> {
        self.inner
            .get_attribute("r")
            .or_else(|| self.inner.get_attribute_qname("r"))
    }

    pub fn set_reference(&mut self, reference: &str) {
        self.inner.set_attribute("r", reference);
    }

    pub fn value(&self) -> Option<String> {
        self.inner.child("v").map(|v| v.inner_text())
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        let value = value.into();
        if let Some(v) = self.inner.child_mut("v") {
            v.text = Some(value);
        } else {
            self.inner
                .append_child(OpenXmlElement::x("v").with_text(value));
        }
    }

    pub fn with_value(reference: &str, value: impl Into<String>) -> Self {
        let mut c = Self::new();
        c.set_reference(reference);
        c.set_value(value);
        c
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Cell> for OpenXmlElement {
    fn from(c: Cell) -> Self {
        c.inner
    }
}

/// Spreadsheet worksheet root (`x:worksheet`).
#[derive(Debug, Clone)]
pub struct Worksheet {
    pub(crate) inner: OpenXmlElement,
}

impl Worksheet {
    pub fn new() -> Self {
        let mut el = OpenXmlElement::x("worksheet");
        el.append_child(OpenXmlElement::x("sheetData"));
        Self { inner: el }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "worksheet" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn sheet_data(&self) -> Option<&OpenXmlElement> {
        self.inner.child("sheetData")
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell> + '_ {
        self.inner
            .descendants()
            .filter(|e| e.local_name == "c")
            .filter_map(|c| Cell::from_element(c.clone()))
    }

    pub fn append_cell_to_row(&mut self, row_index: u32, cell: Cell) {
        let sheet = if let Some(_) = self.inner.child("sheetData") {
            self.inner.child_mut("sheetData").unwrap()
        } else {
            self.inner.append_child(OpenXmlElement::x("sheetData"));
            self.inner.child_mut("sheetData").unwrap()
        };
        let row_s = row_index.to_string();
        let row = if let Some(r) = sheet.children.iter_mut().find(|c| {
            c.local_name == "row"
                && c.get_attribute("r")
                    .or_else(|| c.get_attribute_qname("r"))
                    .map(|v| v == row_s)
                    .unwrap_or(false)
        }) {
            r
        } else {
            sheet.append_child(OpenXmlElement::x("row").with_attribute("r", &row_s));
            sheet.children.last_mut().unwrap()
        };
        row.append_child(cell.into_inner());
    }
}

impl Default for Worksheet {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Worksheet> for OpenXmlElement {
    fn from(w: Worksheet) -> Self {
        w.inner
    }
}

/// PowerPoint slide (`p:sld`).
#[derive(Debug, Clone)]
pub struct Slide {
    pub(crate) inner: OpenXmlElement,
}

impl Slide {
    pub fn new() -> Self {
        let mut el = OpenXmlElement::p("sld");
        let mut c_sld = OpenXmlElement::p("cSld");
        c_sld.append_child(OpenXmlElement::p("spTree"));
        el.append_child(c_sld);
        Self { inner: el }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "sld" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn text(&self) -> String {
        self.inner.inner_text()
    }

    /// Append a simple text body under the shape tree.
    pub fn append_text_box(&mut self, text: impl Into<String>) {
        let text = text.into();
        let tree = self
            .inner
            .child_mut("cSld")
            .and_then(|c| c.child_mut("spTree"));
        if let Some(tree) = tree {
            let mut sp = OpenXmlElement::p("sp");
            let mut tx = OpenXmlElement::p("txBody");
            let mut p = OpenXmlElement::a("p");
            let mut r = OpenXmlElement::a("r");
            r.append_child(OpenXmlElement::a("t").with_text(text));
            p.append_child(r);
            tx.append_child(p);
            sp.append_child(tx);
            tree.append_child(sp);
        }
    }
}

impl Default for Slide {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Slide> for OpenXmlElement {
    fn from(s: Slide) -> Self {
        s.inner
    }
}


/// Word style definition (`w:style`).
#[derive(Debug, Clone)]
pub struct Style {
    pub(crate) inner: OpenXmlElement,
}

impl Style {
    pub fn new(style_id: &str, style_type: &str) -> Self {
        Self {
            inner: OpenXmlElement::w("style")
                .with_attribute_qname("w:type", style_type)
                .with_attribute_qname("w:styleId", style_id),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "style" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn style_id(&self) -> Option<&str> {
        self.inner
            .get_attribute("styleId")
            .or_else(|| self.inner.get_attribute_qname("w:styleId"))
    }

    pub fn style_type(&self) -> Option<&str> {
        self.inner
            .get_attribute("type")
            .or_else(|| self.inner.get_attribute_qname("w:type"))
    }

    pub fn name(&self) -> Option<&str> {
        self.inner.child("name").and_then(|n| {
            n.get_attribute("val")
                .or_else(|| n.get_attribute_qname("w:val"))
        })
    }

    pub fn set_name(&mut self, name: &str) {
        if let Some(n) = self.inner.child_mut("name") {
            n.set_attribute_qname("w:val", name);
        } else {
            self.inner
                .append_child(OpenXmlElement::w("name").with_attribute_qname("w:val", name));
        }
    }
}

impl From<Style> for OpenXmlElement {
    fn from(s: Style) -> Self {
        s.inner
    }
}

/// Word hyperlink (`w:hyperlink`).
#[derive(Debug, Clone)]
pub struct Hyperlink {
    pub(crate) inner: OpenXmlElement,
}

impl Hyperlink {
    pub fn new(relationship_id: &str) -> Self {
        Self {
            inner: OpenXmlElement::w("hyperlink").with_attribute_qname("r:id", relationship_id),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "hyperlink" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn relationship_id(&self) -> Option<&str> {
        self.inner
            .get_attribute("id")
            .or_else(|| self.inner.get_attribute_qname("r:id"))
    }

    pub fn anchor(&self) -> Option<&str> {
        self.inner
            .get_attribute("anchor")
            .or_else(|| self.inner.get_attribute_qname("w:anchor"))
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.inner.append_child(Run::with_text(text).into_inner());
        self
    }

    pub fn text(&self) -> String {
        self.inner.inner_text()
    }
}

impl From<Hyperlink> for OpenXmlElement {
    fn from(h: Hyperlink) -> Self {
        h.inner
    }
}

/// Word comment (`w:comment`).
#[derive(Debug, Clone)]
pub struct Comment {
    pub(crate) inner: OpenXmlElement,
}

impl Comment {
    pub fn new(id: &str, author: &str, initials: &str) -> Self {
        let mut el = OpenXmlElement::w("comment")
            .with_attribute_qname("w:id", id)
            .with_attribute_qname("w:author", author)
            .with_attribute_qname("w:initials", initials);
        el.append_child(Paragraph::with_text("").into_inner());
        Self { inner: el }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "comment" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn id(&self) -> Option<&str> {
        self.inner
            .get_attribute("id")
            .or_else(|| self.inner.get_attribute_qname("w:id"))
    }

    pub fn author(&self) -> Option<&str> {
        self.inner
            .get_attribute("author")
            .or_else(|| self.inner.get_attribute_qname("w:author"))
    }

    pub fn text(&self) -> String {
        self.inner.inner_text()
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.inner.children.retain(|c| c.local_name != "p");
        self.inner
            .append_child(Paragraph::with_text(text).into_inner());
    }
}

impl From<Comment> for OpenXmlElement {
    fn from(c: Comment) -> Self {
        c.inner
    }
}

/// Word header (`w:hdr`).
#[derive(Debug, Clone)]
pub struct Header {
    pub(crate) inner: OpenXmlElement,
}

impl Header {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::w("hdr"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "hdr" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn with_paragraph(mut self, p: Paragraph) -> Self {
        self.inner.append_child(p.into_inner());
        self
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = Paragraph> + '_ {
        self.inner
            .children_by_name("p")
            .filter_map(|c| Paragraph::from_element(c.clone()))
    }

    pub fn text(&self) -> String {
        self.inner.inner_text()
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Header> for OpenXmlElement {
    fn from(h: Header) -> Self {
        h.inner
    }
}

/// Word footer (`w:ftr`).
#[derive(Debug, Clone)]
pub struct Footer {
    pub(crate) inner: OpenXmlElement,
}

impl Footer {
    pub fn new() -> Self {
        Self {
            inner: OpenXmlElement::w("ftr"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "ftr" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement {
        &mut self.inner
    }

    pub fn with_paragraph(mut self, p: Paragraph) -> Self {
        self.inner.append_child(p.into_inner());
        self
    }

    pub fn text(&self) -> String {
        self.inner.inner_text()
    }
}

impl Default for Footer {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Footer> for OpenXmlElement {
    fn from(f: Footer) -> Self {
        f.inner
    }
}

/// Notes container root (`w:footnotes` / `w:endnotes`).
#[derive(Debug, Clone)]
pub struct Notes {
    pub(crate) inner: OpenXmlElement,
}

impl Notes {
    pub fn footnotes() -> Self {
        Self {
            inner: OpenXmlElement::w("footnotes"),
        }
    }

    pub fn endnotes() -> Self {
        Self {
            inner: OpenXmlElement::w("endnotes"),
        }
    }

    pub fn from_element(el: OpenXmlElement) -> Option<Self> {
        if el.local_name == "footnotes" || el.local_name == "endnotes" {
            Some(Self { inner: el })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> OpenXmlElement {
        self.inner
    }

    pub fn as_element(&self) -> &OpenXmlElement {
        &self.inner
    }

    pub fn note_count(&self) -> usize {
        self.inner
            .children
            .iter()
            .filter(|c| c.local_name == "footnote" || c.local_name == "endnote")
            .count()
    }

    pub fn append_note(&mut self, id: &str, text: impl Into<String>) {
        let name = if self.inner.local_name == "endnotes" {
            "endnote"
        } else {
            "footnote"
        };
        let mut note = OpenXmlElement::w(name).with_attribute_qname("w:id", id);
        note.append_child(Paragraph::with_text(text).into_inner());
        self.inner.append_child(note);
    }
}

impl From<Notes> for OpenXmlElement {
    fn from(n: Notes) -> Self {
        n.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraph_run_text_and_style() {
        let mut p = Paragraph::with_text("hello");
        p.set_style_id("Heading1");
        assert_eq!(p.style_id(), Some("Heading1"));
        assert_eq!(p.text(), "hello");
        let mut r = Run::with_text("x");
        r.set_bold(true);
        r.set_font_size_half_points(24);
        assert!(r.bold());
        assert_eq!(r.font_size_half_points(), Some(24));
        p.append_run(r);
        assert!(p.text().contains('x'));
    }

    #[test]
    fn document_with_paragraphs() {
        let doc = Document::with_paragraphs([
            Paragraph::with_text("a"),
            Paragraph::with_text("b"),
        ]);
        let body = doc.body().unwrap();
        let texts: Vec<_> = body.paragraphs().map(|p| p.text()).collect();
        assert_eq!(texts, vec!["a", "b"]);
        let el: OpenXmlElement = doc.into();
        assert_eq!(el.local_name, "document");
    }

    #[test]
    fn table_from_strings() {
        let t = Table::from_strings([["a", "b"], ["c", "d"]]);
        assert_eq!(t.to_strings(), vec![vec!["a", "b"], vec!["c", "d"]]);
    }

    #[test]
    fn worksheet_cells() {
        let mut ws = Worksheet::new();
        ws.append_cell_to_row(1, Cell::with_value("A1", "42"));
        let cells: Vec<_> = ws.cells().collect();
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].reference(), Some("A1"));
        assert_eq!(cells[0].value().as_deref(), Some("42"));
    }

    #[test]
    fn slide_text_box() {
        let mut s = Slide::new();
        s.append_text_box("hi");
        assert!(s.text().contains("hi"));
    }

    #[test]
    fn style_hyperlink_comment_header() {
        let mut st = Style::new("Heading1", "paragraph");
        st.set_name("Heading 1");
        assert_eq!(st.style_id(), Some("Heading1"));
        assert_eq!(st.name(), Some("Heading 1"));
        let h = Hyperlink::new("rId1").with_text("link");
        assert_eq!(h.relationship_id(), Some("rId1"));
        assert!(h.text().contains("link"));
        let mut c = Comment::new("0", "Alice", "A");
        c.set_text("note");
        assert_eq!(c.author(), Some("Alice"));
        assert!(c.text().contains("note"));
        let hdr = Header::new().with_paragraph(Paragraph::with_text("hdr"));
        assert!(hdr.text().contains("hdr"));
        let mut notes = Notes::footnotes();
        notes.append_note("1", "fn");
        assert_eq!(notes.note_count(), 1);
    }
}
