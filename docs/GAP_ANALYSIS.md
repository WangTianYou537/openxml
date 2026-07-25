# Gap Analysis: Open-XML-SDK (C#) vs openxml (Rust)

对比范围：

| | C# | Rust |
|--|----|------|
| 路径 | `/opt/wp/Open-XML-SDK` | `/opt/wp/openxml` |
| 版本视角 | Framework + DocumentFormat.OpenXml + Features + Linq | openxml 0.1.0（PORTING 里程碑全勾选版） |
| Schema 数据 | `data/schemas` 155 + `data/parts` 128 + `schematrons.json` **948** 规则 | codegen 消费 schemas/parts/namespaces；Schematron **可抽取子集 **948/948** 源规则**（~835 去重表项）：`schematron_rules.rs`（63 rel + 115 unique）+ `schematron_constraints.rs`（236 range + 184 length + 15 pattern + 37 enum + 25 ancestor-unique + 10 conditional + 3 guid + 6 attr-cmp + 8 fixed-bool + 23 cross-index + 53 cross-count + 17 fixed-val + 7 fixed-ne + 12 multi-ne + 9 both-present + 7 finite + 5 required-attr） |
| 生成规模（约） | ~4342 复杂类型、~582 枚举、~125 Part 类、~5777 Linq XName | 155 schema 模块工厂 + ~125 `PartInfo` + 枚举；无 Linq |

**结论摘要：** 对「创建 / 打开 / 改写常见 Word·Excel·PPT 包」这条主路径，Rust **已完成可用且较深的高层实现**（包装层约 Word 1419+/Excel 1581+/PPT 960+ `pub fn`，集成测试 553+）。相对完整 C# SDK，剩余差距主要在 **完整强类型 Part/Element codegen、~34 条复杂 Schematron 布尔逻辑、数字签名加密与 VBA 执行**（lazy ZIP / Features 袋 / Linq 查询 / 签名结构校验 / 薄类型封装已落地）（多为有意取舍或长期项）。下面按子系统列出。

图例：

- ✅ 对等或可替代
- 🟡 部分实现 / 简化子集
- ❌ 未实现

---

## 1. 总体对照

