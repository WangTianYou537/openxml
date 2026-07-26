# Porting plan: Open-XML-SDK → Rust (`/opt/wp/openxml`)

This document tracks how the C# Open XML SDK maps onto the Rust crate and what remains.

## Source layout (C#)

| C# path | Role |
|---------|------|
| `src/DocumentFormat.OpenXml.Framework/` | OPC, DOM, simple types, validation |
| `src/DocumentFormat.OpenXml/Packaging/` | Word / Excel / PPT package APIs |
| `data/schemas/*.json` | Schema types for codegen (~157 files) |
| `data/parts/*.json` | Part graph + content/relationship types |
| `data/namespaces.json` | Prefix ↔ URI table |
| `generated/` | Pre-generated typed C# (reference output) |
| `gen/` | Roslyn source generator |

## Rust layout

```text
src/
  opc/            ZIP + [Content_Types].xml + .rels + core props   ✅
  element/        OpenXmlElement DOM + XML R/W                     ✅
  simple_types/   StringValue, OnOffValue, Int32Value, …           ✅
  packaging/      Document packages + parts                        ✅ (core)
  wordprocessing/ Hand-written Word helpers                        ✅ (minimal)
  spreadsheet/    Hand-written Excel helpers                       ✅ (minimal)
  presentation/   Hand-written PPT helpers                         ✅ (minimal)
  generated/      Codegen output from C# data/*.json               ✅
  namespace.rs    Constants                                         ✅
  error.rs        Error type                                        ✅
  bin/openxml-codegen.rs  Schema/part/namespace generator          ✅
```

## Completed (MVP + codegen + parts + Flat OPC)

- [x] OPC package open/create/save
- [x] Content types + relationships round-trip
- [x] Untyped DOM with parse/serialize
- [x] `WordprocessingDocument` create/open/save + paragraph text
- [x] `SpreadsheetDocument` create/open + string grid I/O
- [x] `PresentationDocument` create/open + empty slide
- [x] Integration tests + examples
- [x] `openxml-codegen` binary reading C# `data/schemas`, `data/parts`, `data/namespaces.json`
- [x] Generated WordprocessingML / SpreadsheetML / PresentationML / DrawingML 2D main
- [x] Generated part metadata for all 100+ parts
- [x] Generated namespace prefix table
- [x] Simple types (`StringValue`, `OnOffValue`, integers, `HexBinaryValue`, …)
- [x] Package core properties (`docProps/core.xml`)
- [x] Typed attribute get/set helpers on `OpenXmlElement`
- [x] Word styles, settings, image parts
- [x] Word headers / footers / hyperlinks
- [x] Word comments + find/replace
- [x] Word numbering + theme + document clone
- [x] Word tables (`table_from_strings` / `table_to_strings`)
- [x] Word altChunk (HTML/text/RTF import)
- [x] Excel shared strings + multiple worksheets + column widths
- [x] Excel merge cells + minimal stylesheet + formula cells
- [x] PowerPoint multi-slide + text box helpers + slide size
- [x] Flat OPC (`to_flat_opc` / `from_flat_opc`)
- [x] Schema enum codegen (`HighlightColorValues`, …) implementing `OpenXmlSimpleType`
- [x] Markup Compatibility helpers (`AlternateContent` / Choice / Fallback)
- [x] Strict → Transitional namespace & relationship rewrite

## Next milestones

### M1 — Codegen depth
- [x] Schema/part/namespace JSON readers
- [x] Element factories + `ElementInfo` / `PartInfo` tables
- [x] Attribute + child metadata (with base-type inheritance)
- [x] Generate **all** 155 schema modules (`--schema all`)
- [x] Schema enum types with `as_str` / `from_str` / `OpenXmlSimpleType`
- [x] Typed attribute setter helpers (`bold_val`, `paragraph_with_rsid_*`, …)
- [x] Part child constraints (`is_allowed_child`, `allows_multiple`, `PartChildConstraint`)

### M2 — Simple types + attributes
- [x] `StringValue`, `OnOffValue`, `Int32Value`, `HexBinaryValue`, …
- [x] Attribute get/set by qname + simple-type helpers
- [x] Full `EnumValue`-style enums from schema (as native Rust enums)

### M3 — Full Word part graph
- [x] Styles, settings, headers/footers, images
- [x] Comments, numbering, theme
- [x] Find/replace text across runs
- [x] Tables
- [x] Embedded objects / altChunk

### M4 — Excel & PowerPoint depth
- [x] Shared strings, multiple worksheets
- [x] Multi-slide + text bodies
- [x] Excel column widths
- [x] Excel styles / merged cells / formulas
- [x] Presentation slide size
- [x] Slide masters/layouts (blank master + layout + slide link)

### M5 — Parity features
- [x] Flat OPC
- [x] Markup compatibility helpers (build + resolve AlternateContent)
- [x] Strict ↔ Transitional namespace rewrite
- [x] MC Ignorable processing (strip unsupported prefixes)
- [x] MC ProcessContent / PreserveElements / PreserveAttributes
- [x] Lightweight schema validation (Document/Body/Paragraph/Run rules)
- [x] Ordered particle matching (sequence/choice/group/all) for core Word types
- [x] Particle codegen from schema JSON (`particle_for_class`, 165 Word particles, …)
- [x] Word footnotes / endnotes parts
- [x] PowerPoint notes slides
- [x] Excel bar/column chart parts (embedded literals)
- [x] On-sheet chart anchors / drawings parts (`add_bar_chart_on_sheet`)
- [x] On-sheet image anchors (`add_image_on_sheet`, oneCell/absolute/twoCell)
- [x] MC FileFormatVersions matrix + version-targeted processing
- [x] Complete prefix→version table generated from namespaces.json
- [x] Worksheet cell comments (`add_sheet_comments` / `sheet_comments`)
- [x] Conditional formatting (cellIs + colorScale + dxfs)
- [x] Pivot tables (cache definition/records + pivotTableDefinition)

