# officexml

Rust port of the [Open XML SDK](https://github.com/OfficeDev/Open-XML-SDK) — a library for creating and manipulating Microsoft Office Word (`.docx`), Excel (`.xlsx`), and PowerPoint (`.pptx`) packages.

Library crate name: **`officexml`**. Source: https://github.com/WangTianYou537/openxml  
**Not published to crates.io** — the full package (generated schemas + optional font assets) exceeds registry size limits without dropping functionality.

**移植已完成**（见 [PORTING.md](PORTING.md)）。完整使用说明见 **[docs/USAGE.md](docs/USAGE.md)**。

## Build / CLI

```bash
cargo build --release
cargo run --release --bin svg2pptx -- -o deck.pptx a.svg b.svg c.svg
```

### `svg2pptx` — multi-slide

Each input SVG becomes one full-bleed 16:9 slide:

```bash
svg2pptx -o deck.pptx slide1.svg slide2.svg slide3.svg
svg2pptx --embed-font deck.pptx a.svg b.svg
svg2pptx --font-shape a.svg b.svg          # writes a.pptx
```

| Flag | Behavior |
|------|----------|
| *(default)* | Editable text boxes; system fonts (Times New Roman / Microsoft YaHei when SVG omits `font-family`) |
| `--font-shape` | Outline glyphs as shapes; no text boxes / font embed |
| `--embed-font` | Editable text + on-demand subset EOT (`.fntdata`) embed |
| `--embed-font-fully` | Editable text + full EOT embed of used faces |

Optional embed fonts: place `NotoSansSC-Regular.ttf` / `NotoSansSC-Bold.ttf` under `assets/fonts/`. Subset mode needs Python + `fontTools` (`scripts/subset_ttf.py`).

## GitHub Releases

CI runs on every push to `main`. Pushing a tag `v*` (e.g. `v0.1.0`) builds multi-platform `svg2pptx` binaries and publishes a GitHub Release.

```bash
git tag v0.1.0
git push origin v0.1.0
```



## Status

| Area | Status |
|------|--------|
| OPC package (ZIP, content types, relationships) | ✅ |
| OpenXmlElement DOM + XML read/write | ✅ |
| WordprocessingDocument create / open / save | ✅ |
| Core WordprocessingML (`Document`, `Body`, `Paragraph`, `Run`, `Text`) | ✅ |
| SpreadsheetDocument create / open + simple sheet I/O | ✅ |
| PresentationDocument create / open + empty slide | ✅ |
| Schema codegen (`openxml-codegen` from C# `data/*.json`) | ✅ all 155 schemas |
| Generated typed constructors + attribute/child metadata | ✅ |
| Simple types + attribute get/set helpers | ✅ |
| Package core properties (`docProps/core.xml`) | ✅ |
| Word: styles, settings, images, headers/footers, hyperlinks | ✅ |
| Word: comments, numbering, theme, tables, find/replace, clone | ✅ |
| Word: altChunk (HTML/text/RTF embed) | ✅ |
| Excel: multi-sheet, shared strings, column widths | ✅ |
| Excel: merge cells, minimal styles, formula cells | ✅ |
| PowerPoint: multi-slide + text boxes + slide size | ✅ |
| Schema enum codegen (`OpenXmlSimpleType`) | ✅ |
| Typed attr setters + part child constraints | ✅ |
| PowerPoint slide masters / layouts | ✅ |
| Flat OPC open/save | ✅ |
| Markup Compatibility (AC + Ignorable + ProcessContent/Preserve*) | ✅ |
| Strict → Transitional namespace rewrite | ✅ |
| Word schema validation (rules + ordered particles) | ✅ |
| Particle codegen (`particle_for_class`) | ✅ |
| Word footnotes / endnotes | ✅ |
| PowerPoint notes slides | ✅ |
| Excel bar/column chart parts + on-sheet anchors | ✅ |
| Excel on-sheet images (oneCell anchor) | ✅ |
| Excel worksheet cell comments | ✅ |
| Excel conditional formatting (cellIs, colorScale) | ✅ |
| Excel pivot tables (cache + definition) | ✅ |
| MC FileFormatVersions matrix (from namespaces.json) | ✅ |
| Extended + Custom document properties | ✅ |
| Word CustomXml parts | ✅ |
| CreateFromTemplate / ChangeDocumentType | ✅ |
| Package structure validation | ✅ |
| Excel line/pie charts + fill/numFmt styles | ✅ |
| Encrypted package detection | ✅ |
| Full simple types (Base64/DateTime/List/…) | ✅ |
| Excel defined names | ✅ |
| Word font table + web settings | ✅ |
| Excel table / autoFilter / data validation | ✅ |
| PowerPoint image on slide | ✅ |
| Word SDT content controls | ✅ |
| Excel calculation chain | ✅ |
| Element structural equality | ✅ |
| Thumbnail + MaxCharactersInPart | ✅ |
| Excel comments VML drawing | ✅ |
| Word track changes accept/reject | ✅ |
| PowerPoint table on slide | ✅ |
| OpenSettings MC process mode | ✅ |
| Word document protection + glossary | ✅ |
| PowerPoint notes master | ✅ |
| Excel sheet/workbook protection | ✅ |
| Word embedded package | ✅ |
| Word bookmarks | ✅ |
| Excel freeze panes | ✅ |
| PowerPoint handout master | ✅ |
| Page setup + simple fields | ✅ |
| Streaming XML reader | ✅ |
| Attribute type validation | ✅ |
| Digital signature shells | ✅ |
| Media data parts (audio/video) | ✅ |
| Excel row heights | ✅ |
| Anchor hyperlinks + mail-merge recipients | ✅ |
| Excel scatter chart / tab color / print area | ✅ |
| Word page-number footer | ✅ |
| Word doc variables + TOC field | ✅ |
| Excel sparklines | ✅ |
| Semantic relationship validation | ✅ |
| Excel area chart | ✅ |
| Word background + drop cap | ✅ |
| Excel dimension + shared formulas | ✅ |
| PowerPoint sections | ✅ |
| Word watermark | ✅ |
| Excel active tab | ✅ |
| Excel sheet state / calcPr / iconSet CF | ✅ |
| PowerPoint hide slide | ✅ |
| Excel row outline | ✅ |
| Word even/odd headers / caption / ruby | ✅ |
| Word OMML math | ✅ |
| Excel sheet zoom | ✅ |
| Word docDefaults + bibliography | ✅ |
| Excel external link | ✅ |
| PowerPoint theme | ✅ |
| Word page borders | ✅ |
| Excel rich-text + chartsheet | ✅ |
| Word VBA + commentsExtended shells | ✅ |
| Excel slicer shell + theme | ✅ |
| Word people + customXml props | ✅ |
| Excel connections | ✅ |
| Word printer settings | ✅ |
| PPT pres/view properties | ✅ |
| Excel queryTable + volatileDeps | ✅ |
| PPT comment authors | ✅ |
| Word customUI + document tasks | ✅ |
| Excel timeline shell | ✅ |
| PPT slide comments | ✅ |
| Excel named Title style | ✅ |
| Excel print titles | ✅ |
| Word styles + web extension | ✅ |
| Word track revisions + compat mode | ✅ |
| Excel sheetFormatPr + doughnut chart | ✅ |
| Excel pivot cache real rows | ✅ |
| Word updateFields on open | ✅ |
| PPT table styles | ✅ |
| Word tabs/symbol/mirror margins | ✅ |
| Excel gridlines/headers toggle | ✅ |
| PPT clone slide | ✅ |
| PPT slide transitions | ✅ |
| Excel cell hyperlinks / sort / whole DV | ✅ |
| Word spacing/shading/highlight | ✅ |
| Word page breaks / indent | ✅ |
| Excel page breaks | ✅ |
| PPT appear animation | ✅ |
| Word DATE/TIME fields | ✅ |
| Excel array formula / local names | ✅ |
| PPT slide header/footer | ✅ |
| Word formatting + diagram shell | ✅ |
| PPT slide background | ✅ |
| Excel XML maps | ✅ |
| Excel chart styles / dialogsheet / named views | ✅ |
| Word labels + OLE | ✅ |
| PPT modern comments | ✅ |
| Excel rich values / macrosheet / theme override | ✅ |
| Word stylesWithEffects / vbaData / QAT | ✅ |
| PPT tags / 3D model / slide sync | ✅ |
| Excel threaded comments / revisions / metadata | ✅ |
| Word commentsIds / extensible / toolbars | ✅ |
| Excel chartEx / chartDrawing / ActiveX | ✅ |
| Word legacy diagram / embedded package | ✅ |
| PPT chart drawing | ✅ |
| CustomProperty / Font parts | ✅ |
| Word/PPT embedded charts | ✅ |
| Diagram persist layout | ✅ |
| Semantic unique-attr + Excel/PPT rel validation | ✅ |
| delete_part on Word/Excel/PPT | ✅ |
| Radar/bubble charts + ExtendedPart | ✅ |
| PPT clone_document | ✅ |
| Spreadsheet attr range validation | ✅ |
| Excel border style + set_cell_style | ✅ |
| PPT auto-shape / text-box / remove_slide | ✅ |
| Excel rename/remove sheet | ✅ |
| Excel cell write + Word paragraph style | ✅ |
| Excel get_cell + Word para ops + PPT move_slide | ✅ |
| Word table append/extract + Excel insert/delete rows | ✅ |
| Excel copy_sheet / clear_cell + PPT blank slide | ✅ |
| Excel clear_range + Word run formatting | ✅ |
| Excel range I/O + PPT text replace | ✅ |
| Excel column hidden + Word bullet list | ✅ |
| Excel find/replace sheet + PPT set_slide_text | ✅ |
| Excel used_range + Word stats + PPT notes_text | ✅ |
| Excel move_sheet + Word bookmarks() | ✅ |
| Excel merge/clear + Word heading/hyperlink | ✅ |
| Excel columns + Word table row + PPT all texts | ✅ |
| Excel filter/row hide + Word remove_table + PPT shape_count | ✅ |
| Hyperlink list + macro/part_count helpers | ✅ |
| Remove hyperlink + contains_text helpers | ✅ |
| Defined names + char count + has_notes | ✅ |
| ensure/has styles & theme | ✅ |
| list headers/footers + media/chart counts | ✅ |
| remove header/footer + drawings + master counts | ✅ |
| has charts/comments/footnotes/media flags | ✅ |
| list parts + rel counts + SST/calc flags | ✅ |
| tables/protection + hidden slides + SDT tags | ✅ |
| clear protection + external link inventory | ✅ |
| track/freeze/sections/transition flags | ✅ |
| zoom/tab/print/sheet-state + Word settings getters | ✅ |
| size/background/dimension getters | ✅ |
| doc vars/DV/calc/animation flags | ✅ |
| CF/sparkline/sort + note/comment counts | ✅ |
| slicer/timeline/connections/query/people flags | ✅ |
| handout/tags/slide-sync presence flags | ✅ |
| clear sparklines/print/tab + view getters | ✅ |
| list/clear comments & styles + PPT sections/bg | ✅ |
| col/row dim + freeze/page getters | ✅ |
| clear sheet comments + Word notes/page getters | ✅ |
| PPT transition getter + count_text | ✅ |
| table infos / SST list / row-col hidden | ✅ |
| outline levels + header/footer texts | ✅ |
| external hyperlinks + protection edit | ✅ |
| PPT clear_notes | ✅ |
| remove_table / list DV / clear calcChain | ✅ |
| cell style index + clear headers/footers | ✅ |
| remove external hyperlink + slide titles | ✅ |
| list formulas / page breaks getters | ✅ |
| paragraph style ids + clear numbering | ✅ |
| slides_with_transition | ✅ |
| list CF / clear SST / customXml | ✅ |
| PPT list_media + clear notes/handout masters | ✅ |
| clear external links / pivot infos | ✅ |
| clear glossary + list altChunks/charts | ✅ |
| sheet_format / remove_bookmark | ✅ |
| clear user tags + slide sync | ✅ |
| calc props / clear drawings / thumbnail | ✅ |
| PPT list masters/layouts | ✅ |
| clear slicers/timelines/connections/QT | ✅ |
| clear theme/VBA/font table | ✅ |
| clear styles/settings/media/sigs | ✅ |
| clear people/web/printer/mail-merge | ✅ |
| clear charts/pivots + pres/view props | ✅ |
| clear images/footnotes/endnotes/PPT charts | ✅ |
| named styles / clear altChunks / tableStyles | ✅ |
| doc vars CRUD + clear cell hyperlinks | ✅ |
| content_control_count | ✅ |
| hidden sheets / clear merges / array formulas | ✅ |
| numFmts / shared formulas / slides_with_animation | ✅ |
| style/font table lists + notes/handout counts | ✅ |
| get_defined_name / list_sheet_states / section_count | ✅ |
| sheet_count / table_columns / hidden slides / anchors | ✅ |
| column_count / clear page breaks / style ids | ✅ |
| clear slide comments | ✅ |
| list fills / protection flags / clear watermark | ✅ |
| has_auto_filter / border_count / page borders | ✅ |
| clear slide header/footer | ✅ |
| clear doc background / dxf_count / slides_with_notes | ✅ |
| mirror margins / list_calc_chain / slides_with_bg | ✅ |
| even/odd headers / table_names / slides_with_comments | ✅ |
| is_sheet_hidden / clear even-odd / slides_with_hf | ✅ |
| hidden rows/cols lists / merge count / print flags | ✅ |
| bookmark names / docVar get / image_count | ✅ |
| transition/animation counts | ✅ |
| defined name/drawing/table flags | ✅ |
| page-break counts + HF/hyperlink counts | ✅ |
| notes_count / total_shape_count | ✅ |
| formula/hyperlink/style fill counts | ✅ |
| array/shared formula counts | ✅ |
| Word style/font counts | ✅ |
| PPT chart/comments/bg/hf counts | ✅ |
| slicer/timeline/connection counts | ✅ |
| page margin/setup flags | ✅ |
| people/mail-merge/printer counts | ✅ |
| PPT props/master convenience flags | ✅ |
| active tab / zoom / dimension flags | ✅ |
| mirror margins clear + page size flags | ✅ |
| PPT slide/notes size flags | ✅ |
| core/app/custom property flags + counts | ✅ |
| title/creator convenience getters/setters | ✅ |
| subject/keywords/description/category | ✅ |
| application/company + custom prop CRUD | ✅ |
| lastModifiedBy/revision/language/version/status | ✅ |
| manager/template/hyperlinkBase convenience | ✅ |
| threaded comments / persons inventory | ✅ |
| Word customUI/vbaData/commentsEx flags | ✅ |
| PPT tags/sync/authors/model3d inventory | ✅ |
| clear threaded/persons/chart styles/rich | ✅ |
| clear customUI/vbaData/commentsEx parts | ✅ |
| clear PPT authors/model3d | ✅ |
| query/person/threaded comment lists | ✅ |
| PPT author/tag/sync list entries | ✅ |
| Word tasks/webExt/QAT/diagram/embed clear | ✅ |
| Excel chart/dialog/macro/xmlMaps/sort/rev clear | ✅ |
| Excel metadata/themeOverride/customData clear | ✅ |
| PPT modern comments/authors/chart drawings | ✅ |
| Schematron subset (rel + unique + range/len/pattern) | ✅ |
| validate_schematron (Word/Excel/PPT) | ✅ |
| Schematron attr constraints (~599 extractable/948) | ✅ |
| Excel/PPT thumbnail + digsig + customXml parity | ✅ |
| Excel/PPT embeddings + VBA shell parity | ✅ |
| Excel/PPT customUI parity | ✅ |
| Excel/PPT printer/QAT/toolbars parity | ✅ |
| Excel/PPT label info (MIP) parity | ✅ |
| Excel/PPT web extension shell parity | ✅ |
| Font parts inventory (Word/Excel/PPT) | ✅ |
| Word charts + Excel/PPT diagrams inventory | ✅ |
| Excel/PPT open/create_with_settings | ✅ |
| Excel/PPT create_simple shortcuts | ✅ |
| Excel/PPT validate + customXml props | ✅ |
| from_bytes + remove_part aliases (all docs) | ✅ |
| Excel/PPT images inventory + Word has_images | ✅ |
| Theme list/count inventory (all docs) | ✅ |
| Styles inventory parity (count + PPT aliases) | ✅ |
| Excel/PPT comments inventory (has/clear) | ✅ |
| Excel/PPT add_embedded_object | ✅ |
| Excel/PPT diagram shell (SmartArt parts) | ✅ |
| main_relationship aliases (Excel/PPT) | ✅ |
| Excel list_comments / comment_count | ✅ |
| Excel/PPT embedded_package_part alias | ✅ |
| Excel/PPT legacy diagram text shell | ✅ |
| Excel/PPT add_chart convenience alias | ✅ |
| Excel/PPT add_image media helper | ✅ |
| Excel default styles + style id list | ✅ |
| Media/hyperlink inventory helpers | ✅ |
| Drawings inventory (Word/PPT + Excel existing) | ✅ |
| PPT hyperlink inventory | ✅ |
| Excel clear_tables + defined name aliases | ✅ |
| Hyperlink alias parity (all docs) | ✅ |
| Excel has_workbook/sheet_protection aliases | ✅ |
| create_from_template_as + doc protection alias | ✅ |
| Excel pivot cache + sparkline inventory | ✅ |
| Merge/hidden/master/docvar inventory aliases | ✅ |
| Excel page breaks inventory | ✅ |
| PPT clear_all_notes / has_notes_slides | ✅ |
| PPT bulk clear transitions/animations | ✅ |
| PPT bulk clear backgrounds/header-footers | ✅ |
| validate_schematron_attributes (all docs) | ✅ |
| Excel get_zoom / has_sheet_view helpers | ✅ |
| Excel tab color + dimension aliases | ✅ |
| Word zoom + PPT slide/notes size clear | ✅ |
| Word settings view/grid/tabStop helpers | ✅ |
| Excel workbook view + Word trackRevisions alias | ✅ |
| Word settings hyphenation/embed/preview/gutter flags | ✅ |
| Word hide spelling/grammar/print flags + gridlines alias | ✅ |
| Word more display/print settings flags | ✅ |
| Word forms/privacy/border settings flags | ✅ |
| Word East-Asian/spacing settings flags | ✅ |
| Excel sheet view right-to-left helpers | ✅ |
| Excel showZeros + Word characterSpacingControl | ✅ |
| Excel outline symbols + sheet view type | ✅ |
| Excel workbookPr date1~1513/backup/filterPrivacy | ✅ |
| Excel codeName + PPT rtl/firstSlideNum + Word attachedTemplate | ✅ |
| Excel workbookPr extended flags (showObjects/updateLinks/…) | ✅ |
| PPT presentation attrs (serverZoom/embedFonts/conformance/…) | ✅ |
| Word compat flags + compatSetting inventory | ✅ |
| Excel sheet view/printOptions/pageSetup/calcPr extended | ✅ |
| PPT viewProps lastView/showComments/gridSpacing | ✅ |
| Excel workbookView window/scroll/tabs/visibility | ✅ |
| Word locale/hyphen/bookFold/themeFont/drawingGrid settings | ✅ |
| PPT showPr (loop/narration/animation/mode) + prnPr | ✅ |
| Excel sheetFormatPr baseCol/zeroHeight/thick/outline | ✅ |
| Word proofState spelling/grammar | ✅ |
| Excel sheet header/footer (odd/even/first) | ✅ |
| PPT custom shows inventory | ✅ |
| Word writeProtection shell | ✅ |
| Word sectPr orientation/cols/titlePg/vAlign/type/bidi | ✅ |
| Excel activeCell selection + sortState read/caseSensitive | ✅ |
| Word line numbering / page num fmt / textDirection / gutter | ✅ |
| Excel protectedRanges / ignoredErrors / scenarios | ✅ |
| PPT photoAlbum / kinsoku / slide HF getters | ✅ |
| Word revisionView / captions / mathPr / clrSchemeMapping | ✅ |
| Excel customSheetViews inventory | ✅ |
| Excel oleObjects / controls / webPublishItems | ✅ |
| PPT modifyVerifier shell | ✅ |
| Word XML/schema settings flags | ✅ |
| Word rsids / attachedSchema / saveThroughXslt | ✅ |
| PPT embeddedFontLst inventory | ✅ |
| Excel sheetPr outlinePr/pageSetUpPr/filterMode/transition | ✅ |
| Word doNotIncludeSubdocsInStats | ✅ |
| Excel phoneticPr / customWorkbookViews / dataConsolidate | ✅ |
| Word activeWritingStyle | ✅ |
| PPT customerData list shell | ✅ |
| Excel autoFilter columns (values/top10) + dataConsolidate refs | ✅ |
| Word mailMerge settings shell | ✅ |
| PPT slide cSld name | ✅ |
| Excel autoFilter custom/dynamic + column kind | ✅ |
| Word mailMerge ODSO + activeRecord | ✅ |
| PPT notes cSld name | ✅ |
| Excel hyperlink tooltip/location | ✅ |
| Word mailMerge destination/subject/address/attachment | ✅ |
| PPT notes header/footer flags | ✅ |
| Excel DV messages/date/textLength + blank filter | ✅ |
| Word mailMerge blankLines/linkToQuery/checkErrors/connect | ✅ |
| Excel col bestFit/style/outlineLevel/collapsed | ✅ |
| PPT showPr sldRg/sldAll/custShow | ✅ |
| Excel DV decimal/custom + showDropDown | ✅ |
| Word stylePaneFormatFilter | ✅ |
| PPT show pen color | ✅ |
| Excel row thickTop/thickBot/collapsed/style | ✅ |
| Word saveXmlDataOnly/useXSLT/alwaysMergeEmptyNS | ✅ |
| Excel DV time type | ✅ |
| Word themeFontLang eastAsia/bidi + autoCaption | ✅ |
| PPT notes/handout master HF | ✅ |
| Excel fileVersion/fileSharing/oleSize | ✅ |
| Word webSettings flags/encoding/ppi | ✅ |
| PPT slide size type | ✅ |
| Excel functionGroups inventory | ✅ |
| Word footnotePr/endnotePr settings | ✅ |
| PPT defaultTextStyle shell | ✅ |
| Excel sheetProtection extended permission flags | ✅ |
| Word webSettings folder/filename/singleFile/targetScreen | ✅ |
| PPT slide master header/footer flags | ✅ |
| Word docGrid type/charSpace + page border options | ✅ |
| PPT slide layout header/footer flags | ✅ |
| created/modified timestamps | ✅ |
| part_content_type / package rels list | ✅ |
| content-type overrides + part rel lists | ✅ |
| AppVersion / DocSecurity / stats props | ✅ |
| typed custom i4/bool + app flags | ✅ |
| create_presentation example | ✅ |
| sheet relationship list/count | ✅ |
| part bytes + package rel target | ✅ |
| Excel/PPT Flat OPC | ✅ |
| settings + strict rewrite (all docs) | ✅ |
| change_document_type + close (all docs) | ✅ |
| Excel workbookProtection lockRevision + flags_ex | ✅ |
| Excel tableStyleInfo / totals / rename / set_ref | ✅ |
| Excel table column totals + rename column | ✅ |
| Excel calcMode / fullCalcOnLoad / calcCompleted | ✅ |
| Excel sheetCalcPr + DV disablePrompts | ✅ |
| Word drawingGrid origin + displayEvery | ✅ |
| Word saveFormsData / doNotEmbedSmartTags | ✅ |
| Word documentProtection_ex (formatting lock) | ✅ |
| Word styleLockQFSet | ✅ |
| PPT comment author append/lookup/remove | ✅ |
| PPT showMasterSp / showMasterPhAnim | ✅ |
| PPT slide layout type/preserve/matchingName | ✅ |
| Excel table comment/insertRow/totalsRowShown/published | ✅ |
| Word mailMerge query get/set | ✅ |
| PPT layout showMasterSp | ✅ |
| Word EA break/table/grid settings flags | ✅ |
| Excel tableType / connectionId | ✅ |
| PPT layout userDrawn | ✅ |
| Word print/layout legacy settings flags | ✅ |
| PPT section rename/remove | ✅ |
| Word table layout settings flags (growAutofit/…) | ✅ |
| Excel dataValidations window position | ✅ |
| Word mailMerge viewMergedData setter | ✅ |
| Excel table displayName + dxf ids | ✅ |
| Word spaceForUL setting | ✅ |
| PPT slide master preserve | ✅ |
| Word autofit/hangul/splitPgBreak settings flags | ✅ |
| Excel table cell styles | ✅ |
| PPT notes showMasterSp | ✅ |
| Excel table insertRowShift + border dxf ids | ✅ |
| Word mailMerge mainDocumentType/dataType setters | ✅ |
| PPT layout/notes showMasterPhAnim | ✅ |
| Word alignTables/forgetLastTab/ansiKerning/cachedColBalance | ✅ |
| Excel table column uniqueName + column dxf ids | ✅ |
| PPT custom show rename/lookup | ✅ |
| Word suppress/convMailMerge/subFont settings | ✅ |
| Word ODSO field maps / colDelim / fHdr | ✅ |
| Excel table column cell styles + queryFieldId | ✅ |
| Excel multi sort conditions | ✅ |
| PPT custom show set slides + verifier spinCount | ✅ |
| Word ODSO udl/type | ✅ |
| Excel table id/ref getters | ✅ |
| PPT customer data remove | ✅ |
| Excel DV remove/operator/allowBlank | ✅ |
| Word ODSO recipientData + clear field maps | ✅ |
| PPT show mode browse/kiosk getters | ✅ |
| Excel DV type/formulas + filter column buttons/remove | ✅ |
| PPT print color mode | ✅ |
| Excel color/icon autoFilter | ✅ |
| Word ODSO table/src individual setters | ✅ |
| PPT printWhat | ✅ |
| Excel DV errorStyle/showMessages getters + totalsRowCount | ✅ |
| PPT browse scrollbar update | ✅ |
| Excel sort range update + DV sqref change | ✅ |
| PPT kiosk restart update | ✅ |
| Excel DV imeMode + remove sort condition | ✅ |
| PPT clear print properties | ✅ |
| Excel sortMethod/columnSort/customList/sortBy | ✅ |
| PPT remove embedded font by typeface | ✅ |
| Excel sort condition icon/dxf/descending | ✅ |
| Word compatSetting remove/get | ✅ |
| Excel table column id + customFilters and flag | ✅ |
| PPT photoAlbum/kinsoku attribute updates | ✅ |
| Excel DV message field setters | ✅ |
| PPT modifyVerifier attribute updates | ✅ |
| Word clear forceUpgrade | ✅ |
| Excel defaultRowHeight/defaultColWidth helpers | ✅ |
| Excel sort condition details reader | ✅ |
| PPT clear serverZoom/bookmarkIdSeed/conformance | ✅ |
| Excel selection sqref + HF scale/align getters | ✅ |
| Word writeProtection algorithmName shell | ✅ |

## Architecture

Mirrors the C# SDK layers:

```text
packaging/      WordprocessingDocument, SpreadsheetDocument, PresentationDocument
wordprocessing/ Document, Body, Paragraph, Run, Text helpers
spreadsheet/    Workbook, Worksheet, Row, Cell helpers
presentation/   Presentation, Slide helpers
element/        OpenXmlElement DOM, XML parse/serialize
opc/            ZIP package, [Content_Types].xml, .rels
namespace/      OOXML namespace, content-type, relationship constants
```

The C# SDK generates thousands of strongly-typed classes from JSON schemas under `Open-XML-SDK/data/`. The Rust port starts with a hand-written core and will add a codegen step that consumes the same JSON.

## Documentation

- **[使用指南 / User Guide](docs/USAGE.md)** — 安装、Word/Excel/PPT API、MC、校验、codegen、C# 对照
- **[与 C# 差距分析 / Gap Analysis](docs/GAP_ANALYSIS.md)** — 相对 Open-XML-SDK 未实现/部分实现功能清单
- [PORTING.md](PORTING.md) — 移植里程碑与架构决策
- `cargo doc --no-deps --open` — 生成 API 文档

## Usage

### Create a Word document

```rust
use officexml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
use officexml::wordprocessing::{body, document, paragraph, run, text};

let mut doc = WordprocessingDocument::create(
    "hello.docx",
    WordprocessingDocumentType::Document,
)?;

doc.add_main_document_part().set_document(document(vec![body(vec![
    paragraph(vec![run(vec![text("Hello from Rust!")])]),
])]));

doc.save()?;
```

### Read paragraphs

```rust
use officexml::packaging::WordprocessingDocument;

let mut doc = WordprocessingDocument::open("hello.docx", false)?;
for p in doc.paragraph_texts()? {
    println!("{p}");
}
```

### Create a spreadsheet

```rust
use officexml::packaging::{SpreadsheetDocument, SpreadsheetDocumentType};

let mut doc = SpreadsheetDocument::create("grid.xlsx", SpreadsheetDocumentType::Workbook)?;
doc.write_sheet_strings("Sheet1", &[
    vec!["Name", "Score"],
    vec!["Alice", "95"],
])?;
doc.save()?;
```

### Examples

```bash
cargo run --example create_word -- /tmp/hello.docx
cargo run --example read_word -- /tmp/hello.docx
cargo run --example create_report -- /tmp/report.docx
cargo run --example create_spreadsheet -- /tmp/report.xlsx
```

## Development

```bash
cargo test
cargo build --release
```

### Regenerate schema bindings

The typed WordprocessingML constructors under `src/generated/` are produced from the C# SDK data files:

```bash
cargo run --bin openxml-codegen -- \
  --data /opt/wp/Open-XML-SDK/data \
  --out src/generated
```

Use `--schema all` to emit every schema JSON (large). Default is `wordprocessingml_2006_main`.

See [PORTING.md](PORTING.md) for the full milestone plan and C# ↔ Rust API map.

### Porting notes

Source of truth for the full API surface:

- Framework: `Open-XML-SDK/src/DocumentFormat.OpenXml.Framework/`
- Typed packages: `Open-XML-SDK/src/DocumentFormat.OpenXml/Packaging/`
- Schema data: `Open-XML-SDK/data/schemas/`, `data/parts/`, `data/namespaces.json`
- Generated C# (reference): `Open-XML-SDK/generated/`

## License

MIT (same as the upstream Open XML SDK).