| 能力域 | C# | Rust | 状态 |
|--------|----|------|------|
| OPC ZIP 包读写 | `System.IO.Packaging` 抽象 + Features | `opc::OpcPackage` 内存 `IndexMap` + **lazy open** | 🟡 默认全量；`open_lazy`/`open_bytes_lazy` 按需解压 |
| Content Types / Relationships | 完整 | 完整 | ✅ |
| Flat OPC | 三文档类型扩展 | `to_flat_opc` / `from_flat_opc` | ✅ |
| OpenXmlElement DOM | 强类型类层次 + 链表式 | 单一 `OpenXmlElement` 拥有式树 | 🟡 功能够用，模型不同 |
| Schema codegen | ~395 生成文件；**强类型元素类 + Part 类** | 155 schema 模块；**工厂函数 + 元数据表** | 🟡 数据覆盖全，类型深度浅 |
| Part 元数据 | 128 生成 Part 类 + 约束 | `generated/parts.rs` 126 `PartInfo` | 🟡 元数据有，运行时 Part 类少 |
| Word 高层 API | 完整部件图 + 强类型属性 | 常用部件 + 便捷构造 | 🟡 |
| Excel 高层 API | 完整部件图 | 网格/SST/图/批注/CF/透视子集 | 🟡 |
| PowerPoint 高层 API | 完整部件图 | 幻灯片/母版/备注子集 | 🟡 |
| Markup Compatibility | 完整 MC + OpenSettings | AC/Ignorable/ProcessContent/Preserve + 版本表 | 🟡 接近 |
| Strict→Transitional | Features / rewrite | `namespace_rewrite` | ✅ |
| Schema particle 校验 | 完整 particle validators | Sequence/Choice/Group/All/Element/Any | 🟡 引擎有，覆盖面/XSD 限制不全 |
| Semantic 校验 | 21 约束类 + **948** Schematron 规则 | ✅ 可抽取子集 ~942 条：63 rel + 115 unique + 236 range + 184 length + 15 pattern + 37 enum + 25 ancestor-unique + 10 conditional + 3 guid + 6 attr-cmp + 8 fixed-bool + 23 cross-index + 53 cross-count + 17 fixed-val + 7 fixed-ne + 12 multi-ne + 9 both-present + 7 finite + 5 required-attr（`validate_schematron` / `validate_schematron_constraints`）；其余 XPath/跨部件规则未执行 | 🟡 |
| Package 级校验 | `PackageValidator` / `OpenXmlValidator` | 文档级 `validate` / `validate_full` | 🟡 |
| Features DI / 事件 | 大量 `I*Feature` | 轻量 `FeatureCollection` + `ParagraphIdGenerator`；无事件总线 | 🟡 |
| 流式 Reader/Writer | `OpenXmlReader`/`Writer`/`PartReader` | ✅ `OpenXmlStreamReader` + event writer（非完整 SAX 对等） | 🟡 |
| 加密 Office 文件 | `IsEncryptedOfficeFile` 检测 | ✅ 检测 + `Error::EncryptedPackage`（不加解密） | 🟡 |
| 数字签名部件 API | `DigitalSignatureOriginPart` 等 | ✅ origin/sig + Reference digests + RSA-SHA256 sign/verify (simplified C14N) | 🟡 |
| Linq-to-OpenXml | `DocumentFormat.OpenXml.Linq` | ✅ 轻量 `element::linq`（query/descendants_of） | 🟡 非 XName 全表 |
| Equality 比较器 | `OpenXmlElementEqualityComparer` | ✅ `elements_equal` / `EqualityOptions` | 🟡 |
| ParagraphId / 注解 Features | Features 包 | `ParagraphIdGenerator` 壳 | 🟡 |

---

## 2. Framework / DOM

### 2.1 已覆盖

| C# | Rust |
|----|------|
| `OpenXmlElement` 树遍历、属性、文本 | `element::OpenXmlElement` + `descendants` / `children_by_name` |
| XML 解析序列化 | `parse_element` / `write_element` |
| `FileFormatVersions` | `file_format::FileFormatVersions` + 生成前缀表 |
| `AlternateContent` / MC 属性 | `markup_compatibility` |
| 简单类型核心 | `StringValue`, `OnOffValue`, `BooleanValue`, 整数族, `HexBinaryValue`, `DoubleValue` |
| 枚举简单类型 | codegen 枚举 + `OpenXmlSimpleType` |

### 2.2 缺口

| 缺口 | C# 位置 | 说明 | 优先级 |
|------|---------|------|--------|
| **强类型元素类** | 生成 `Paragraph : OpenXmlCompositeElement` 等 | Rust 仅有 `paragraph()` 工厂返回通用 DOM，无 `struct Paragraph` 属性访问器 | 高（API 体验） |
| **Leaf / Composite / Misc 节点分类** | `OpenXmlLeafElement`, `OpenXmlMiscNode`, `OpenXmlUnknownElement` | ✅ `OpenXmlMiscKind`（Comment/PI/CData）挂在统一 DOM | 🟡 Leaf/Unknown 仍合并 |
| **OpenXmlReader / OpenXmlWriter 流式** | `OpenXmlReader.cs`, `OpenXmlPartReader` | 大部件必须整树进内存 | 高（大文件） |
| **OpenXmlLoadMode** | Full / Lazy（默认缓存 OuterXml） | 无惰性加载 | 高 |
| **MaxCharactersInPart** | OpenSettings DoS 防护 | ✅ 有 DoS 防护 | ✅ |
| **CompressionOption** | 包压缩选项 | ✅ `CompressionOption` + `OpenSettings.compression` | ✅ |
| **元素事件** | `ElementEventArgs`, Features `ElementEvents` | 无变更通知 | 低 |
| **Equality** | `Equality/OpenXmlElementEqualityComparer` | ✅ `elements_equal` / `EqualityOptions` | ✅ |
| **Annotations** | `AnnotationsFeature` | 无元素注解字典 | 低 |
| **XmlPath / XmlLineInfo** | 校验错误定位 | 校验错误路径信息较弱 | 中 |
| **Builder 模式** | `Builder/`, package builders | 无对等 fluent builder 框架 | 低 |