### M6 — Gap-analysis P0/P1 follow-ups
- [x] Extended file properties (`docProps/app.xml`) + Custom properties (`docProps/custom.xml`)
- [x] Word CustomXml parts (`add_custom_xml_part` / `custom_xml_parts`)
- [x] `CreateFromTemplate` + `ChangeDocumentType` (Word; Excel template clone)
- [x] Package structure validation (`validate_package` — main part + rel targets)
- [x] Excel line/pie chart parts + fill/numFmt stylesheet helpers
- [x] Encrypted Office detection (`IsEncryptedOfficeFile` / `Error::EncryptedPackage`)
- [x] Full simple-type set (Base64, DateTime, List, TrueFalse*, Int16/Byte/…)
- [x] Excel defined names (`set_defined_names` / `defined_names`)
- [x] Word font table + web settings parts
- [x] Excel table / autoFilter / data validation list
- [x] PowerPoint image on slide
- [x] Word SDT content controls (`sdt_block` / `sdt_run` / `collect_sdt_tags`)
- [x] Excel calculation chain part
- [x] Element structural equality (`elements_equal`)
- [x] Thumbnail part + `MaxCharactersInPart` DoS guard
- [x] Excel comments VML drawing companion
- [x] Word track changes (`ins`/`del` helpers + accept/reject)
- [x] PowerPoint table on slide
- [x] OpenSettings MarkupCompatibility process mode
- [x] Word document protection + glossary document part
- [x] PowerPoint notes master
- [x] Excel sheet/workbook protection
- [x] Word embedded package part
- [x] Word bookmarks (`bookmark_start`/`end`, `collect_bookmarks`)
- [x] Excel freeze panes
- [x] PowerPoint handout master
- [x] Word/Excel page setup + Word simple fields
- [x] Streaming XML reader (`OpenXmlStreamReader`)
- [x] Linq-style element query helpers (`element::linq`)
- [x] Lazy ZIP part load (`OpcPackage::open_lazy` / `open_bytes_lazy` / `load_part`)
- [x] Attribute simple-type validation
- [x] PPT CreateFromTemplate
- [x] Digital signature origin/sig part shells (no crypto)
- [x] Media data parts (audio/video) + PPT attach helpers
- [x] Excel row heights / hidden rows
- [x] Word internal (anchor) hyperlinks + mail-merge recipients part
- [x] Excel scatter chart + tab color + print area
- [x] Word page-number footer helper
- [x] Word document variables + TOC field
- [x] Excel sparklines (`x14`)
- [x] Semantic relationship-id validation subset
- [x] Excel area chart
- [x] Word document background + drop cap
- [x] Excel sheet dimension + shared formulas
- [x] PowerPoint sections (`p14:sectionLst`)
- [x] Word VML text watermark
- [x] Excel active tab (`bookViews`)
- [x] Excel sheet state + calcPr + dataBar/iconSet CF
- [x] PowerPoint hide slide
- [x] Excel row outline levels
- [x] Word even/odd headers + caption field + ruby
- [x] Word OMML math (fraction)
- [x] Excel sheet zoom
- [x] Word docDefaults + bibliography customXml
- [x] Excel external workbook link shell
- [x] PowerPoint theme part
- [x] Word page borders
- [x] Excel rich-text cells + chartsheet
- [x] Word VBA project shell + commentsExtended
- [x] Excel slicer shell + workbook theme
- [x] Word people part + customXml itemProps
- [x] Excel connections part
- [x] Word printer settings shell
- [x] PPT presentation/view properties
- [x] Excel queryTable + volatileDependencies shells
- [x] PPT comment authors
- [x] Word customUI + document tasks shells
- [x] Excel timeline shell
- [x] PPT slide comments
- [x] Excel named Title cell style
- [x] Excel print titles
- [x] Word paragraph styles + table style helper + web extension shell
- [x] Word track revisions + compatibility mode settings
- [x] Excel sheetFormatPr + doughnut chart
- [x] Excel pivot cache with real row data
- [x] Word updateFields on open
- [x] PPT table styles part
- [x] Word tabs/symbol/mirror margins
- [x] Excel show gridlines/headers
- [x] PPT clone slide + notes size
- [x] PPT slide transitions (fade/dissolve/custom)
- [x] Excel cell hyperlinks + sort state + whole-number DV
- [x] Word paragraph spacing/shading/highlight
- [x] Word page/column breaks + indent + page number start
- [x] Excel row/col page breaks
- [x] PPT simple appear animation
- [x] Word DATE/TIME/AUTHOR fields + run spacing/lang
- [x] Excel array formula + local defined names + show formulas
- [x] PPT slide header/footer flags
- [x] Word run/paragraph formatting (jc, borders, strike, underline, color, size)
- [x] Word SmartArt/diagram shell
- [x] PPT solid slide background
- [x] Excel XML maps shell
- [x] Excel chart styles/colors, dialogsheet, named sheet views, custom data
- [x] Word label info + embedded OLE object
- [x] PPT modern comments/authors
- [x] Excel single-cell table, rich values, feature bags, macrosheet, theme override
- [x] Word stylesWithEffects, vbaData, customization, QAT
- [x] PPT user tags, 3D model, slide sync
- [x] Excel threaded comments, persons, revisions, sort map, metadata, rich data extras
- [x] Word commentsIds, commentsExtensible, attached toolbars
- [x] Excel chart drawing, extended chart, intl macrosheet, web image, ActiveX control
- [x] Word legacy diagram text + embedded package part
- [x] PPT chart drawing shell
- [x] CustomProperty + Font parts (Word/Excel/PPT)
- [x] Word/PPT embedded chart parts
- [x] Diagram persist layout (drawing) part in diagram shell
- [x] Semantic validation: unique-attribute + expanded rel rules (Word/Excel/PPT)
- [x] Excel/PPT validate_relationships + delete_part APIs
- [x] Radar/bubble charts + ExtendedPart + PPT clone_document
- [x] Spreadsheet attribute range validation subset
- [x] Excel border stylesheet + set_cell_style
- [x] PPT auto-shape / text-box on slide + remove_slide
- [x] Excel rename_sheet / remove_sheet
- [x] Excel set_cell_value/number/styled + Word paragraph styles
- [x] Excel get_cell_value + Word append/remove paragraphs + PPT move_slide
- [x] Word append_table / body_tables_as_strings
- [x] Excel insert_rows / delete_row
- [x] Excel copy_sheet / clear_cell + PPT add_blank_slide
- [x] Excel clear_range + Word run/paragraph formatting helpers
- [x] Excel read_range/write_range + PPT replace_slide_text
- [x] Excel set_column_hidden + Word append_bullet_list
- [x] Excel find_cells / replace_in_sheet + PPT set_slide_text
- [x] Excel used_range + Word paragraph_count/word_count/insert_paragraph_at + PPT notes_text
- [x] Excel move_sheet / sheet_names + Word bookmarks()
- [x] Excel merge_range/unmerge/clear_sheet/cell_count + Word heading/hyperlink/clear_body
- [x] Excel insert/delete column + Word append_table_row + PPT all_slide_texts
- [x] Excel auto_filter get/clear + set_row_hidden + Word remove_table + PPT shape_count
- [x] Excel list_cell_hyperlinks + is_macro_enabled/part_count (all docs)
- [x] Excel remove_cell_hyperlink/shared_string_count + Word/PPT contains_text
- [x] Excel add/remove defined name + sheet_index + Word char count + PPT has_notes
- [x] ensure/has styles & theme helpers (Word/Excel/PPT)
- [x] list_headers/footers + media/chart counts
- [x] remove header/footer + list drawings + master/layout counts
- [x] has_charts/comments/footnotes/numbering/media flags
- [x] list_part_uris + relationship counts + has_shared_strings/calc_chain
- [x] Excel tables/protection flags + PPT hidden slides + Word SDT tags
- [x] clear sheet/workbook/document protection + external link list/count
- [x] track revisions / watermark / freeze panes / sections / transition flags
- [x] zoom/tab color/print area/sheet state getters + updateFields/compat mode
- [x] slide/notes size + background getters + sheet_dimension
- [x] doc vars/customXml/glossary flags + DV/calc chain + animation/hf flags
- [x] CF/sparkline/sort/DV clear flags + footnote/endnote/comment counts
- [x] Excel slicer/timeline/connections/queryTable/pivot counts + presence flags
- [x] OpenXmlMiscNode parity (comment / PI / CDATA via `OpenXmlMiscKind`)
- [x] Stream open/save: `open_stream` / `open_stream_with_settings` / `write_to` (Word/Excel/PPT)
- [x] `CompressionOption` on OPC package + `OpenSettings.compression`

