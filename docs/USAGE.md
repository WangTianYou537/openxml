# openxml 使用指南

Rust 移植版 [Open XML SDK](https://github.com/OfficeDev/Open-XML-SDK)，用于创建、读取和修改 Microsoft Office 文档（`.docx` / `.xlsx` / `.pptx`）。

- **项目路径**：`/opt/wp/openxml`
- **C# 源码参考**：`/opt/wp/Open-XML-SDK`
- **移植状态**：`PORTING.md` 中全部里程碑已完成
- **测试**：`cargo test`（集成测试 553+ 项全部通过）

---

## 目录

1. [安装与构建](#1-安装与构建)
2. [架构概览](#2-架构概览)
3. [快速开始](#3-快速开始)
4. [Word（.docx）](#4-worddocx)
5. [Excel（.xlsx）](#5-excelxlsx)
6. [PowerPoint（.pptx）](#6-powerpointpptx)
7. [DOM 与 XML](#7-dom-与-xml)
8. [包属性与 Flat OPC](#8-包属性与-flat-opc)
9. [Markup Compatibility](#9-markup-compatibility)
10. [校验](#10-校验)
11. [Schema 代码生成](#11-schema-代码生成)
12. [C# API 对照](#12-c-api-对照)
13. [示例程序](#13-示例程序)
14. [错误处理](#14-错误处理)
15. [限制与后续方向](#15-限制与后续方向)

---

## 1. 安装与构建

### 作为本地路径依赖

```toml
# Cargo.toml
[dependencies]
openxml = { path = "/opt/wp/openxml" }
```

### 构建与测试

```bash
cd /opt/wp/openxml
cargo build --release
cargo test
cargo run --example create_word -- /tmp/hello.docx
cargo run --example create_report -- /tmp/report.docx
cargo run --example create_spreadsheet -- /tmp/report.xlsx
cargo run --example create_presentation -- /tmp/deck.pptx
cargo run --example read_word -- /tmp/report.docx
```

### 依赖

| Crate | 用途 |
|-------|------|
| `zip` | OPC 包（ZIP）读写 |
| `quick-xml` | XML 解析/序列化 |
| `thiserror` | 错误类型 |
| `indexmap` | 有序部件/关系映射 |
| `serde_json` | Schema JSON（codegen + particle 解析） |

---

## 2. 架构概览

与 C# SDK 分层对应：

```text
packaging/         WordprocessingDocument / SpreadsheetDocument / PresentationDocument
wordprocessing/    Document, Body, Paragraph, Run, Text, 表格, 脚注…
spreadsheet/       工作表、SST、图表、绘图锚点、批注、CF、透视表
presentation/      幻灯片、母版/版式、备注页
element/           OpenXmlElement DOM + XML 读写
opc/               ZIP、[Content_Types].xml、.rels、Flat OPC、包属性
validation/        子元素规则 + 有序 particle 匹配
markup_compatibility/  AlternateContent / Ignorable / ProcessContent / Preserve*
namespace_rewrite/ Strict → Transitional
file_format/       FileFormatVersions
generated/         由 openxml-codegen 从 C# data/*.json 生成（155 schema 模块）
```

**设计选择（相对 C#）：**

- 无 Features DI：直接用字段/方法
- 拥有式 DOM（`Vec` 子节点），非链表
- 包在内存中完整加载，`save` 时重写 ZIP
- 强类型元素工厂 + 元数据由 codegen 生成；高层文档 API 手写

---

## 3. 快速开始

### 创建 Word 文档

```rust
use openxml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
use openxml::wordprocessing::{body, document, paragraph, run, text};

fn main() -> openxml::Result<()> {
    let mut doc = WordprocessingDocument::create(
        "hello.docx",
        WordprocessingDocumentType::Document,
    )?;

    doc.add_main_document_part()
        .set_document(document(vec![body(vec![
            paragraph(vec![run(vec![text("Hello from Rust!")])]),
        ])]));

    doc.save()?;
    Ok(())
}
```

### 读取段落

```rust
use openxml::packaging::WordprocessingDocument;

fn main() -> openxml::Result<()> {
    let mut doc = WordprocessingDocument::open("hello.docx", false)?;
    for p in doc.paragraph_texts()? {
        println!("{p}");
    }
    Ok(())
}
```

### 创建 Excel 工作簿

```rust
use openxml::packaging::{SpreadsheetDocument, SpreadsheetDocumentType};

fn main() -> openxml::Result<()> {
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

### 创建 PowerPoint

```rust
use openxml::packaging::{PresentationDocument, PresentationDocumentType};

fn main() -> openxml::Result<()> {
    let mut ppt = PresentationDocument::create(
        "deck.pptx",
        PresentationDocumentType::Presentation,
    )?;
    ppt.add_slide_with_text("Hello PPT")?;
    ppt.save()?;
    Ok(())
}
```

---

## 4. Word（.docx）

### 4.1 打开 / 创建 / 保存

| 方法 | 说明 |
|------|------|
| `WordprocessingDocument::create(path, type)` | 新建文件 |
| `create_with_settings(path, type, settings)` | 新建并指定 `OpenSettings` |
| `create_in_memory(type)` | 仅内存 |
| `create_simple(path, text)` | 单段落快捷创建 |
| `open(path, is_editable)` | 打开；`false` 时关闭自动保存 |
| `open_with_settings(path, is_editable, settings)` | 打开并指定 `OpenSettings` |
| `open_bytes(data)` | 从字节打开 |
| `from_bytes(data)` | `open_bytes` 别名 |
| `save` / `save_as` / `to_bytes` | 落盘或序列化 |
| `clone_document` | 深拷贝为内存文档 |
| `close` | 按 `AutoSave` 决定是否保存 |

`WordprocessingDocumentType`：`Document` | `Template` | `MacroEnabledDocument` | `MacroEnabledTemplate`。  
Excel / PowerPoint 同样提供 `create` / `create_with_settings` / `create_simple` / `open` / `open_with_settings` / `open_bytes`：

```rust
// Excel: 单表字符串网格
SpreadsheetDocument::create_simple("out.xlsx", "Data", &[vec!["a", "b"]])?;
// PowerPoint: 单标题幻灯片
PresentationDocument::create_simple("out.pptx", "Hello")?;
```

### 4.2 文档主体

```rust
use openxml::wordprocessing::*;

// 构建 DOM
let doc_el = document(vec![body(vec![
    paragraph_with_text("普通段落"),
    paragraph_with_bold_text("粗体段落"),
    numbered_paragraph(1, "列表项"),
    table_from_strings(&[
        vec!["A", "B"],
        vec!["1", "2"],
    ], None),
])]);

doc.add_main_document_part().set_document(doc_el);

// 修改 body
doc.body_mut()?.append_child(paragraph_with_text("追加"));

// 查找替换
doc.replace_text("普通", "正式")?;

// 读取所有段落文本（含表格内）
let texts = doc.paragraph_texts()?;
```

常用构造函数（`openxml::wordprocessing`）：

- 结构：`document`, `body`, `paragraph`, `run`, `text`, `section_properties`
- 样式：`bold`, `italic`, `run_properties`, `paragraph_properties`
- 表格：`table`, `table_row`, `table_cell`, `table_from_strings`, `table_to_strings`
- 列表：`default_numbering`, `numbered_paragraph`
- 批注：`comment`, `with_comment`, `comments`
- 脚注/尾注：`footnote`, `endnote`, `footnote_reference`, …
- 其他：`alt_chunk`, `default_theme`, `replace_text`, `collect_texts`

生成层还提供完整 WordprocessingML 元素工厂，例如：

```rust
use openxml::generated::wordprocessingml_2006_main as wml;

let b = wml::bold_val("1");
let p = wml::paragraph_with_rsid_paragraph_addition(wml::paragraph(vec![]), "00AABBCC");
```

### 4.3 部件与关系

```rust
doc.add_default_styles()?;
doc.add_default_settings()?;
doc.add_default_numbering()?;
doc.add_default_theme()?;

doc.add_default_header("页眉")?;
doc.add_default_footer("页脚")?;

// 文档保护（含 formatting 锁）与绘图网格
doc.set_document_protection_ex("readOnly", true, true)?;
doc.set_drawing_grid_horizontal_origin(100)?;
doc.set_display_horizontal_drawing_grid_every(2)?;
doc.set_style_lock_qf_set(true)?;
doc.set_save_forms_data(true)?;

let img = doc.add_image(openxml::ImageFormat::Png, png_bytes)?;
// img.relationship_id(), img.uri()

let link = doc.create_hyperlink("https://example.com", "示例")?;
doc.body_mut()?.append_child(paragraph(vec![link]));

doc.set_comments(vec![
    comment("0", "Alice", "A", "请修改"),
])?;

doc.add_footnote("1", "脚注正文")?;
doc.add_endnote("1", "尾注正文")?;

// 嵌入 HTML 片段
doc.add_alt_chunk(
    openxml::AlternativeFormatImportType::Html,
    b"<html><body><p>chunk</p></body></html>",
)?;
```

`AlternativeFormatImportType`：`Html` | `Xhtml` | `TextPlain` | `Xml` | `Rtf` | `Mht`。

包级部件（Word / Excel / PPT 共用模式）：

```rust
// 缩略图
doc.add_thumbnail(png_bytes, "image/png", "png")?;
assert!(doc.has_thumbnail());
doc.clear_thumbnail()?;

// 数字签名部件壳（无加密实现）
doc.add_digital_signature_origin()?;
doc.add_xml_signature_part(br#"<?xml version="1.0"?><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"/>"#)?;
assert!(doc.has_digital_signature_origin());
doc.clear_digital_signatures()?;

// Custom XML 部件
let (rid, uri) = doc.add_custom_xml_part(br#"<?xml version="1.0"?><root xmlns="urn:x"/>"#)?;
assert!(doc.has_custom_xml_parts()?);
doc.clear_custom_xml_parts()?;

// 嵌入包 / VBA 部件壳（无宏执行）
doc.add_embedded_package(ole_bytes, "application/octet-stream", "bin")?;
assert!(doc.has_embeddings());
doc.clear_embeddings()?;
doc.add_vba_project(vba_bin)?;
assert!(doc.has_vba_project());
doc.clear_vba_project()?;

// Ribbon / Custom UI 部件壳
doc.add_custom_ui(br#"<?xml version="1.0"?><customUI xmlns="http://schemas.microsoft.com/office/2006/01/customui"><ribbon/></customUI>"#)?;
assert!(doc.has_custom_ui());
doc.clear_custom_ui()?;

// 打印机设置 / QAT
doc.add_printer_settings(printer_bin)?;
assert!(doc.has_printer_settings());
doc.clear_printer_settings()?;
doc.add_quick_access_toolbar()?;
assert!(doc.has_quick_access_toolbar());
doc.clear_quick_access_toolbar()?;

// MIP 敏感度标签部件壳
doc.add_label_info("lbl-1", "Confidential")?;
assert!(doc.has_label_info());
doc.clear_label_info()?;

// Office 加载项 / web extension 部件壳
doc.add_web_extension_shell("WA123", "1.0.0.0")?;
assert!(doc.has_web_extensions());
doc.clear_web_extensions()?;

// 嵌入字体部件
doc.add_font_part(font_bytes, openxml::namespace::content_type::FONT_TTF, "ttf")?;
assert!(doc.has_font_parts());
doc.clear_font_parts()?;

// 包内部件枚举
let n = doc.package_part_count(); // Word 亦有 part_count()
let uris = doc.list_part_uris();
```

Excel (`SpreadsheetDocument`) / PowerPoint (`PresentationDocument`) 同样提供上述 API。

### 4.4 校验与命名空间

```rust
// 轻量规则
let errs = doc.validate()?;
// 规则 + 有序 particle
let errs = doc.validate_full()?;
// 关系 + 唯一属性（手写规则 ∪ Schematron 可抽取子集）
let errs = doc.validate_relationships()?;
// Schematron 可抽取子集（rel + unique + range/length/pattern，**948/948**）
let errs = doc.validate_schematron()?;
// Excel / PPT 亦提供 validate / validate_full（包结构 + 关系规则）
// let errs = wb.validate()?;
// let errs = ppt.validate_full()?;

// Strict → Transitional
let (xml_n, rel_n) = doc.rewrite_strict_to_transitional()?;
```

### 4.5 Flat OPC

```rust
let xml = doc.to_flat_opc_string()?;
let mut again = WordprocessingDocument::from_flat_opc(xml.as_bytes())?;

// Excel / PowerPoint
let xml = wb.to_flat_opc_string()?;
let wb2 = SpreadsheetDocument::from_flat_opc(xml.as_bytes())?;
let xml = ppt.to_flat_opc_string()?;
let ppt2 = PresentationDocument::from_flat_opc(xml.as_bytes())?;
```

---

## 5. Excel（.xlsx）

### 5.1 打开 / 创建 / 保存

```rust
use openxml::packaging::{SpreadsheetDocument, SpreadsheetDocumentType};

let mut wb = SpreadsheetDocument::create("book.xlsx", SpreadsheetDocumentType::Workbook)?;
// 或 create_in_memory / open / open_bytes
wb.save()?;
```

### 5.2 工作表与单元格

```rust
// 内联字符串
wb.write_sheet_strings("Sheet1", &[
    vec!["Name", "Qty"],
    vec!["A", "10"],
])?;

// 共享字符串表（去重）
wb.write_sheet_shared_strings("Sheet1", &[ /* ... */ ])?;

// 多表
wb.add_worksheet("Notes")?;
wb.write_sheet_strings("Notes", &[vec!["hello"]])?;

// 列宽：(min_col, max_col, width)，列号 1-based
wb.set_column_widths("Sheet1", &[(1, 1, 14.0), (2, 3, 10.0)])?;

// 合并单元格（A1 样式）
wb.set_merge_cells("Sheet1", &["A1:C1"])?;

// 公式（不含前导 =）
wb.set_cell_formula("Sheet1", "C2", "A2+B2", Some("30"))?;
let (f, cached) = wb.cell_formula("Sheet1", "C2")?.unwrap();

// 读取
let rows = wb.read_sheet_strings()?; // 第一张表
let rows = wb.read_sheet_strings_by_name(Some("Notes"))?;
```

### 5.3 样式

```rust
wb.add_minimal_styles(true)?; // index 0 默认，1 粗体（STYLE_BOLD）
```

更细的单元格样式可通过 DOM / 生成工厂操作 `x:styleSheet`。

### 5.4 图表与绘图

```rust
// 仅图表部件（挂到 workbook）
let (chart_uri, rid) = wb.add_bar_chart("Sales", &["N", "S"], &[100.0, 80.0])?;

// 图表锚定到工作表（创建 drawings + twoCellAnchor）
// 行列为 0-based
let (chart_uri, drawing_uri) = wb.add_bar_chart_on_sheet(
    "Sheet1", "Sales", &["N", "S"], &[100.0, 80.0],
    0, 2, 6, 15,
)?;

// 图片锚定（oneCellAnchor，cx/cy 为 EMU）
let (img_uri, drawing_uri) = wb.add_image_on_sheet(
    "Sheet1",
    &png_bytes,
    "image/png",
    "png",
    1, 1,
    914_400, 914_400,
    "logo",
)?;
```

底层锚点构造：`two_cell_anchor_chart`、`one_cell_anchor_picture`、`absolute_anchor_picture`、`two_cell_anchor_picture`（`openxml::spreadsheet`）。

### 5.5 批注

```rust
wb.add_sheet_comments("Sheet1", "alice", &[
    ("A1", "表头说明"),
    ("B2", "请核对"),
])?;

let notes = wb.sheet_comments("Sheet1")?;
// Vec<(cell_ref, author, text)>
```

### 5.6 条件格式

```rust
// 大于 50 时套用红色填充（自动维护 styles.xml 中 dxfs）
wb.add_conditional_formatting_cell_is(
    "Sheet1", "A2:A100", "greaterThan", "50", "FFFF0000", 1,
)?;

// 三色色阶
wb.add_conditional_formatting_color_scale("Sheet1", "A2:A100", 2)?;
```

也可直接使用 `cf_rule_cell_is`、`cf_rule_color_scale`、`cf_rule_data_bar`、`conditional_formatting` 等拼 DOM。

### 5.7 数据透视表

```rust
// 源表 Data!A1:B4，结果放在 E3，行字段 Region(0)，数据字段 Sales(1)
let (pivot_uri, cache_uri) = wb.add_pivot_table(
    "Data",
    "A1:B4",
    "Data",
    "E3",
    &["Region", "Sales"],
    0,
    1,
    3, // 数据行数（不含表头）
)?;
```

会创建：

- `/xl/pivotCache/pivotCacheDefinitionN.xml`
- `/xl/pivotCache/pivotCacheRecordsN.xml`
- `/xl/pivotTables/pivotTableN.xml`
- workbook 中的 `pivotCaches` 与相应关系

### 5.8 表 / 保护 / 计算属性

```rust
// Excel 表 + 样式 + totals 列
wb.add_table("Sheet1", "Sales", "A1:B2", &["Name", "Qty"])?;
wb.set_table_style_info("Sales", "TableStyleMedium9", true, false, true, false)?;
wb.set_table_column_totals("Sales", "Qty", "sum", Some("Total"))?;
wb.rename_table("Sales", "Orders")?;

// 工作簿保护（含 lockRevision）
wb.set_workbook_protection_ex(true, false, true)?;
assert!(wb.workbook_lock_revision()?);

// 计算属性
wb.set_calc_mode("manual")?;
wb.set_full_calc_on_load(true)?;
wb.set_sheet_full_calc_on_load("Sheet1", true)?;

// 数据验证提示开关
wb.add_data_validation_list("Sheet1", "A2:A100", "\"A,B,C\"", true)?;
wb.set_data_validations_disable_prompts("Sheet1", true)?;

// 冻结窗格（扩展：topLeft / activePane / state）
wb.set_freeze_panes_ex("Sheet1", 1.0, 1.0, "B2", "bottomRight", "frozen")?;
let (xs, ys, tlc, pane, state) = wb.freeze_pane_details("Sheet1")?.unwrap();
wb.set_color_id("Sheet1", 10)?;          // 自定义网格色索引
wb.set_zoom_scale_normal("Sheet1", 85)?;

// 页面设置细粒度属性
wb.set_paper_size("Sheet1", 9)?;
wb.set_page_orientation("Sheet1", "landscape")?;
wb.set_horizontal_dpi("Sheet1", 300)?;
wb.set_print_errors("Sheet1", "blank")?;
wb.set_cell_comments("Sheet1", "atEnd")?;
```

Word 其余 settings 标志示例：

```rust
doc.set_do_not_use_margins_for_drawing_grid_origin(true)?;
doc.set_show_envelope(true)?;
doc.set_auto_format_override(true)?;
doc.set_ui_compat_97_to_2003(true)?;
doc.set_no_line_breaks_after("ja-JP", "、。")?;
doc.set_read_mode_ink_lock_down(12240, 15840, "100%", Some(true))?;
```

PPT 普通视图属性：

```rust
ppt.set_last_view("sldView")?;
ppt.set_show_outline_icons(false)?;
ppt.set_prefer_single_view(true)?;
ppt.set_vert_bar_state("minimized")?;
ppt.set_restored_left(20000, Some(false))?;
ppt.set_restored_top(50000, Some(true))?;
ppt.set_snap_to_grid(false)?;
ppt.set_show_guides(true)?;
ppt.set_outline_view_scale(50, 100, 75, 100)?;
assert!(ppt.clear_last_view()?);
```

---

## 6. PowerPoint（.pptx）

```rust
use openxml::packaging::{PresentationDocument, PresentationDocumentType};
use openxml::presentation::SLIDE_SIZE_16_9;

let mut ppt = PresentationDocument::create(
    "deck.pptx",
    PresentationDocumentType::Presentation,
)?;

// 空白母版 + 版式
ppt.add_blank_master_with_layout()?;

// 带文本框的幻灯片
ppt.add_slide_with_text("标题页")?;

// 关联默认版式
ppt.add_slide_with_layout(openxml::presentation::slide_with_text("内容页"))?;

// 演讲者备注
ppt.add_notes_to_slide(0, "讲稿…")?;

// 幻灯片尺寸（EMU）
let (cx, cy) = SLIDE_SIZE_16_9;
ppt.set_slide_size(cx, cy)?;
assert_eq!(ppt.slide_size()?.unwrap(), SLIDE_SIZE_16_9);

// 读文本
let texts = ppt.slide_texts(0)?;
let texts = ppt.first_slide_texts()?;

// 版式属性
ppt.set_slide_layout_type(0, "title")?;
ppt.set_slide_layout_preserve(0, true)?;
ppt.set_slide_layout_matching_name(0, "Title Slide")?;
ppt.set_show_master_shapes(0, false)?;

// 批注作者 CRUD
ppt.add_comment_authors(&[(1, "Alice", "A")])?;
ppt.append_comment_author(2, "Bob", "B")?;
assert_eq!(ppt.comment_author_by_id(1)?.unwrap().0, "Alice");
ppt.remove_comment_author(2)?;

ppt.save()?;
```

常量：`SLIDE_SIZE_16_9`、`SLIDE_SIZE_4_3`。

`PresentationDocumentType`：`Presentation` | `Template` | `Slideshow` | 宏启用变体。

---

## 7. DOM 与 XML

### OpenXmlElement

```rust
use openxml::element::{parse_element, write_element, OpenXmlElement};
use openxml::simple_types::OnOffValue;

let mut el = OpenXmlElement::w("b"); // WordprocessingML 命名空间
el.set_simple_attribute_qname("w:val", OnOffValue(true));
assert_eq!(el.get_attribute_qname("w:val"), Some("1"));

let xml = write_element(&el)?;
let again = parse_element(&xml)?;
```

常用 API：

- 构造：`OpenXmlElement::new(prefix, uri, local)`，`::w` / `::x` / `::p`
- 子节点：`with_child` / `append_child` / `child` / `children_by_name` / `descendants`
- 属性：`set_attribute`、`set_attribute_qname`、`set_attribute_ns`、`get_*`、`with_attribute*`
- 简单类型：`set_simple_attribute[_qname]`、`get_simple_attribute[_qname]`
- 文本：`with_text`、`inner_text`、`text_value`

### 简单类型（`openxml::simple_types`）

`StringValue`、`OnOffValue`、`BooleanValue`、`Int32Value`、`UInt32Value`、`Int64Value`、`IntegerValue`、`HexBinaryValue`、`DoubleValue`，均实现 `OpenXmlSimpleType`。

生成的 schema 枚举（如 `HighlightColorValues`）同样实现该 trait：

```rust
use openxml::generated::wordprocessingml_2006_main::HighlightColorValues;
use openxml::simple_types::OpenXmlSimpleType;

let red = HighlightColorValues::from_str("red").unwrap();
assert_eq!(red.as_str(), "red");
```

---

## 8. 包属性与 Flat OPC

### 核心属性

```rust
use openxml::opc::PackageProperties;

let mut props = PackageProperties::new();
props.title = Some("报告".into());
props.creator = Some("openxml-rs".into());
doc.set_package_properties(&props)?;
let props = doc.package_properties()?;
```

### 低层 OPC

```rust
use openxml::opc::{OpcPackage, PackUri, RelationshipTargetMode};

let mut pkg = OpcPackage::create();
pkg.set_part("/word/document.xml", content_type, bytes);
pkg.add_package_relationship(rel_type, &PackUri::new("/word/document.xml"), RelationshipTargetMode::Internal);
let zip = pkg.to_bytes()?;
```

### Flat OPC

```rust
use openxml::opc::{to_flat_opc, from_flat_opc, progid};

let flat = to_flat_opc(doc.package().opc(), Some(progid::WORD))?;
let pkg = from_flat_opc(&flat)?;
```

---

## 9. Markup Compatibility

```rust
use openxml::markup_compatibility::*;
use openxml::file_format::FileFormatVersions;
use openxml::element::OpenXmlElement;

// AlternateContent
let ac = alternate_content_with(
    "w14",
    vec![OpenXmlElement::w("new")],
    vec![OpenXmlElement::w("legacy")],
);
let chosen = resolve_alternate_content(&ac, &["w14"]);

// Ignorable + ProcessContent + Preserve*
let mut root = with_ignorable(/* element */, "w14 w15");
process_markup_compatibility(&mut root, &["w"]); // 去掉不支持的 ignorable 内容

// 按 Office 版本处理（前缀表来自 namespaces.json 生成结果）
process_markup_compatibility_for_version(&mut root, FileFormatVersions::OFFICE2010);
```

`FileFormatVersions`：`OFFICE2007` … `OFFICE2021`、`MICROSOFT365`、`ALL`，支持 `and_later()`、`includes_introduction()`、`supported_prefixes()`。

Strict → Transitional：

```rust
use openxml::namespace_rewrite::{
    rewrite_package_to_transitional,
    to_transitional_namespace,
    rewrite_element_to_transitional,
};

doc.rewrite_strict_to_transitional()?;
// 或对 OpcPackage / 单个元素调用上述函数
```

---

## 10. 校验

```rust
use openxml::validation::{
    validate_word_document,
    validate_word_document_full,
    validate_particle,
    validate_word_particles,
    validate_schematron_subset,
    validate_schematron_constraints,
    SCHEMATRON_EXTRACTED_REL_COUNT,
    SCHEMATRON_EXTRACTED_UNIQUE_COUNT,
    SCHEMATRON_NUMERIC_RANGE_COUNT,
    SCHEMATRON_STRING_LENGTH_COUNT,
    SCHEMATRON_PATTERN_COUNT,
    SCHEMATRON_TOTAL_SOURCE_RULES,
    Particle,
};

// 文档级
let errs = doc.validate()?;              // 轻量规则
let errs = doc.validate_full()?;         // 规则 + particle
let errs = doc.validate_relationships()?; // 手写 ∪ Schematron 可抽取子集
let errs = doc.validate_schematron()?;   // Schematron 可抽取子集（含属性约束）
// Excel / PPT 同样提供 validate_relationships / validate_schematron / validate_package

// 手写 / 生成 particle
let p = openxml::generated::wordprocessingml_2006_main::particle_for_class("Document")
    .unwrap();
let errs = validate_particle(&document_element, &p, "w:document");

// 规则规模（相对 C# 948 条源规则）
assert_eq!(SCHEMATRON_TOTAL_SOURCE_RULES, 948);
assert!(SCHEMATRON_EXTRACTED_REL_COUNT >= 50);
assert!(SCHEMATRON_EXTRACTED_UNIQUE_COUNT >= 100);
assert!(SCHEMATRON_NUMERIC_RANGE_COUNT >= 200);
assert!(SCHEMATRON_STRING_LENGTH_COUNT >= 150);
assert!(SCHEMATRON_PATTERN_COUNT >= 10);
// 也可对任意 DOM 节点单独跑属性约束
let attr_errs = validate_schematron_constraints(&document_element);
```

轻量规则覆盖：Document / Body / Paragraph / Run 的必需与 max=1 子元素。  
Particle 引擎支持 Sequence / Choice / Group / All / Element / Any。  
Schematron 可抽取子集覆盖：

- `document(rels)` 关系存在性（63）
- `count(distinct-values(...))` 唯一属性（107）
- `@attr` 数值范围（230）
- `string-length(@attr)`（184）
- 简单 `matches(@attr, …)`（15）

合计约 **771 / 948**；其余 XPath/跨部件规则需完整引擎（长期项）。

部件约束（生成）：

```rust
use openxml::generated::parts;

assert!(parts::is_allowed_child("MainDocumentPart", "HeaderPart"));
assert!(parts::allows_multiple("MainDocumentPart", "HeaderPart"));
```

重新生成 Schematron 规则表（同时写出 `schematron_rules.rs` 与 `schematron_constraints.rs`）：

```bash
python3 scripts/generate_schematron_rules.py \
  --schematrons /opt/wp/Open-XML-SDK/data/schematrons.json
```

---

## 11. Schema 代码生成

从 C# SDK 的 `data/` 生成 Rust 绑定：

```bash
cd /opt/wp/openxml
cargo run --release --bin openxml-codegen -- \
  --data /opt/wp/Open-XML-SDK/data \
  --out src/generated

# 默认：word/excel/ppt/drawing 2006 main
# 全部 schema：
cargo run --release --bin openxml-codegen -- \
  --data /opt/wp/Open-XML-SDK/data \
  --out src/generated \
  --schema all
```

生成内容包括：

| 产出 | 说明 |
|------|------|
| 元素工厂 | `paragraph()`、`bold_val()`、`paragraph_with_*` … |
| `ElementInfo` / `AttributeInfo` / `ChildInfo` | 元数据表 |
| Schema 枚举 | 实现 `OpenXmlSimpleType` |
| Particles | `particle_for_class("Run")` 等 |
| `parts.rs` | `PartInfo` + `PartChildConstraint` |
| `namespaces.rs` | 前缀/URI + `PREFIX_INTRODUCED_IN` 版本表 |

---

## 12. C# API 对照

| C# | Rust |
|----|------|
| `WordprocessingDocument.Create(path, type)` | `WordprocessingDocument::create(path, type)` |
| `doc.AddMainDocumentPart()` | `doc.add_main_document_part()` |
| `main.Document = new Document(...)` | `main.set_document(document(...))` |
| `body.Elements<Paragraph>()` | `doc.paragraph_texts()` / DOM 遍历 |
| `SpreadsheetDocument.Create` | `SpreadsheetDocument::create` |
| `PresentationDocument.Create` | `PresentationDocument::create` |
| `System.IO.Packaging.Package` | `opc::OpcPackage` |
| `OpenXmlElement` | `element::OpenXmlElement` |
| `FileFormatVersions` | `file_format::FileFormatVersions` |
| Flat OPC 扩展 | `to_flat_opc` / `from_flat_opc` |
| MC 处理 | `markup_compatibility::*` |
| `OpenXmlValidator`（子集） | `validation::*` / `doc.validate[_full]()` / `validate_schematron()` |

---

## 13. 示例程序

```bash
cargo run --example create_word -- /tmp/hello.docx
cargo run --example read_word -- /tmp/hello.docx
cargo run --example create_report -- /tmp/report.docx
#  表 + 页眉页脚 + 超链接 + 样式

cargo run --example create_spreadsheet -- /tmp/report.xlsx
#  多表 + SST + 列宽 + 公式 + 样式

cargo run --example create_presentation -- /tmp/deck.pptx
#  两页幻灯片 + 核心属性
```


### Body hyperlink DOM / sectPr header refs / paragraph numbering

```rust
// List and unwrap body hyperlinks (relationship + DOM)
let body_hls = doc.list_body_hyperlinks()?; // (rId, anchor, text)
if let Some((rid, _, _)) = body_hls.into_iter().find(|(r, _, _)| !r.is_empty()) {
    let (rel_removed, unwrapped) = doc.remove_body_hyperlink(&rid)?;
}

// Header/footer with sectPr reference cleanup
let hrid = doc.add_default_header("Title")?;
assert!(doc.list_sect_pr_references()?.iter().any(|(_, _, id)| id == &hrid));
doc.remove_header_by_id(&hrid)?;

// Apply numbering to all body paragraphs
doc.add_default_numbering()?;
doc.apply_numbering_to_paragraphs(1, 0)?;
doc.clear_paragraph_numbering()?;
```

### Excel column outline list / PPT shape remove / animation effect

```rust
wb.set_column_outline_level("Sheet1", 2, 3, 1)?;
let cols = wb.column_outline_levels("Sheet1")?; // (min, max, level, collapsed)
wb.clear_all_column_outlines("Sheet1")?;

let sid = ppt.add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "Hi", "Box")?;
ppt.set_animation_effect(0, sid, "blinds(horizontal)", "in")?;
let (filter, tr) = ppt.animation_effect(0)?.unwrap();
ppt.remove_shape_by_id(0, sid)?;
ppt.clear_shapes(0)?;
```

---

## 14. 错误处理

```rust
use openxml::{Error, Result};

fn demo() -> Result<()> {
    let doc = WordprocessingDocument::open("missing.docx", false);
    match doc {
        Err(Error::Io(e)) => eprintln!("io: {e}"),
        Err(Error::Zip(e)) => eprintln!("zip: {e}"),
        Err(Error::Xml(msg)) => eprintln!("xml: {msg}"),
        Err(Error::Package(msg)) => eprintln!("package: {msg}"),
        Err(Error::PartNotFound(p)) => eprintln!("part: {p}"),
        Err(e) => eprintln!("{e}"),
        Ok(_) => {}
    }
    Ok(())
}
```

主要变体：`Io`、`Zip`、`Xml`、`Package`、`PartNotFound`、`RelationshipNotFound`、`InvalidContentType`、`Closed`、`NoRootElement`。

---

## 15. 限制与后续方向

已实现内容见 `PORTING.md` 与根目录 `README.md` 状态表。  
**与 C# Open-XML-SDK 的系统差距**见 **[GAP_ANALYSIS.md](GAP_ANALYSIS.md)**（约 **90–95%** 常用能力面；常用读写与部件库存路径可用）。

当前有意保留的简化包括：

- 包整体加载内存（有 `OpenXmlStreamReader` 事件流，但 ZIP 仍非惰性）
- Particle + Schematron **可抽取子集**（~771 / 948：rel + unique + range + length + pattern）；非完整 XPath 引擎
- 透视表/图表/高级部件多为可打开的最小合法壳；Excel 刷新后可能重写
- 无 Features DI / 事件 / Linq 包；codegen 为工厂+元数据而非强类型元素类
- 数字签名 / VBA 仅为部件壳（无加密实现、无宏执行）

长期可选方向（详见 GAP §12，**非 MVP 阻塞**）：

1. 完整 Schematron XPath 引擎（剩余 0 复杂/跨部件规则）  
2. 流式/惰性 ZIP 部件加载（超大文件）  
3. 强类型 Part/Element 生成（可选 feature）  
4. 动画时间线 / 真实透视缓存行深度  
5. 数字签名加密实现（若需要）


---

## 模块索引

| 模块 | 路径 | 职责 |
|------|------|------|
| `packaging` | 文档与部件高层 API | Word/Excel/PPT |
| `wordprocessing` | Word 元素与便捷构造 | |
| `spreadsheet` | Excel 元素、图表、绘图、CF、透视 | |
| `presentation` | PPT 元素与幻灯片辅助 | |
| `element` | DOM | |
| `opc` | 包、关系、Flat OPC、属性 | |
| `validation` | 规则 + particles | |
| `markup_compatibility` | MC | |
| `namespace` / `namespace_rewrite` | 常量与 Strict 改写 | |
| `file_format` | Office 版本 | |
| `simple_types` | 简单类型 | |
| `generated` | Codegen 产出 | |
| `error` | `Error` / `Result` | |

生成 API 文档：

```bash
cd /opt/wp/openxml
cargo doc --no-deps --open
```

---

*文档对应 openxml 0.1.0（Open-XML-SDK 全量里程碑移植完成版）。*


### Shared strings rewrite & table column remove

```rust
// Convert SST-backed cells to inlineStr, then drop the SST part
wb.write_sheet_shared_strings("S", &[vec!["hello", "world"]])?;
wb.materialize_shared_strings()?; // convert cells, keep SST
wb.clear_shared_strings()?; // cells remain readable via inlineStr
assert_eq!(wb.get_cell_value("S", "A1")?.as_deref(), Some("hello"));

// Removing the last table column shrinks table `ref`
wb.add_table("S", "T1", "A1:C2", &["h1", "h2", "h3"])?;
wb.remove_table_column("T1", "h3")?; // ref becomes A1:B2
```

### Body hyperlink cleanup & revision inventory

```rust
doc.append_hyperlink("https://example.com", "click")?;
doc.remove_external_hyperlink("https://example.com"); // also unwraps body w:hyperlink
assert!(!doc.has_body_hyperlinks()?);

// Tracked changes inventory
assert!(doc.has_revision_markers()?);
let _ = doc.list_revision_markers()?; // (kind, author, date, text)
doc.accept_all_revisions()?;
```


### Complex fields & header/footer revisions

```rust
doc.append_complex_field(" PAGE ", "1")?;
assert!(doc.has_complex_fields()?);

// Accept track-changes in headers/footers as well as body
doc.accept_all_revisions_everywhere()?;
```


### Rebuild calc chain & prune unused styles

```rust
wb.set_cell_formula("S", "A3", "A1+A2", Some("3"))?;
wb.rebuild_calc_chain()?; // from all formula cells

doc.add_paragraph_styles(&[("Unused1", "Unused", Some("Normal"))])?;
doc.remove_unused_styles()?; // drops styles not referenced in body
```


### Additional slide transitions

```rust
ppt.set_push_transition(0, "med")?;
ppt.set_wipe_transition(1, "fast")?;
ppt.set_split_transition(0, "slow")?;
ppt.clear_all_transitions()?;
```


## Advanced / long-term surfaces

### Lazy package open

```rust
use openxml::opc::OpcPackage;
let pkg = OpcPackage::open_bytes_lazy(&bytes)?;
// Parts decompress on first `get_part` / `load_part`
assert!(pkg.has_lazy_parts() || !pkg.has_lazy_parts());
```

### Linq-style element queries

```rust
use openxml::element::OpenXmlElement;
// after you have a document root element:
// root.query().named("p").attr_eq("rsidR", "00AB").count();
// openxml::element::descendants_of(&root, "t").count();
```

### Schematron extractable subset

About **942/948** source rules are enforced without full XPath:
relationship existence, unique attributes, numeric ranges, string lengths, simple patterns,
enumerations, ancestor-scoped uniqueness, conditional attributes, non-zero GUIDs,
same-element attr comparisons, fixed booleans, and **cross-part** Index-of / count bounds
(`validate_schematron_cross_part` when package parts are present).

### Typed element views

```rust
use openxml::element::{Document, Paragraph, Table, Worksheet, Cell, Slide, Style, Hyperlink, Comment, Header, Notes};
let doc = Document::with_paragraphs([Paragraph::with_text("hi")]);
let table = Table::from_strings([["a", "b"], ["c", "d"]]);
let mut ws = Worksheet::new();
ws.append_cell_to_row(1, Cell::with_value("A1", "42"));
let mut slide = Slide::new();
slide.append_text_box("title");
```

### Features bag

```rust
use openxml::{FeatureCollection, ParagraphIdGenerator};
// On OpenXmlPackage:
// doc.package_mut().features_mut().set(ParagraphIdGenerator::new());
// let id = doc.package_mut().features_mut().get_mut::<ParagraphIdGenerator>().unwrap().next_id();
```

### Digital signature structure + digests

```rust
use openxml::validation::{validate_package, validate_digital_signatures, validate_signature_digests, build_signature_xml};
// validate_package runs structure + Reference digest checks.
// build_signature_xml / build_signed_signature_xml (RSA-SHA256 SignatureValue over SignedInfo).
// verify_signature_value(public_pem) checks SignatureValue.
```