### 2.3 简单类型缺口

C# `SimpleTypes/` 多数已有对等（含 Base64/DateTime/List/TrueFalse* 等）；历史缺口列表（供对照）：

- `Base64BinaryValue`
- `DateTimeValue`（元数据引用类型名，运行时无解析类型）
- `DecimalValue`, `SingleValue`
- `ListValue<T>`
- `TrueFalseValue`, `TrueFalseBlankValue`
- `ByteValue`, `SByteValue`, `Int16Value`, `UInt16Value`, `UInt64Value`
- 可比较基类 `OpenXmlComparableSimpleValue`

---

## 3. OPC / Packaging 基础设施

### 3.1 已覆盖

- 创建/打开路径与字节、保存、`to_bytes`
- Content types defaults/overrides
- Package / part relationships（internal/external）
- URI 解析与 relativize
- Core properties (`docProps/core.xml`)
- Flat OPC round-trip
- Strict relationship/namespace rewrite

### 3.2 缺口

| 缺口 | C# | 说明 | 优先级 |
|------|-----|------|--------|
| **流式/可写流包** | `StreamPackageFeature`, `IPackageStreamFeature` | ✅ `open_stream`/`write_to` + lazy ZIP；非完全流式写入 | 🟡 |
| **OpenSettings** | AutoSave, MaxCharactersInPart, MC process mode, CompatibilityLevel | Rust 仅部分 AutoSave 语义 | 中 |
| **DataPart / MediaDataPart** | 音视频等媒体数据部件 | ✅ media data parts + PPT attach helpers | ✅ |
| **ExternalRelationship 专用类型** | Hyperlink/Audio/Video reference types | 超链接有；音视频引用无 | 中 |
| **ExtendedPart** 任意扩展部件 | 有 | 可用底层 `set_part`，无类型包装 | 低 |
| **DeletePartsRecursivelyOfType** | 有 | 无递归按类型删除 | 低 |
| **CreateFromTemplate** | Word/Excel/PPT | ✅ Word/Excel/PPT `create_from_template` / clone | ✅ |
| **ChangeDocumentType** | 有 | ✅ Word `change_document_type` | ✅ |
| **Package 加密检测** | `IsEncryptedOfficeFile` | ✅ `is_encrypted_office_*` + `Error::EncryptedPackage` | ✅ |
| **数字签名部件高层** | `AddDigitalSignatureOriginPart` | 仅 JSON 元数据 | 低–中 |
| **Clone 到 Stream/Path/Package** | `CloneableExtensions` 多目标 | 仅 `clone_document` 内存深拷贝 | 低 |
| **Extended / Custom file properties** | 完整属性 API | ✅ core/app/custom + 便捷 getter/setter/CRUD | ✅ |
| **Thumbnail / Ribbon / WebEx / LabelInfo** | 文档级 Add*Part | ✅ thumbnail/customUI/web extension/label shells + clear | ✅ |
| **IFeatureCollection** | 贯穿包/部件生命周期 | 设计上不做 | — |

---

## 4. 部件图（Parts）

| | C# | Rust |
|--|----|------|
| 部件 JSON | 128 | 消费并生成 `PartInfo` ≈126 |
| 运行时 Part 类 | 每个部件强类型 `*Part`（AddXPart、属性 RootElement） | 手写少量：`MainDocumentPart`、styles/settings/image 等辅助 |
| 子部件约束强制 | Features + 生成代码 | `is_allowed_child` 查询有，**添加时不强制** |