- [x] Word people/mail-merge/web settings/printer settings presence flags
- [x] PPT handout master / user tags / slide sync presence flags
- [x] Excel clear_sparklines / clear_print_area/titles / clear_sheet_tab_color
- [x] Excel show_formulas / show_row_col_headers getters
- [x] Word list_comments / clear_comments / list_styles
- [x] PPT list_sections / clear_sections / clear_slide_background
- [x] Excel column_widths / row_heights / freeze_panes getters
- [x] Excel get_page_margins / get_page_setup / clear_sheet_comments
- [x] Word list_footnotes/endnotes / page_size / page_margins / count_text
- [x] PPT get_slide_transition
- [x] Excel table_infos / shared_strings_list / is_row_hidden / is_column_hidden
- [x] Excel row_outline_levels getter
- [x] Word header_texts / footer_texts / list_external_hyperlinks / document_protection_edit
- [x] PPT clear_notes
- [x] Excel remove_table / list_data_validations / clear_calc_chain / get_cell_style_index
- [x] Word clear_headers/footers / remove_external_hyperlink
- [x] PPT slide_title / slide_titles
- [x] Excel list_formulas / row_breaks / col_breaks getters
- [x] Word paragraph_style_ids / clear_numbering
- [x] PPT slides_with_transition
- [x] Excel list_conditional_formatting / clear_shared_strings
- [x] Word remove/clear custom_xml parts
- [x] PPT list_media / clear_notes_master / clear_handout_master
- [x] Excel clear_external_links / list_pivot_tables / pivot_table_infos
- [x] Word clear_glossary / list_alt_chunks
- [x] PPT list_charts
- [x] Excel sheet_format getter
- [x] Word remove_bookmark
- [x] PPT clear_user_defined_tags / clear_slide_sync_data
- [x] Excel get_calc_properties / clear_drawings
- [x] Word has/clear_thumbnail
- [x] PPT list_masters / list_layouts
- [x] Excel clear_slicers/timelines/connections/query_tables/volatile_deps/theme
- [x] Word clear_theme / clear_vba_project / clear_font_table
- [x] PPT clear_theme
- [x] Excel clear_styles / list_media
- [x] Word clear_styles / clear_settings / digital signature inventory+clear
- [x] PPT clear_media
- [x] Word clear_people / mail_merge / web_settings / printer_settings
- [x] Excel clear_charts / clear_pivot_tables
- [x] PPT has/clear presentation + view properties
- [x] Word clear_images / clear_footnotes / clear_endnotes
- [x] Excel clear_media
- [x] PPT clear_charts
- [x] Excel list_named_styles
- [x] Word clear_alt_chunks
- [x] PPT has/clear_table_styles
- [x] Word remove/clear document variables + content_control_count
- [x] Excel clear_cell_hyperlinks
- [x] Excel list_hidden_sheets / clear_merge_cells / list_array_formulas
- [x] Excel list_number_formats / list_shared_formulas
- [x] PPT slides_with_animation
- [x] Excel list_style_fonts
- [x] Word list_font_names
- [x] PPT notes_master_count / handout_master_count
- [x] Excel get_defined_name / list_sheet_states
- [x] PPT section_count
- [x] Excel sheet_count / table_columns
- [x] Word list_anchor_hyperlinks
- [x] PPT list_hidden_slides
- [x] Excel column_count / clear_row_breaks / clear_col_breaks
- [x] Word list_style_ids
- [x] PPT clear_slide_comments
- [x] Excel list_fills / sheet_protection_flags / workbook_protection_flags
- [x] Word clear_watermark
- [x] Excel has_auto_filter / border_count
- [x] Word has/clear_page_borders / page_border_color
- [x] PPT clear_slide_header_footer
- [x] Word clear_document_background
- [x] Excel dxf_count
- [x] PPT slides_with_notes
- [x] Word mirror_margins_enabled
- [x] Excel list_calc_chain
- [x] PPT slides_with_background
- [x] Word even_odd_headers_enabled
- [x] Excel table_names
- [x] PPT slides_with_comments (per-slide rel only)
- [x] Excel is_sheet_hidden
- [x] Word clear_even_odd_headers
- [x] PPT slides_with_header_footer
- [x] Excel list_hidden_rows / list_hidden_columns
- [x] Excel has_print_area / has_print_titles
- [x] Excel merge_cell_count / has_merge_cells
- [x] Word get_document_variable / has_bookmarks / list_bookmark_names / image_count
- [x] PPT transition_count / animation_count
- [x] Excel defined_name_count / has_defined_names / drawing_count / has_tables
- [x] Excel row/col break counts + has_* flags
- [x] Word header_count / footer_count / external_hyperlink_count / has_external_hyperlinks
- [x] PPT notes_count / total_shape_count
- [x] Excel formula_count / has_formulas / cell_hyperlink_count / has_cell_hyperlinks
- [x] Excel named_style/number_format/style_font/fill counts
- [x] Excel array_formula_count / shared_formula_count
- [x] Word style_count / font_count
- [x] PPT has_charts / slide_comments_count / background_count / header_footer_count
- [x] Excel slicer_count / timeline_count / connection_count
- [x] Excel has_page_margins / has_page_setup
- [x] Word people_count / mail_merge_recipient_count / printer_settings_count
- [x] PPT has_any_properties / has_any_master_extras / extra_master_count
- [x] Excel has/clear_active_tab / has/clear_zoom
- [x] Excel has/clear_sheet_dimension / has_sheet_format
- [x] Word clear_mirror_margins / has_page_size / has_page_margins
- [x] PPT has_slide_size / has_notes_size
- [x] Word/Excel/PPT has_package/extended/custom properties + custom_property_count
- [x] Word/Excel/PPT set_title/title / set_creator/creator convenience
- [x] CustomProperties len/is_empty/names/remove/clear
- [x] Word/Excel/PPT subject/keywords/description/category convenience
- [x] Word/Excel/PPT application/company convenience
- [x] Word/Excel/PPT custom property string get/set/remove/clear
- [x] Word/Excel/PPT last_modified_by / revision / language / version / content_status
- [x] Word/Excel/PPT manager / template / hyperlink_base convenience
- [x] Excel threaded comments / persons / chart styles / rich data / feature bag flags
- [x] Word custom UI / vbaData / stylesWithEffects / commentsIds/Ex/Extensible flags
- [x] PPT tag/sync counts / comment authors / model3d flags
- [x] Excel clear_threaded_comments / clear_persons / clear_chart_styles / clear_named_sheet_views / clear_rich_data / clear_feature_property_bag
- [x] Word clear_custom_ui / clear_vba_data / clear_styles_with_effects / clear_comments_ids/extensible/extended
- [x] PPT clear_comment_authors / clear_model_3d
- [x] Excel list_query_tables / query_table_infos / list_persons / list_threaded_comment_entries / list_chart_styles
- [x] PPT list_comment_authors / list_user_defined_tag_entries / list_slide_sync_parts
- [x] Word document_tasks / web_extensions / customization / QAT / label_info / attached_toolbars / diagrams / embeddings inventory+clear
- [x] Excel chartsheet/dialogsheet/macrosheet/xmlMaps/sortMap/revision/single-cell/ActiveX/toolbars inventory+clear
- [x] Excel cell metadata / chart drawings / theme override / custom data / supporting property bags inventory+clear
- [x] PPT modern comments/authors + chart drawings inventory+clear
- [x] Schematron extractable subset: 63 rel + 115 unique-attr + 236 numeric range + 184 string-length + 15 pattern + 37 enum + 25 ancestor-unique + 10 conditional + 3 guid + 6 attr-cmp + 8 fixed-bool + 23 cross-index + 53 cross-count + 17 fixed-val + 7 fixed-ne + 12 multi-ne + 9 both-present + 7 finite + 5 required-attr (948 of 948 source rules)
- [x] `validate_schematron` / `validate_schematron_subset` / `validate_schematron_constraints` + merged rule tables for Word/Excel/PPT
- [x] `scripts/generate_schematron_rules.py` regenerator (rules + constraints)
- [x] Excel/PPT thumbnail / digital-signature shell / customXml inventory APIs (parity with Word)
- [x] Excel/PPT embeddings + VBA project shell inventory APIs (parity with Word)
- [x] Excel/PPT customUI add/has/clear (parity with Word)
- [x] Excel/PPT printer settings + QAT; PPT attached toolbars (parity with Word)
- [x] Excel/PPT MIP label info add/has/clear (parity with Word)
- [x] Excel/PPT web extension shell add/has/clear (parity with Word)
- [x] Word/Excel/PPT font parts has/list/count/clear inventory
- [x] Word chart inventory (has/list/count/clear); Excel/PPT diagram inventory
- [x] Excel/PPT `open_with_settings` / `create_with_settings` (parity with Word)
- [x] Excel/PPT `create_simple` shortcuts (parity with Word)
- [x] Excel/PPT `validate`/`validate_full` convenience + `add_custom_xml_properties`
- [x] Word/Excel/PPT `from_bytes` + `remove_part` aliases
- [x] Excel/PPT images has/list/count/clear; Word `has_images`
- [x] Word/Excel/PPT theme_count / list_themes
- [x] styles_count (Word/Excel) + PPT has_styles/clear_styles aliases
- [x] Excel/PPT comments has/list/clear inventory (parity with Word)
- [x] Excel/PPT `add_embedded_object` OLE shell (parity with Word)
- [x] Excel/PPT `add_diagram_shell` SmartArt parts (parity with Word)
- [x] Excel/PPT `list_main_relationships` / `main_relationship_count` aliases
- [x] Excel `list_comments` / `comment_count` over classic comments parts
- [x] Excel/PPT `add_embedded_package_part` alias
- [x] Excel/PPT `add_legacy_diagram_text` shell
- [x] Excel/PPT `add_chart` convenience (bar chart / slide 0)
- [x] Excel/PPT `add_image` media-part helper (unanchored)
- [x] Excel `add_default_styles` / `list_style_ids` / `style_count`
- [x] Word media aliases; Excel has_media + hyperlink inventory (has/list/count/clear)
- [x] Word/PPT drawings has/list/count/clear inventory
- [x] PPT slide hyperlink list/count/clear inventory
- [x] Excel `clear_tables` / `list_defined_names` / `clear_defined_names`
- [x] Hyperlink has/list/clear naming parity across Word/Excel/PPT
- [x] Excel `has_workbook_protection` / `has_sheet_protection` aliases
- [x] Excel/PPT `create_from_template_as`; Word `has_document_protection`
- [x] Excel pivot cache has/list/count/clear + sheets_with_sparklines
- [x] Excel merge/hidden inventory; PPT master/layout aliases; Word list_document_variables
- [x] Excel page breaks has/list/clear inventory
- [x] PPT `has_notes_slides` / `clear_all_notes`
- [x] PPT `has_any_transition` / `clear_all_transitions` / animation bulk clear
- [x] PPT bulk clear backgrounds + header/footers
- [x] Word/Excel/PPT `validate_schematron_attributes` convenience
- [x] Excel `get_zoom` / `has_sheet_view` helpers
- [x] Excel tab color set/get/clear + dimension/autofilter aliases
- [x] Word settings zoom set/get/clear; PPT clear_slide_size / clear_notes_size
- [x] Word settings view / defaultTabStop / docGrid helpers
- [x] Excel workbook view inventory; Word `has_track_revisions`
- [x] Word settings autoHyphenation / embedTrueTypeFonts / savePreviewPicture / gutterAtTop
- [x] Word hideSpelling/GrammaticalErrors / printHiddenText / printFormsData; Excel gridlines_visible
- [x] Word displayBackgroundShape / pageBoundaries / autoCompress / printTwoOnOne / strictFirstAndLastChars
- [x] Word formsDesign / removePersonalInfo / shadeFormData / printPostScript / border surround flags
- [x] Word East-Asian/spacing settings (FE layout, kinsoku, suppress spacing, etc.)
- [x] Excel sheet view `rightToLeft` set/get helpers
- [x] Excel `showZeros` helpers; Word `characterSpacingControl`
- [x] Excel `showOutlineSymbols` + `sheet_view_type`
- [x] Excel workbookPr `date1904` / `backupFile` / `filterPrivacy`
- [x] Excel codeName/themeVersion/refreshAllConnections; PPT firstSlideNum/rtl; Word attachedTemplate
- [x] Excel workbookPr extended: dateCompatibility/showObjects/updateLinks/autoCompressPictures/…
- [x] PPT presentation attrs: serverZoom/compatMode/embedTrueTypeFonts/conformance/…
- [x] Word compat inventory: set/has/list compat flags + compatSetting + clear
- [x] Excel sheetView (tabSelected/showRuler/topLeftCell/zoom scales) + printOptions + pageSetup attrs + sheetPr + calcPr extended
- [x] PPT viewProps lastView/showComments/gridSpacing
- [x] Excel workbookView firstSheet/tabRatio/window/scroll/tabs/visibility/autoFilterDateGrouping
- [x] Word settings decimalSymbol/listSeparator/hyphenation/bookFold/themeFontLang/drawingGrid/…
- [x] PPT showPr (loop/narration/animation/timings/mode) + prnPr frame/hidden/scale
- [x] Excel sheetFormatPr baseColWidth/zeroHeight/thickTop/Bottom/outline levels + clear
- [x] Word proofState spelling/grammar
- [x] Excel sheet headerFooter odd/even/first + flags
- [x] PPT custom shows (add/list/remove/clear)
- [x] Word writeProtection recommended shell
- [x] Word sectPr orientation/columns/titlePg/vAlign/section type/bidi/header-footer distance
- [x] Excel activeCell/selection + sortState read/caseSensitive
- [x] Word line numbering / page number format / textDirection / gutter / paperSrc / rtlGutter
- [x] Excel protectedRanges / ignoredErrors / scenarios inventory
- [x] PPT photoAlbum / kinsoku / slide headerFooter getters
- [x] Word revisionView / documentType / captions / mathPr / clrSchemeMapping
- [x] Excel customSheetViews inventory
- [x] Excel oleObjects / controls / webPublishItems shells
- [x] PPT modifyVerifier shell
- [x] Word forceUpgrade / XML validation-related settings flags
- [x] Word rsids / attachedSchema / saveThroughXslt
- [x] PPT embeddedFontLst inventory
- [x] Excel sheetPr outlinePr/pageSetUpPr/filterMode/transitionEvaluation/Entry
- [x] Word doNotIncludeSubdocsInStats
- [x] Excel phoneticPr / customWorkbookViews / dataConsolidate
- [x] Word activeWritingStyle
- [x] PPT customerData list shell
- [x] Excel autoFilter filterColumn (values/top10) + dataConsolidate dataRefs
- [x] Word mailMerge settings shell
- [x] PPT slide cSld/@name
- [x] Excel autoFilter customFilters/dynamicFilter + column kind
- [x] Word mailMerge ODSO/activeRecord
- [x] PPT notes cSld/@name
- [x] Excel hyperlink tooltip + location hyperlinks
- [x] Word mailMerge destination/subject/addressFieldName/mailAsAttachment
- [x] PPT notes header/footer flags
- [x] Excel dataValidation messages/errorStyle/date/textLength + autoFilter blank
- [x] Word mailMerge doNotSuppressBlankLines/linkToQuery/checkErrors/connectString
- [x] Excel col bestFit/style/outlineLevel/collapsed
- [x] PPT showPr sldRg/sldAll/custShow
- [x] Excel dataValidation decimal/custom + showDropDown
- [x] Word stylePaneFormatFilter flags
- [x] PPT showPr penClr
- [x] Excel row thickTop/thickBot/collapsed/style
- [x] Word saveXmlDataOnly/useXSLTWhenSaving/alwaysMergeEmptyNamespace
- [x] Excel dataValidation time
- [x] Word themeFontLang eastAsia/bidi + autoCaption
- [x] PPT notesMaster/handoutMaster header-footer flags
- [x] Excel fileVersion/fileSharing/oleSize
- [x] Word webSettings flags/encoding/pixelsPerInch
- [x] PPT sldSz/@type
- [x] Excel functionGroups inventory
- [x] Word footnotePr/endnotePr in settings
- [x] PPT defaultTextStyle shell
- [x] Excel sheetProtection extended permission flags
- [x] Word webSettings doNotOrganize/doNotUseLongFileNames/doNotSaveAsSingleFile/targetScreenSz
- [x] PPT slide master header/footer flags
- [x] Word docGrid type/charSpace + page border display/offsetFrom/zOrder
- [x] PPT slide layout header/footer flags
- [x] package_part_count alias on Word + Excel/PPT
- [x] Word/Excel/PPT set_created/created / set_modified/modified
- [x] Word/Excel/PPT part_content_type / list_package_relationships
- [x] Excel package_relationship_count / workbook_relationship_count
- [x] Word/Excel/PPT list_content_type_overrides / main|workbook|presentation rel lists
- [x] Word/Excel/PPT application_version / doc_security / pages/words/characters / app_slides/notes
- [x] typed custom props i4/bool + list names
- [x] SharedDoc/LinksUpToDate/HyperlinksChanged/ScaleCrop/TotalTime + Word lines/paragraphs stats
- [x] PPT hidden slides/mmClips + list_slide_relationships
- [x] example create_presentation
- [x] Excel list_sheet_relationships / sheet_relationship_count
- [x] Word/Excel/PPT has_part / get_part_bytes / set_part_bytes / part_size / package_relationship_target
- [x] Excel/PPT Flat OPC to_flat_opc_string / from_flat_opc
- [x] Word/Excel/PPT settings/auto_save/rewrite_strict/is_encrypted_office_file
- [x] Excel/PPT change_document_type + close
- [x] PPT PresentationDocumentType::from_content_type
- [x] Excel freeze_panes_ex / freeze_pane_details / colorId / zoomScaleNormal / workbookViewId
- [x] Word doNotUseMarginsForDrawingGridOrigin / showEnvelope / autoFormatOverride / uiCompat97To2003
- [x] PPT normalViewPr (showOutlineIcons/preferSingleView/snapVertSplitter/bar states/restoredLeft/Top) + clear_last_view
- [x] Excel pageSetup attrs (paperSize/orientation/DPI/errors/cellComments/usePrinterDefaults) + clear_page_setup
- [x] Word noLineBreaksAfter/Before (lang/val kinsoku char lists) + readModeInkLockDown
- [x] PPT slideViewPr snapToGrid/snapToObjects/showGuides + sorter showFormatting
- [x] Excel customHeight / print gridLinesSet / custom sheet view attr update+remove
- [x] Excel custom workbook view attr update/remove; Word smartTagType CRUD
- [x] PPT notesViewPr snapToGrid/snapToObjects/showGuides
- [x] Excel page margins set/clear/attr + protected range attrs + scenario attrs/inputs + cellWatches
- [x] Word schemaLibrary CRUD + web encoding/screen/ppi clear helpers
- [x] PPT slide view guide list add/list/clear
- [x] Excel remove_ignored_error + dataConsolidate attrs/refs clear
- [x] Word font table entry add/get/remove/ensure
- [x] Excel cell hyperlink attrs/details update
- [x] Word numbering abstractNum/num inventory + level set/get
- [x] PPT outlineViewPr scale
- [x] Excel definedName attrs/details + connection list/update/remove
- [x] Word style basedOn/next/link get/set
- [x] Excel sparklines list/attrs + CF rule attrs + external link targets
- [x] Word style flags (qFormat/semiHidden/locked/uiPriority) + formProt
- [x] PPT notesTextViewPr/sorterViewPr scale
- [x] Excel pivot rename/attrs/location + chart title list/set
- [x] Word glossary docPart list/append/remove + comment attrs/remove/by_id
- [x] PPT transition details + attr update
- [x] Excel shared formula clear group/all
- [x] Word content control tag set/remove
- [x] PPT animation shape id list
- [x] Excel clear_array_formulas
- [x] Word rename_bookmark
- [x] PPT set_notes_text (replace notes)
- [x] Excel sheet comment text update/remove
- [x] Word append_simple_field/toc_field + list_simple_fields
- [x] PPT set_slide_text_at / slide_text_node_count
- [x] Excel unmerge_range / is_merged_range
- [x] Word/PPT theme name list/set
- [x] Word remove_style / has_style
- [x] PPT list_slide_transitions
- [x] Excel named style rename/remove
- [x] Word set_header_text / set_footer_text
- [x] PPT list_notes_texts















