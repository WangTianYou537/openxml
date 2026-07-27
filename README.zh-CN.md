# officexml

[English](README.md)

[Open XML SDK](https://github.com/OfficeDev/Open-XML-SDK) 的纯 Rust 移植，用于创建与编辑 Microsoft Office 包：

| 格式 | 扩展名 | 入口类型 |
|------|--------|----------|
| Word | `.docx` / `.dotx` | `WordprocessingDocument` |
| Excel | `.xlsx` / `.xlsm` | `SpreadsheetDocument` |
| PowerPoint | `.pptx` / `.ppsx` | `PresentationDocument` |

**Crate 名：** `officexml` · **仓库：** https://github.com/WangTianYou537/openxml  
**未发布到 crates.io** — 完整包（生成 schema + 内置字体）在不删功能的前提下会超过注册表体积限制。

许可证：**MIT**（与上游 Open XML SDK 一致）。

---

## 特性概览

- **OPC 包** — ZIP、`[Content_Types].xml`、关系、Flat OPC、core/app/custom 属性
- **OpenXmlElement DOM** — 解析 / 写出 / 遍历 / 改写，含 Markup Compatibility（AC、Ignorable、ProcessContent）
- **Word / Excel / PowerPoint** — 创建、打开、保存、克隆；样式、页眉页脚、图表、表格、备注、母版等
- **校验** — 包结构、部件约束、schema particle、语义关系规则
- **Schema 代码生成** — 从 C# SDK 的 `data/*.json` 生成类型化构造器与部件图
- **`svg2pptx` CLI** — 多页 SVG → 原生 DrawingML 形状（可编辑文本或字形轮廓）

完整里程碑见 [PORTING.md](PORTING.md)。API 指南：[docs/USAGE.md](docs/USAGE.md)。与 C# 差距：[docs/GAP_ANALYSIS.md](docs/GAP_ANALYSIS.md)。

---

## 构建

```bash
cargo build --release
cargo test --locked --all-targets
cargo doc --no-deps --open
```

### 作为路径依赖

```toml
[dependencies]
officexml = { path = "/path/to/openxml" }
```

---

## 快速开始

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
        paragraph(vec![run(vec![text("来自 Rust 的问候！")])]),
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
        vec!["姓名", "分数"],
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
    ppt.add_slide_with_text("标题页")?;
    ppt.add_slide_with_text("第二页")?;
    ppt.save()?;
    Ok(())
}
```

### 示例程序

```bash
cargo run --example create_word -- /tmp/hello.docx
cargo run --example read_word -- /tmp/hello.docx
cargo run --example create_report -- /tmp/report.docx
cargo run --example create_spreadsheet -- /tmp/report.xlsx
cargo run --example create_presentation -- /tmp/deck.pptx
cargo run --example create_ppt_from_svg_shapes -- /tmp/from-svg.pptx
```

---

## `svg2pptx` 命令行

将一个或多个 SVG 转为 `.pptx`。每个 SVG 对应一页铺满 16:9 的幻灯片，内容为 **原生 DrawingML 形状**（不是嵌入的 SVG 图片）。

```bash
cargo build --release --bin svg2pptx

# 多页
./target/release/svg2pptx -o deck.pptx a.svg b.svg c.svg

# 省略 -o 时，输出为 <第一个文件名>.pptx
./target/release/svg2pptx a.svg b.svg
```

### 字体模式

| 参数 | 行为 |
|------|------|
| *（默认）* | 可编辑文本框；SVG 未写 `font-family` 时使用系统字体（Times New Roman / 微软雅黑） |
| `--font-shape` | 字形转为几何轮廓 — 无文本框、不嵌字体 |
| `--embed-font` | 可编辑文本 + 按需 **子集** EOT（`.fntdata`）嵌入已用字体 |
| `--embed-font-fully` | 可编辑文本 + **完整** EOT 嵌入已用字体 |

```bash
svg2pptx --font-shape -o outlined.pptx poster.svg
svg2pptx --embed-font -o embedded.pptx poster.svg
svg2pptx --embed-font-fully -o full.pptx poster.svg
```

### 内置字体

跨平台文本测量 / 轮廓不依赖系统字体包。仓库自带：

```text
assets/fonts/
  LiberationSans-*.ttf    # Arial / Helvetica 替代
  LiberationSerif-*.ttf   # Times New Roman 替代
  DejaVuSans-*.ttf
  NotoSansSC-*.ttf        # CJK 嵌入（TrueType，ODTTF 友好）