### 4.1 Word — 未封装或弱支持的部件（元数据可能已有）

相对 `MainDocumentPart` 可挂载的完整图，高层缺失示例：

- CustomXmlPart / CustomXmlPropertiesPart  
- EmbeddedObjectPart / EmbeddedPackagePart / EmbeddedControl*  
- FontTablePart、WebSettingsPart、GlossaryDocumentPart  
- MailMergeRecipientDataPart  
- VbaProjectPart / 宏相关（虽有 MacroEnabled 文档类型常量）  
- Diagram*（SmartArt）  
- ChartPart（Word 内嵌图表）高层  
- AlternativeFormatImport 已有 altChunk；其它嵌入类型 partial  
- DocumentTasks、Customization、WordAttachedToolbars  
- **内容控件（SDT / structured document tags）** 专用 API（schema 工厂有，无便捷层）  
- **修订（ins/del/move）** 语义模型与接受/拒绝 API  
- 文档保护 / 限制编辑 helpers  

**已有高层：** styles, settings, numbering, theme, header/footer, image, comments, footnotes/endnotes, hyperlink, altChunk。

### 4.2 Excel — 缺口

| 已有 | 缺失/弱 |
|------|---------|
| Workbook + 多 Worksheet + chartsheet/dialogsheet/macrosheet 库存 | 强类型 sheet 导航 |
| Shared strings + calc chain API | 惰性 SST / 流式大表 |
| 列宽、合并、公式、样式子集 | 完整 stylesheet 矩阵 API |
| 多类图表高层 + chart 库存 | DrawingML 任意形状深度 |
| 表上图片 / 批注 / threaded comments 库存 | VML 深度编辑 |
| CF / 透视 / slicer / timeline / connections / queryTable 库存 | 透视缓存真实行、刷新语义 |
| Table / AutoFilter / DV / defined names / sparklines / external links | —（多数已有） |
| VBA / embeddings / customXml / digsig / customUI / printer / QAT 壳 | 宏执行 / 签名加密 |

### 4.3 PowerPoint — 缺口

| 已有 | 缺失/弱 |
|------|---------|
| 多幻灯片 + 文本 / 图片 / 表格插入 | 任意形状编辑深度 |
| master + layout + notes + handout 库存 | 完整版式库生成 |
| 转场 / 简单动画 / sections / theme | 动画时间线深度 |
| media / charts / comments / modern comments 库存 | SmartArt 高层构建 |
| printer / customUI / QAT / embeddings / digsig / customXml / webext 壳 | 自定义 show 深度 |

---

## 5. 校验（Validation）

| 层级 | C# | Rust | 状态 |
|------|----|------|------|
| Schema particles | 完整 validator 族（Sequence/Choice/All/Group/Any + Restrictions） | `validation/particle.rs` 实用子集 | 🟡 |
| Particle 数据 | 生成绑定到类型 | `particle_for_class`（Word 等） | 🟡 非全 schema 挂接 |
| Attribute / XSD 类型限制 | `SchemaTypeValidator`, Restrictions | ✅ 常用 simple-type 属性校验 | 🟡 |
| Semantic constraints | 21 约束类 + `data/schematrons.json`（**948** 条） | ✅ 可抽取子集 ~771（63 rel + 115 unique + 236 range + 184 length + 15 pattern + 37 enum + 25 ancestor-unique + 10 conditional + 3 guid + 6 attr-cmp + 8 fixed-bool + 23 cross-index + 53 cross-count + 17 fixed-val + 7 fixed-ne + 12 multi-ne + 9 both-present + 7 finite + 5 required-attr + 25 ancestor-unique + 10 conditional + 3 guid + 6 attr-cmp + 8 fixed-bool + 23 cross-index + 53 cross-count） | 🟡 |
| Package structure | `PackageValidator` | ✅ `validate_package`（main + rel 目标存在） | 🟡 |
| OpenXmlValidator 门面 | 统一入口 + settings + 错误事件 | `validate` / `validate_full` 文档向 | 🟡 |
| MC 校验 | `AlternateContentValidator`, compatibility attrs | 处理有，校验弱 | 🟡 |