## API mapping cheat sheet

| C# | Rust |
|----|------|
| `WordprocessingDocument.Create(path, type)` | `WordprocessingDocument::create(path, type)` |
| `doc.AddMainDocumentPart()` | `doc.add_main_document_part()` |
| `main.Document = new Document(...)` | `main.set_document(document(...))` |
| `doc.MainDocumentPart.Document.Body.Elements<Paragraph>()` | `doc.paragraph_texts()` / DOM walk |
| `System.IO.Packaging.Package` | `opc::OpcPackage` |
| `OpenXmlElement` | `element::OpenXmlElement` |

- [x] Word `remove_external_hyperlink` unwraps body `w:hyperlink` elements
- [x] Excel `clear_shared_strings` rewrites `t="s"` cells to `inlineStr`
- [x] Excel `materialize_shared_strings` (inline without dropping SST)
- [x] Excel `remove_chart` strips drawing anchors that reference the chart (relative target resolve)
- [x] Excel `remove_table_column` shrinks table `ref` when removing last column
- [x] Word SDT `content_control_infos` / kind detection / `clear_content_controls`
- [x] Word revision marker inventory (`list`/`has`/`count` + insertion/deletion counts)
- [x] Inventory `has_*` companions (autoFilter/cellWatch/hidden cols/sort/scenarios/web publish/threaded comments; bibliography/autoCaptions/compat/glossary/anchor HL/fonts/numbering; PPT shapes/transitions/names/tags/sync/comments)

- [x] Word `remove_chart` / `remove_chart_at`
- [x] Excel `unhide_all_columns` / `unhide_all_rows` + sheets_with_hidden_*
- [x] PPT `remove_animation_for_shape` / `list_slide_animation_effects`
- [x] Word `clear_external_hyperlinks` (rel + body unwrap)

- [x] Excel `clear_all_outlines` / `clear_all_row_outlines`
- [x] Word `remove_drawing` by URI

- [x] Excel `remove_empty_sheets`

- [x] Word/Excel `remove_embedding` by URI

- [x] Word complex fields (`complex_field` / list/append/has/count)

- [x] Word accept/reject revisions in headers/footers (+ everywhere)

- [x] Word `clear_complex_fields` (keep result text)

- [x] PPT `remove_empty_slides`

- [x] Excel `clear_all_conditional_formatting` / `clear_all_data_validations`

- [x] Excel `clear_all_merged_cells` / `clear_all_cell_hyperlinks`

- [x] Word `clear_all_fields` (simple + complex)

- [x] Excel `clear_all_auto_filters` / `clear_all_sparklines` / `clear_all_freeze_panes`

- [x] Excel `rebuild_calc_chain` from formula cells

- [x] Excel `clear_all_sheet_comments`

- [x] Word `list_used_style_ids` / `list_unused_style_ids` / `remove_unused_styles`

- [x] Word `clear_bookmarks`

- [x] Excel `clear_all_tables`

- [x] Word/Excel/PPT `remove_media` by URI

- [x] Excel `clear_formulas` / `clear_all_formulas` (keep cached values)

- [x] PPT `unhide_all_slides` / `hide_slides`

- [x] Word `list_custom_xml_part_uris`

- [x] Excel `clear_column_widths` / `clear_all_column_widths` / `clear_row_heights`

- [x] Word `clear_all_notes` (footnotes + endnotes)

- [x] Excel `clear_all_tab_colors` / `clear_all_sheet_protection` / `clear_all_sheet_code_names`

- [x] PPT `has_rtl`

- [x] PPT `clear_first_slide_num` / `has_first_slide_num`

- [x] Excel `clear_all_page_setup` / `clear_all_print_options`

- [x] Excel `clear_all_zoom` / `clear_all_sort_state` / `sheets_with_sort_state`

- [x] Excel `unhide_all_sheets` / `clear_all_sheet_format` / `sheets_with_zoom`

- [x] Word settings clear companions (track revisions, do_not_*, forms, personal info, …)

- [x] PPT `has_bookmark_id_seed` / `clear_show_special_pls_on_title_sld`

- [x] Word exhaustive `clear_*` companions for bool settings (has_/set_ pairs)

- [x] Excel sheet-view/print/page `clear_*` companions for bool setters

- [x] PPT `clear_rtl`

- [x] PPT exhaustive `clear_*` companions for presentation bool settings

- [x] Word `add_person` / `remove_mail_merge_odso_field_map` / `remove_complex_fields_matching`

- [x] Excel `clear_all_shared_formulas`

- [x] PPT push/wipe/split/cover/wheel/random transition helpers

- [x] Excel `clear_all_array_formulas` / `sheets_with_array_formulas`

- [x] PPT blinds/checker/circle/diamond/plus/newsflash/strips/wedge/zoom transitions

