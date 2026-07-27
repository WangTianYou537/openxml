# officexml

[中文文档](README.zh-CN.md)

Pure-Rust port of the [Open XML SDK](https://github.com/OfficeDev/Open-XML-SDK) for creating and editing Microsoft Office packages:

| Format | Extension | Entry type |
|--------|-----------|------------|
| Word | `.docx` / `.dotx` | `WordprocessingDocument` |
| Excel | `.xlsx` / `.xlsm` | `SpreadsheetDocument` |
| PowerPoint | `.pptx` / `.ppsx` | `PresentationDocument` |

**Crate:** `officexml` · **Repo:** https://github.com/WangTianYou537/openxml  
**Not on crates.io** — full package (generated schemas + bundled fonts) exceeds registry size limits without cutting features.

License: **MIT** (same as upstream Open XML SDK).

---

## Highlights

- **OPC packaging** — ZIP, `[Content_Types].xml`, relationships, Flat OPC, core/app/custom properties
- **OpenXmlElement DOM** — parse / write / walk / rewrite with Markup Compatibility (AC, Ignorable, ProcessContent)
- **Word / Excel / PowerPoint** — create, open, save, clone; styles, headers/footers, charts, tables, notes, masters, …
- **Validation** — package structure, part constraints, schema particles, semantic relationship rules
- **Schema codegen** — typed constructors and part graph from the C# SDK `data/*.json`
- **`svg2pptx` CLI** — multi-slide SVG → native DrawingML shapes (editable text or glyph outlines)

Full milestone list: [PORTING.md](PORTING.md). API guide: [docs/USAGE.md](docs/USAGE.md). Gap vs C#: [docs/GAP_ANALYSIS.md](docs/GAP_ANALYSIS.md).

---

## Build

```bash
cargo build --release
cargo test --locked --all-targets
cargo doc --no-deps --open
```

### As a path dependency

```toml
[dependencies]
officexml = { path = "/path/to/openxml" }
```

---

## Quick start

### Word

```rust
use officexml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
use officexml::wordprocessing::{body, document, paragraph, run, text};

fn main() -> officexml::Result<()> {
    let mut doc = WordprocessingDocument::create(
        "hello.docx",
        WordprocessingDocumentType::Document,
    )?;
    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![run(vec![text("Hello from Rust!")])]),
    ])]));
    doc.set_title("Hello")?;
    doc.save()?;
    Ok(())
}
```

```rust
let mut doc = WordprocessingDocument::open("hello.docx", false)?;
for p in doc.paragraph_texts()? {
    println!("{p}");
}
```

### Excel

```rust
use officexml::packaging::{SpreadsheetDocument, SpreadsheetDocumentType};

fn main() -> officexml::Result<()> {
    let mut wb = SpreadsheetDocument::create(
        "grid.xlsx",
        SpreadsheetDocumentType::Workbook,
    )?;
    wb.write_sheet_strings("Sheet1", &[
        vec!["Name", "Score"],
        vec!["Alice", "95"],
    ])?;
    wb.save()?;
    Ok(())
}
```

### PowerPoint

```rust
use officexml::packaging::{PresentationDocument, PresentationDocumentType};

fn main() -> officexml::Result<()> {
    let mut ppt = PresentationDocument::create(
        "deck.pptx",
        PresentationDocumentType::Presentation,
    )?;
    ppt.add_slide_with_text("Title slide")?;
    ppt.add_slide_with_text("Second slide")?;
    ppt.save()?;
    Ok(())
}
```

### Examples

```bash
cargo run --example create_word -- /tmp/hello.docx
cargo run --example read_word -- /tmp/hello.docx
cargo run --example create_report -- /tmp/report.docx
cargo run --example create_spreadsheet -- /tmp/report.xlsx
cargo run --example create_presentation -- /tmp/deck.pptx
cargo run --example create_ppt_from_svg_shapes -- /tmp/from-svg.pptx
```

---

## `svg2pptx` CLI

Convert one or more SVGs into a `.pptx`. Each SVG becomes a full-bleed 16:9 slide of **native DrawingML shapes** (not an embedded SVG picture).

```bash
cargo build --release --bin svg2pptx

# multi-slide
./target/release/svg2pptx -o deck.pptx a.svg b.svg c.svg

# output defaults to <first-stem>.pptx
./target/release/svg2pptx a.svg b.svg
```

### Font modes

| Flag | Behavior |
|------|----------|
| *(default)* | Editable text boxes; system faces when SVG omits `font-family` (Times New Roman / Microsoft YaHei) |
| `--font-shape` | Outline glyphs as geometry — no text boxes, no font embed |
| `--embed-font` | Editable text + on-demand **subset** EOT (`.fntdata`) of used faces |
| `--embed-font-fully` | Editable text + **full** EOT embed of used faces |

```bash
svg2pptx --font-shape -o outlined.pptx poster.svg
svg2pptx --embed-font -o embedded.pptx poster.svg
svg2pptx --embed-font-fully -o full.pptx poster.svg
```

### Bundled fonts

Cross-platform measurement / outlining does not require system font packages. The checkout ships:

```text
assets/fonts/
  LiberationSans-*.ttf    # Arial / Helvetica stand-in
  LiberationSerif-*.ttf   # Times New Roman stand-in
  DejaVuSans-*.ttf
  NotoSansSC-*.ttf        # CJK embed (TrueType, ODTTF-safe)
```

Subset embed (`--embed-font`) needs Python 3 + [fontTools](https://github.com/fonttools/fonttools) (`scripts/subset_ttf.py`).

---

## Architecture

```text
src/
  packaging/       WordprocessingDocument / SpreadsheetDocument / PresentationDocument
  wordprocessing/  Document, Body, Paragraph, Run, Text, tables, notes, …
  spreadsheet/     Workbook, sheets, SST, charts, CF, pivots, …
  presentation/    Slides, masters/layouts, notes, svg_to_shapes
  element/         OpenXmlElement DOM + XML R/W
  opc/             ZIP, content types, relationships, Flat OPC, properties
  validation/      Package / particle / semantic validators
  markup_compatibility/  AlternateContent, Ignorable, ProcessContent
  generated/       Codegen from C# data/*.json (parts, schemas, particles)
  bin/svg2pptx.rs  Multi-slide SVG → PPTX CLI
  bin/openxml-codegen.rs  Schema/part generator (feature = "codegen")
```

Mirrors the C# layers in `DocumentFormat.OpenXml.Framework` + `DocumentFormat.OpenXml/Packaging`.

### Regenerate schema bindings

```bash
cargo run --features codegen --bin openxml-codegen -- \
  --data /path/to/Open-XML-SDK/data \
  --out src/generated
```

Default schema is `wordprocessingml_2006_main`; pass `--schema all` for the full set (large).

---

## Status (summary)

| Area | Status |
|------|--------|
| OPC package (ZIP, content types, rels, Flat OPC) | ✅ |
| OpenXmlElement DOM + MC | ✅ |
| Word / Excel / PowerPoint create · open · save | ✅ |
| Styles, headers/footers, images, hyperlinks, tables | ✅ |
| Excel multi-sheet, SST, charts, CF, pivots, protection | ✅ |
| PPT masters/layouts, notes, transitions, theme | ✅ |
| Package + particle + semantic validation | ✅ |
| Schema / part / particle codegen | ✅ |
| SVG → native DrawingML (`svg2pptx`) | ✅ |
| Digital signature crypto / VBA execute | partial / non-goal |

See [PORTING.md](PORTING.md) for the complete checklist.

---

## CI & releases

GitHub Actions runs `cargo test --locked --all-targets` on **Ubuntu, Windows, and macOS** for every push to `main`.

Tag a version to publish multi-platform `svg2pptx` binaries:

```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## Documentation

| Doc | Content |
|-----|---------|
| [docs/USAGE.md](docs/USAGE.md) | Install, Word/Excel/PPT APIs, MC, validation, codegen, C# map |
| [docs/GAP_ANALYSIS.md](docs/GAP_ANALYSIS.md) | Gaps vs upstream Open-XML-SDK |
| [PORTING.md](PORTING.md) | Port milestones and design notes |
| `cargo doc --no-deps --open` | Generated rustdoc |

---

## Development

```bash
cargo test --locked --all-targets
cargo build --release --bin svg2pptx
```

Upstream reference (C#):

- Framework: `Open-XML-SDK/src/DocumentFormat.OpenXml.Framework/`
- Packages: `Open-XML-SDK/src/DocumentFormat.OpenXml/Packaging/`
- Schema data: `Open-XML-SDK/data/schemas/`, `data/parts/`, `data/namespaces.json`

---

## License

MIT — same as the [Open XML SDK](https://github.com/OfficeDev/Open-XML-SDK).