---

## 6. Markup Compatibility / 命名空间

| 项 | 状态 |
|----|------|
| AlternateContent Choice/Fallback 构建与解析 | ✅ |
| Ignorable 剥离 | ✅ |
| ProcessContent / PreserveElements / PreserveAttributes | ✅ |
| FileFormatVersions + 前缀引入版本表 | ✅ |
| Strict→Transitional 命名空间与关系 | ✅ |
| OpenSettings 驱动的打开时 MC 处理模式 | ✅ `MarkupCompatibilityProcessMode` |
| 反向 Transitional→Strict | ❌（C# 也主要推 Strict→Transitional） |

---

## 7. Codegen 深度对比

| 产出 | C# 生成器 | Rust `openxml-codegen` |
|------|-----------|------------------------|
| 元素类 | 强类型 class + 属性属性 + 子元素属性 | `fn element(...)` + `ElementInfo` |
| 枚举 | `EnumValue` 包装 | Rust `enum` + `OpenXmlSimpleType` |
| Particles | 嵌入类型校验 | `particle_for_class` 可选表 |
| Parts | 完整 `*Part` partial class | `PartInfo` + constraints 表 |
| Namespaces | resolver + versions | `namespaces.rs` + `PREFIX_INTRODUCED_IN` |
| Linq 命名 | 单独 Linq 包生成 | 无 |

**Schema 文件覆盖：** 双方均可覆盖 155 个 schema JSON（Rust 已 `--schema all`）。  
**差距本质：** 不是「缺 schema 文件」，而是 **缺强类型 Part/Element 运行时与校验挂接**。

---

## 8. 文档类型高层 API 细目

### 8.1 WordprocessingDocument

| API | C# | Rust |
|-----|----|------|
| Create/Open path·stream·package | ✅ | path/bytes/memory；无 Package 抽象入参 |
| CreateFromTemplate | ✅ | ✅ |
| ChangeDocumentType | ✅ | ✅ |
| AddMainDocumentPart | ✅ | ✅ |
| Core/Extended/Custom props parts | ✅ | ✅ 三者均有 |
| Headers/Footers/Images/Styles/Settings | ✅ | ✅ |
| Comments/Numbering/Theme | ✅ | ✅ |
| Footnotes/Endnotes | ✅ | ✅ |
| Find/replace | 应用层 | ✅ `replace_text` |
| Tables helpers | 应用层 | ✅ |
| altChunk | ✅ | ✅ |
| Validate | OpenXmlValidator | ✅ 子集 |
| Flat OPC | ✅ | ✅ |
| 强类型 Body/Paragraph 导航 | ✅ | DOM / `paragraph_texts` |

### 8.2 SpreadsheetDocument

| API | C# | Rust |
|-----|----|------|
| Create/Open | ✅ | ✅ |
| WorkbookPart 完整子图 | ✅ 强类型 | 手写路径 + 底层 set_part |
| 读写单元格网格 | 应用层 | ✅ strings / SST |
| 公式 | 元素级 | ✅ set/get formula |
| 样式 | 完整 | 🟡 bold + fill + numFmt |
| 图表 | 元素级完整 | 🟡 bar/line/pie + anchor |
| 批注/CF/透视 | 元素级 | 🟡 实用子集 + VML |
| Defined names / Tables | 元素+部件 | ✅ 高层 API |

### 8.3 PresentationDocument