- [x] Excel `has_data_validation(sqref)`

- [x] Core props has/clear (title/creator/subject/description/keywords/category/…)

- [x] Hyperlink base / language / calc mode has-clear companions

- [x] Extended props has/clear (application/company/manager/template/…)

- [x] Core revision/content_status/created/modified has-clear

- [x] Excel `clear_date1904` / `clear_code_name` / `clear_filter_privacy`

- [x] Word `clear_compat_flag` / `clear_web_settings_flag`

- [x] Word auto-captions remove/clear; page size/margins set/clear; content-control alias set/clear
- [x] Excel sort-condition clear (sheet + all); slicer-cache remove/clear; CF rule remove; style-font remove
- [x] Excel scenario input CRUD; cell-watch remove/all; OLE/control remove by shapeId
- [x] PPT notes/handout master header-footer clear; outlineViewPr clear
- [x] PPT animation duration get/set/clear; animation filter update; shape solid-fill/line get/set/clear
- [x] Word footnote/endnote text getters + has_footnote/has_endnote
- [x] PPT shape transform get/set; Excel sparkline append + remove_sparkline
- [x] PPT shape rotation + preset geom; Excel chart title has/clear
- [x] Excel local defined name clear/all; external link target clear; PPT clear_shape_preset_geom
- [x] Word caption definition remove/has; Excel clear_workbook_views
- [x] Word math_font clear; Excel named style clear/all; PPT shape flipH/flipV
- [x] PPT clear_all_notes_text; Excel list_borders inventory
- [x] Excel list_dxfs/has_dxfs; Word clear_paragraph_styles; PPT has/clear_use_timings (attr remove)
- [x] Excel clear_table_totals_row/comment; PPT hide_slide/unhide_slide
- [x] PPT list/clear_all shape text; Excel chart legend has/set/clear
- [x] Excel chart axis title list/set/clear; PPT shape font size get/set/clear
- [x] Word extended stats has/clear (pages/words/characters/lines/paragraphs); Excel clear_print_area_for_sheet; PPT shape bold
- [x] Excel unhide_very_hidden_sheets; PPT shape italic; Word set/clear_all_runs_bold
- [x] Word set/clear_all_runs_italic/underline; PPT shape font color; Excel has_data_bars/icon_sets/color_scales
- [x] Word set/clear_all_runs_color; Excel remove_cf_rules_by_type; PPT shape underline
- [x] Word run highlight/strike/caps/vanish/size/font batch; Excel zoom_scale has/clear + workbookViewId clear; PPT font name/strike + clear_all_shape_fill

## Completion status (practical port)

The MVP + depth milestones above are **complete**. The packaging surface is large and tested:

| Surface | Approx. `pub fn` | Integration tests |
|---------|------------------|-------------------|
| SpreadsheetDocument | ~1581 | shared suite |
| WordprocessingDocument | ~1419 | shared suite |
| PresentationDocument | ~960 | shared suite |

**Long-term progress (this pass):**