```

子集嵌入（`--embed-font`）需要 Python 3 + [fontTools](https://github.com/fonttools/fonttools)（`scripts/subset_ttf.py`）。

---

## 架构

```text
src/
  packaging/       WordprocessingDocument / SpreadsheetDocument / PresentationDocument
  wordprocessing/  Document、Body、Paragraph、Run、Text、表格、脚注…
  spreadsheet/     Workbook、工作表、SST、图表、条件格式、数据透视…
  presentation/    幻灯片、母版/版式、备注、svg_to_shapes
  element/         OpenXmlElement DOM + XML 读写
  opc/             ZIP、内容类型、关系、Flat OPC、属性
  validation/      包 / particle / 语义校验器
  markup_compatibility/  AlternateContent、Ignorable、ProcessContent
  generated/       从 C# data/*.json 代码生成（部件、schema、particle）
  bin/svg2pptx.rs  多页 SVG → PPTX CLI
  bin/openxml-codegen.rs  schema/部件生成器（feature = "codegen"）
```

分层对应 C# 的 `DocumentFormat.OpenXml.Framework` 与 `DocumentFormat.OpenXml/Packaging`。

### 重新生成 schema 绑定

```bash
cargo run --features codegen --bin openxml-codegen -- \
  --data /path/to/Open-XML-SDK/data \
  --out src/generated
```

默认 schema 为 `wordprocessingml_2006_main`；完整集使用 `--schema all`（体积较大）。

---

## 状态（摘要）

| 领域 | 状态 |
|------|------|
| OPC 包（ZIP、内容类型、关系、Flat OPC） | ✅ |
| OpenXmlElement DOM + MC | ✅ |
| Word / Excel / PowerPoint 创建 · 打开 · 保存 | ✅ |
| 样式、页眉页脚、图片、超链接、表格 | ✅ |
| Excel 多表、SST、图表、条件格式、透视、保护 | ✅ |
| PPT 母版/版式、备注、切换、主题 | ✅ |
| 包 + particle + 语义校验 | ✅ |
| Schema / 部件 / particle 代码生成 | ✅ |
| SVG → 原生 DrawingML（`svg2pptx`） | ✅ |
| 数字签名加密 / VBA 执行 | 部分 / 非目标 |

完整清单见 [PORTING.md](PORTING.md)。

---

## CI 与发布

每次推送到 `main` 时，GitHub Actions 在 **Ubuntu、Windows、macOS** 上运行 `cargo test --locked --all-targets`。

打标签可发布多平台 `svg2pptx` 二进制：

```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## 文档

| 文档 | 内容 |
|------|------|
| [docs/USAGE.md](docs/USAGE.md) | 安装、Word/Excel/PPT API、MC、校验、codegen、C# 对照 |
| [docs/GAP_ANALYSIS.md](docs/GAP_ANALYSIS.md) | 相对上游 Open-XML-SDK 的差距 |
| [PORTING.md](PORTING.md) | 移植里程碑与设计说明 |
| `cargo doc --no-deps --open` | 生成 rustdoc |

---

## 开发

```bash
cargo test --locked --all-targets
cargo build --release --bin svg2pptx
```

上游参考（C#）：

- Framework：`Open-XML-SDK/src/DocumentFormat.OpenXml.Framework/`
- 包 API：`Open-XML-SDK/src/DocumentFormat.OpenXml/Packaging/`
- Schema 数据：`Open-XML-SDK/data/schemas/`、`data/parts/`、`data/namespaces.json`

---

## 许可证

MIT — 与 [Open XML SDK](https://github.com/OfficeDev/Open-XML-SDK) 相同。