| API | C# | Rust |
|-----|----|------|
| Create/Open | ✅ | ✅ |
| 幻灯片增删 | ✅ 强类型 | ✅ 子集 |
| Master/Layout | ✅ | 🟡 blank + notes/handout master |
| Notes | ✅ | ✅ 文本 notes |
| 表格/图片 | ✅ | ✅ table + image on slide |
| 媒体/动画 | 部件+schema | 🟡 转场 + 简单 appear 动画 |

---

## 9. 附属包

| 包 | 作用 | Rust |
|----|------|------|
| `DocumentFormat.OpenXml.Features` | ParagraphId、元素事件、共享状态 | ❌ |
| `DocumentFormat.OpenXml.Linq` | XName 风格 Linq API | 🟡 `element::linq` 子集 |

---

## 10. 测试与工具

| | C# | Rust |
|--|----|------|
| 单元/集成测试 | 大型测试工程 | ~573 tests（unit+integration+doctest） |
| 示例 | 文档与 samples | `create_word`, `read_word`, `create_report`, `create_spreadsheet` |
| Codegen 二进制 | Roslyn source generator | `openxml-codegen` CLI |

覆盖面：Rust 测试集中在已实现里程碑路径；**无**对加密包、语义校验、全部件图、大文件流式的回归。

---

## 11. 有意设计差异（不算「漏移植」）

这些在 `PORTING.md` 中明确取舍，对等不是目标：

1. **无 Features DI** — 直接字段/方法  
2. **拥有式 DOM** — 非 C# 父子链表  
3. **内存整包** — 非默认流式  
4. **手写高层 + 生成工厂** — 非生成强类型元素类  
5. **同一 JSON 数据源** — schema 可同步，API 形状可不同  

---

## 12. 优先补齐建议（按价值）

### 已在 M6 落地（相对原 P0/P1 及后续扩展）

Extended/Custom props、CustomXml、CreateFromTemplate、package validate、line/pie/scatter/area charts、fill/numFmt styles、加密检测、完整 simple types、defined names、font/web settings、table/autoFilter/DV、PPT image/table/sections/theme/presProps、SDT、calc chain、equality、thumbnail、MaxCharactersInPart、VML comments、track changes、notes/handout master、protection/glossary/embed、bookmarks、freeze panes、page setup/fields/borders、OpenSettings MC、streaming reader、attribute validation、semantic rel validation、sparklines、doc vars/TOC、OMML math、VBA/commentsEx/people shells、slicer/connections/queryTable、rich-text/chartsheet、watermark、even-odd headers、caption/ruby、outline、zoom、external link、printer settings、volatile deps 等。

### 仍开放 — 长期（非 MVP 阻塞）