- [x] Schematron extractable subset expanded to ****948/948**** (added ancestor-unique, conditional attrs, non-zero GUID, attr-compare, fixed-bool, cross-part Index-of / count)
- [x] `validate_schematron_cross_part` for package-aware Index-of / count bounds
- [x] Digital signature structure + **Reference digest verify** (`validate_digital_signatures` + `validate_signature_digests` / `build_signature_xml`; RSA SignatureValue still empty shell)
- [x] Lazy ZIP open (`open_lazy` / `open_bytes_lazy` / `load_part` / `materialize_all`)
- [x] Linq-style DOM queries (`ElementQuery` / `descendants_of` / …)
- [x] Typed element views (hand) + **generated** `generated::typed_elements` (**1378** wrappers from Word/Excel/PPT schema JSON)
- [x] Typed element views: Document/Body/Paragraph/Run/Text/Table/Cell/Worksheet/Slide/Style/Hyperlink/Comment/Header/Footer/Notes
- [x] Lightweight Features bag (`FeatureCollection` + `ParagraphIdGenerator` on `OpenXmlPackage`)
- [x] VBA project inventory + CFB parse (`vba_project_bytes` / `list_vba_parts` / `inspect_vba_project` / `opc::CfbFile`; no macro execution)
- [x] Part delete parity: `remove_part` strips inbound rels; `delete_part_and_orphans` / `delete_part_by_id` / `delete_parts_of_content_type` (C# DeletePart orphan cascade)
- [x] `AddExternalRelationship` / `ExternalRelationships` on OPC + Word/Excel/PPT documents
- [x] `PackageEvents` / `PackageEventType` feature hub (C# `IPackageEventsFeature` shell); raised on package save/close
- [x] `PartUriHelper` + `RelatedPart` / `related_parts` / `parts_of_relationship_type` (C# PartUriHelper + GetPartsOfType shell)
- [x] `TypedPart` runtime handle over generated `PartInfo` (add/find/save/children; constraint-checked)
- [x] `ReferenceRelationship` / `HyperlinkRelationship` / audio·video·media reference types
- [x] Element + package `Annotations` (C# AnnotationsFeature); `ChangeIdOfPart` / `GetPartById` / `GetIdOfPart`
- [x] `DataPart` / `DataPartReferenceRelationship` / `IdPartPair`; `CreateMediaDataPart`, `DeleteUnusedDataParts`, `DeleteReferenceRelationship`
- [x] `PartExtensionProvider`; `OpenXmlPart` UnloadRootElement / IsRootElementLoaded / GetParentParts
- [x] `GetAllParts` BFS; DOM `OuterXml`/`InnerXml`, InsertBefore/After, RemoveChild, First/LastChild
- [x] `StrictRelationshipFound` / strict namespace detect; `DeleteParts` batch; MC `ProcessAllParts`
- [x] DOM path/sibling helpers: Next/PreviousSibling-at, Remove/Replace path, GetOrAddFirstChild, find_path
- [x] `ExtendedPart` type; `OpenXmlPart::reload`/`save_root`; `CanSave`/`FileOpenAccess`/`Close` + unused data-part cleanup
- [x] Cross-package `copy_part_from` / `import_part` (C# `AddPart` when part is foreign)
- [x] Excel/PPT packaging parity: delete_parts, ChangeIdOfPart, IdPartPair, media data parts, CreateRelationshipToPart, ExtendedPart
- [x] Word `add_typed_child_part` (AddNewPart via PartInfo)
- [x] DOM: CloneNode(shallow), Get/Set/Clear attributes, InsertAt, RemoveAllChildren by name, LookupNamespace/Prefix, Elements/GetFirstChild, ns decl add/remove
- [x] Package `delete_data_part` (C# DeletePart(DataPart)) on OpenXmlPackage + Word/Excel/PPT
- [x] EqualityOptions/hash/ElementComparer expanded; OpenXmlPartWriter SAX writer shell
- [x] OpenXmlPartReader cursor (Read/GetText/LoadCurrentElement/ElementState)
- [x] OpenXmlElementContext / LoadMode / element mutation events feature
- [x] FileFormatVersions::at_least / and_earlier; package compare_packages / PackageDiff
- [x] Digsig packaging helpers: ensure origin, add/list/clear signature parts (no full crypto)
- [x] OpenXmlDomReader DOM cursor (Read/Skip over element trees)
- [x] PartConstraintFeature / PartConstraintRule (IPartConstraintFeature shell)
- [x] PackageValidator-style `validate_package_constraints` (PartIsNotAllowed / RequiredPart / OnlyOne / InvalidContentType / DataPartRef) wired into `validate_package` + Word/Excel/PPT
- [x] `XmlPath` + path-from-indices (owned-DOM); `OpenXmlUnknownElement` marker; `OpenXmlValidator` facade (max errors / file format / package+element)
- [x] `PartRootEvents` (IPartRootEventsFeature) + raise on load/reload/save/unload
- [x] Fluent package builders (`WordprocessingDocumentBuilder` / spreadsheet / presentation + middleware/properties shell; C# experimental `IPackageBuilder` subset)
- [x] `PartEvents` (IPartEventsFeature) distinct from PackageEvents/PartRootEvents; raise on OpenXmlPackage set_part/delete_part
- [x] `delete_parts_recursively_of_relationship_type` (C# DeletePartsRecursivelyOfType stand-in by rel URI) on OpcPackage + OpenXmlPackage + Word/Excel/PPT
- [x] CloneableExtensions subset: `clone_to_path` / `clone_to_bytes` / `clone_to_writer` on Word/Excel/PPT (plus existing `clone_document`)
- [x] `validate_alternate_content` (C# AlternateContentValidator structure rules) + wired into OpenXmlValidator::validate_element
- [x] `validate_mc_attributes` (C# CompatibilityRuleAttributesValidator: Ignorable/Preserve*/ProcessContent/MustUnderstand)
- [x] `ValidationError::id` / `error_type` (C# ValidationErrorInfo.Id / ErrorType subset)
- [x] `OpenXmlValidator::on_validation_error` (ValidationErrorEventArgs shell) + `OpenXmlElement::has_attributes`
- [x] `OpenXmlElement::write_to` (C# `WriteTo`)
- [x] `XmlLineInfo` + reader `get_line_info` (Empty shell)
- [x] `OpenXmlElement::copy_attributes_from` / `copy_children_from` (C# CloneImp helpers)
- [x] `FeatureCollection::{get_required,get_or_add}` + `AddExtendedAttribute` shell
- [x] `MarkupCompatibilityAttributes` + `OpenXmlQualifiedName` shells
- [x] `OpenXmlDomReader::{read_first_child,read_next_sibling,load_current_element}`
- [x] `OpenXmlPartReader::{read_first_child,read_next_sibling,has_attributes}`
- [x] `OpenXmlPartWriter` WriteStartElement overloads + WriteStartDocument(standalone) + from Part/Dom reader
- [x] `ValidationSettings` + `OpenXmlValidator::{settings,with_settings,set_file_format}`
- [x] `OpenXmlElement::{remove_attribute_ns,get/set_open_xml_attribute,mc_attributes}`
- [x] `OpenXmlPartWriter::{write_comment,write_cdata,write_processing_instruction,write_chars,write_char_entity,write_entity_ref}`
- [x] `OpenXmlNamespace` URI value type + crate re-export
- [x] `TypedPart::is_in_version` shell
- [x] `OpenXmlPartWriter` deferred start + WriteAttribute/WriteAttributeString/WriteNamespaceDeclaration
- [x] Part/Dom reader `namespace_declarations` / `get_attribute` / `close`
- [x] `OpenXmlAttribute::{xml_qualified_name,matches}`
- [x] `ValidationError::{description,xml_path,with_id}` + `OpenXmlAttribute` Display + reader `attribute_count`
- [x] `FileFormatVersions::{any,ensure_supported,office_year}` + `OpenXmlElement::child_elements`
- [x] `OpenXmlElementContext` XmlnsUri/XmlnsPrefix/LazySteps + `OpenXmlSimpleType::{has_value,is_in_version}`
- [x] `OpenXmlElement::{is_leaf_element,is_leaf_text_element,is_composite_element,xml_space,set_xml_space,preserves_space}`
- [x] `ValidationCache` + OpenXmlValidator cache; Reader/Writer `Create` factories
- [x] `OpenXmlPartWriter::{write_empty_element,write_empty_from_element,write_full_end_element}`
- [x] Stream/Part reader real `XmlLineInfo` line tracking (byte consume counter)
- [x] Part reader `read_misc_nodes`/`encoding`/`standalone_xml`/`has_value` shells
- [x] `FileOpenAccess` alias + `PackageMode::{can_read,can_write}`
- [x] `OpenXmlPartReaderOptions` (IgnoreWhitespace / MaxCharactersInPart / ReadMiscellaneousNodes)
- [x] `CompatibilityLevel` on OpenSettings
- [x] `ValidationError` Node/RelatedNode/RelatedPart path shells
- [x] `OpenXmlAttribute::{from_qualified_name,from_parts,namespace_uri_str,prefix_str}`
- [x] Reader `get_attribute_at` / `get_attribute_ns` + `element_type_name` shell
- [x] Document-order `order_at_paths` / `is_before_at` / `is_after_at`
- [x] `PackageCapabilities` + OpenXmlPackage::package_capabilities
- [x] `ValidationContext` shell (settings/cache/errors/expected children)
- [x] `OpenXmlPartWriterSettings` + `with_settings` / `create_with_encoding` / encoding+standalone declaration
- [x] Public `McContext` push/pop shell (`AttributeAction` / `ElementAction` / `McQualifiedName`)
- [x] `Error::{InvalidMcContent,NamespaceNotUnderstand}`; McContext hard-fail parse paths
- [x] Stream `XmlEvent::{Comment,ProcessingInstruction,CData}`; PartReader `is_misc_node` + ReadMiscellaneousNodes
- [x] `OpenXmlElement::{create_from_text,create_from_cdata,create_from_significant_whitespace}`
- [x] `ApplicationType` / `DisposableFeature` / `MainPartFeature` / `DocumentTypeFeature`; wired on Word/Excel/PPT packages
- [x] `OpenXmlContent` + `OpenXmlElement::with` (C# functional `With` extensions)
- [x] `SchemaTrackingFeature` / `StrictNamespaceFeature` shells on package features
- [x] `OpenXmlPart::{standalone_declaration,save_to_stream,save_to_part}` (C# part-root Save/Stream)
- [x] Wire public `McContext` into `process_markup_compatibility` (+ `process_markup_compatibility_with_context`)
- [x] Route document/part `set_part` writes through `OpenXmlPackage::set_part` so `PartEvents` fire
- [x] Capture XML `standalone` on part load (`parse_xml_standalone`)
- [x] `RelationshipFilterFeature` / `PackageRelationshipBuilder` / `PackageFactoryFeature` / `ProgrammaticIdentifierFeature`
- [x] `OpenXmlPackage::{add_package_relationship,add_part_relationship}` apply relationship filters
- [x] `ContentTypeFeature` / `LockFeature` / `PartsFeature` (IContentTypeFeature / ILockFeature / IPartsFeature shells)
- [x] `PartFactoryFeature` / `KnownDataPartFeature` shells + package accessors
- [x] Route Word/Excel/PPT `add_*_relationship` through filtered `OpenXmlPackage` APIs
- [x] Stream/Part reader capture XML declaration encoding + standalone; DomReader declaration shells
- [x] `PackageStreamFeature` / `PackagePartFeature` / `PackageInitializerFeature` shells
- [x] `PartUriFeature` shell; seed Parts/PartUri on `from_opc`; `open_bytes` records package stream
- [x] `DataPartsFeature` / `PartRelationshipsFeature` / `ReferenceRelationshipsFeature` / `TypedPartFactoryFeature` shells
- [x] Package accessors; track part/ref relationships on add_*; seed DataParts on `from_opc`; media create/delete updates bag
- [x] Word/Excel/PPT `add_external_relationship` routes through filtered package API
- [x] `TargetFeature` / `RootElementFeature` / `SaveFeature` / `PackageFeature` shells + package accessors
- [x] `OpenXmlPackage::{save,save_as}` run `ISaveFeature` hooks before package events
- [x] `OpenXmlNamespaceResolverFeature` / `RandomNumberGeneratorFeature` / `ContainerDisposableFeature` / `ElementEventsFeature` shells + package accessors
- [x] Close path runs container disposable hooks
- [x] `ParagraphIdCollectionFeature` / unique `ParagraphIdGenerator::create_unique_paragraph_id` / `SharedFeatureRegistry`
- [x] Package accessors + `sync_paragraph_id_generator_from_collection`
- [x] `OpenXmlPackage::{add_hyperlink_relationship,add_data_part_reference_relationship}` filtered + feature tracking
- [x] Word/Excel/PPT hyperlink + data-part reference adds route through package APIs
- [x] `OpenXmlPackage::delete_reference_relationship` updates part/ref feature bags; documents route through it
- [x] Word/Excel/PPT document builders run `PackageInitializerFeature` after middleware
- [x] Transitional → Strict package/element rewrite (`rewrite_package_to_strict` / `rewrite_transitional_to_strict`)
- [x] `from_opc` seeds `PartRelationshipsFeature` / `ReferenceRelationshipsFeature` from existing package + part relationships
- [x] `OpenXmlPackage::{id_part_pairs,get_reference_relationship,get_part_by_id,get_id_of_part,data_part_reference_relationships,hyperlink_relationships}`
- [x] `OpenXmlPackage::{create_relationship_to_part,change_id_of_part}` apply filters + keep part/ref feature bags in sync
- [x] Word/Excel/PPT route IdPartPair / Get* / CreateRelationshipToPart / ChangeIdOfPart / media / data-part-ref / hyperlink queries through package APIs
- [x] `OpenXmlPackage::{delete_part_and_orphans,delete_part_by_id,delete_parts_of_content_type,delete_parts_by_ids}` event/feature-aware cascades
- [x] Word/Excel/PPT route orphan/content-type delete paths through package APIs
- [x] `OpenXmlPackage::{add_external_relationship_with_id,set_external_relationship_target}` keep ref feature bags
- [x] Word `set_hyperlink_target` / `remove_hyperlink_by_id` / attachedTemplate set/clear route through package APIs
- [x] `FeatureEventArgs` / `FeatureEventHub` (C# `FeatureEventArgs` / `IFeatureEvent` / `IRaiseFeatureEvent` shell)
- [x] Media data-part create consults package `PartExtensionProvider` when extension is omitted
- [x] `OpenXmlPackage::delete_reference_relationships` bulk helper; Word/Excel/PPT bulk rel id removes route through it
- [x] Document packages route `remove_part` → `delete_part` and most single/bulk relationship removes through feature-aware package APIs
- [x] `DefaultFeatures` / `ElementMetadata` / `ElementMetadataFactoryFeature` / `OpenXmlSchemaType` shells (C# DefaultFeatures.Shared + IElementMetadataFactoryFeature)
- [x] `FilePackageFeature` path metadata; `from_opc` seeds PackageFeature/FilePackageFeature/PackageStreamFeature + DefaultFeatures
- [x] `OpenXmlPackage::{replace_part_relationships,set_package_properties,set_extended_properties,set_custom_properties}` feature-aware
- [x] Word/Excel/PPT property setters route through package APIs; PPT layout/master template rels use `replace_part_relationships`

- [x] `OpenXmlPackage::{set_content_type_default,clear_content_type_override,add_media_part}` feature-aware media/content-type helpers
- [x] Word `parts.rs` set_part paths + typed_part delete + PPT/Excel content-type defaults + PPT audio/video media attach route through package APIs

- [x] Document rewrite_strict/transitional route through `OpenXmlPackage` helpers
- [x] `OpenXmlPackage::{to_flat_opc,to_flat_opc_string,from_flat_opc}` + feature seed on open
- [x] `AnnotationsFeature::{len,is_empty,clear}` + `OpenXmlPackage::annotations`

- [x] `PartUriFeature::{create_part_uri,ensure_unique_part_uri}` + `OpenXmlPackage::create_part_uri`
- [x] `OpenXmlPackage` compression_option / package_properties getters / has_package_properties

- [x] `OpenXmlPackage::{external_relationships,clone_package,clone_package_to_path}` (C# ExternalRelationships / Clone shells)

- [x] `MediaDataPartType` table + `OpenXmlPackage::create_media_data_part_typed[_with_data]`
- [x] Package relationship list wrappers + auto_save/MC/compatibility settings accessors

- [x] `OpenXmlPackageException` shell + known ExceptionMessages helpers
- [x] `OpenXmlPackageValidationResult` shell (C# packaging validation event)

- [x] `PartTypeInfo` + `OpenXmlPackage::create_part_from_type_info`

- [x] `try_get_part_by_id` / `is_child_part` / `get_parts_of_content_type`
- [x] `OpenXmlPackage::add_extended_part` (C# `AddExtendedPart`, feature-aware)
- [x] `MalformedUriHandlingFeature` / `RewrittenUri` / `enable_uri_handling` shells
- [x] `PackageStreamFeature::enable_writeable_stream` + package wrapper
- [x] Expanded `OpenXmlPackageException` ExceptionMessages helpers
- [x] `SupportedRelationship<T>` marker trait; `PackageFeature` capability helpers

- [x] `clone_package_with_settings` / `root_part_uri` / `is_encrypted_office_*` wrappers
- [x] `delete_external_relationship` / `get_parts_of_relationship_type` / `can_save_capability`
- [x] `OpenXmlPackageValidationResult` typed factory helpers

- [x] `OpenXmlPart` container helpers (Parts/GetPartById/AddExtendedPart/…)
- [x] `TargetFeature::{with_extension,with_name}` (UpdatedExtensionTargetFeature shell)

- [x] `PartAnnotationsFeature` + part/package annotation APIs; `FixedContentTypePart` marker
- [x] `ContentTypeFeature` is_constant accessors on package

- [x] `OpenXmlPart::{change_id_of_part,delete_part_by_id,delete_*_relationship}`
- [x] `ProgrammaticIdentifierFeature` program_id/set/reset + package wrappers

- [x] `OpenXmlPart` add external/hyperlink/data-part-ref relationships
- [x] `OpenXmlPackage::dispose` / `open_settings` accessors

- [x] `MediaDataPart` alias + DataPart target defaults / is_media_data_part
- [x] `OpenXmlPackage` data-part stream/feed/reference-query helpers

- [x] `ExternalRelationship` type + ReferenceRelationship accessors
- [x] Audio/Video/Media reference relationship constructors + package typed lists

- [x] `PackageRelationshipBuilder` with_id/target helpers; `FeatureCollectionDebugView`
- [x] `enable_uri_handling` registers malformed-external relationship filter

- [x] `PartUriHelper::reserve_uri` / package `reserve_part_uri` (C# `ReserveUri`)

- [x] `ApplicationTypeFeature` (C# `IApplicationTypeFeature`) + package `application_type_feature`
- [x] `OpenXmlPackage::{get,has}_{external,hyperlink}_relationship` + OpenXmlPart mirrors
- [x] `IdPartPair` / `RelatedPart` accessors; `RelatedPart::to_id_part_pair`
- [x] `OpenXmlPackageValidationResult` data_part_reference_id + message_id helpers
- [x] `OpenXmlPart::{delete_parts,delete_parts_by_ids}`

- [x] `ValidationContext::{create_error,mc_context,current_path,max_number_of_errors}` shells
- [x] `PackagePartFeature::{uri,is_bound}`; `RootElementFeature::{contains,clear,registered_type_names}`

- [x] `ValidationStack` / `ValidationElement` / `StateManager` / `ValidationErrorEventArgs`
- [x] `ValidationStack` full-frame inheritance, reusable cleared frames, value/property pushes, and panic-safe scopes
- [x] `StateManager` key-only typed cache semantics + mutable/replace/remove operations
- [x] `ValidationContext::clear` C# error-only semantics + explicit full `reset`
- [x] `ValidationCancellationToken` + `check_if_cancelled` / `try_add_error` / `try_create_error` Result APIs (C# `CheckIfCancelled` throw path)
- [x] `ValidationErrorSink` stack-frame error routing (C# `Stack.Push(Errors.Add)` / `Current.AddError`) + scoped `with_error_sink`
- [x] Scoped `with_expected_children_collection` toggle; `reset` restores MC context + collect flag
- [x] `ValidationTraverser` MC-aware preorder walk (`validating_traverse_tree`): AC branch selection via `GetContentFromACBlock`, unknown ProcessContent promotion, out-of-version + misc skips
- [x] `TraversalOptions::SelectAlternateContent` descendants + context-level `validating_traverse` (per-child stack frame, budget stop, cancellation Err)
- [x] `McContext::get_content_from_ac_block` Choice/Fallback selection with Requires prefix resolution + version availability
- [x] `DocumentValidator` orchestration: package frame + structure errors, reachable-part walk, per-part schema/constraint passes, `Sch_MissingPartRootElement` for empty XML parts, `ExceptionError` for malformed XML, cancellation-aware
- [x] `OpenXmlValidator` document APIs: `validate_document_package/part/dom_element` (+`_with_token`) with MC process-settings version-mismatch guard and reserved-element rejection
- [x] `DocumentValidator` schema pass includes `AlternateContentValidator` + `CompatibilityRuleAttributesValidator` (MC structural/attribute errors typed MarkupCompatibility)
- [x] `ExpectedChildren` (elements + xsd:any namespaces, merge/count/clear, `Fmt_ListOfPossibleElements` message) + `ParticleMatch` / `ParticleMatchInfo` shells
- [x] Particle `get_required_elements` / `get_expected_elements` walks (Choice requires all alternatives); mismatch errors append expected-children list under `CollectExpectedChildren`
- [x] Simple-type restrictions: `verify_token` (xsd:token), `verify_ncname` / `is_valid_qname` (xsd:QName), `validate_any_uri` (xsd:anyURI)
- [x] `SemanticValidationLevel` flags + `SemanticConstraintGate` (level/version/application gating from stack frames, C# `SemanticConstraint.Validate` shell)
- [x] `compose_schema/mc/validation_error` context helpers + `ValidationResources` message table subset (part path from stack frame, node/related-node paths, unknown-id fallback)
- [x] `word::particle_for` registry lookup; DocumentValidator validates non-document roots via traverser walk + per-element particles
- [x] `OpenXmlElementExtensionMethods`: `get_xpath_index`, `get_attribute_value_ex`, `can_contain_child` / `try_create_valid_child` (generated WordprocessingML children tables), `is_in_version`
- [x] `Sch_UndeclaredAttribute` extended-attribute check against generated attribute tables (inherited MC Ignorable, xml:/xmlns/mc: skips) wired into DocumentValidator
- [x] `Sch_InvalidChildinLeafElement` leaf-content check from generated `is_leaf`/`is_leaf_text` metadata (one error per element)
- [x] `Sch_AttributeValueDataTypeDetailed` lexical checks against generated attribute `type_name` (hex/base64/int families/OnOff/dateTime)
- [x] `XsdAnyNamespace` modes on `Particle::Any` (##any/##other/##local/##targetNamespace vs parent target namespace) + wildcard tokens in expected-children
- [x] `Particle::Versioned` + `build_for(version)` pruning (C# `ParticleConstraint.Build`); particle validation builds version-pruned trees first
- [x] `logical_children_mc` / `get_first_child_mc` / `get_next_child_mc` MC child cursor (AC branch resolution, Ignorable skip, ProcessContent promotion)
- [x] Element-level semantic constraint shells: `SemanticConstraint` trait + `AttributeCannotOmit` / `MutualExclusive` / `ValueLength` / `ValueRange` / `ValueSet` / `RequiredConditionToValue` / `AbsentConditionToValue` / `MinMax` / `Pair` + `validate_element_constraints`
- [x] Remaining attribute semantic constraints: `AttributeValuePatternConstraint` (lightweight regex subset), `AttributeValueLessEqualToAnother`, `AttributeAbsentConditionToNonValue`, `AttributeValueConditionToAnother`
- [x] Part/package semantic constraints: `SemanticConstraintContext` + `validate_element_constraints_with_part`; `RelationshipExist` / `RelationshipType` / `UniqueAttributeValue` / `ParentType` / `ReferenceExist` / `IndexReference`
- [x] `DocumentValidator` constraint pass runs full `validate_schematron_constraints`; part validation adds relationship/uniqueness + cross-part Schematron via `validate_part_semantic`
- [x] `XsdType` + `ParticleType` enums; `ValidationCache::get_constraint` version-builds/memos Word particles; DocumentValidator prefers cache for non-document roots
- [x] Attribute lexical checks route through `XsdType::validate_lexical`; `Particle::Any` supports `AnyWithUri` + `any_with_uri` constructor; `XsdAnyNamespace::namespace_string`
- [x] `OpenXmlValidator::validate_element` routes through `DocumentValidator`/`validate_dom_element`; `ValidationContext::get_particle_constraint` (C# `GetParticleConstraint`)
- [x] Particle mismatch errors use `Sch_InvalidElementContentExpectingComplex` / `Sch_IncompleteContentExpectingComplex` ids; Word/Excel/PPT validate_* route through DocumentValidator package orchestration + package constraints
- [x] Particle validation always collects expected/required children into Sch_* mismatch messages; `validate_particle_with_context` is public; Word particle walk enables collection
- [x] `ValidationContext` embeds stack + state manager
- [x] `OpenSettings` fluent builders + `from_other` copy ctor

- [x] `PartConstraintFeature` data/part rule splits + required_relationship_types
- [x] `validate_package_constraint_results` + `OpenXmlPackageValidationResult::from_validation_error`
- [x] `OpenXmlValidator::validate_package_constraint_results`

- [x] `PackageEqualityOptions` fluent builders / `structure_only`; `PackageDiff` helpers
- [x] `OpenXmlPackage::{relationship,relationships}_by_type` + package-level variants

- [x] `MarkupCompatibilityProcessSettings` builders / process_mode accessors
- [x] `OpenXmlPart::{relationship,relationships}_by_type`

- [x] `ElementMetadata` attribute/child/count/availability/schema_type helpers
- [x] `ElementMetadataFactoryFeature` remove/clear/type_names/is_empty
- [x] `OpenXmlNamespaceResolverFeature` extended namespace enumerate/remove/clear helpers

- [x] `PartExtensionProvider` enumerate/remove/clear helpers + package register/remove wrappers
- [x] `PartUriHelper` reserved URI enumerate/unreserve/sequence inspection
- [x] `PartUriFeature` + `OpenXmlPackage` ensure-unique/unreserve/query wrappers

- [x] Explicit `ValidationErrorType` storage/builders with heuristic fallback
- [x] `OpenXmlValidator::on_validation_error_event` mutable replacement callback
- [x] Package constraint errors carry explicit `ValidationErrorType::Package`

- [x] Borrowed target-version MC validation children (Choice/Fallback, ProcessContent, ignored/misc nodes)
- [x] Version-aware lightweight child and ordered particle validation entry points
- [x] `OpenXmlValidator` threads configured `FileFormatVersions` through Word validation

- [x] PackageValidator version awareness: `relationship_introduced_in` + `PartConstraintRule.availability`/`applies_to`; `rules_for_version` / `try_get_rule_for_version` / `missing_required_for_version`
- [x] `validate_package_constraints_for_version` / `validate_package_constraint_results_for_version` / `validate_part_constraints_for_version` / `validate_package_for_version` (C# `version.AtLeast(rule.FileFormat)` + `part.IsInVersion`)
- [x] `DocumentValidator` / `OpenXmlValidator` / `OpenXmlPackage` pass target `FileFormatVersions` into package constraint walk
- [x] `SchemaTypeValidator` shell: `validate_schema_type` / `validate_schema_types_in_tree` / `is_reserved_element` / empty-root leaf check; DocumentValidator schema pass routes through it with inherited MC Ignorable
- [x] Framework `IValidator` shell: `Validator` trait + `VersionGate` / `RequiredValidator` / `StringValidator` / `NumberValidator` / `EnumValidator` / `UnionValidator` / `ListValidator` / `OfficeVersionValidator` + `validate_value` helper
- [x] `validate_attribute_value_types` routes declared attributes through `validate_attribute_with_type_name` / `TypeNameValidator` (Number/OnOff/HexBinary/token-family) with XsdType lexical fallback
- [x] Expand `ValidationResources` message table: remaining Sch_* (All/Union/Empty/Length/TotalDigits/Unexpected/WrongType/StringIsNotValidValue), Sem_AttributeValueUniqueInDocument / Sem_CellValue, Pkg_* / Fmt_* / TypeName_* ids
- [x] Particle mismatch classification: `Sch_AllElement` for xs:all duplicates; `Sch_UnexpectedElementContentExpectingComplex` when parent can contain child; `Sch_InvalidElementContentWrongType` via `try_create_valid_child`
- [x] Spreadsheet `Sem_CellValue` (C# `CellType` IValidator): `validate_spreadsheet_cell_values` for boolean/date/number `c@t` + `v` lexical checks; DocumentValidator constraint pass
- [x] SpreadsheetML particle registry (`workbook`/`sheets`/`worksheet`/`sheetData`/`row`/`c`); combined `particle_for` + ValidationCache/SchemaTypeValidator routing
- [x] PresentationML particle registry (`presentation`/`sld`/`sldLayout`/`sldMaster`/`cSld`/`spTree`/`sp`/`pic`); combined `particle_for` covers Word+Spreadsheet+Presentation
- [x] `VersionGate` matches C# `VersionedValidator` (InitialVersion/exact/all); `SimpleTypeValidator` + integer/OnOff convenience wrappers
- [x] `NameProvider`/`NameProviderValidator`; `OfficeVersionValidator` skips empty values and MC-ignorable namespaces (C# parity)
- [x] `OpenXmlElement` non-misc child/sibling helpers (`first_non_misc_element_child` / `next_non_misc_element_sibling_*` / `index_of_child`) — C# `GetFirstNonMiscElementChild` / `GetNextNonMiscElementSibling`
- [x] `validate_spreadsheet_particles[_for_version]` / `validate_presentation_particles[_for_version]` recursive walks for worksheet/workbook and sld/presentation roots
- [x] `ValidationSettings.application_type` + OpenXmlValidator/ValidationContext accessors; document package validation seeds from package ApplicationTypeFeature when settings are ALL
- [x] `NumberValidator::total_digits` (C# TotalDigits / `Sch_TotalDigitsConstraintFailed`)
- [x] DocumentValidator runs structured Word/Spreadsheet/Presentation particle walks for known package roots (deduped against schema-type pass); re-export `word`/`spreadsheet`/`presentation` particle modules
- [x] `DocumentValidator::parts_to_be_validated` filters by `relationship_introduced_in` (C# `part.IsInVersion(version)`)
- [x] DocumentValidator package structure errors use `Pkg_` MessageId prefix (C# `Pkg_` + PackageValidator MessageId)
- [x] Align `Pkg_*` ValidationResources text with C# resx; `ExpectedChildren::expected_children_message` uses `Fmt_*` resources
- [x] `format_validation_resource` public helper; package constraint errors use C# `Pkg_*` resource templates via `err_pkg`
- [x] `UniqueAttributeValueConstraint` parent-scoped early-return matches C# (null when parent configured)
- [x] `OpenXmlPackageValidationResult::into_pkg_validation_error` / `part_name_and_uri` (C# ValidatePackageStructure mapping); DocumentValidator uses it for package errors
- [x] `from_validation_error` strips `Pkg_` prefix; `part_display_name` resolves class names from content type
- [x] Spreadsheet particles: `sst` / `styleSheet` / `fonts` / `fills` / `borders` / `cellXfs`; walks + DocumentValidator roots
- [x] Word particles: `sectPr` / `sdt` / `sdtContent` / `hyperlink` / `drawing`
- [x] Presentation particles: `notes` / `notesMaster` roots in registry, walks, and DocumentValidator
- [x] Word part-root particles: `styles` / `numbering` / `fonts` / `comments` / `footnotes` / `endnotes`
- [x] Word particles: `hdr` / `ftr` / `abstractNum`; DocumentValidator routes styles/numbering/fonts/comments/footnotes/endnotes/hdr/ftr through particle registry
- [x] Word particles: `settings` / `webSettings` / `glossaryDocument` (via generated content models); DocumentValidator routes them
- [x] AC unprefixed-attribute parity (`MC_ErrorOnUnprefixedAttributeName`); `choice()` emits `mc:Requires`
- [x] DrawingML particles: `theme` / `themeOverride` / `chartSpace`; Spreadsheet `chartsheet`; Presentation `handoutMaster` / `presentationPr`; combined registry + DocumentValidator walks
- [x] Spreadsheet particles: `calcChain` / `connections` / `externalLink` / `table` / `queryTable` / `pivotTableDefinition` + DocumentValidator walks
- [x] Presentation particles: `cmLst` / `cmAuthorLst` / `tagLst` / `viewPr`; Drawing `wsDr` worksheet drawing + walks
- [x] Word `commentsEx` / `people` (w15); Spreadsheet `pivotCacheDefinition` / `pivotCacheRecords` / `metadata` / `dialogsheet` + DocumentValidator routes
- [x] Drawing/diagram particles: `userShapes` / `colorsDef` / `dataModel` / `layoutDef` / `styleDef` + DocumentValidator walks
- [x] Schema attribute/leaf validation covers Spreadsheet/Presentation/Drawing (not only Word): undeclared attrs, type lexical checks, leaf child rejection
- [x] `CanContainChild` / `TryCreateValidChild` multi-schema (w/x/p/a); `StringValidator::id()` + ID type-name mapping
- [x] Drawing `tblStyleLst` particle; `OpenXmlElement::descendants_named` (C# `Descendants<T>` by name)
- [x] `RootElementFeature` unregister / try_create_element / seed_common_part_roots / registered_entries
- [x] `PartFactoryFeature` seed_from_generated_parts / unregister; DefaultFeatures.ensure_on seeds RootElement + PartFactory
- [x] `TypedPartFactoryFeature` seed_from_generated_parts / unregister; DefaultFeatures + package accessor auto-seed
- [x] `ElementMetadataFactoryFeature::seed_common_elements` from generated schema tables; DefaultFeatures auto-seeds
- [x] `FileFormatVersions::ensure_element_in_version` + OpenXmlValidator DOM gate (C# `ThrowIfNotInVersion`)
- [x] `TypedPart::is_in_version` uses relationship year heuristic; `/2011/` maps to Office2010
- [x] Spreadsheet particles: `volTypes` / `singleXmlCells` / `MapInfo` + DocumentValidator walks
- [x] OpenXmlValidator part path: `ensure_relationship_in_version` (C# `ThrowIfNotInVersion(OpenXmlPart)`)
- [x] Spreadsheet shared-workbook particles (`headers` / `revisions` / `users`) + Presentation `sldSyncPr`
- [x] Modern comments/slicers: `commentsIds` / `commentsExtensible` / `ThreadedComments` / `slicers` / `slicerCacheDefinition`
- [x] Timelines/named views/authors: `timelines` / `timelineCacheDefinition` / `namedSheetViews` / `personList` / `authorLst`

**Still intentionally deferred / partial:**

1. Exclusive W3C C14N + full X.509 certificate chain validation for Office digsig profiles  
2. VBA **bytecode execution** (CFB inventory only; intentional non-goal)  
3. 1:1 API parity with every C# strongly-typed Part class method surface (generated `*Part` wrappers still thin / metadata-driven)  
4. DomReader true line info (no source stream for pure DOM walks)  
5. Full experimental IPackageFactory middleware DI graph (builders + initializer hooks covered)

Regenerate Schematron tables / typed wrappers:

```bash
python3 scripts/generate_schematron_rules.py
python3 scripts/generate_typed_elements.py
```

Regenerate Schematron tables:

```bash
python3 scripts/generate_schematron_rules.py
```

## Design choices (Rust-idiomatic)

1. **Lightweight Features bag** — `FeatureCollection` type-keyed services; not a full C# DI/event graph.
2. **Owned DOM** — `OpenXmlElement` is an owned tree (Vec children), not a linked list.
3. **In-memory package + optional lazy ZIP** — default loads parts; `open_lazy` defers decompress.
4. **Hand-written core first** — codegen lands once the runtime model is stable.
5. **Same JSON data** — generator reads `Open-XML-SDK/data` so schemas stay in sync.