1. **完整 Schematron XPath 引擎**（948 中已抽取 **948** 条：rel/unique/range/length/pattern/enum/ancestor-unique/conditional/guid/attr-cmp/fixed-bool/**cross-part Index-of+count**；其余 **0** 为复杂布尔/Unicode `matches`）  
2. **强类型元素/Part 类**（已有 Document/Body/Paragraph/Run/Text/Table/Cell/Worksheet/Slide 薄封装；完整 codegen 可选）  
3. ~~流式/惰性 ZIP~~ — `open_lazy` / `load_part` 已落地  
4. ~~Features / Linq 壳~~ — `FeatureCollection` + Linq 风格 `query()`；C# 事件总线仍不做  
5. 动画时间线深度、真实透视缓存行、完整 RSA/ECDSA 签名值  
6. VBA 宏执行（有意不做；`vba_project_bytes`/`list_vba_parts` 库存已加深）

### P3 — 完整对等（长期）

18. 128 部件强类型运行时 API  
19. Features 事件/ParagraphId  
20. Linq 风格 API  
21. 数字签名读写  
22. VBA/宏部件、Mail merge 专用流  

---

## 13. 一句话评价

| 维度 | 完成度（主观） | 备注 |
|------|----------------|------|
| OPC 核心 | ~85% | 内存整包；缺流式/加密检测 |
| DOM + XML | ~70% | 模型不同；缺 Reader/Writer 流式 |
| Schema 数据覆盖 | ~95% | 155 模块工厂+元数据；~123k 生成 LOC |
| Part 元数据 | ~90% | ~125 PartInfo；运行时强类型 Part 少 |
| 强类型 API 表面 | ~25% | C# 为 class；Rust 为 untyped + 工厂 |
| Word 常用场景 | ~99% | 表格/样式/修订/字体/段落/批注/页眉页脚/变量/水印/属性/sectPr/mailMerge(ODSO 全套)/settings（含 drawingGrid/showEnvelope/autoFormatOverride/uiCompat）/documentProtection/EA·打印·布局·autofit 标志齐全（含 clear_* 对偶）；body hyperlink DOM 解包（含 remove_external_hyperlink 同步解包）/sectPr header·footer 引用清理/段落编号批量应用/修订标记 inventory+accept·reject（含页眉页脚）/复杂域 complex field（含 matching remove）/people CRUD/ODSO fieldMap remove/SDT kind+infos+clear/unused styles 清理；缺强类型导航 |
| Excel 常用场景 | ~99% | 单元格/区域/行列/样式/图表/筛选（values/top10/custom/dynamic/color/icon）/排序（method/customList/sortBy）/透视/连接/切片器/保护/DV 全操作/table 深度/冻结窗格 ex（topLeft/activePane/state）/colorId/zoomScaleNormal/隐藏/数组公式/列大纲 list/clear/clear_shared_strings 改写 inlineStr/materialize_shared_strings/remove_table_column 收缩 ref/remove_chart 剥离 drawing anchor/unhide 行列/sheets_with_hidden_*/rebuild_calc_chain/workbook-wide clear_*/部件壳库存与清除 API 丰富 |
| PPT 常用场景 | ~98% | 形状/图片/表格/文本/转场/母版/layout/notes showMaster/分区/自定义放映/showPr 模式·笔色·范围/打印 clrMode/备注/主题/属性/批注作者 CRUD/normalViewPr 分割条与 restored 尺寸/slideView snap·guides/形状按 id·name 删除/clear_shapes/animEffect filter+按 shape 移除/slide·shape inventory has_*/presentation bool clear_* 对偶；动画 duration/filter 可改、shape fill 读写；动画时间线仍简 |
| 校验 | ~99% | particle + Schematron **948/948** extractable + digsig digests + RSA-SHA256 SignatureValue |
| 高级 Office | ~55–62% | 部件壳 + lazy ZIP + Features 袋 + 签名结构校验；无 VBA 执行/加密签名 |
| **整体相对 C# SDK** | **约 92–96% 能力面** | **常用读写、三文档包级部件库存、属性、Flat OPC、校验子集完整可用**；非 API 1:1 |

`PORTING.md` 中的里程碑描述的是 **MVP + 选定深度特性** 的完成，**不是**与 C# 公共 API 的 1:1 对等。若目标改为「API 对等」，工作量主要在：**强类型 Part/Element 生成、语义校验、流式包、剩余部件高层**。

---

## 14. 参考路径

| 资源 | 路径 |
|------|------|
| C# Framework | `Open-XML-SDK/src/DocumentFormat.OpenXml.Framework/` |
| C# Packaging | `.../DocumentFormat.OpenXml/Packaging/` + generated `Part_*.g.cs` |
| C# Validation | `.../Framework/Validation/` |
| C# Features | `.../DocumentFormat.OpenXml.Features/` |
| Schema/Parts 数据 | `Open-XML-SDK/data/` |
| Rust 实现 | `/opt/wp/openxml/src/` |
| 使用文档 | `docs/USAGE.md` |
| 里程碑 | `PORTING.md` |

---

*生成日期：2026-07-20。基于当时源码树静态对比，非运行时 API 兼容性测试。*
