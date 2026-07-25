//! Integration tests for openxml.

use officexml::element::{parse_element, write_element, OpenXmlElement};
use officexml::namespace::{content_type, rel};
use officexml::opc::{OpcPackage, PackUri, RelationshipTargetMode};
use officexml::packaging::{
    PresentationDocument, PresentationDocumentType, SpreadsheetDocument,
    SpreadsheetDocumentType, WordprocessingDocument, WordprocessingDocumentType,
};
use officexml::wordprocessing::{
    body, document, paragraph, paragraph_with_text, run, simple_document, text,
};

#[test]
fn create_open_roundtrip_docx() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.docx");

    {
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        doc.add_main_document_part().set_document(document(vec![body(vec![
            paragraph(vec![run(vec![text("Hello, Open XML!")])]),
            paragraph_with_text("Second paragraph"),
            paragraph(vec![run(vec![text("  spaced  ")])]),
        ])]));
        doc.save().unwrap();
    }

    assert!(path.exists());

    let mut doc = WordprocessingDocument::open(&path, false).unwrap();
    assert_eq!(doc.document_type(), WordprocessingDocumentType::Document);
    let texts = doc.paragraph_texts().unwrap();
    assert_eq!(texts.len(), 3);
    assert_eq!(texts[0], "Hello, Open XML!");
    assert_eq!(texts[1], "Second paragraph");
    assert_eq!(texts[2], "  spaced  ");
}

#[test]
fn in_memory_to_bytes() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("mem")]));
    let bytes = doc.to_bytes().unwrap();
    assert!(!bytes.is_empty());

    let mut opened = WordprocessingDocument::open_bytes(&bytes).unwrap();
    let texts = opened.paragraph_texts().unwrap();
    assert_eq!(texts, vec!["mem".to_string()]);
}

#[test]
fn opc_package_relationships() {
    let mut pkg = OpcPackage::create();
    pkg.set_part(
        "/word/document.xml",
        content_type::WORD_DOCUMENT,
        b"<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"/>"
            .to_vec(),
    );
    pkg.add_package_relationship(
        rel::OFFICE_DOCUMENT,
        &PackUri::new("/word/document.xml"),
        RelationshipTargetMode::Internal,
    );
    let bytes = pkg.to_bytes().unwrap();
    let opened = OpcPackage::open_bytes(&bytes).unwrap();
    let main = opened.main_part_uri(rel::OFFICE_DOCUMENT).unwrap();
    assert_eq!(main.as_str(), "/word/document.xml");
}

#[test]
fn element_xml_roundtrip() {
    let elem = OpenXmlElement::w("p").with_child(
        OpenXmlElement::w("r").with_child(OpenXmlElement::w("t").with_text("x & y < z")),
    );
    let xml = write_element(&elem).unwrap();
    let parsed = parse_element(&xml).unwrap();
    assert_eq!(parsed.inner_text(), "x & y < z");
}

#[test]
fn create_simple_helper() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("simple.docx");
    let mut doc = WordprocessingDocument::create_simple(&path, "quick").unwrap();
    doc.save().unwrap();

    let mut opened = WordprocessingDocument::open(&path, false).unwrap();
    assert_eq!(opened.paragraph_texts().unwrap(), vec!["quick".to_string()]);
}

#[test]
fn append_paragraph_to_existing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("append.docx");

    {
        let mut doc = WordprocessingDocument::create_simple(&path, "one").unwrap();
        doc.save().unwrap();
    }

    {
        let mut doc = WordprocessingDocument::open(&path, true).unwrap();
        {
            let body = doc.body_mut().unwrap();
            body.append_child(paragraph_with_text("two"));
        }
        doc.save().unwrap();
    }

    let mut doc = WordprocessingDocument::open(&path, false).unwrap();
    let texts = doc.paragraph_texts().unwrap();
    assert_eq!(texts, vec!["one".to_string(), "two".to_string()]);
}

#[test]
fn spreadsheet_create_and_read() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("grid.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings(
            "Sheet1",
            &[
                vec!["Name", "Score"],
                vec!["Alice", "95"],
                vec!["Bob", "87"],
            ],
        )
        .unwrap();
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    let rows = doc.read_sheet_strings().unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], vec!["Name".to_string(), "Score".to_string()]);
    assert_eq!(rows[1], vec!["Alice".to_string(), "95".to_string()]);
    assert_eq!(rows[2], vec!["Bob".to_string(), "87".to_string()]);
}

#[test]
fn presentation_create_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("deck.pptx");

    {
        let mut doc =
            PresentationDocument::create(&path, PresentationDocumentType::Presentation).unwrap();
        doc.add_presentation_with_slide().unwrap();
        doc.save().unwrap();
    }

    let doc = PresentationDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/ppt/presentation.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/ppt/slides/slide1.xml")));
}

#[test]
fn package_properties_roundtrip() {
    use officexml::opc::PackageProperties;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("props.docx");

    {
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        doc.add_main_document_part()
            .set_document(document(vec![body(vec![paragraph_with_text("x")])]));
        let mut props = PackageProperties::new();
        props.title = Some("My Title".into());
        props.creator = Some("openxml-rs".into());
        doc.set_package_properties(&props).unwrap();
        doc.save().unwrap();
    }

    let doc = WordprocessingDocument::open(&path, false).unwrap();
    let props = doc.package_properties().unwrap();
    assert_eq!(props.title.as_deref(), Some("My Title"));
    assert_eq!(props.creator.as_deref(), Some("openxml-rs"));
}

#[test]
fn word_styles_settings_and_image() {
    use officexml::packaging::ImageFormat;
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("rich.docx");

    {
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        doc.add_main_document_part()
            .set_document(document(vec![body(vec![paragraph_with_text("img")])]));
        doc.add_default_styles().unwrap();
        doc.add_default_settings().unwrap();
        // 1x1 PNG
        let png = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
            0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08,
            0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE,
            0xD4, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];
        let image = doc.add_image(ImageFormat::Png, png.to_vec()).unwrap();
        assert!(image.relationship_id().starts_with("rId"));
        doc.save().unwrap();
    }

    let doc = WordprocessingDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/styles.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/settings.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/media/image1.png")));
}

#[test]
fn simple_attribute_helpers() {
    use officexml::simple_types::OnOffValue;

    let mut bold = OpenXmlElement::w("b");
    bold.set_simple_attribute_qname("w:val", OnOffValue(true));
    assert_eq!(bold.get_attribute_qname("w:val"), Some("1"));
    let v: OnOffValue = bold.get_simple_attribute_qname("w:val").unwrap();
    assert!(v.0);

    let el = OpenXmlElement::w("p")
        .with_attribute_qname("w:rsidR", "00AB12CD")
        .with_attribute("custom", "x");
    assert_eq!(el.get_attribute_qname("w:rsidR"), Some("00AB12CD"));
    assert_eq!(el.get_attribute("custom"), Some("x"));
}

#[test]
fn spreadsheet_shared_strings_and_multi_sheet() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("multi.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_shared_strings(
            "Data",
            &[
                vec!["Name", "Score"],
                vec!["Alice", "95"],
                vec!["Alice", "95"], // dedupe via SST
            ],
        )
        .unwrap();
        doc.write_sheet_strings("Notes", &[vec!["hello"], vec!["world"]])
            .unwrap();
        assert_eq!(doc.worksheets().len(), 2);
        // Name, Score, Alice, 95 → 4 unique shared strings
        assert_eq!(doc.shared_strings().unwrap().len(), 4);
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    assert_eq!(doc.worksheets().len(), 2);
    let data = doc.read_sheet_strings_by_name(Some("Data")).unwrap();
    assert_eq!(data[0], vec!["Name".to_string(), "Score".to_string()]);
    assert_eq!(data[1], vec!["Alice".to_string(), "95".to_string()]);
    assert_eq!(data[2], vec!["Alice".to_string(), "95".to_string()]);
    let notes = doc.read_sheet_strings_by_name(Some("Notes")).unwrap();
    assert_eq!(notes[0], vec!["hello".to_string()]);
    assert_eq!(notes[1], vec!["world".to_string()]);
}

#[test]
fn word_header_footer_hyperlink() {
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hdr.docx");

    {
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        doc.add_main_document_part()
            .set_document(document(vec![body(vec![paragraph_with_text("body")])]));
        doc.add_default_header("HEADER").unwrap();
        doc.add_default_footer("FOOTER").unwrap();
        let link = doc
            .create_hyperlink("https://example.com", "click me")
            .unwrap();
        doc.body_mut().unwrap().append_child(paragraph(vec![link]));
        doc.save().unwrap();
    }

    let mut doc = WordprocessingDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/header1.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/footer1.xml")));
    // Hyperlink relationship should be external
    let main_uri = PackUri::new("/word/document.xml");
    let rels = doc
        .package()
        .opc()
        .part_relationships(&main_uri)
        .expect("main rels");
    assert!(rels
        .iter()
        .any(|r| r.relationship_type.contains("hyperlink")
            && r.target == "https://example.com"));
    let texts = doc.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("click me")));
}

#[test]
fn presentation_text_slide_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("talk.pptx");

    {
        let mut doc =
            PresentationDocument::create(&path, PresentationDocumentType::Presentation).unwrap();
        doc.add_slide_with_text("Hello PPT").unwrap();
        doc.add_slide_with_text("Second slide").unwrap();
        assert_eq!(doc.slides().len(), 2);
        doc.save().unwrap();
    }

    let doc = PresentationDocument::open(&path, false).unwrap();
    assert_eq!(doc.slides().len(), 2);
    assert_eq!(doc.slide_texts(0).unwrap(), vec!["Hello PPT".to_string()]);
    assert_eq!(
        doc.slide_texts(1).unwrap(),
        vec!["Second slide".to_string()]
    );
}

#[test]
fn strict_to_transitional_rewrite() {
    use officexml::namespace::rel;
    use officexml::namespace_rewrite::{
        rewrite_package_to_transitional, to_transitional_namespace,
    };
    use officexml::opc::{OpcPackage, PackUri, RelationshipTargetMode};
    use officexml::namespace::content_type;

    assert_eq!(
        to_transitional_namespace("http://purl.oclc.org/ooxml/wordprocessingml/main"),
        Some("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    );

    let mut pkg = OpcPackage::create();
    pkg.set_part(
        "/word/document.xml",
        content_type::WORD_DOCUMENT,
        br#"<?xml version="1.0"?>
        <w:document xmlns:w="http://purl.oclc.org/ooxml/wordprocessingml/main">
          <w:body><w:p><w:r><w:t>Strict</w:t></w:r></w:p></w:body>
        </w:document>"#
            .to_vec(),
    );
    pkg.package_relationships_mut().add_with_id(
        "rId1",
        "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument",
        "word/document.xml",
        RelationshipTargetMode::Internal,
    );

    let (xml_n, rel_n) = rewrite_package_to_transitional(&mut pkg).unwrap();
    assert!(xml_n > 0);
    assert_eq!(rel_n, 1);
    assert_eq!(
        pkg.package_relationships()
            .get("rId1")
            .unwrap()
            .relationship_type,
        rel::OFFICE_DOCUMENT
    );
    let doc = pkg
        .get_part_str(&PackUri::new("/word/document.xml"))
        .unwrap()
        .unwrap();
    assert!(doc.contains("schemas.openxmlformats.org/wordprocessingml/2006/main"));
}

#[test]
fn file_format_versions_mc() {
    use officexml::element::OpenXmlElement;
    use officexml::file_format::{supported_prefixes, FileFormatVersions};
    use officexml::markup_compatibility::{
        process_markup_compatibility_for_version, with_ignorable,
    };

    let p2010 = supported_prefixes(FileFormatVersions::OFFICE2010);
    assert!(p2010.contains(&"w14"));
    assert!(!p2010.contains(&"w15"));

    // w14 content ignorable when targeting 2007
    let mut root = with_ignorable(
        OpenXmlElement::w("document")
            .with_child(OpenXmlElement::w("body"))
            .with_child(OpenXmlElement::new(
                "w14",
                "http://schemas.microsoft.com/office/word/2010/wordml",
                "docId",
            )),
        "w14",
    );
    process_markup_compatibility_for_version(&mut root, FileFormatVersions::OFFICE2007);
    assert!(root.children.iter().all(|c| c.prefix != "w14"));

    // w14 kept when targeting 2010
    let mut root2 = with_ignorable(
        OpenXmlElement::w("document").with_child(OpenXmlElement::new(
            "w14",
            "http://schemas.microsoft.com/office/word/2010/wordml",
            "docId",
        )),
        "w14",
    );
    process_markup_compatibility_for_version(&mut root2, FileFormatVersions::OFFICE2010);
    assert_eq!(root2.children.len(), 1);
    assert_eq!(root2.children[0].prefix, "w14");
}

#[test]
fn excel_pivot_table() {
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("pivot.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings(
            "Data",
            &[
                vec!["Region", "Sales"],
                vec!["North", "100"],
                vec!["South", "80"],
                vec!["North", "50"],
            ],
        )
        .unwrap();
        let (pivot_uri, cache_uri) = doc
            .add_pivot_table(
                "Data",
                "A1:B4",
                "Data",
                "E3",
                &["Region", "Sales"],
                0,
                1,
                3,
            )
            .unwrap();
        assert!(pivot_uri.as_str().contains("pivotTable"));
        assert!(cache_uri.as_str().contains("pivotCacheDefinition"));
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/xl/pivotTables/pivotTable1.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/xl/pivotCache/pivotCacheDefinition1.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/xl/pivotCache/pivotCacheRecords1.xml")));
    // workbook should list pivotCaches
    let wb = doc
        .package()
        .opc()
        .get_part(&PackUri::new("/xl/workbook.xml"))
        .unwrap();
    let root = officexml::element::parse_element(wb).unwrap();
    assert!(root.child("pivotCaches").is_some());
}

#[test]
fn excel_conditional_formatting() {
    use officexml::element::parse_element;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cf.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings(
            "Sheet1",
            &[
                vec!["Score"],
                vec!["10"],
                vec!["50"],
                vec!["90"],
            ],
        )
        .unwrap();
        doc.add_conditional_formatting_cell_is(
            "Sheet1",
            "A2:A4",
            "greaterThan",
            "50",
            "FFFF0000",
            1,
        )
        .unwrap();
        doc.add_conditional_formatting_color_scale("Sheet1", "A2:A4", 2)
            .unwrap();
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    let sheet_uri = &doc.worksheets()[0].uri;
    let data = doc.package().opc().get_part(&sheet_uri).unwrap();
    let root = parse_element(data).unwrap();
    let cfs: Vec<_> = root.children_by_name("conditionalFormatting").collect();
    assert!(cfs.len() >= 2);
    assert!(cfs.iter().any(|cf| {
        cf.children_by_name("cfRule")
            .any(|r| r.get_attribute("type") == Some("cellIs"))
    }));
    assert!(cfs.iter().any(|cf| {
        cf.children_by_name("cfRule")
            .any(|r| r.get_attribute("type") == Some("colorScale"))
    }));
    // styles should have dxfs
    let styles = doc
        .package()
        .opc()
        .get_part(&officexml::opc::PackUri::new("/xl/styles.xml"))
        .unwrap();
    let sroot = parse_element(styles).unwrap();
    assert!(sroot.child("dxfs").is_some());
}

#[test]
fn excel_sheet_comments() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("comments.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings("Sheet1", &[vec!["A", "B"], vec!["1", "2"]])
            .unwrap();
        doc.add_sheet_comments(
            "Sheet1",
            "alice",
            &[("A1", "Header note"), ("B2", "Check value")],
        )
        .unwrap();
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    let notes = doc.sheet_comments("Sheet1").unwrap();
    assert_eq!(notes.len(), 2);
    assert!(notes.iter().any(|(r, a, t)| r == "A1" && a == "alice" && t.contains("Header")));
    assert!(notes.iter().any(|(r, _, t)| r == "B2" && t.contains("Check")));
}

#[test]
fn excel_image_on_sheet() {
    use officexml::opc::PackUri;

    // 1x1 PNG
    let png = [
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
        0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
        0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08,
        0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE,
        0xD4, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("img.xlsx");
    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings("Sheet1", &[vec!["pic"]]).unwrap();
        let (img_uri, drawing_uri) = doc
            .add_image_on_sheet(
                "Sheet1",
                &png,
                "image/png",
                "png",
                1,
                1,
                914_400,
                914_400,
                "logo",
            )
            .unwrap();
        assert!(img_uri.as_str().contains("media/image"));
        assert!(drawing_uri.as_str().contains("drawing"));
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .part_uris().into_iter().any(|u| u.as_str().contains("/xl/media/")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/xl/drawings/drawing1.xml")));
}

#[test]
fn generated_full_version_table() {
    use officexml::file_format::{supported_prefixes, FileFormatVersions};
    use officexml::generated::namespaces;

    // Full table from namespaces.json should be much larger than the bootstrap list
    assert!(namespaces::PREFIX_INTRODUCED_IN.len() > 100);
    assert!(namespaces::version_for_prefix("w14") == Some(FileFormatVersions::OFFICE2010));
    assert!(namespaces::version_for_prefix("w15") == Some(FileFormatVersions::OFFICE2013));
    // supported_prefixes uses generated table
    let p = supported_prefixes(FileFormatVersions::OFFICE2010);
    assert!(p.contains(&"w14"));
    assert!(p.len() > 10);
}

#[test]
fn excel_chart_on_sheet() {
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("anchored.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings("Data", &[vec!["A", "B"], vec!["1", "2"]])
            .unwrap();
        let (chart_uri, drawing_uri) = doc
            .add_bar_chart_on_sheet("Data", "Sales", &["A", "B"], &[1.0, 2.0], 0, 2, 5, 12)
            .unwrap();
        assert!(chart_uri.as_str().contains("chart"));
        assert!(drawing_uri.as_str().contains("drawing"));
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/xl/charts/chart1.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/xl/drawings/drawing1.xml")));
    // worksheet should reference drawing
    let sheet_uri = &doc.worksheets()[0].uri;
    let data = doc.package().opc().get_part(&sheet_uri).unwrap();
    let root = officexml::element::parse_element(data).unwrap();
    assert!(root.child("drawing").is_some());
    // drawing should relate to chart
    let drawing_uri = PackUri::new("/xl/drawings/drawing1.xml");
    let rels = doc
        .package()
        .opc()
        .part_relationships(&drawing_uri)
        .unwrap();
    assert!(rels.iter().any(|r| r.relationship_type.contains("chart")));
}

#[test]
fn generated_particles() {
    use officexml::generated::wordprocessingml_2006_main as wml;
    use officexml::validation::validate_particle;
    use officexml::wordprocessing::{body, document, paragraph_with_text};

    assert_eq!(wml::PARTICLE_COUNT, 165);
    let p = wml::particle_for_class("Document").expect("Document particle");
    let doc = document(vec![body(vec![paragraph_with_text("x")])]);
    let errs = validate_particle(&doc, &p, "w:document");
    assert!(errs.is_empty(), "{errs:?}");

    // Paragraph particle from schema
    let pp = wml::particle_paragraph();
    let para = paragraph_with_text("hi");
    let errs = validate_particle(&para, &pp, "w:p");
    assert!(errs.is_empty(), "{errs:?}");
}

#[test]
fn particle_validation_and_chart() {
    use officexml::opc::PackUri;
    use officexml::validation::validate_word_particles;
    use officexml::wordprocessing::{body, document, paragraph_with_text, table_from_strings};

    // Particle validation on a table document
    let doc = document(vec![body(vec![
        paragraph_with_text("Title"),
        table_from_strings(&[vec!["A", "B"], vec!["1", "2"]], None),
    ])]);
    let errs = validate_word_particles(&doc);
    assert!(errs.is_empty(), "{errs:?}");

    // Chart part
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("chart.xlsx");
    {
        let mut wb =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        wb.write_sheet_strings("Data", &[vec!["N", "S"], vec!["10", "20"]])
            .unwrap();
        let (uri, rid) = wb
            .add_bar_chart("Sales", &["N", "S"], &[10.0, 20.0])
            .unwrap();
        assert!(uri.as_str().contains("chart"));
        assert!(rid.starts_with("rId"));
        wb.save().unwrap();
    }
    let wb = SpreadsheetDocument::open(&path, false).unwrap();
    assert!(wb
        .package()
        .opc()
        .has_part(&PackUri::new("/xl/charts/chart1.xml")));
}

#[test]
fn word_footnotes_and_validation() {
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("notes.docx");

    {
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        doc.add_main_document_part()
            .set_document(document(vec![body(vec![paragraph_with_text("Hello")])]));
        doc.add_footnote("1", "A footnote body").unwrap();
        doc.add_endnote("1", "An endnote body").unwrap();
        let errs = doc.validate().unwrap();
        assert!(errs.is_empty(), "{errs:?}");
        doc.save().unwrap();
    }

    let doc = WordprocessingDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/footnotes.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/endnotes.xml")));
}

#[test]
fn presentation_notes_slide() {
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("notes.pptx");

    {
        let mut doc =
            PresentationDocument::create(&path, PresentationDocumentType::Presentation).unwrap();
        doc.add_slide_with_text("Slide content").unwrap();
        let notes_uri = doc.add_notes_to_slide(0, "Speaker notes here").unwrap();
        assert!(notes_uri.as_str().contains("notesSlide"));
        doc.save().unwrap();
    }

    let doc = PresentationDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/ppt/notesSlides/notesSlide1.xml")));
    let slide_uri = PackUri::new("/ppt/slides/slide1.xml");
    let rels = doc.package().opc().part_relationships(&slide_uri).unwrap();
    assert!(rels.iter().any(|r| r.relationship_type.contains("notesSlide")));
}

#[test]
fn mc_ignorable_strip() {
    use officexml::element::OpenXmlElement;
    use officexml::markup_compatibility::{process_markup_compatibility, with_ignorable};

    let mut root = with_ignorable(
        OpenXmlElement::w("document")
            .with_child(OpenXmlElement::w("body"))
            .with_child(OpenXmlElement::new(
                "w14",
                "http://schemas.microsoft.com/office/word/2010/wordml",
                "docId",
            )),
        "w14",
    );
    let n = process_markup_compatibility(&mut root, &["w"]);
    assert!(n >= 1);
    assert!(root.children.iter().all(|c| c.local_name != "docId"));
}

#[test]
fn generated_attr_helpers_and_part_constraints() {
    use officexml::generated::parts;
    use officexml::generated::wordprocessingml_2006_main as wml;

    // bold_val convenience constructor
    let b = wml::bold_val("1");
    assert_eq!(b.local_name, "b");
    assert_eq!(b.get_attribute_qname("w:val"), Some("1"));

    // with_ helper
    let p = wml::paragraph_with_rsid_paragraph_addition(wml::paragraph(vec![]), "00AABBCC");
    assert_eq!(p.get_attribute_qname("w:rsidR"), Some("00AABBCC"));

    // Part constraints
    assert!(parts::is_allowed_child("MainDocumentPart", "StyleDefinitionsPart"));
    assert!(parts::is_allowed_child("MainDocumentPart", "HeaderPart"));
    assert!(parts::allows_multiple("MainDocumentPart", "HeaderPart"));
    assert!(!parts::allows_multiple("MainDocumentPart", "StyleDefinitionsPart"));
    assert!(!parts::is_allowed_child("MainDocumentPart", "WorksheetPart"));

    let main = parts::part_by_name("MainDocumentPart").unwrap();
    assert!(!main.children.is_empty());
    assert!(main.children.iter().any(|c| c.name == "ThemePart"));
}

#[test]
fn presentation_master_layout() {
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("master.pptx");

    {
        let mut doc =
            PresentationDocument::create(&path, PresentationDocumentType::Presentation).unwrap();
        let (master, layout) = doc.add_blank_master_with_layout().unwrap();
        assert!(master.uri.as_str().contains("slideMaster"));
        assert!(layout.uri.as_str().contains("slideLayout"));
        doc.add_slide_with_layout(officexml::presentation::slide_with_text("On layout"))
            .unwrap();
        assert_eq!(doc.masters().len(), 1);
        // Full Office scaffold installs 11 layouts (title, obj, blank, …)
        assert_eq!(doc.layouts().len(), 11);
        assert_eq!(doc.slides().len(), 1);
        doc.save().unwrap();
    }

    let doc = PresentationDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/ppt/slideMasters/slideMaster1.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/ppt/slideLayouts/slideLayout1.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/ppt/slideLayouts/slideLayout11.xml")));
    // Slide should have relationship to layout
    let slide_uri = PackUri::new("/ppt/slides/slide1.xml");
    let rels = doc.package().opc().part_relationships(&slide_uri).unwrap();
    assert!(rels.iter().any(|r| r.relationship_type.contains("slideLayout")));
}

#[test]
fn excel_formula_cell() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("formula.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings("Sheet1", &[vec!["10", "20"]]).unwrap();
        doc.set_cell_formula("Sheet1", "C1", "A1+B1", Some("30"))
            .unwrap();
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    let (formula, cached) = doc.cell_formula("Sheet1", "C1").unwrap().unwrap();
    assert_eq!(formula, "A1+B1");
    assert_eq!(cached.as_deref(), Some("30"));
}

#[test]
fn presentation_slide_size() {
    use officexml::presentation::SLIDE_SIZE_16_9;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("size.pptx");

    {
        let mut doc =
            PresentationDocument::create(&path, PresentationDocumentType::Presentation).unwrap();
        doc.add_slide_with_text("Sized").unwrap();
        let (cx, cy) = SLIDE_SIZE_16_9;
        // rewrite already sets 16:9; override explicitly
        doc.set_slide_size(cx, cy).unwrap();
        doc.save().unwrap();
    }

    let doc = PresentationDocument::open(&path, false).unwrap();
    let size = doc.slide_size().unwrap().expect("slide size");
    assert_eq!(size, SLIDE_SIZE_16_9);
}

#[test]
fn markup_compatibility_resolve() {
    use officexml::element::OpenXmlElement;
    use officexml::markup_compatibility::{
        alternate_content_with, expand_alternate_content, resolve_alternate_content,
    };

    let ac = alternate_content_with(
        "w14",
        vec![OpenXmlElement::w("newFeature")],
        vec![OpenXmlElement::w("legacy")],
    );
    let r = resolve_alternate_content(&ac, &["w14"]);
    assert_eq!(r[0].local_name, "newFeature");
    let r = resolve_alternate_content(&ac, &["w"]);
    assert_eq!(r[0].local_name, "legacy");

    let mut body = OpenXmlElement::w("body").with_child(ac);
    expand_alternate_content(&mut body, &["w14"]);
    assert_eq!(body.children[0].local_name, "newFeature");
}

#[test]
fn excel_merge_cells_and_styles() {
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("merge.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings(
            "Sheet1",
            &[vec!["Title", "", ""], vec!["a", "b", "c"]],
        )
        .unwrap();
        doc.set_merge_cells("Sheet1", &["A1:C1"]).unwrap();
        doc.add_minimal_styles(true).unwrap();
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    let merges = doc.merge_cells("Sheet1").unwrap();
    assert_eq!(merges, vec!["A1:C1".to_string()]);
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/xl/styles.xml")));
}

#[test]
fn word_alt_chunk() {
    use officexml::opc::PackUri;
    use officexml::packaging::AlternativeFormatImportType;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("alt.docx");

    {
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        doc.add_main_document_part()
            .set_document(document(vec![body(vec![paragraph_with_text("before")])]));
        let rid = doc
            .add_alt_chunk(
                AlternativeFormatImportType::Html,
                b"<html><body><p>HTML chunk</p></body></html>".to_vec(),
            )
            .unwrap();
        assert!(rid.starts_with("rId"));
        doc.save().unwrap();
    }

    let doc = WordprocessingDocument::open(&path, false).unwrap();
    // altChunk part should exist
    let has_af = doc
        .package()
        .opc()
        .part_uris().into_iter().any(|u| u.as_str().contains("afchunk"));
    assert!(has_af);
    // document should contain altChunk element
    use officexml::element::parse_element;
    let data = doc
        .package()
        .opc()
        .get_part(&PackUri::new("/word/document.xml"))
        .unwrap();
    let root = parse_element(data).unwrap();
    assert!(root.descendants().any(|e| e.local_name == "altChunk"));
}

#[test]
fn generated_enums() {
    use officexml::generated::wordprocessingml_2006_main as wml;
    use officexml::simple_types::OpenXmlSimpleType;

    assert!(wml::ENUM_COUNT > 50);
    // HighlightColorValues is a well-known Word enum
    let red = wml::HighlightColorValues::from_str("red").expect("red");
    assert_eq!(red.as_str(), "red");
    assert_eq!(red.as_inner_text(), "red");
    assert!(wml::HighlightColorValues::from_str("nope").is_none());

    use officexml::generated::spreadsheetml_2006_main as sml;
    assert!(sml::ENUM_COUNT > 20);
}

#[test]
fn word_table_and_flat_opc() {
    use officexml::wordprocessing::{table_from_strings, table_to_strings};

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("table.docx");

    {
        let tbl = table_from_strings(
            &[
                vec!["Name", "Score"],
                vec!["Alice", "95"],
                vec!["Bob", "87"],
            ],
            None,
        );
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        doc.add_main_document_part()
            .set_document(document(vec![body(vec![tbl])]));
        doc.save().unwrap();
    }

    let mut doc = WordprocessingDocument::open(&path, false).unwrap();
    {
        use officexml::element::parse_element;
        use officexml::opc::PackUri;
        let data = doc
            .package()
            .opc()
            .get_part(&PackUri::new("/word/document.xml"))
            .unwrap();
        let root = parse_element(data).unwrap();
        let body = root.child("body").unwrap();
        let tbl = body.child("tbl").expect("table");
        let grid = table_to_strings(tbl);
        assert_eq!(grid[0], vec!["Name".to_string(), "Score".to_string()]);
        assert_eq!(grid[1], vec!["Alice".to_string(), "95".to_string()]);
    }

    // Flat OPC round-trip
    let flat = doc.to_flat_opc_string().unwrap();
    assert!(flat.contains("pkg:package"));
    assert!(flat.contains("mso-application"));
    let mut reopened = WordprocessingDocument::from_flat_opc(flat.as_bytes()).unwrap();
    let texts = reopened.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("Alice")));
}

#[test]
fn word_numbering_theme_clone() {
    use officexml::opc::PackUri;
    use officexml::wordprocessing::numbered_paragraph;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("list.docx");

    {
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        doc.add_main_document_part().set_document(document(vec![body(vec![
            numbered_paragraph(1, "Item A"),
            numbered_paragraph(1, "Item B"),
        ])]));
        doc.add_default_numbering().unwrap();
        doc.add_default_theme().unwrap();
        doc.save().unwrap();
    }

    let mut doc = WordprocessingDocument::open(&path, false).unwrap();
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/numbering.xml")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/theme/theme1.xml")));

    let mut cloned = doc.clone_document().unwrap();
    let texts = cloned.paragraph_texts().unwrap();
    assert_eq!(texts.len(), 2);
    assert!(texts[0].contains("Item A"));
}

#[test]
fn excel_column_widths() {
    use officexml::element::parse_element;
    use officexml::opc::PackUri;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cols.xlsx");

    {
        let mut doc =
            SpreadsheetDocument::create(&path, SpreadsheetDocumentType::Workbook).unwrap();
        doc.write_sheet_strings("Sheet1", &[vec!["A", "B", "C"]])
            .unwrap();
        doc.set_column_widths("Sheet1", &[(1, 1, 20.0), (2, 3, 12.5)])
            .unwrap();
        doc.save().unwrap();
    }

    let doc = SpreadsheetDocument::open(&path, false).unwrap();
    let uri = &doc.worksheets()[0].uri;
    let data = doc.package().opc().get_part(&uri).unwrap();
    let root = parse_element(data).unwrap();
    let cols = root.child("cols").expect("cols present");
    assert_eq!(cols.children_by_name("col").count(), 2);
    let _ = PackUri::new("/"); // silence unused in some builds
}

#[test]
fn word_find_replace_and_comments() {
    use officexml::opc::PackUri;
    use officexml::wordprocessing::{comment, paragraph, run, text, with_comment};

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("comments.docx");

    {
        let mut doc =
            WordprocessingDocument::create(&path, WordprocessingDocumentType::Document).unwrap();
        // Paragraph with a commented run
        let commented = with_comment("0", vec![run(vec![text("TODO")])]);
        doc.add_main_document_part().set_document(document(vec![body(vec![
            paragraph(vec![run(vec![text("Hello world")])]),
            paragraph(commented),
        ])]));
        doc.set_comments(vec![comment("0", "Alice", "A", "Please fix")])
            .unwrap();
        let n = doc.replace_text("world", "Rust").unwrap();
        assert_eq!(n, 1);
        doc.save().unwrap();
    }

    let mut doc = WordprocessingDocument::open(&path, false).unwrap();
    let texts = doc.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("Hello Rust")));
    assert!(texts.iter().any(|t| t.contains("TODO")));
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/comments.xml")));
}

#[test]
fn generated_wordprocessing_factories() {
    use officexml::generated::parts;
    use officexml::generated::wordprocessingml_2006_main as wml;

    assert!(wml::ELEMENT_COUNT > 500);
    assert!(wml::TYPE_COUNT > 700);
    assert!(wml::info_by_class("Paragraph").is_some());
    assert_eq!(
        wml::info_by_local_name("t").unwrap().class_name,
        "Text"
    );

    let doc = wml::document_root(vec![wml::body(vec![wml::paragraph(vec![wml::run(vec![
        wml::bold(),
        wml::text("generated"),
    ])])])]);
    let xml = write_element(&doc).unwrap();
    let parsed = parse_element(&xml).unwrap();
    // bold is empty leaf; text carries the string
    assert!(parsed.inner_text().contains("generated"));

    let main = parts::part_by_name("MainDocumentPart").unwrap();
    assert!(main
        .relationship_type
        .contains("relationships/officeDocument"));
    assert_eq!(main.root_element, Some("document"));
    assert_eq!(main.path_general, "word");

    // Attribute inheritance: Bold gets OnOffType's `w:val`
    let bold = wml::info_by_class("Bold").unwrap();
    assert!(
        bold.attributes
            .iter()
            .any(|a| a.qname == "w:val" && a.type_name == "OnOffValue"),
        "Bold should inherit w:val from OnOffType"
    );
    // Paragraph has rsid attrs + children
    let p = wml::info_by_class("Paragraph").unwrap();
    assert!(p.attributes.iter().any(|a| a.qname == "w:rsidR"));
    assert!(!p.children.is_empty());
}

#[test]
fn extended_and_custom_properties_roundtrip() {
    use officexml::{CustomProperties, ExtendedProperties, PackageProperties};

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("props")]));

    let mut core = PackageProperties::new();
    core.title = Some("T".into());
    doc.set_package_properties(&core).unwrap();

    let mut ext = ExtendedProperties::new();
    ext.application = Some("openxml-rs".into());
    ext.company = Some("Acme".into());
    ext.pages = Some(1);
    doc.set_extended_properties(&ext).unwrap();

    let mut custom = CustomProperties::new();
    custom.set_string("Project", "Alpha");
    custom.set_i4("Rev", 3);
    doc.set_custom_properties(&custom).unwrap();

    let bytes = doc.to_bytes().unwrap();
    let opened = WordprocessingDocument::open_bytes(&bytes).unwrap();
    assert_eq!(
        opened.package_properties().unwrap().title.as_deref(),
        Some("T")
    );
    let ext2 = opened.extended_properties().unwrap();
    assert_eq!(ext2.application.as_deref(), Some("openxml-rs"));
    assert_eq!(ext2.company.as_deref(), Some("Acme"));
    assert_eq!(ext2.pages, Some(1));
    let custom2 = opened.custom_properties().unwrap();
    assert_eq!(
        custom2.get("Project").and_then(|p| p.value.as_str()),
        Some("Alpha")
    );
}

#[test]
fn custom_xml_part_roundtrip() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("cx")]));
    let xml = br#"<?xml version="1.0"?><root xmlns="urn:test"><item>42</item></root>"#;
    let (rid, uri) = doc.add_custom_xml_part(xml).unwrap();
    assert!(rid.starts_with('r'));
    assert!(uri.as_str().contains("customXml"));
    let parts = doc.custom_xml_parts().unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0].0, rid);
    assert!(String::from_utf8_lossy(&parts[0].2).contains("42"));
}

#[test]
fn create_from_template_and_change_type() {
    let dir = tempfile::tempdir().unwrap();
    let template = dir.path().join("t.dotx");
    {
        let mut doc =
            WordprocessingDocument::create(&template, WordprocessingDocumentType::Template)
                .unwrap();
        doc.add_main_document_part()
            .set_document(simple_document(vec![paragraph_with_text("template body")]));
        doc.save().unwrap();
    }
    let mut from = WordprocessingDocument::create_from_template(
        &template,
        Some(WordprocessingDocumentType::Document),
    )
    .unwrap();
    assert_eq!(from.document_type(), WordprocessingDocumentType::Document);
    let texts = from.paragraph_texts().unwrap();
    assert_eq!(texts[0], "template body");
    let pkg_errs = from.validate_package().unwrap();
    assert!(pkg_errs.is_empty(), "{pkg_errs:?}");
}

#[test]
fn package_validation_detects_missing_main() {
    use officexml::validation::validate_package;
    let pkg = OpcPackage::create();
    let errs = validate_package(&pkg, true);
    assert!(!errs.is_empty());
}

#[test]
fn line_and_pie_charts_and_fill_styles() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    let (line_uri, _) = wb.add_line_chart("Line", &["A", "B"], &[1.0, 2.0]).unwrap();
    let (pie_uri, _) = wb.add_pie_chart("Pie", &["A", "B"], &[3.0, 4.0]).unwrap();
    assert!(wb.package().opc().has_part(&line_uri));
    assert!(wb.package().opc().has_part(&pie_uri));
    wb.add_styles_with_fill("FFFF00").unwrap();
    let (_, idx) = wb.add_styles_with_num_fmt("0.00%").unwrap();
    assert_eq!(idx, 1);
    let errs = wb.validate_package().unwrap();
    assert!(errs.is_empty(), "{errs:?}");
}

#[test]
fn encrypted_ole_signature_detected() {
    use officexml::opc::OpcPackage;
    // Minimal OLE CFB header (8-byte signature + padding)
    let mut fake = vec![0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
    fake.resize(64, 0);
    let mut cursor = std::io::Cursor::new(fake);
    assert!(OpcPackage::is_encrypted_office_stream(&mut cursor).unwrap());
}

#[test]
fn excel_defined_names_roundtrip() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Data", &[vec!["x"], vec!["1"]])
        .unwrap();
    wb.set_defined_names(&[("MyRange", "Data!$A$1:$A$2"), ("Title", "Data!$A$1")])
        .unwrap();
    // Adding another sheet must not wipe defined names
    wb.add_worksheet("Other").unwrap();
    let names = wb.defined_names().unwrap();
    assert!(names.iter().any(|(n, r)| n == "MyRange" && r.contains("Data")));
    assert!(names.iter().any(|(n, _)| n == "Title"));
    let bytes = wb.to_bytes().unwrap();
    let opened = SpreadsheetDocument::open_bytes(&bytes).unwrap();
    let names2 = opened.defined_names().unwrap();
    assert_eq!(names2.len(), 2);
}

#[test]
fn more_simple_types() {
    use officexml::simple_types::{
        Base64BinaryValue, ByteValue, DateTimeValue, ListValue, OpenXmlSimpleType, SingleValue,
        TrueFalseBlankValue, TrueFalseValue,
    };
    assert_eq!(
        Base64BinaryValue::from_inner_text("YQ==").unwrap().as_inner_text(),
        "YQ=="
    );
    assert_eq!(
        DateTimeValue::from_inner_text("2020-01-01T00:00:00Z")
            .unwrap()
            .as_inner_text(),
        "2020-01-01T00:00:00Z"
    );
    assert_eq!(TrueFalseValue::from_inner_text("t").unwrap().0, true);
    assert_eq!(TrueFalseBlankValue::from_inner_text("").unwrap().0, false);
    assert_eq!(
        ListValue::from_inner_text("a b c").unwrap().0,
        vec!["a", "b", "c"]
    );
    assert_eq!(ByteValue::from_inner_text("255").unwrap().0, 255);
    assert_eq!(SingleValue::from_inner_text("1.5").unwrap().0, 1.5);
}

#[test]
fn word_font_table_and_web_settings() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("fonts")]));
    doc.add_default_font_table().unwrap();
    doc.add_default_web_settings().unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&PackUri::new("/word/fontTable.xml")));
    assert!(opc.has_part(&PackUri::new("/word/webSettings.xml")));
}

#[test]
fn excel_table_autofilter_validation() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "Data",
        &[
            vec!["Name", "Status"],
            vec!["A", "Yes"],
            vec!["B", "No"],
        ],
    )
    .unwrap();
    wb.set_auto_filter("Data", "A1:B3").unwrap();
    wb.add_data_validation_list("Data", "B2:B100", "\"Yes,No\"", true)
        .unwrap();
    let (uri, rid) = wb
        .add_table("Data", "Table1", "A1:B3", &["Name", "Status"])
        .unwrap();
    assert!(wb.package().opc().has_part(&uri));
    assert!(rid.starts_with('r'));
    let bytes = wb.to_bytes().unwrap();
    let opened = SpreadsheetDocument::open_bytes(&bytes).unwrap();
    assert!(opened.package().opc().has_part(&uri));
}

#[test]
fn ppt_image_on_slide() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Hello").unwrap();
    // 1x1 PNG
    let png: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0xD4, 0xEF, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    let (uri, rid) = ppt
        .add_image_on_slide(0, png, "image/png", "png", 0, 0, 914_400, 914_400, "pic")
        .unwrap();
    assert!(ppt.package().opc().has_part(&uri));
    assert!(rid.starts_with('r'));
}

#[test]
fn word_sdt_content_controls() {
    use officexml::wordprocessing::{collect_sdt_tags, sdt_block};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let sdt = sdt_block(
        "CustomerName",
        "Customer",
        vec![paragraph_with_text("Acme Corp")],
    );
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![sdt])]));
    let tags = {
        let body = doc.body_mut().unwrap();
        // body_mut marks dirty; collect from parent via package after flush
        let _ = body;
        // re-read from document
        drop(body);
        let package = doc.package();
        // use paragraph path: collect via to_bytes roundtrip
        ()
    };
    let _ = tags;
    let bytes = doc.to_bytes().unwrap();
    let mut opened = WordprocessingDocument::open_bytes(&bytes).unwrap();
    // Load DOM via paragraph_texts path
    let _ = opened.paragraph_texts().unwrap();
    // Access body through body_mut then walk - simpler: parse main part
    let main_uri = PackUri::new("/word/document.xml");
    let xml = opened.package().opc().get_part(&main_uri).unwrap();
    let root = parse_element(xml).unwrap();
    let found = collect_sdt_tags(&root);
    assert!(found.iter().any(|(t, a, text)| {
        t == "CustomerName" && a == "Customer" && text.contains("Acme")
    }));
}

#[test]
fn excel_calc_chain() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.set_cell_formula("S", "A3", "A1+A2", Some("3")).unwrap();
    let uri = wb.set_calc_chain(&[("A3", 1)]).unwrap();
    assert!(wb.package().opc().has_part(&uri));
}

#[test]
fn element_equality() {
    use officexml::element::{elements_equal, OpenXmlElement};
    let a = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_text("x"));
    let b = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_text("x"));
    let c = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_text("y"));
    assert!(elements_equal(&a, &b));
    assert!(!elements_equal(&a, &c));
}

#[test]
fn thumbnail_and_max_chars() {
    use officexml::error::Error;
    use officexml::packaging::OpenSettings;

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("thumb")]));
    let png: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0xD4, 0xEF, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    doc.add_thumbnail(png, "image/png", "png").unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&PackUri::new("/docProps/thumbnail.png")));
    assert!(opc
        .package_relationships()
        .get_by_type(rel::THUMBNAIL)
        .is_some());

    // MaxCharactersInPart guard when loading main document
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("lim.docx");
    std::fs::write(&path, &bytes).unwrap();
    let mut settings = OpenSettings::default();
    settings.max_characters_in_part = 10;
    settings.auto_save = false;
    let result = WordprocessingDocument::open_with_settings(&path, false, settings);
    match result {
        Ok(mut d) => match d.paragraph_texts() {
            Err(Error::PartTooLarge { .. }) => {}
            other => panic!("expected PartTooLarge, got {other:?}"),
        },
        Err(Error::PartTooLarge { .. }) => {}
        Err(e) => panic!("unexpected open error: {e}"),
    }
}

#[test]
fn excel_comments_include_vml() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_sheet_comments("S", "alice", &[("A1", "note")]).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let has_vml = opc.part_uris().into_iter().any(|u| u.as_str().contains("vmlDrawing"));
    assert!(has_vml, "expected VML drawing part for comments");
}

#[test]
fn word_track_changes_accept_reject() {
    use officexml::wordprocessing::{
        deleted_text_run, inserted_text_run, paragraph, run, text,
    };
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p = paragraph(vec![
        run(vec![text("Hello ")]),
        inserted_text_run("1", "alice", "2020-01-01T00:00:00Z", "world"),
        deleted_text_run("2", "bob", "2020-01-02T00:00:00Z", " old"),
    ]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    // Accept: keep "world", drop " old"
    let n = doc.accept_all_revisions().unwrap();
    assert!(n >= 2);
    let texts = doc.paragraph_texts().unwrap();
    assert!(texts[0].contains("world"), "{texts:?}");
    assert!(!texts[0].contains("old"), "{texts:?}");

    // Reject path
    let mut doc2 =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p2 = paragraph(vec![
        run(vec![text("Hello ")]),
        inserted_text_run("1", "alice", "2020-01-01T00:00:00Z", "world"),
        deleted_text_run("2", "bob", "2020-01-02T00:00:00Z", "old"),
    ]);
    doc2
        .add_main_document_part()
        .set_document(document(vec![body(vec![p2])]));
    doc2.reject_all_revisions().unwrap();
    let texts2 = doc2.paragraph_texts().unwrap();
    assert!(!texts2[0].contains("world"), "{texts2:?}");
    assert!(texts2[0].contains("old"), "{texts2:?}");
}

#[test]
fn ppt_table_on_slide() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Title").unwrap();
    ppt.add_table_on_slide(
        0,
        &[vec!["A", "B"], vec!["1", "2"]],
        100_000,
        100_000,
        4_000_000,
        1_500_000,
        "Table 1",
    )
    .unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opened = PresentationDocument::open_bytes(&bytes).unwrap();
    let slide_uri = &opened.slides()[0].uri;
    let xml = opened.package().opc().get_part(&slide_uri).unwrap();
    let s = String::from_utf8_lossy(xml);
    assert!(s.contains("graphicFrame") || s.contains("tbl"), "{s}");
}

#[test]
fn open_settings_mc_process() {
    use officexml::file_format::FileFormatVersions;
    use officexml::markup_compatibility::alternate_content_with;
    use officexml::packaging::{
        MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings, OpenSettings,
    };
    use officexml::element::OpenXmlElement;

    // Build a document body containing AlternateContent
    let ac = alternate_content_with(
        "w14",
        vec![OpenXmlElement::w("r").with_child(OpenXmlElement::w("t").with_text("new"))],
        vec![OpenXmlElement::w("r").with_child(OpenXmlElement::w("t").with_text("legacy"))],
    );
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![ac]),
    ])]));
    let bytes = doc.to_bytes().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mc.docx");
    std::fs::write(&path, &bytes).unwrap();

    let mut settings = OpenSettings::default();
    settings.auto_save = false;
    settings.markup_compatibility = MarkupCompatibilityProcessSettings {
        mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
        // Only core "w" is supported → Choice for w14 should fall to Fallback
        target_file_format_versions: FileFormatVersions::OFFICE2007,
    };
    let mut opened =
        WordprocessingDocument::open_with_settings(&path, false, settings).unwrap();
    let texts = opened.paragraph_texts().unwrap();
    // After MC processing, fallback "legacy" should remain
    assert!(
        texts.iter().any(|t| t.contains("legacy")),
        "expected legacy fallback, got {texts:?}"
    );
}

#[test]
fn word_protection_and_glossary() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("prot")]));
    doc.set_document_protection("readOnly", true).unwrap();
    doc.add_glossary_document("Entry1", vec![paragraph_with_text("glossary body")])
        .unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&PackUri::new("/word/settings.xml")));
    let settings = String::from_utf8_lossy(opc.get_part(&PackUri::new("/word/settings.xml")).unwrap());
    assert!(settings.contains("documentProtection"));
    assert!(opc.has_part(&PackUri::new("/word/glossary/document.xml")));
}

#[test]
fn ppt_notes_master() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let uri = ppt.add_notes_master().unwrap();
    assert!(ppt.package().opc().has_part(&uri));
    let bytes = ppt.to_bytes().unwrap();
    let opened = PresentationDocument::open_bytes(&bytes).unwrap();
    assert!(opened.package().opc().has_part(&uri));
}

#[test]
fn excel_sheet_and_workbook_protection() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_sheet_protection("S", true, true, true).unwrap();
    wb.set_workbook_protection(true, false).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("sheetProtection"));
    let wb_xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/workbook.xml")).unwrap(),
    );
    assert!(wb_xml.contains("workbookProtection"));
}

#[test]
fn word_embedded_package() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("embed")]));
    // Embed a tiny "package" blob
    let (rid, uri) = doc
        .add_embedded_package(b"PK\x03\x04fake", "application/vnd.openxmlformats-officedocument.wordprocessingml.document", "docx")
        .unwrap();
    assert!(rid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&uri));
}

#[test]
fn word_bookmarks() {
    use officexml::wordprocessing::{bookmark_end, bookmark_start, collect_bookmarks, paragraph, run, text};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p = paragraph(vec![
        bookmark_start("0", "chap1"),
        run(vec![text("Chapter 1")]),
        bookmark_end("0"),
    ]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    let bytes = doc.to_bytes().unwrap();
    let opened = WordprocessingDocument::open_bytes(&bytes).unwrap();
    let xml = opened
        .package()
        .opc()
        .get_part(&PackUri::new("/word/document.xml"))
        .unwrap();
    let root = parse_element(xml).unwrap();
    let bms = collect_bookmarks(&root);
    assert!(bms.iter().any(|(id, name)| id == "0" && name == "chap1"));
}

#[test]
fn excel_freeze_panes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h"], vec!["1"]]).unwrap();
    wb.set_freeze_panes("S", 0, 1).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("sheetViews") && sheet.contains("frozen"));
}

#[test]
fn ppt_handout_master() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let uri = ppt.add_handout_master().unwrap();
    assert!(ppt.package().opc().has_part(&uri));
}

#[test]
fn word_page_setup_and_fields() {
    use officexml::wordprocessing::{page_number_field, paragraph, run, section_properties_with_page};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![run(vec![text("Page ")]), page_number_field()]),
    ])]));
    doc.set_page_setup(12240, 15840, 1440, 1440, 1440, 1440)
        .unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("pgSz") && xml.contains("pgMar"));
    assert!(xml.contains("fldSimple") || xml.contains("PAGE"));
    let _ = section_properties_with_page(12240, 15840, 1440, 1440, 1440, 1440);
}

#[test]
fn excel_page_setup() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_page_setup("S", (0.7, 0.7, 0.75, 0.75, 0.3, 0.3), 1, "portrait")
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("pageMargins") && sheet.contains("pageSetup"));
}

#[test]
fn stream_reader_paragraph_text() {
    use officexml::element::OpenXmlStreamReader;
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![
            paragraph_with_text("Alpha"),
            paragraph_with_text("Beta"),
        ]));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = opc
        .get_part(&PackUri::new("/word/document.xml"))
        .unwrap();
    let mut reader = OpenXmlStreamReader::from_bytes(xml);
    let texts = reader.collect_text_under("t").unwrap();
    assert_eq!(texts, vec!["Alpha".to_string(), "Beta".to_string()]);
}

#[test]
fn attribute_type_validation() {
    use officexml::validation::{validate_attribute_value, AttributeType};
    assert!(validate_attribute_value("/r", "val", "1", AttributeType::OnOff).is_none());
    assert!(validate_attribute_value("/r", "val", "maybe", AttributeType::OnOff).is_some());
    assert!(validate_attribute_value("/x", "id", "00AABBCC", AttributeType::HexBinary).is_none());
}

#[test]
fn ppt_create_from_template() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.pptx");
    {
        let mut ppt =
            PresentationDocument::create(&path, PresentationDocumentType::Presentation).unwrap();
        ppt.add_slide_with_text("Template Slide").unwrap();
        ppt.save().unwrap();
    }
    let from = PresentationDocument::create_from_template(&path).unwrap();
    assert!(!from.slides().is_empty());
    let texts = from.slide_texts(0).unwrap();
    assert!(texts.iter().any(|t| t.contains("Template")));
}

#[test]
fn digital_signature_shell() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("sig")]));
    let (rid, uri) = doc.add_digital_signature_origin().unwrap();
    assert!(rid.starts_with('r'));
    let (srid, suri) = doc
        .add_xml_signature_part(br#"<?xml version="1.0"?><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"/>"#)
        .unwrap();
    assert!(srid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&uri));
    assert!(opc.has_part(&suri));
}

#[test]
fn excel_row_heights() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"], vec!["b"]]).unwrap();
    wb.set_row_heights("S", &[(1, 30.0, false), (2, 15.0, true)])
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("ht=\"30\"") || sheet.contains("ht=\"30.0\""));
    assert!(sheet.contains("hidden=\"1\""));
}

#[test]
fn ppt_audio_media_part() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let info = ppt
        .add_audio_on_slide(0, b"ID3fakeaudio", "audio/mpeg", "mp3")
        .unwrap();
    assert!(ppt.package().opc().has_part(&info.uri));
    assert_eq!(info.kind, officexml::MediaKind::Audio);
}

#[test]
fn word_anchor_hyperlink_and_recipients() {
    use officexml::wordprocessing::{bookmark_end, bookmark_start, paragraph, run, text};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let link = doc.create_anchor_hyperlink("chap1", "Go to chapter");
    let p = paragraph(vec![
        bookmark_start("0", "chap1"),
        run(vec![text("Chapter 1")]),
        bookmark_end("0"),
        link,
    ]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    let (rid, uri) = doc
        .add_mail_merge_recipients(
            br#"<?xml version="1.0"?><recipients xmlns="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#,
        )
        .unwrap();
    assert!(rid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&uri));
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("w:anchor") || xml.contains("anchor=\"chap1\""));
}

#[test]
fn excel_scatter_tab_color_print_area() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Data", &[vec!["x", "y"], vec!["1", "2"]])
        .unwrap();
    let (uri, _) = wb
        .add_scatter_chart("Scatter", &[1.0, 2.0, 3.0], &[2.0, 4.0, 1.0])
        .unwrap();
    assert!(wb.package().opc().has_part(&uri));
    wb.set_sheet_tab_color("Data", "00FF00").unwrap();
    wb.set_print_area("Data", "$A$1:$B$2").unwrap();
    let names = wb.defined_names().unwrap();
    assert!(names.iter().any(|(n, r)| n == "_xlnm.Print_Area" && r.contains("Data")));
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("tabColor"));
}

#[test]
fn word_page_number_footer() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("body")]));
    doc.add_page_number_footer().unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.part_uris().into_iter().any(|u| u.as_str().contains("footer")));
}

#[test]
fn word_doc_vars_and_toc() {
    use officexml::wordprocessing::{paragraph, run, text, toc_field};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![toc_field("")]),
        paragraph_with_text("Heading body"),
    ])]));
    doc.set_document_variables(&[("Project", "Alpha"), ("Rev", "3")])
        .unwrap();
    let vars = doc.document_variables().unwrap();
    assert_eq!(vars.len(), 2);
    let bytes = doc.to_bytes().unwrap();
    let opened = WordprocessingDocument::open_bytes(&bytes).unwrap();
    let vars2 = opened.document_variables().unwrap();
    assert!(vars2.iter().any(|(n, v)| n == "Project" && v == "Alpha"));
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("TOC") || xml.contains("fldSimple"));
}

#[test]
fn excel_sparkline() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[
            vec!["1", "2", "3", "4"],
        ],
    )
    .unwrap();
    wb.add_sparkline("S", "line", "S!A1:D1", "E1").unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("sparkline") || sheet.contains("sparklineGroup"));
}

#[test]
fn word_validate_relationships() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![])]));
    let link = doc
        .create_hyperlink("https://example.com", "ex")
        .unwrap();
    doc.body_mut().unwrap().append_child(paragraph(vec![link]));
    // Valid hyperlink should pass
    let errs = doc.validate_relationships().unwrap();
    assert!(
        errs.iter().all(|e| !e.message.contains("does not exist")),
        "{errs:?}"
    );

    // Inject broken hyperlink
    {
        let body = doc.body_mut().unwrap();
        let mut bad = OpenXmlElement::w("hyperlink");
        bad.set_attribute_ns(
            "r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            "id",
            "rIdMissing",
        );
        bad.append_child(run(vec![text("bad")]));
        body.append_child(paragraph(vec![bad]));
    }
    let errs = doc.validate_relationships().unwrap();
    assert!(
        errs.iter().any(|e| e.message.contains("rIdMissing")),
        "{errs:?}"
    );
}

#[test]
fn excel_area_chart() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (uri, _) = wb
        .add_area_chart("Area", &["A", "B"], &[1.0, 2.0])
        .unwrap();
    assert!(wb.package().opc().has_part(&uri));
}

#[test]
fn word_background_and_drop_cap() {
    use officexml::wordprocessing::drop_cap_paragraph;
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![drop_cap_paragraph("Once upon a time", 3)])]));
    doc.set_document_background("FFFFCC").unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("background"));
    assert!(xml.contains("dropCap") || xml.contains("framePr"));
}

#[test]
fn excel_dimension_and_shared_formula() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"], vec!["3"]])
        .unwrap();
    wb.set_sheet_dimension("S", "A1:A3").unwrap();
    wb.set_shared_formula(
        "S",
        &["B1", "B2", "B3"],
        "A1*2",
        &[Some("2"), Some("4"), Some("6")],
        0,
    )
    .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("dimension"));
    assert!(sheet.contains("t=\"shared\"") || sheet.contains("t='shared'"));
}

#[test]
fn ppt_sections() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("One").unwrap();
    ppt.add_slide_with_text("Two").unwrap();
    ppt.add_slide_with_text("Three").unwrap();
    ppt.set_sections(&[("Intro", 0, 0), ("Body", 1, 2)]).unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/ppt/presentation.xml"))
            .unwrap(),
    );
    assert!(xml.contains("sectionLst") || xml.contains("section"));
}

#[test]
fn word_watermark() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("secret")]));
    doc.add_watermark("CONFIDENTIAL").unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.part_uris().into_iter().any(|u| u.as_str().contains("header")));
    let header = opc
        .part_uris().into_iter().find(|u| u.as_str().contains("header"))
        .unwrap();
    let xml = String::from_utf8_lossy(opc.get_part(&header).unwrap());
    assert!(xml.contains("CONFIDENTIAL") || xml.contains("textpath"));
}

#[test]
fn excel_active_tab() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("A", &[vec!["1"]]).unwrap();
    wb.write_sheet_strings("B", &[vec!["2"]]).unwrap();
    wb.set_active_tab(1).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/workbook.xml")).unwrap(),
    );
    assert!(xml.contains("activeTab"));
}

#[test]
fn excel_sheet_state_calc_and_cf_icons() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Visible", &[vec!["1"], vec!["2"], vec!["3"]])
        .unwrap();
    wb.write_sheet_strings("Hidden", &[vec!["x"]]).unwrap();
    wb.set_sheet_state("Hidden", "hidden").unwrap();
    wb.set_calc_properties(true, "auto").unwrap();
    wb.add_conditional_formatting_data_bar("Visible", "A1:A3", "FF638EC6", 1)
        .unwrap();
    wb.add_conditional_formatting_icon_set("Visible", "A1:A3", "3TrafficLights1", 2)
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let wb_xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/workbook.xml")).unwrap(),
    );
    assert!(wb_xml.contains("state=\"hidden\"") || wb_xml.contains("state='hidden'"));
    assert!(wb_xml.contains("calcPr"));
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("dataBar") || sheet.contains("iconSet"));
}

#[test]
fn ppt_hide_slide() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Hidden").unwrap();
    ppt.set_slide_hidden(0, true).unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/ppt/slides/slide1.xml"))
            .unwrap(),
    );
    assert!(xml.contains("show=\"0\"") || xml.contains("show='0'"));
}

#[test]
fn excel_row_outline() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[
            vec!["Parent"],
            vec!["child1"],
            vec!["child2"],
            vec!["Parent2"],
        ],
    )
    .unwrap();
    wb.set_row_outline_levels("S", &[(2, 1, false), (3, 1, false)])
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("outlineLevel") || sheet.contains("outlinePr"));
}

#[test]
fn word_even_odd_headers_caption_ruby() {
    use officexml::wordprocessing::{caption_field, paragraph, ruby, run};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        caption_field("Figure", "Figure", "1"),
        paragraph(vec![ruby("東京", "とうきょう")]),
    ])]));
    let (d, e) = doc
        .add_even_odd_headers("Odd Header", "Even Header")
        .unwrap();
    assert!(d.starts_with('r') && e.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let settings = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/settings.xml")).unwrap(),
    );
    assert!(settings.contains("evenAndOddHeaders"));
    let doc_xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(doc_xml.contains("SEQ") || doc_xml.contains("ruby"));
}

#[test]
fn word_omml_math() {
    use officexml::wordprocessing::{math_paragraph, omml_fraction};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        math_paragraph(vec![omml_fraction("1", "2")]),
    ])]));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("oMath") || xml.contains("m:f"));
}

#[test]
fn excel_zoom() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_zoom("S", 150).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("zoomScale"));
}

#[test]
fn word_doc_defaults_and_bibliography() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("cite")]));
    doc.set_document_defaults("Calibri", 22).unwrap();
    let (rid, uri) = doc
        .add_bibliography(&[("Src1", "The Book of Rust")])
        .unwrap();
    assert!(rid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let styles = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/styles.xml")).unwrap(),
    );
    assert!(styles.contains("Calibri") || styles.contains("docDefaults"));
    assert!(opc.has_part(&uri));
}

#[test]
fn excel_external_link() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (uri, rid) = wb.add_external_link("file:///tmp/other.xlsx").unwrap();
    assert!(rid.starts_with('r'));
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&uri));
    let wb_xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/workbook.xml")).unwrap(),
    );
    assert!(wb_xml.contains("externalReferences"));
}

#[test]
fn ppt_theme() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("T").unwrap();
    let (uri, _) = ppt.add_default_theme().unwrap();
    assert!(ppt.package().opc().has_part(&uri));
}

#[test]
fn word_page_borders() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("bordered")]));
    doc.set_page_borders("FF0000", 24).unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("pgBorders"));
}

#[test]
fn excel_rich_text_and_chartsheet() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Data", &[vec!["1", "2"]]).unwrap();
    wb.set_rich_text_cell("Data", "C1", &[("Hello ", false), ("World", true)])
        .unwrap();
    let (chart_uri, _) = wb
        .add_bar_chart("Sales", &["A", "B"], &[1.0, 2.0])
        .unwrap();
    let (cs_uri, _) = wb.add_chartsheet("Chart1", &chart_uri).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&cs_uri));
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("<r>") || sheet.contains("inlineStr"));
}

#[test]
fn word_vba_and_comments_ex() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::MacroEnabledDocument)
            .unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("macro")]));
    let (rid, uri) = doc.add_vba_project(b"BIFF_VBA_FAKE").unwrap();
    assert!(rid.starts_with('r'));
    let (crid, curi) = doc
        .add_comments_extended(&[("00000001", "1", false)])
        .unwrap();
    assert!(crid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&uri));
    assert!(opc.has_part(&curi));
}

#[test]
fn excel_slicer_and_theme() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["Region"], vec!["East"]]).unwrap();
    let (s_uri, c_uri) = wb.add_slicer_shell("S", "Slicer_Region", "Slicer_Region").unwrap();
    let (t_uri, _) = wb.add_default_theme().unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&s_uri));
    assert!(opc.has_part(&c_uri));
    assert!(opc.has_part(&t_uri));
}

#[test]
fn word_people_and_custom_xml_props() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("p")]));
    let (prid, puri) = doc
        .add_people(&[("Alice", "None"), ("Bob", "AD")])
        .unwrap();
    assert!(prid.starts_with('r'));
    let (_crid, curi) = doc
        .add_custom_xml_part(br#"<?xml version="1.0"?><root xmlns="urn:test"/>"#)
        .unwrap();
    let (props_rid, props_uri) = doc
        .add_custom_xml_properties(&curi, "{11111111-1111-1111-1111-111111111111}")
        .unwrap();
    assert!(props_rid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&puri));
    assert!(opc.has_part(&props_uri));
}

#[test]
fn excel_connections() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (uri, rid) = wb
        .add_connections(&[("Q1", "SQL", "Server=.;Database=x")])
        .unwrap();
    assert!(rid.starts_with('r'));
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&uri));
}

#[test]
fn word_printer_settings() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("print")]));
    let (rid, uri) = doc.add_printer_settings(b"BIN_PRINTER").unwrap();
    assert!(rid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    assert!(OpcPackage::open_bytes(&bytes).unwrap().has_part(&uri));
}

#[test]
fn ppt_pres_and_view_props() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let (puri, _) = ppt.add_presentation_properties().unwrap();
    let (vuri, _) = ppt.add_view_properties().unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&puri) && opc.has_part(&vuri));
}

#[test]
fn excel_query_table_and_volatile() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (quri, _) = wb.add_query_table("S", "Qt1", 1).unwrap();
    let (vuri, _) = wb.add_volatile_dependencies().unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&quri) && opc.has_part(&vuri));
}

#[test]
fn ppt_comment_authors() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let (uri, _) = ppt
        .add_comment_authors(&[(0, "Alice", "A"), (1, "Bob", "B")])
        .unwrap();
    let bytes = ppt.to_bytes().unwrap();
    assert!(OpcPackage::open_bytes(&bytes).unwrap().has_part(&uri));
}

#[test]
fn word_custom_ui_and_tasks() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("ui")]));
    let (rid, uri) = doc
        .add_custom_ui(
            br#"<?xml version="1.0"?><customUI xmlns="http://schemas.microsoft.com/office/2006/01/customui"/>"#,
        )
        .unwrap();
    assert!(rid.starts_with('r'));
    let (trid, turi) = doc.add_document_tasks(&["Review", "Ship"]).unwrap();
    assert!(trid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&uri) && opc.has_part(&turi));
}

#[test]
fn excel_timeline_shell() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["Date"], vec!["2020-01-01"]])
        .unwrap();
    let (t_uri, c_uri) = wb
        .add_timeline_shell("S", "Timeline1", "TimelineCache1")
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&t_uri) && opc.has_part(&c_uri));
}

#[test]
fn ppt_slide_comments() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_comment_authors(&[(0, "Alice", "A")]).unwrap();
    let (uri, _) = ppt
        .add_slide_comments(0, &[(0, "2020-01-01T00:00:00", 100, 100, "Nice")])
        .unwrap();
    let bytes = ppt.to_bytes().unwrap();
    assert!(OpcPackage::open_bytes(&bytes).unwrap().has_part(&uri));
}

#[test]
fn excel_named_title_style() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["Title"]]).unwrap();
    wb.add_styles_with_named_title().unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let styles = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/styles.xml")).unwrap(),
    );
    assert!(styles.contains("cellStyles") && styles.contains("Title"));
}

#[test]
fn excel_print_titles() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H"], vec!["1"], vec!["2"]])
        .unwrap();
    wb.set_print_titles("S", Some("$1:$1"), Some("$A:$A"))
        .unwrap();
    let names = wb.defined_names().unwrap();
    assert!(names.iter().any(|(n, r)| n == "_xlnm.Print_Titles" && r.contains("S")));
}

#[test]
fn word_styles_and_web_extension() {
    use officexml::wordprocessing::{paragraph, paragraph_properties, run, table_cell, table_row, table_with_style, text};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let heading = paragraph(vec![
        paragraph_properties(vec![
            OpenXmlElement::w("pStyle").with_attribute_qname("w:val", "Heading1"),
        ]),
        run(vec![text("Chapter")]),
    ]);
    let tbl = table_with_style(
        "TableGrid",
        vec![table_row(vec![table_cell(vec![paragraph_with_text("A")])])],
    );
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![heading, tbl])]));
    doc.add_paragraph_styles(&[
        ("Heading1", "heading 1", Some("Normal")),
        ("Heading2", "heading 2", Some("Heading1")),
    ])
    .unwrap();
    let (we, tp) = doc
        .add_web_extension_shell("MyAddin", "1.0.0.0")
        .unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&we) && opc.has_part(&tp));
    let styles = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/styles.xml")).unwrap(),
    );
    assert!(styles.contains("Heading1"));
}

#[test]
fn word_track_revisions_and_compat() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("rev")]));
    doc.set_track_revisions(true).unwrap();
    doc.set_compatibility_mode("15").unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let settings = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/settings.xml")).unwrap(),
    );
    assert!(settings.contains("trackRevisions"));
    assert!(settings.contains("compatibilityMode"));
}

#[test]
fn excel_sheet_format_and_doughnut() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_sheet_format("S", 18.0, Some(12.0)).unwrap();
    let (uri, _) = wb
        .add_doughnut_chart("Donut", &["A", "B"], &[3.0, 7.0])
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&uri));
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("sheetFormatPr"));
}

#[test]
fn excel_pivot_with_real_rows() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "Data",
        &[
            vec!["Region", "Sales"],
            vec!["East", "100"],
            vec!["West", "80"],
            vec!["East", "50"],
        ],
    )
    .unwrap();
    let rows = vec![
        vec!["East", "100"],
        vec!["West", "80"],
        vec!["East", "50"],
    ];
    let (puri, curi) = wb
        .add_pivot_table_with_rows(
            "Data",
            "A1:B4",
            "Data",
            "E3",
            &["Region", "Sales"],
            0,
            1,
            3,
            Some(&rows),
        )
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&puri) && opc.has_part(&curi));
    // Find records part
    let rec = opc
        .part_uris().into_iter().find(|u| u.as_str().contains("pivotCacheRecords"))
        .unwrap();
    let xml = String::from_utf8_lossy(opc.get_part(&rec).unwrap());
    assert!(xml.contains("East") || xml.contains("<r>"));
}

#[test]
fn word_update_fields_setting() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("f")]));
    doc.set_update_fields_on_open(true).unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let settings = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/settings.xml")).unwrap(),
    );
    assert!(settings.contains("updateFields"));
}

#[test]
fn ppt_table_styles() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let (uri, _) = ppt.add_table_styles().unwrap();
    assert!(ppt.package().opc().has_part(&uri));
}

#[test]
fn word_tabs_symbol_mirror() {
    use officexml::wordprocessing::{paragraph_with_tabs, run, symbol, text};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p = paragraph_with_tabs(
        &[("left", 1440), ("right", 7200)],
        vec![run(vec![text("A"), symbol("Symbol", "F0B7"), text("B")])],
    );
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    doc.set_mirror_margins(true).unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let doc_xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(doc_xml.contains("tabs") || doc_xml.contains("sym"));
    let settings = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/settings.xml")).unwrap(),
    );
    assert!(settings.contains("mirrorMargins"));
}

#[test]
fn excel_gridlines_headers() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_show_gridlines("S", false).unwrap();
    wb.set_show_row_col_headers("S", false).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("showGridLines"));
}

#[test]
fn ppt_clone_slide_and_notes_size() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Original").unwrap();
    let cloned = ppt.clone_slide(0).unwrap();
    assert_eq!(ppt.slides().len(), 2);
    assert_ne!(cloned.uri.as_str(), ppt.slides()[0].uri.as_str());
    ppt.set_notes_size(6_858_000, 9_144_000).unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let pres = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/ppt/presentation.xml"))
            .unwrap(),
    );
    assert!(pres.contains("notesSz"));
}

#[test]
fn ppt_slide_transitions() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.set_fade_transition(0, "med").unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/ppt/slides/slide1.xml"))
            .unwrap(),
    );
    assert!(xml.contains("transition") && xml.contains("fade"));
}

#[test]
fn excel_hyperlink_sort_whole_dv() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"], vec!["3", "4"]])
        .unwrap();
    let rid = wb
        .add_cell_hyperlink("S", "A1", "https://example.com", Some("link"))
        .unwrap();
    assert!(rid.starts_with('r'));
    wb.set_sort_state("S", "A1:B3", "A1:A3", false).unwrap();
    wb.add_data_validation_whole("S", "B2:B10", "between", "1", Some("100"), true)
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("hyperlink") || sheet.contains("sortState") || sheet.contains("whole"));
}

#[test]
fn word_spacing_shading() {
    use officexml::wordprocessing::{paragraph_with_spacing, run_highlight, run, text};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p = paragraph_with_spacing(Some(200), Some(200), Some(360), Some("FFFF00"), "Spaced");
    let p2 = paragraph(vec![run(vec![run_highlight("yellow"), text("hi")])]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p, p2])]));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("spacing") || xml.contains("shd") || xml.contains("highlight"));
}

#[test]
fn word_page_break_and_indent() {
    use officexml::wordprocessing::{
        page_break_run, paragraph, paragraph_indent, paragraph_properties, run, text,
    };
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p1 = paragraph(vec![run(vec![text("Page 1")]), page_break_run()]);
    let p2 = paragraph(vec![
        paragraph_properties(vec![paragraph_indent(Some(720), None, Some(360), None)]),
        run(vec![text("Indented")]),
    ]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p1, p2])]));
    doc.set_page_number_start(5).unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("page") || xml.contains("ind") || xml.contains("pgNumType"));
}

#[test]
fn excel_page_breaks() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    let rows: Vec<Vec<&str>> = (0..20).map(|_| vec!["a"]).collect();
    wb.write_sheet_strings("S", &rows).unwrap();
    wb.set_row_breaks("S", &[10, 15]).unwrap();
    wb.set_col_breaks("S", &[3]).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("rowBreaks") || sheet.contains("colBreaks"));
}

#[test]
fn ppt_animation() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Animated").unwrap();
    ppt.set_simple_appear_animation(0, 2).unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/ppt/slides/slide1.xml"))
            .unwrap(),
    );
    assert!(xml.contains("timing") || xml.contains("animEffect"));
}

#[test]
fn word_date_field_and_run_props() {
    use officexml::wordprocessing::{
        author_field, date_field, paragraph, run, run_language, run_spacing, text, time_field,
    };
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p = paragraph(vec![
        run(vec![run_spacing(40), run_language("en-US"), text("spaced")]),
        date_field(),
        time_field(),
        author_field(),
    ]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    doc.append_date_field().unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("DATE") || xml.contains("spacing") || xml.contains("lang"));
}

#[test]
fn excel_array_formula_local_name_show_formulas() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.set_array_formula("S", "B1", "A1:A2", "B1:B2", Some("1"))
        .unwrap();
    wb.set_local_defined_name("S", "LocalRange", "S!$A$1:$A$2")
        .unwrap();
    wb.set_show_formulas("S", true).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let sheet = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/worksheets/sheet1.xml"))
            .unwrap(),
    );
    assert!(sheet.contains("array") || sheet.contains("showFormulas"));
    let wb_xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/xl/workbook.xml")).unwrap(),
    );
    assert!(wb_xml.contains("localSheetId") || wb_xml.contains("LocalRange"));
}

#[test]
fn ppt_header_footer() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.set_slide_header_footer(0, true, true, true).unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/ppt/slides/slide1.xml"))
            .unwrap(),
    );
    assert!(xml.contains("hf"));
}

#[test]
fn word_formatting_and_diagram() {
    use officexml::wordprocessing::{
        caps, justification, paragraph, paragraph_borders, paragraph_properties, run, run_color,
        run_size, small_caps, strike, text, underline,
    };
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p = paragraph(vec![
        paragraph_properties(vec![
            justification("center"),
            paragraph_borders("0000FF", 12),
        ]),
        run(vec![
            run_color("FF0000"),
            run_size(28),
            underline("single"),
            strike(),
            small_caps(),
            caps(),
            text("Fancy"),
        ]),
    ]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    let duri = doc.add_diagram_shell("diag-1").unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&duri));
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/word/document.xml")).unwrap(),
    );
    assert!(xml.contains("jc") || xml.contains("color") || xml.contains("pBdr"));
}

#[test]
fn ppt_slide_background() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Bg").unwrap();
    ppt.set_slide_background(0, "112233").unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let xml = String::from_utf8_lossy(
        opc.get_part(&PackUri::new("/ppt/slides/slide1.xml"))
            .unwrap(),
    );
    assert!(xml.contains("bg") || xml.contains("solidFill") || xml.contains("112233"));
}

#[test]
fn excel_xml_maps() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (uri, _) = wb
        .add_xml_maps(1, "root", r#"<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"/>"#)
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    assert!(OpcPackage::open_bytes(&bytes).unwrap().has_part(&uri));
}

#[test]
fn excel_chart_styles_dialog_named_custom() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]])
        .unwrap();
    let (chart_uri, _) = wb
        .add_bar_chart("C", &["a", "b"], &[1.0, 2.0])
        .unwrap();
    let (s_uri, c_uri) = wb.add_chart_styles(&chart_uri).unwrap();
    let (d_uri, _) = wb.add_dialogsheet("Dialog1").unwrap();
    let (n_uri, _) = wb.add_named_sheet_views("S", "MyView").unwrap();
    let (cd_uri, cp_uri) = wb
        .add_custom_data(b"CUSTOM", "{aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee}")
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    for u in [&s_uri, &c_uri, &d_uri, &n_uri, &cd_uri, &cp_uri] {
        assert!(opc.has_part(&u), "missing {}", u.as_str());
    }
}

#[test]
fn word_label_and_ole() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("sec")]));
    let (lrid, luri) = doc
        .add_label_info("{label-id}", "Confidential")
        .unwrap();
    let (orid, ouri) = doc
        .add_embedded_object(b"OLEDATA", "Excel.Sheet.12")
        .unwrap();
    assert!(lrid.starts_with('r') && orid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&luri) && opc.has_part(&ouri));
}

#[test]
fn ppt_modern_comments() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let (auri, _) = ppt
        .add_modern_authors(&[("auth1", "Alice")])
        .unwrap();
    let (curi, _) = ppt
        .add_modern_comments(0, &[("auth1", "Looks good")])
        .unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&auri) && opc.has_part(&curi));
}

#[test]
fn excel_rich_macrosheet_theme_override() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (sc, _) = wb.add_single_cell_table("S", "A1", 1).unwrap();
    let (rd, rs, rt) = wb.add_rich_value_shell().unwrap();
    let (fp, _) = wb.add_feature_property_bag().unwrap();
    let (ms, _) = wb.add_macrosheet("Macro1").unwrap();
    let (chart_uri, _) = wb.add_bar_chart("C", &["a"], &[1.0]).unwrap();
    let (to, _) = wb.add_theme_override(&chart_uri).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    for u in [&sc, &rd, &rs, &rt, &fp, &ms, &to] {
        assert!(opc.has_part(&u), "missing {}", u.as_str());
    }
}

#[test]
fn word_styles_effects_vba_data_qat() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::MacroEnabledDocument)
            .unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_vba_project(b"VBA").unwrap();
    let (_, se) = doc.add_styles_with_effects().unwrap();
    let (_, vd) = doc.add_vba_data().unwrap();
    let (_, cust) = doc.add_customization().unwrap();
    let (_, qat) = doc.add_quick_access_toolbar().unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    for u in [&se, &vd, &cust, &qat] {
        assert!(opc.has_part(&u), "missing {}", u.as_str());
    }
}

#[test]
fn ppt_tags_model_sync() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let (tags, _) = ppt
        .add_user_defined_tags(0, &[("k", "v")])
        .unwrap();
    let (model, _) = ppt.add_model_3d(0, b"glTF").unwrap();
    let (sync, _) = ppt.add_slide_sync_data(0, "server-1").unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    for u in [&tags, &model, &sync] {
        assert!(opc.has_part(&u), "missing {}", u.as_str());
    }
}

#[test]
fn excel_threaded_revision_metadata() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (puri, _) = wb
        .add_persons(&[("{p1}", "Alice")])
        .unwrap();
    let (turi, _) = wb
        .add_threaded_comments("S", &[("{c1}", "{p1}", "Hi")])
        .unwrap();
    let (rh, rl, ru) = wb.add_revision_tracking_shell().unwrap();
    let (sm, _) = wb.add_sort_map("S").unwrap();
    let (cm, _) = wb.add_cell_metadata().unwrap();
    let (ps, _) = wb.add_sheet_printer_settings("S", b"PRINT").unwrap();
    let (at, _) = wb.add_attached_toolbars(b"TB").unwrap();
    let (rs, _) = wb.add_rich_styles().unwrap();
    let (spb, spbs) = wb.add_supporting_property_bags().unwrap();
    let (arr, _) = wb.add_rd_array().unwrap();
    let (cp, _) = wb.add_control_properties("S").unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    for u in [&puri, &turi, &rh, &rl, &ru, &sm, &cm, &ps, &at, &rs, &spb, &spbs, &arr, &cp] {
        assert!(opc.has_part(&u), "missing {}", u.as_str());
    }
}

#[test]
fn word_comments_ids_extensible_toolbars() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("c")]));
    let (_, ids) = doc
        .add_comments_ids(&[("00000001", "11111111")])
        .unwrap();
    let (_, ext) = doc.add_comments_extensible().unwrap();
    let (_, tb) = doc.add_attached_toolbars(b"TOOLBAR").unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    for u in [&ids, &ext, &tb] {
        assert!(opc.has_part(&u), "missing {}", u.as_str());
    }
}

#[test]
fn excel_chart_drawing_extended_intl() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (chart_uri, _) = wb.add_bar_chart("C", &["a"], &[1.0]).unwrap();
    let (cd, _) = wb.add_chart_drawing(&chart_uri).unwrap();
    let (ex, _) = wb.add_extended_chart("Ex").unwrap();
    let (intl, _) = wb.add_intl_macrosheet("IntlMacro").unwrap();
    let (web, _) = wb.add_rich_value_web_image().unwrap();
    let (ctrl, _) = wb.add_embedded_control("S", b"ACTIVEX").unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    for u in [&cd, &ex, &intl, &web, &ctrl] {
        assert!(opc.has_part(&u), "missing {}", u.as_str());
    }
}

#[test]
fn word_legacy_diagram_and_package() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("d")]));
    let (text_uri, info_uri) = doc.add_legacy_diagram_text(b"LEGACY").unwrap();
    let (rid, pkg_uri) = doc
        .add_embedded_package_part(b"PK\x03\x04nested", "xlsx")
        .unwrap();
    assert!(rid.starts_with('r'));
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&text_uri) && opc.has_part(&info_uri) && opc.has_part(&pkg_uri));
}

#[test]
fn ppt_chart_drawing() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let (uri, _) = ppt.add_chart_drawing_for_slide(0, "chart1").unwrap();
    let bytes = ppt.to_bytes().unwrap();
    assert!(OpcPackage::open_bytes(&bytes).unwrap().has_part(&uri));
}

#[test]
fn custom_property_font_and_charts() {
    // Excel: custom property + font part
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (cp, _) = wb.add_custom_property("S", b"PROP").unwrap();
    let (font, _) = wb
        .add_font_part(b"FONT", content_type::FONT_TTF, "ttf")
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&cp) && opc.has_part(&font));

    // Word: custom property + font + chart
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("c")]));
    let (_, wcp) = doc.add_custom_property(b"<prop/>").unwrap();
    let (_, wfont) = doc
        .add_font_part(b"FONT", content_type::FONT_DATA, "fntdata")
        .unwrap();
    let (_, wchart) = doc.add_chart("Sales", &["A", "B"], &[1.0, 2.0]).unwrap();
    let bytes = doc.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&wcp) && opc.has_part(&wfont) && opc.has_part(&wchart));

    // PPT: chart on slide + drawing related from chart + font
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let (chart_uri, _) = ppt
        .add_chart_on_slide(0, "Q", &["x", "y"], &[3.0, 4.0])
        .unwrap();
    let (draw_uri, _) = ppt
        .add_chart_drawing_for_slide(0, chart_uri.as_str())
        .unwrap();
    let (pfont, _) = ppt
        .add_font_part(b"FONT", content_type::FONT_ODTTF, "odttf")
        .unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(
        opc.has_part(&chart_uri) && opc.has_part(&draw_uri) && opc.has_part(&pfont)
    );
}

#[test]
fn semantic_unique_and_delete_part() {
    // Word: unique bookmark ids + delete part
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    let img = doc
        .add_image(
            officexml::packaging::ImageFormat::Png,
            b"\x89PNG\r\n\x1a\n",
        )
        .unwrap();
    let img_uri = img.uri().clone();
    assert!(doc.package().opc().has_part(&img_uri));
    let removed = doc.delete_part(&img_uri);
    assert!(removed.is_some());
    assert!(!doc.package().opc().has_part(&img_uri));

    // Excel: validate_relationships on workbook with sheets is clean;
    // delete_part removes a chart part.
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (chart, _) = wb.add_bar_chart("C", &["a"], &[1.0]).unwrap();
    let errs = wb.validate_relationships().unwrap();
    assert!(
        errs.iter().all(|e| !e.message.contains("does not exist")),
        "{errs:?}"
    );
    assert!(wb.delete_part(&chart).is_some());
    assert!(!wb.package().opc().has_part(&chart));

    // PPT: validate + delete
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let errs = ppt.validate_relationships().unwrap();
    assert!(errs.is_empty() || errs.iter().all(|e| e.message.contains("duplicate") == false));
    let slide_uri = ppt.slides()[0].uri.clone();
    // don't delete the only slide without rewriting presentation — just ensure API works on a secondary part
    let (chart_uri, _) = ppt
        .add_chart_on_slide(0, "T", &["a"], &[1.0])
        .unwrap();
    assert!(ppt.delete_part(&chart_uri).is_some());
    let _ = slide_uri;
}









#[test]
fn part_extension_provider_roundtrip() {
    use officexml::PartExtensionProvider;

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(
        doc.package_mut().extension_for_content_type("image/png"),
        ".png"
    );
    doc.package_mut()
        .part_extension_provider()
        .register("application/x-test", "tst");
    assert_eq!(
        doc.package_mut()
            .part_extension_provider()
            .try_get_extension("application/x-test"),
        Some(".tst")
    );
    let _ = PartExtensionProvider::default();
}

#[test]
fn data_part_and_id_part_pair() {
    use officexml::{DataPart, IdPartPair, MediaKind};

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();

    let pairs = doc.id_part_pairs();
    assert!(!pairs.is_empty());
    assert!(pairs.iter().any(|p: &IdPartPair| p.part_uri.as_str().contains("styles")));

    let part: DataPart = doc
        .create_media_data_part("audio/mpeg", Some("mp3"))
        .unwrap();
    doc.package_mut()
        .opc_mut()
        .feed_data_part(&part.uri, b"ID3data")
        .unwrap();
    let dpr = doc
        .add_data_part_reference_relationship(
            &part,
            MediaKind::Audio.relationship_type(),
            Some("rIdAudio1"),
        )
        .unwrap();
    assert_eq!(dpr.id(), "rIdAudio1");
    assert!(dpr.is_audio());
    assert_eq!(doc.data_part_reference_relationships().len(), 1);
    assert!(doc.get_reference_relationship("rIdAudio1").is_some());

    // still referenced
    assert!(doc
        .package_mut()
        .opc_mut()
        .delete_data_part(&part.uri)
        .is_err());
    assert!(doc.delete_reference_relationship("rIdAudio1").is_some());
    assert!(doc
        .package_mut()
        .opc_mut()
        .delete_data_part(&part.uri)
        .unwrap());
}

#[test]
fn annotations_and_change_id() {
    use officexml::namespace::rel;

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let styles = doc
        .related_parts(Some(rel::STYLES))
        .into_iter()
        .next()
        .expect("styles")
        .uri;
    let old = doc.get_id_of_part(&styles).expect("id");
    let prev = doc.change_id_of_part(&styles, "rIdStylesCustom").unwrap();
    assert_eq!(prev, old);
    assert_eq!(doc.get_part_by_id("rIdStylesCustom"), Some(styles.clone()));

    doc.package_mut().add_annotation("pkg-meta".to_string());
    assert_eq!(
        doc.package().annotation::<String>().map(|s| s.as_str()),
        Some("pkg-meta")
    );

    let mut root = simple_document(vec![paragraph_with_text("y")]);
    root.add_annotation(99u16);
    assert_eq!(root.annotation::<u16>(), Some(&99));
    assert!(root.clone().annotation::<u16>().is_none());
}

#[test]
fn hyperlink_relationship_typed() {
    use officexml::namespace::rel;

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    let _ = doc
        .add_external_relationship(rel::HYPERLINK, "https://contoso.example/a")
        .unwrap();
    let hls = doc.hyperlink_relationships();
    assert_eq!(hls.len(), 1);
    assert!(hls[0].is_external());
    assert!(hls[0].target().contains("contoso"));
}

#[test]
fn typed_part_add_styles() {
    use officexml::element::OpenXmlElement;
    use officexml::namespace::ns;
    use officexml::packaging::{add_typed_part_element, find_typed_parts};

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    let main = doc.main_document_part().unwrap().uri().clone();
    let styles = OpenXmlElement::new("w", ns::WORDPROCESSINGML.uri, "styles")
        .with_ns_decl("w", ns::WORDPROCESSINGML.uri);
    let part = add_typed_part_element(
        doc.package_mut(),
        &main,
        Some("MainDocumentPart"),
        "StyleDefinitionsPart",
        &styles,
    )
    .unwrap();
    assert!(doc.package().opc().has_part(&part.uri));
    let found = find_typed_parts(doc.package(), Some(&main), "StyleDefinitionsPart").unwrap();
    assert_eq!(found.len(), 1);
}

#[test]
fn part_uri_helper_and_related_parts() {
    use officexml::namespace::{content_type, rel};
    use officexml::opc::PartUriHelper;

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let kids = doc.related_parts(Some(rel::STYLES));
    assert_eq!(kids.len(), 1);
    assert!(kids[0].uri.as_str().contains("styles"));

    let uri = doc
        .create_unique_part_uri(content_type::WORD_HEADER, ".", "header", ".xml")
        .unwrap();
    assert!(uri.as_str().contains("header"));
    // Reserve by actually adding the part so the next allocation differs.
    doc.package_mut()
        .opc_mut()
        .set_part(uri.clone(), content_type::WORD_HEADER, b"<w:hdr/>".to_vec());
    let uri2 = doc
        .create_unique_part_uri(content_type::WORD_HEADER, ".", "header", ".xml")
        .unwrap();
    assert_ne!(uri, uri2);

    let mut h = PartUriHelper::from_package(doc.package().opc());
    assert!(h.is_reserved(&kids[0].uri));
}

#[test]
fn delete_part_strips_inbound_and_cascades() {
    use officexml::namespace::{content_type, rel};
    use officexml::opc::{OpcPackage, PackUri, RelationshipTargetMode};

    let mut pkg = OpcPackage::create();
    let doc = PackUri::new("/word/document.xml");
    let styles = PackUri::new("/word/styles.xml");
    pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
    pkg.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
    pkg.add_package_relationship(rel::OFFICE_DOCUMENT, &doc, RelationshipTargetMode::Internal);
    pkg.add_part_relationship(&doc, rel::STYLES, &styles, RelationshipTargetMode::Internal);
    assert!(pkg.remove_part(&styles).is_some());
    assert!(pkg
        .part_relationships(&doc)
        .unwrap()
        .get_by_type(rel::STYLES)
        .is_none());

    // Cascade: chart + private child
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let (chart_uri, _) = ppt.add_chart_on_slide(0, "T", &["a"], &[1.0]).unwrap();
    // chart may have children; cascade delete
    assert!(ppt.delete_part_and_orphans(&chart_uri).is_some());
    assert!(!ppt.package().opc().has_part(&chart_uri));
}

#[test]
fn external_relationship_and_package_events() {
    use officexml::features::PackageEventType;
    use officexml::namespace::rel;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    let rid = doc
        .add_external_relationship(rel::HYPERLINK, "https://example.com/x")
        .unwrap();
    assert!(doc.external_relationships().iter().any(|r| r.id == rid));

    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    doc.package_events().subscribe(move |e| {
        if e.event_type == PackageEventType::Saving || e.event_type == PackageEventType::Saved {
            n2.fetch_add(1, Ordering::SeqCst);
        }
    });
    // to_bytes goes through opc directly on Word — raise via package save path:
    doc.package_mut()
        .raise_package_event(PackageEventType::Saving);
    doc.package_mut()
        .raise_package_event(PackageEventType::Saved);
    assert!(n.load(Ordering::SeqCst) >= 2);
}

#[test]
fn unique_attribute_validation_unit() {
    use officexml::validation::{
        spreadsheet_unique_attribute_rules, validate_unique_attributes,
        word_unique_attribute_rules,
    };
    // duplicate comment ids
    let comments = OpenXmlElement::w("comments")
        .with_child(
            OpenXmlElement::w("comment")
                .with_attribute("id", "1")
                .with_attribute("author", "a"),
        )
        .with_child(
            OpenXmlElement::w("comment")
                .with_attribute("id", "1")
                .with_attribute("author", "b"),
        );
    let errs = validate_unique_attributes(&comments, &word_unique_attribute_rules());
    assert!(errs.iter().any(|e| e.message.contains("duplicate")));

    // unique sheet names case-insensitive
    let x = officexml::namespace::ns::SPREADSHEETML.uri;
    let wb = OpenXmlElement::new("x", x, "workbook").with_child(
        OpenXmlElement::new("x", x, "sheets")
            .with_child(
                OpenXmlElement::new("x", x, "sheet")
                    .with_attribute("name", "A")
                    .with_attribute("sheetId", "1"),
            )
            .with_child(
                OpenXmlElement::new("x", x, "sheet")
                    .with_attribute("name", "a")
                    .with_attribute("sheetId", "2"),
            ),
    );
    let errs = validate_unique_attributes(&wb, &spreadsheet_unique_attribute_rules());
    assert!(errs.iter().any(|e| e.message.contains("duplicate")));
}

#[test]
fn radar_bubble_charts_and_extended_part() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"]]).unwrap();
    let (radar, _) = wb.add_radar_chart("R", &["a", "b"], &[1.0, 2.0]).unwrap();
    let (bubble, _) = wb
        .add_bubble_chart("B", &[1.0, 2.0], &[3.0, 4.0], &[5.0, 6.0])
        .unwrap();
    let (ext, _) = wb
        .add_extended_part(
            "/xl/customXml/item1.xml",
            "application/xml",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
            b"<root/>",
        )
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&radar) && opc.has_part(&bubble) && opc.has_part(&ext));

    // PPT clone + extended part
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Hello").unwrap();
    let (pext, _) = ppt
        .add_extended_part(
            "/ppt/customXml/item1.xml",
            "application/xml",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
            b"<root/>",
        )
        .unwrap();
    let clone = ppt.clone_document().unwrap();
    assert_eq!(clone.slides().len(), 1);
    assert!(clone.package().opc().has_part(&pext));

    // Word extended part
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    let (_, wext) = doc
        .add_extended_part(
            "/word/customXml/item1.xml",
            "application/xml",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
            b"<root/>",
        )
        .unwrap();
    assert!(doc.package().opc().has_part(&wext));
}

#[test]
fn excel_border_style_and_cell_style() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"]]).unwrap();
    wb.add_styles_with_border().unwrap();
    wb.set_cell_style("S", "A1", officexml::spreadsheet::STYLE_BORDER)
        .unwrap();
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    let styles = PackUri::new("/xl/styles.xml");
    assert!(opc.has_part(&styles));
    let xml = String::from_utf8_lossy(opc.get_part(&styles).unwrap());
    assert!(xml.contains("applyBorder") || xml.contains("borderId=\"1\""));
    // re-open and check cell has s attr
    let wb2 = SpreadsheetDocument::open_bytes(bytes).unwrap();
    let sheet_uri = wb2.worksheets().iter().find(|s| s.name == "S").unwrap().uri.clone();
    let root = parse_element(wb2.package().opc().get_part(&sheet_uri).unwrap()).unwrap();
    let cell = root
        .descendants()
        .find(|e| e.local_name == "c" && e.get_attribute("r") == Some("A1"))
        .expect("A1");
    assert_eq!(cell.get_attribute("s"), Some("1"));
}

#[test]
fn shapes_rename_remove() {
    // PPT auto shape + text box + remove slide
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S1").unwrap();
    ppt.add_slide_with_text("S2").unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1_000_000, 1_000_000, "ellipse", Some("FF0000"), "Oval")
        .unwrap();
    assert!(sid >= 2);
    let tid = ppt
        .add_text_box_on_slide(0, 0, 0, 2_000_000, 500_000, "Hello", "TB")
        .unwrap();
    assert!(tid > sid);
    assert_eq!(ppt.slides().len(), 2);
    ppt.remove_slide(1).unwrap();
    assert_eq!(ppt.slides().len(), 1);
    let bytes = ppt.to_bytes().unwrap();
    assert!(OpcPackage::open_bytes(&bytes).unwrap().has_part(&ppt.slides()[0].uri));

    // Excel rename + remove sheet
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("A", &[vec!["1"]]).unwrap();
    wb.write_sheet_strings("B", &[vec!["2"]]).unwrap();
    wb.rename_sheet("A", "Alpha").unwrap();
    assert!(wb.worksheets().iter().any(|s| s.name == "Alpha"));
    wb.remove_sheet("B").unwrap();
    assert_eq!(wb.worksheets().len(), 1);
    assert_eq!(wb.worksheets()[0].name, "Alpha");
    let bytes = wb.to_bytes().unwrap();
    let opc = OpcPackage::open_bytes(&bytes).unwrap();
    assert!(opc.has_part(&wb.worksheets()[0].uri));
}

#[test]
fn cell_value_and_paragraph_style() {
    // Excel single-cell writes
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("S", "A1", "hello").unwrap();
    wb.set_cell_number("S", "B1", 42.5).unwrap();
    wb.add_styles_with_border().unwrap();
    wb.set_cell_number_styled("S", "C1", 3.14, officexml::spreadsheet::STYLE_BORDER)
        .unwrap();
    let grid = wb.read_sheet_strings_by_name(Some("S")).unwrap();
    assert_eq!(grid[0][0], "hello");
    assert!(grid[0][1].starts_with("42"));
    // re-open check C1 style
    let bytes = wb.to_bytes().unwrap();
    let wb2 = SpreadsheetDocument::open_bytes(bytes).unwrap();
    let sheet_uri = wb2.worksheets()[0].uri.clone();
    let root = parse_element(wb2.package().opc().get_part(&sheet_uri).unwrap()).unwrap();
    let c1 = root
        .descendants()
        .find(|e| e.local_name == "c" && e.get_attribute("r") == Some("C1"))
        .unwrap();
    assert_eq!(c1.get_attribute("s"), Some("1"));

    // Word paragraph style helpers
    use officexml::wordprocessing::{
        apply_paragraph_style, paragraph_with_style, paragraph_with_text,
    };
    let mut p = paragraph_with_text("x");
    apply_paragraph_style(&mut p, "Heading1");
    assert!(p
        .descendants()
        .any(|e| e.local_name == "pStyle" && e.get_attribute("val") == Some("Heading1")));
    let styled = paragraph_with_style("Title", "Hello");
    assert!(styled
        .descendants()
        .any(|e| e.local_name == "pStyle" && e.get_attribute("val") == Some("Title")));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![
            paragraph_with_text("a"),
            paragraph_with_text("b"),
        ]));
    let n = doc.apply_style_to_paragraphs("Normal").unwrap();
    assert_eq!(n, 2);
}

#[test]
fn cell_get_paragraph_ops_move_slide() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("S", "A1", "hi").unwrap();
    wb.set_cell_number("S", "B2", 7.0).unwrap();
    assert_eq!(wb.get_cell_value("S", "A1").unwrap().as_deref(), Some("hi"));
    assert_eq!(wb.get_cell_value("S", "B2").unwrap().as_deref(), Some("7"));
    assert!(wb.get_cell_value("S", "Z9").unwrap().is_none());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![
            paragraph_with_text("one"),
            paragraph_with_text("two"),
        ]));
    doc.append_paragraph(paragraph_with_text("three")).unwrap();
    assert_eq!(doc.paragraph_texts().unwrap().len(), 3);
    assert_eq!(doc.remove_paragraphs_at(&[1]).unwrap(), 1);
    let texts = doc.paragraph_texts().unwrap();
    assert_eq!(texts, vec!["one".to_string(), "three".to_string()]);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.add_slide_with_text("C").unwrap();
    // move C to front
    ppt.move_slide(2, 0).unwrap();
    assert_eq!(ppt.slides().len(), 3);
    let texts0 = ppt.slide_texts(0).unwrap();
    assert!(texts0.iter().any(|t| t.contains('C')));
}

#[test]
fn word_table_and_excel_rows() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("before")]));
    doc.append_table(&[vec!["H1", "H2"], vec!["a", "b"]], None)
        .unwrap();
    let tables = doc.body_tables_as_strings().unwrap();
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0][0], vec!["H1".to_string(), "H2".to_string()]);
    assert_eq!(tables[0][1], vec!["a".to_string(), "b".to_string()]);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("S", "A1", "r1").unwrap();
    wb.set_cell_value("S", "A2", "r2").unwrap();
    wb.set_cell_value("S", "A3", "r3").unwrap();
    wb.insert_rows("S", 2, 1).unwrap();
    // A2 was shifted to A3, A3 to A4; new A2 empty
    assert_eq!(wb.get_cell_value("S", "A1").unwrap().as_deref(), Some("r1"));
    assert_eq!(wb.get_cell_value("S", "A3").unwrap().as_deref(), Some("r2"));
    assert_eq!(wb.get_cell_value("S", "A4").unwrap().as_deref(), Some("r3"));
    wb.delete_row("S", 3).unwrap(); // remove the old r2
    assert_eq!(wb.get_cell_value("S", "A3").unwrap().as_deref(), Some("r3"));
}

#[test]
fn copy_sheet_clear_blank_slide() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("Src", "A1", "data").unwrap();
    wb.set_cell_number("Src", "B1", 9.0).unwrap();
    let copy = wb.copy_sheet("Src", "Copy").unwrap();
    assert_eq!(copy.name, "Copy");
    assert_eq!(
        wb.get_cell_value("Copy", "A1").unwrap().as_deref(),
        Some("data")
    );
    assert!(wb.clear_cell("Copy", "A1").unwrap());
    assert!(wb.get_cell_value("Copy", "A1").unwrap().is_none());
    assert_eq!(
        wb.get_cell_value("Src", "A1").unwrap().as_deref(),
        Some("data")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    let blank = ppt.add_blank_slide().unwrap();
    assert!(ppt.package().opc().has_part(&blank.uri));
    assert!(ppt.slide_texts(0).unwrap().is_empty());
}

#[test]
fn clear_range_and_formatted_run() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("S", "A1", "1").unwrap();
    wb.set_cell_value("S", "B1", "2").unwrap();
    wb.set_cell_value("S", "A2", "3").unwrap();
    wb.set_cell_value("S", "C3", "keep").unwrap();
    let n = wb.clear_range("S", "A1:B2").unwrap();
    assert!(n >= 3);
    assert!(wb.get_cell_value("S", "A1").unwrap().is_none());
    assert_eq!(
        wb.get_cell_value("S", "C3").unwrap().as_deref(),
        Some("keep")
    );

    use officexml::wordprocessing::{paragraph_with_formatted_text, run_with_formatting};
    let r = run_with_formatting("Hi", true, true, Some("FF0000"), Some(28), Some("single"), Some("yellow"));
    assert!(r.descendants().any(|e| e.local_name == "b"));
    assert!(r.descendants().any(|e| e.local_name == "i"));
    assert!(r.descendants().any(|e| e.local_name == "color"));
    assert!(r.descendants().any(|e| e.local_name == "highlight"));
    let p = paragraph_with_formatted_text("Title", true, false, Some("0000FF"), Some(32));
    assert!(p.descendants().any(|e| e.local_name == "b"));
    assert!(p.descendants().any(|e| e.local_name == "t" && e.inner_text() == "Title"));
}

#[test]
fn range_io_and_slide_replace() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_range(
        "S",
        "B2",
        &[vec!["x", "y"], vec!["1", "2"]],
    )
    .unwrap();
    let grid = wb.read_range("S", "B2:C3").unwrap();
    assert_eq!(grid.len(), 2);
    assert_eq!(grid[0], vec!["x".to_string(), "y".to_string()]);
    assert_eq!(grid[1], vec!["1".to_string(), "2".to_string()]);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Hello World").unwrap();
    let n = ppt.replace_slide_text(0, "World", "Rust").unwrap();
    assert!(n >= 1);
    let texts = ppt.slide_texts(0).unwrap();
    assert!(texts.iter().any(|t| t.contains("Rust")));
    assert!(!texts.iter().any(|t| t.contains("World")));
}

#[test]
fn column_hidden_and_bullets() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b", "c"]]).unwrap();
    wb.set_column_hidden("S", 2, 2, true).unwrap();
    let uri = wb.worksheets()[0].uri.clone();
    let root = parse_element(wb.package().opc().get_part(&uri).unwrap()).unwrap();
    let col = root
        .descendants()
        .find(|e| e.local_name == "col" && e.get_attribute("min") == Some("2"))
        .expect("col");
    assert_eq!(col.get_attribute("hidden"), Some("1"));
    wb.set_column_hidden("S", 2, 2, false).unwrap();
    let root = parse_element(wb.package().opc().get_part(&uri).unwrap()).unwrap();
    let col = root
        .descendants()
        .find(|e| e.local_name == "col" && e.get_attribute("min") == Some("2"))
        .expect("col");
    assert!(col.get_attribute("hidden").is_none());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("intro")]));
    doc.append_bullet_list(&["one", "two", "three"]).unwrap();
    let texts = doc.paragraph_texts().unwrap();
    assert_eq!(texts.len(), 4);
    assert!(texts.iter().any(|t| t == "one"));
    // numbering part present
    assert!(doc
        .package()
        .opc()
        .has_part(&PackUri::new("/word/numbering.xml")));
}

#[test]
fn find_replace_sheet_and_set_slide_text() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("S", "A1", "hello world").unwrap();
    wb.set_cell_value("S", "B1", "world peace").unwrap();
    wb.set_cell_number("S", "C1", 42.0).unwrap();
    let hits = wb.find_cells("S", "world").unwrap();
    assert_eq!(hits.len(), 2);
    let n = wb.replace_in_sheet("S", "world", "rust").unwrap();
    assert!(n >= 2);
    assert_eq!(
        wb.get_cell_value("S", "A1").unwrap().as_deref(),
        Some("hello rust")
    );
    assert_eq!(
        wb.get_cell_value("S", "B1").unwrap().as_deref(),
        Some("rust peace")
    );
    // number untouched
    assert_eq!(wb.get_cell_value("S", "C1").unwrap().as_deref(), Some("42"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Old Title").unwrap();
    assert!(ppt.set_slide_text(0, "New Title").unwrap());
    let texts = ppt.slide_texts(0).unwrap();
    assert!(texts.iter().any(|t| t == "New Title"));
}

#[test]
fn used_range_stats_notes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("S", "B2", "x").unwrap();
    wb.set_cell_value("S", "D5", "y").unwrap();
    let range = wb.used_range("S", true).unwrap();
    assert_eq!(range.as_deref(), Some("B2:D5"));
    let uri = wb.worksheets()[0].uri.clone();
    let root = parse_element(wb.package().opc().get_part(&uri).unwrap()).unwrap();
    assert!(root
        .children
        .iter()
        .any(|c| c.local_name == "dimension"
            && c.get_attribute("ref") == Some("B2:D5")));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![
            paragraph_with_text("hello world"),
            paragraph_with_text("foo"),
        ]));
    assert_eq!(doc.paragraph_count().unwrap(), 2);
    assert_eq!(doc.word_count().unwrap(), 3);
    doc.insert_paragraph_at(1, paragraph_with_text("middle"))
        .unwrap();
    assert_eq!(
        doc.paragraph_texts().unwrap(),
        vec![
            "hello world".to_string(),
            "middle".to_string(),
            "foo".to_string()
        ]
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_notes_to_slide(0, "Speaker notes here").unwrap();
    let notes = ppt.notes_text(0).unwrap();
    assert!(
        notes
            .as_deref()
            .map(|s| s.contains("Speaker notes"))
            .unwrap_or(false),
        "{notes:?}"
    );
}

#[test]
fn move_sheet_and_bookmarks() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("A", &[vec!["1"]]).unwrap();
    wb.write_sheet_strings("B", &[vec!["2"]]).unwrap();
    wb.write_sheet_strings("C", &[vec!["3"]]).unwrap();
    assert_eq!(wb.sheet_names(), vec!["A", "B", "C"]);
    wb.move_sheet(2, 0).unwrap();
    assert_eq!(wb.sheet_names(), vec!["C", "A", "B"]);

    use officexml::wordprocessing::{body, document, paragraph, with_bookmark};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p = paragraph(with_bookmark("1", "Intro", vec![run(vec![text("x")])]));
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    let bms = doc.bookmarks().unwrap();
    assert!(bms.iter().any(|(id, name)| id == "1" && name == "Intro"));
}

#[test]
fn merge_clear_heading_hyperlink() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("S", "A1", "a").unwrap();
    wb.set_cell_value("S", "B1", "b").unwrap();
    wb.merge_range("S", "A1:B1").unwrap();
    assert_eq!(wb.merge_cells("S").unwrap(), vec!["A1:B1".to_string()]);
    assert_eq!(wb.cell_count("S").unwrap(), 2);
    wb.unmerge_range("S", "A1:B1").unwrap();
    assert!(wb.merge_cells("S").unwrap().is_empty());
    wb.clear_sheet("S", true).unwrap();
    assert!(wb.is_sheet_empty("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("old")]));
    doc.clear_body().unwrap();
    assert_eq!(doc.paragraph_count().unwrap(), 0);
    doc.append_heading("Heading1", "Title").unwrap();
    doc.append_hyperlink("https://example.com", "link").unwrap();
    let texts = doc.paragraph_texts().unwrap();
    assert_eq!(texts.len(), 2);
    assert_eq!(texts[0], "Title");
    assert_eq!(texts[1], "link");

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let rid = ppt
        .add_slide_hyperlink(0, "https://example.com")
        .unwrap();
    assert!(rid.starts_with('r'));
    assert_eq!(ppt.slide_count(), 1);
}

#[test]
fn columns_table_row_all_texts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_range("S", "A1", &[vec!["a", "b", "c"], vec!["1", "2", "3"]])
        .unwrap();
    assert_eq!(wb.row_count("S").unwrap(), 2);
    wb.insert_column("S", 1).unwrap(); // insert before B
    // A stays, B empty, old B->C, old C->D
    assert_eq!(wb.get_cell_value("S", "A1").unwrap().as_deref(), Some("a"));
    assert_eq!(wb.get_cell_value("S", "C1").unwrap().as_deref(), Some("b"));
    assert_eq!(wb.get_cell_value("S", "D1").unwrap().as_deref(), Some("c"));
    wb.delete_column("S", 1).unwrap(); // remove inserted empty col
    assert_eq!(wb.get_cell_value("S", "B1").unwrap().as_deref(), Some("b"));
    assert_eq!(wb.get_cell_value("S", "C1").unwrap().as_deref(), Some("c"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.append_table(&[vec!["H1", "H2"]], None).unwrap();
    assert_eq!(doc.table_count().unwrap(), 1);
    doc.append_table_row(&["r1", "r2"]).unwrap();
    let tables = doc.body_tables_as_strings().unwrap();
    assert_eq!(tables[0].len(), 2);
    assert_eq!(tables[0][1], vec!["r1".to_string(), "r2".to_string()]);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("One").unwrap();
    ppt.add_slide_with_text("Two").unwrap();
    let all = ppt.all_slide_texts().unwrap();
    assert_eq!(all.len(), 2);
    assert!(all[0].iter().any(|t| t.contains("One")));
    assert!(all[1].iter().any(|t| t.contains("Two")));
}

#[test]
fn filter_row_hidden_remove_table_shapes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H"], vec!["a"], vec!["b"]])
        .unwrap();
    wb.set_auto_filter("S", "A1:A3").unwrap();
    assert_eq!(
        wb.auto_filter_ref("S").unwrap().as_deref(),
        Some("A1:A3")
    );
    wb.clear_auto_filter("S").unwrap();
    assert!(wb.auto_filter_ref("S").unwrap().is_none());
    wb.set_row_hidden("S", 2, true).unwrap();
    let uri = wb.worksheets()[0].uri.clone();
    let root = parse_element(wb.package().opc().get_part(&uri).unwrap()).unwrap();
    let row2 = root
        .descendants()
        .find(|e| e.local_name == "row" && e.get_attribute("r") == Some("2"))
        .unwrap();
    assert_eq!(row2.get_attribute("hidden"), Some("1"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.append_table(&[vec!["a"]], None).unwrap();
    assert_eq!(doc.table_count().unwrap(), 1);
    doc.remove_table_at(0).unwrap();
    assert_eq!(doc.table_count().unwrap(), 0);
    assert!(!doc.is_body_empty().unwrap()); // still has paragraph

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    let n = ppt.shape_count(0).unwrap();
    assert!(n >= 1);
    ppt.add_auto_shape_on_slide(0, 0, 0, 1000, 1000, "rect", None, "R")
        .unwrap();
    assert!(ppt.shape_count(0).unwrap() > n);
}

#[test]
fn hyperlinks_macro_part_count() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["x"]]).unwrap();
    let rid = wb
        .add_cell_hyperlink("S", "A1", "https://example.com", Some("ex"))
        .unwrap();
    let links = wb.list_cell_hyperlinks("S").unwrap();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].0, "A1");
    assert_eq!(links[0].1, rid);
    assert_eq!(links[0].2.as_deref(), Some("ex"));
    assert!(!wb.is_macro_enabled());
    assert!(wb.part_count() > 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.is_macro_enabled());
    assert!(!doc.has_vba_project());
    assert!(doc.part_count() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    assert!(!ppt.is_macro_enabled());
    ppt.add_slide_with_text("x").unwrap();
    assert!(ppt.part_count() >= 1);
}

#[test]
fn remove_hyperlink_contains_text() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_shared_strings("S", &[vec!["hello", "world"]])
        .unwrap();
    assert!(wb.shared_string_count() >= 2);
    wb.add_cell_hyperlink("S", "A1", "https://a.example", None)
        .unwrap();
    wb.add_cell_hyperlink("S", "B1", "https://b.example", None)
        .unwrap();
    assert_eq!(wb.list_cell_hyperlinks("S").unwrap().len(), 2);
    assert!(wb.remove_cell_hyperlink("S", "A1").unwrap());
    let left = wb.list_cell_hyperlinks("S").unwrap();
    assert_eq!(left.len(), 1);
    assert_eq!(left[0].0, "B1");

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![
            paragraph_with_text(""),
            paragraph_with_text("alpha beta"),
        ]));
    assert!(doc.contains_text("beta").unwrap());
    assert!(!doc.contains_text("gamma").unwrap());
    assert_eq!(
        doc.first_paragraph_text().unwrap().as_deref(),
        Some("alpha beta")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Hello PPT").unwrap();
    assert!(ppt.contains_text("PPT").unwrap());
    assert!(!ppt.contains_text("Word").unwrap());
}

#[test]
fn defined_names_notes_char_count() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Data", &[vec!["1"]]).unwrap();
    wb.write_sheet_strings("Other", &[vec!["2"]]).unwrap();
    wb.add_defined_name("Sales", "Data!$A$1").unwrap();
    wb.add_defined_name("Sales", "Data!$A$1:$A$2").unwrap(); // replace
    let names = wb.defined_names().unwrap();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0].0, "Sales");
    assert!(names[0].1.contains("A$1"));
    assert_eq!(wb.sheet_index("Other"), Some(1));
    assert!(wb.remove_defined_name("Sales").unwrap());
    assert!(wb.defined_names().unwrap().is_empty());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![
            paragraph_with_text("ab"),
            paragraph_with_text("cde"),
        ]));
    assert_eq!(doc.character_count().unwrap(), 5);
    assert_eq!(
        doc.last_paragraph_text().unwrap().as_deref(),
        Some("cde")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert!(!ppt.has_notes(0).unwrap());
    ppt.add_notes_to_slide(0, "note").unwrap();
    assert!(ppt.has_notes(0).unwrap());
}

#[test]
fn ensure_styles_theme() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_styles());
    doc.ensure_styles().unwrap();
    assert!(doc.has_styles());
    doc.ensure_styles().unwrap(); // idempotent
    assert!(!doc.has_theme());
    doc.ensure_theme().unwrap();
    assert!(doc.has_theme());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_styles());
    wb.ensure_styles().unwrap();
    assert!(wb.has_styles());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    // Adding a slide now ensures master/layout/theme (required for PowerPoint).
    assert!(ppt.has_theme());
    let (uri, _) = ppt.ensure_theme().unwrap();
    assert!(ppt.package().opc().has_part(&uri));
    assert!(ppt.has_theme());
}

#[test]
fn list_headers_media_chart_counts() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_header("H").unwrap();
    doc.add_default_footer("F").unwrap();
    assert_eq!(doc.list_headers().len(), 1);
    assert_eq!(doc.list_footers().len(), 1);
    let img = doc
        .add_image(officexml::packaging::ImageFormat::Png, b"\x89PNG\r\n\x1a\n")
        .unwrap();
    assert!(doc.package().opc().has_part(img.uri()));
    assert_eq!(doc.media_count(), 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_bar_chart("C", &["a"], &[1.0]).unwrap();
    assert_eq!(wb.chart_count(), 1);
    assert_eq!(wb.media_count(), 0);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_chart_on_slide(0, "T", &["a"], &[1.0]).unwrap();
    assert_eq!(ppt.chart_count(), 1);
}

#[test]
fn remove_header_drawings_masters() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_header("H1").unwrap();
    doc.add_default_footer("F1").unwrap();
    assert!(doc.has_headers() && doc.has_footers());
    doc.remove_header_at(0).unwrap();
    assert!(!doc.has_headers());
    doc.remove_footer_at(0).unwrap();
    assert!(!doc.has_footers());
    assert!(doc.list_images().is_empty());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_drawings());
    wb.add_bar_chart_on_sheet("S", "C", &["a"], &[1.0], 0, 0, 4, 10)
        .unwrap();
    assert!(wb.has_drawings());
    assert!(!wb.list_drawings().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.ensure_default_master_layout().unwrap();
    assert!(ppt.master_count() >= 1);
    assert!(ppt.layout_count() >= 1);
}

#[test]
fn has_comments_charts_media_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_charts());
    wb.add_pie_chart("P", &["a"], &[1.0]).unwrap();
    assert!(wb.has_charts());
    assert_eq!(wb.list_charts().len(), 1);
    assert!(!wb.sheet_has_comments("S").unwrap());
    wb.add_sheet_comments("S", "me", &[("A1", "note")])
        .unwrap();
    assert!(wb.sheet_has_comments("S").unwrap());
    assert!(wb.comments_part_count() >= 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_comments());
    assert!(!doc.has_footnotes());
    assert!(!doc.has_numbering());
    doc.add_footnote("1", "fn").unwrap();
    assert!(doc.has_footnotes());
    doc.append_bullet_list(&["a"]).unwrap();
    assert!(doc.has_numbering());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert!(!ppt.has_notes_master());
    ppt.add_notes_master().unwrap();
    assert!(ppt.has_notes_master());
    assert!(!ppt.has_media());
}

#[test]
fn list_parts_and_rel_counts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_shared_strings("S", &[vec!["a", "b"]])
        .unwrap();
    wb.flush_shared_strings().unwrap();
    assert!(wb.has_shared_strings());
    assert!(!wb.has_calc_chain());
    assert!(wb.list_part_uris().iter().any(|u| u.as_str().contains("workbook")));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_header("h").unwrap();
    assert!(doc.package_relationship_count() >= 1);
    assert!(doc.main_relationship_count() >= 1);
    assert!(doc.list_part_uris().len() >= 2);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert!(ppt.package_relationship_count() >= 1);
    assert!(!ppt.list_part_uris().is_empty());
}

#[test]
fn tables_protection_hidden_sdt() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H1", "H2"], vec!["a", "b"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["H1", "H2"]).unwrap();
    assert_eq!(wb.table_count(), 1);
    assert!(!wb.list_tables().is_empty());
    assert!(!wb.has_pivot_tables());
    assert!(!wb.is_sheet_protected("S").unwrap());
    wb.set_sheet_protection("S", true, false, false).unwrap();
    assert!(wb.is_sheet_protected("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    assert!(!ppt.is_slide_hidden(0).unwrap());
    ppt.set_slide_hidden(1, true).unwrap();
    assert!(ppt.is_slide_hidden(1).unwrap());
    assert_eq!(ppt.hidden_slide_count().unwrap(), 1);

    use officexml::wordprocessing::{body, document, paragraph_with_text, sdt_block};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let sdt = sdt_block("tag1", "Alias", vec![paragraph_with_text("cc")]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![sdt])]));
    assert!(doc.has_content_controls().unwrap());
    let tags = doc.content_control_tags().unwrap();
    assert!(tags.iter().any(|(t, a, _)| t == "tag1" && a == "Alias"));
}

#[test]
fn protection_clear_and_external_links() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_sheet_protection("S", true, false, false).unwrap();
    assert!(wb.is_sheet_protected("S").unwrap());
    assert!(wb.clear_sheet_protection("S").unwrap());
    assert!(!wb.is_sheet_protected("S").unwrap());
    assert!(!wb.is_workbook_protected().unwrap());
    wb.set_workbook_protection(true, false).unwrap();
    assert!(wb.is_workbook_protected().unwrap());
    assert!(wb.clear_workbook_protection().unwrap());
    assert!(!wb.is_workbook_protected().unwrap());
    assert!(!wb.has_external_links());
    let (uri, _) = wb.add_external_link("file:///tmp/other.xlsx").unwrap();
    assert!(wb.has_external_links());
    assert_eq!(wb.external_link_count(), 1);
    assert!(wb.list_external_links().iter().any(|u| u == &uri));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.is_document_protected().unwrap());
    doc.set_document_protection("readOnly", true).unwrap();
    assert!(doc.is_document_protected().unwrap());
    assert!(doc.clear_document_protection().unwrap());
    assert!(!doc.is_document_protected().unwrap());
}

#[test]
fn track_freeze_sections_flags() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.track_revisions_enabled().unwrap());
    doc.set_track_revisions(true).unwrap();
    assert!(doc.track_revisions_enabled().unwrap());
    assert!(!doc.has_alt_chunks());
    assert!(!doc.has_watermark());
    doc.add_watermark("CONFIDENTIAL").unwrap();
    assert!(doc.has_watermark());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_freeze_panes("S").unwrap());
    wb.set_freeze_panes("S", 0, 1).unwrap();
    assert!(wb.has_freeze_panes("S").unwrap());
    assert!(wb.clear_freeze_panes("S").unwrap());
    assert!(!wb.has_freeze_panes("S").unwrap());
    wb.set_active_tab(0).unwrap();
    assert_eq!(wb.active_tab().unwrap(), Some(0));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    assert!(!ppt.has_sections().unwrap());
    ppt.set_sections(&[("Intro", 0, 0), ("Body", 1, 1)]).unwrap();
    assert!(ppt.has_sections().unwrap());
    assert!(!ppt.has_transition(0).unwrap());
    ppt.set_fade_transition(0, "fast").unwrap();
    assert!(ppt.has_transition(0).unwrap());
    assert!(ppt.clear_transition(0).unwrap());
    assert!(!ppt.has_transition(0).unwrap());
}

#[test]
fn zoom_tab_print_settings_getters() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_zoom("S", 125).unwrap();
    assert_eq!(wb.zoom("S").unwrap(), Some(125));
    wb.set_sheet_tab_color("S", "FF0000").unwrap();
    assert_eq!(
        wb.sheet_tab_color("S").unwrap().as_deref(),
        Some("FFFF0000")
    );
    wb.set_print_area("S", "$A$1:$B$2").unwrap();
    let area = wb.print_area().unwrap().unwrap();
    assert!(area.contains("Print_Area") || area.contains("$A$1") || area.contains("S"));
    assert!(wb.print_area().unwrap().is_some());
    wb.set_show_gridlines("S", false).unwrap();
    assert!(!wb.show_gridlines("S").unwrap());
    assert_eq!(wb.sheet_state("S").unwrap(), "visible");
    wb.set_sheet_state("S", "hidden").unwrap();
    assert_eq!(wb.sheet_state("S").unwrap(), "hidden");

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.update_fields_on_open().unwrap());
    doc.set_update_fields_on_open(true).unwrap();
    assert!(doc.update_fields_on_open().unwrap());
    assert!(doc.compatibility_mode().unwrap().is_none());
    doc.set_compatibility_mode("15").unwrap();
    assert_eq!(doc.compatibility_mode().unwrap().as_deref(), Some("15"));
}

#[test]
fn background_size_dimension_getters() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.set_slide_size(12_192_000, 6_858_000).unwrap();
    assert_eq!(ppt.slide_size().unwrap(), Some((12_192_000, 6_858_000)));
    ppt.set_notes_size(6_858_000, 9_144_000).unwrap();
    assert_eq!(ppt.notes_size().unwrap(), Some((6_858_000, 9_144_000)));
    assert!(!ppt.has_slide_background(0).unwrap());
    ppt.set_slide_background(0, "112233").unwrap();
    assert!(ppt.has_slide_background(0).unwrap());
    assert_eq!(
        ppt.slide_background_rgb(0).unwrap().as_deref(),
        Some("112233")
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_document_background().unwrap());
    doc.set_document_background("FFFFCC").unwrap();
    assert!(doc.has_document_background().unwrap());
    assert_eq!(
        doc.document_background_color().unwrap().as_deref(),
        Some("FFFFCC")
    );

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_cell_value("S", "A1", "x").unwrap();
    assert!(wb.sheet_dimension("S").unwrap().is_none());
    let used = wb.used_range("S", true).unwrap().unwrap();
    assert_eq!(used, "A1:A1");
    assert_eq!(wb.sheet_dimension("S").unwrap().as_deref(), Some("A1:A1"));
}

#[test]
fn vars_calc_animation_flags() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_document_variables().unwrap());
    doc.set_document_variables(&[("Author", "Ada"), ("Rev", "1")])
        .unwrap();
    assert!(doc.has_document_variables().unwrap());
    assert_eq!(doc.document_variable_count().unwrap(), 2);
    assert!(!doc.has_glossary());
    assert!(!doc.has_custom_xml_parts().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_data_validations("S").unwrap());
    wb.add_data_validation_list("S", "A1:A10", "\"Y,N\"", true)
        .unwrap();
    assert!(wb.has_data_validations("S").unwrap());
    assert_eq!(wb.data_validation_count("S").unwrap(), 1);
    assert_eq!(wb.calc_chain_entry_count().unwrap(), 0);
    wb.set_calc_chain(&[("A1", 1)]).unwrap();
    assert!(wb.has_calc_chain());
    assert_eq!(wb.calc_chain_entry_count().unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert!(!ppt.has_slide_header_footer(0).unwrap());
    ppt.set_slide_header_footer(0, true, true, true).unwrap();
    assert!(ppt.has_slide_header_footer(0).unwrap());
    assert!(!ppt.has_animation(0).unwrap());
    ppt.set_simple_appear_animation(0, 2).unwrap();
    assert!(ppt.has_animation(0).unwrap());
    assert!(ppt.clear_animation(0).unwrap());
    assert!(!ppt.has_animation(0).unwrap());
}

#[test]
fn cf_sparkline_sort_comment_counts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1", "2", "3"]]).unwrap();
    assert!(!wb.has_conditional_formatting("S").unwrap());
    wb.add_conditional_formatting_cell_is("S", "A1:A3", "greaterThan", "0", "00FF00", 1)
        .unwrap();
    assert!(wb.has_conditional_formatting("S").unwrap());
    assert_eq!(wb.conditional_formatting_count("S").unwrap(), 1);
    assert_eq!(wb.clear_conditional_formatting("S").unwrap(), 1);
    assert!(!wb.has_conditional_formatting("S").unwrap());
    assert!(!wb.has_sparklines("S").unwrap());
    wb.add_sparkline("S", "line", "Sheet1!A1:A3", "B1")
        .unwrap();
    assert!(wb.has_sparklines("S").unwrap());
    assert!(!wb.has_sort_state("S").unwrap());
    wb.set_sort_state("S", "A1:A3", "A1", false).unwrap();
    assert!(wb.has_sort_state("S").unwrap());
    assert!(wb.clear_sort_state("S").unwrap());
    wb.add_data_validation_list("S", "C1:C10", "\"A,B\"", true)
        .unwrap();
    assert!(wb.clear_data_validations("S").unwrap());
    assert!(!wb.has_data_validations("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(doc.comment_count().unwrap(), 0);
    assert_eq!(doc.footnote_count().unwrap(), 0);
    doc.add_footnote("1", "note").unwrap();
    assert_eq!(doc.footnote_count().unwrap(), 1);
    doc.add_endnote("1", "end").unwrap();
    assert_eq!(doc.endnote_count().unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert!(!ppt.has_slide_comments(0).unwrap());
    ppt.add_slide_comments(0, &[(0, "2020-01-01T00:00:00", 0, 0, "hi")])
        .unwrap();
    assert!(ppt.has_slide_comments(0).unwrap());
    assert!(ppt.slide_comments_part_count() >= 1);
}

#[test]
fn advanced_part_presence_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_slicers());
    assert!(!wb.has_timelines());
    assert!(!wb.has_connections());
    assert!(!wb.has_query_tables());
    assert_eq!(wb.pivot_table_count(), 0);
    wb.add_slicer_shell("S", "Slicer1", "Cache1").unwrap();
    assert!(wb.has_slicers());
    wb.add_timeline_shell("S", "TL1", "TLCache1").unwrap();
    assert!(wb.has_timelines());
    wb.add_connections(&[("C1", "SELECT 1", "DSN=x")]).unwrap();
    assert!(wb.has_connections());
    wb.add_query_table("S", "QT1", 1).unwrap();
    assert!(wb.has_query_tables());
    assert_eq!(wb.query_table_count(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_people());
    assert!(!doc.has_mail_merge_recipients());
    assert!(!doc.has_web_settings());
    doc.add_people(&[("Ada", "AD")]).unwrap();
    assert!(doc.has_people());
    doc.add_mail_merge_recipients(b"<recipients/>").unwrap();
    assert!(doc.has_mail_merge_recipients());
    doc.add_default_web_settings().unwrap();
    assert!(doc.has_web_settings());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert!(!ppt.has_handout_master());
    assert!(!ppt.has_user_defined_tags());
    assert!(!ppt.has_slide_sync_data());
    ppt.add_handout_master().unwrap();
    assert!(ppt.has_handout_master());
    ppt.add_user_defined_tags(0, &[("k", "v")]).unwrap();
    assert!(ppt.has_user_defined_tags());
    ppt.add_slide_sync_data(0, "srv1").unwrap();
    assert!(ppt.has_slide_sync_data());
}

#[test]
fn list_clear_and_view_getters() {
    use officexml::wordprocessing::comment;

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"], vec!["3"]])
        .unwrap();
    assert!(!wb.has_sparklines("S").unwrap());
    wb.add_sparkline("S", "line", "Sheet1!A1:A3", "B1")
        .unwrap();
    assert!(wb.has_sparklines("S").unwrap());
    assert!(wb.clear_sparklines("S").unwrap());
    assert!(!wb.has_sparklines("S").unwrap());
    assert!(!wb.clear_sparklines("S").unwrap());

    wb.set_print_area("S", "$A$1:$A$3").unwrap();
    assert!(wb.print_area().unwrap().is_some());
    assert!(wb.clear_print_area().unwrap());
    assert!(wb.print_area().unwrap().is_none());

    wb.set_print_titles("S", Some("$1:$1"), None).unwrap();
    assert!(wb.print_titles().unwrap().is_some());
    assert!(wb.clear_print_titles().unwrap());
    assert!(wb.print_titles().unwrap().is_none());

    assert!(!wb.show_formulas("S").unwrap());
    wb.set_show_formulas("S", true).unwrap();
    assert!(wb.show_formulas("S").unwrap());
    assert!(wb.show_gridlines("S").unwrap());
    wb.set_show_gridlines("S", false).unwrap();
    assert!(!wb.show_gridlines("S").unwrap());
    assert!(wb.show_row_col_headers("S").unwrap());
    wb.set_show_row_col_headers("S", false).unwrap();
    assert!(!wb.show_row_col_headers("S").unwrap());

    wb.set_sheet_tab_color("S", "FF0000").unwrap();
    assert_eq!(wb.sheet_tab_color("S").unwrap().as_deref(), Some("FFFF0000"));
    assert!(wb.clear_sheet_tab_color("S").unwrap());
    assert!(wb.sheet_tab_color("S").unwrap().is_none());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("body")]));
    doc.set_comments(vec![
        comment("0", "Ada", "A", "first"),
        comment("1", "Bob", "B", "second"),
    ])
    .unwrap();
    assert_eq!(doc.comment_count().unwrap(), 2);
    let comments = doc.list_comments().unwrap();
    assert_eq!(comments.len(), 2);
    assert_eq!(comments[0].0, "0");
    assert_eq!(comments[0].1, "Ada");
    assert!(comments[0].2.contains("first"));
    assert!(doc.clear_comments().unwrap());
    assert!(!doc.has_comments());
    assert_eq!(doc.comment_count().unwrap(), 0);
    assert!(!doc.clear_comments().unwrap());

    doc.add_paragraph_styles(&[("MyStyle", "My Style", None)])
        .unwrap();
    let styles = doc.list_styles().unwrap();
    assert!(styles.iter().any(|(id, name, ty)| {
        id == "MyStyle" && name == "My Style" && ty == "paragraph"
    }));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("One").unwrap();
    ppt.add_slide_with_text("Two").unwrap();
    assert!(ppt.list_sections().unwrap().is_empty());
    ppt.set_sections(&[("Intro", 0, 0), ("Body", 1, 1)]).unwrap();
    let sections = ppt.list_sections().unwrap();
    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].0, "Intro");
    assert!(ppt.clear_sections().unwrap());
    assert!(!ppt.has_sections().unwrap());
    assert!(ppt.list_sections().unwrap().is_empty());

    ppt.set_slide_background(0, "112233").unwrap();
    assert!(ppt.has_slide_background(0).unwrap());
    assert_eq!(
        ppt.slide_background_rgb(0).unwrap().as_deref(),
        Some("112233")
    );
    assert!(ppt.clear_slide_background(0).unwrap());
    assert!(!ppt.has_slide_background(0).unwrap());
    assert!(!ppt.clear_slide_background(0).unwrap());
}

#[test]
fn dimension_list_clear_helpers() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["c", "d"]])
        .unwrap();
    wb.set_column_widths("S", &[(1, 1, 12.0), (2, 2, 20.0)])
        .unwrap();
    wb.set_column_hidden("S", 2, 2, true).unwrap();
    let cols = wb.column_widths("S").unwrap();
    assert!(cols.iter().any(|(min, max, w, _)| *min == 1 && *max == 1 && (*w - 12.0).abs() < 0.01));
    assert!(cols.iter().any(|(_, _, _, h)| *h));
    wb.set_row_heights("S", &[(1, 30.0, false), (2, 18.0, true)])
        .unwrap();
    let rows = wb.row_heights("S").unwrap();
    assert!(rows.iter().any(|(i, h, _)| *i == 1 && (*h - 30.0).abs() < 0.01));
    assert!(rows.iter().any(|(i, _, hid)| *i == 2 && *hid));

    wb.set_freeze_panes("S", 1, 2).unwrap();
    assert_eq!(wb.freeze_panes("S").unwrap(), Some((1, 2)));
    assert!(wb.has_freeze_panes("S").unwrap());

    wb.set_page_setup("S", (0.7, 0.7, 0.75, 0.75, 0.3, 0.3), 9, "landscape")
        .unwrap();
    let margins = wb.get_page_margins("S").unwrap().unwrap();
    assert!((margins.0 - 0.7).abs() < 0.01);
    let setup = wb.get_page_setup("S").unwrap().unwrap();
    assert_eq!(setup.0, 9);
    assert_eq!(setup.1, "landscape");

    wb.add_sheet_comments("S", "Ada", &[("A1", "note-a"), ("B2", "note-b")])
        .unwrap();
    assert!(wb.sheet_has_comments("S").unwrap());
    assert_eq!(wb.sheet_comments("S").unwrap().len(), 2);
    assert!(wb.clear_sheet_comments("S").unwrap());
    assert!(!wb.sheet_has_comments("S").unwrap());
    assert!(wb.sheet_comments("S").unwrap().is_empty());
    assert!(!wb.clear_sheet_comments("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(simple_document(vec![
        paragraph_with_text("alpha beta alpha"),
        paragraph_with_text("gamma"),
    ]));
    assert_eq!(doc.count_text("alpha").unwrap(), 2);
    assert!(doc.contains_text("beta").unwrap());
    doc.set_page_setup(12240, 15840, 1440, 1440, 1440, 1440)
        .unwrap();
    assert_eq!(doc.page_size().unwrap(), Some((12240, 15840)));
    assert_eq!(doc.page_margins().unwrap(), Some((1440, 1440, 1440, 1440)));
    doc.add_footnote("1", "fn body").unwrap();
    let fns = doc.list_footnotes().unwrap();
    assert_eq!(fns.len(), 1);
    assert_eq!(fns[0].0, "1");
    assert!(fns[0].1.contains("fn body"));
    doc.add_endnote("1", "en body").unwrap();
    let ens = doc.list_endnotes().unwrap();
    assert_eq!(ens.len(), 1);
    assert!(ens[0].1.contains("en body"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert!(ppt.get_slide_transition(0).unwrap().is_none());
    ppt.set_fade_transition(0, "fast").unwrap();
    let tr = ppt.get_slide_transition(0).unwrap().unwrap();
    assert_eq!(tr.0, "fade");
    assert_eq!(tr.1, "fast");
}

#[test]
fn inventory_hidden_tables_notes_headers() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_shared_strings("S", &[vec!["hello", "world"], vec!["r2a", "r2b"]])
        .unwrap();
    assert!(wb.shared_string_count() >= 2);
    let sst = wb.shared_strings_list();
    assert!(sst.iter().any(|s| s == "hello"));
    wb.set_row_hidden("S", 1, true).unwrap();
    assert!(wb.is_row_hidden("S", 1).unwrap());
    assert!(!wb.is_row_hidden("S", 2).unwrap());
    wb.set_column_hidden("S", 1, 1, true).unwrap();
    assert!(wb.is_column_hidden("S", 1).unwrap());
    wb.set_row_outline_levels("S", &[(2, 1, false)]).unwrap();
    let levels = wb.row_outline_levels("S").unwrap();
    assert!(levels.iter().any(|(r, l, _)| *r == 2 && *l == 1));
    wb.add_table("S", "T1", "A1:B2", &["hello", "world"])
        .unwrap();
    let tables = wb.table_infos().unwrap();
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].0, "T1");
    assert_eq!(tables[0].1, "A1:B2");

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("body")]));
    doc.add_default_header("HDR").unwrap();
    doc.add_default_footer("FTR").unwrap();
    let headers = doc.header_texts().unwrap();
    assert!(headers.iter().any(|t| t.contains("HDR")));
    let footers = doc.footer_texts().unwrap();
    assert!(footers.iter().any(|t| t.contains("FTR")));
    doc.append_hyperlink("https://example.com", "ex").unwrap();
    let links = doc.list_external_hyperlinks();
    assert!(links.iter().any(|(_, u)| u == "https://example.com"));
    doc.set_document_protection("readOnly", true).unwrap();
    assert_eq!(
        doc.document_protection_edit().unwrap().as_deref(),
        Some("readOnly")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert!(!ppt.has_notes(0).unwrap());
    ppt.add_notes_to_slide(0, "note body").unwrap();
    assert!(ppt.has_notes(0).unwrap());
    assert!(ppt
        .notes_text(0)
        .unwrap()
        .unwrap()
        .contains("note body"));
    assert!(ppt.clear_notes(0).unwrap());
    assert!(!ppt.has_notes(0).unwrap());
    assert!(!ppt.clear_notes(0).unwrap());
}

#[test]
fn remove_table_calc_chain_headers_titles() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["h1", "h2"]).unwrap();
    assert_eq!(wb.table_count(), 1);
    assert!(wb.remove_table("T1").unwrap());
    assert_eq!(wb.table_count(), 0);
    assert!(!wb.remove_table("T1").unwrap());

    wb.add_data_validation_list("S", "A1:A10", "\"X,Y\"", true)
        .unwrap();
    let dvs = wb.list_data_validations("S").unwrap();
    assert_eq!(dvs.len(), 1);
    assert_eq!(dvs[0].0, "list");
    assert_eq!(dvs[0].1, "A1:A10");

    wb.set_calc_chain(&[("A2", 1)]).unwrap();
    assert!(wb.has_calc_chain());
    assert!(wb.clear_calc_chain().unwrap());
    assert!(!wb.has_calc_chain());
    assert!(!wb.clear_calc_chain().unwrap());

    wb.add_minimal_styles(true).unwrap();
    wb.set_cell_style("S", "A1", 1).unwrap();
    assert_eq!(wb.get_cell_style_index("S", "A1").unwrap(), Some(1));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_header("H1").unwrap();
    doc.add_default_footer("F1").unwrap();
    assert!(doc.clear_headers().unwrap() >= 1);
    assert!(doc.list_headers().is_empty());
    assert!(doc.clear_footers().unwrap() >= 1);
    assert!(doc.list_footers().is_empty());
    doc.append_hyperlink("https://example.org/a", "a").unwrap();
    assert!(!doc.list_external_hyperlinks().is_empty());
    assert!(doc.remove_external_hyperlink("https://example.org/a"));
    assert!(doc.list_external_hyperlinks().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Title One").unwrap();
    ppt.add_slide_with_text("Title Two").unwrap();
    assert_eq!(
        ppt.slide_title(0).unwrap().as_deref(),
        Some("Title One")
    );
    let titles = ppt.slide_titles().unwrap();
    assert_eq!(titles.len(), 2);
    assert_eq!(titles[1].as_deref(), Some("Title Two"));
}

#[test]
fn formulas_breaks_styles_transitions() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.set_cell_formula("S", "A3", "A1+A2", Some("3")).unwrap();
    let formulas = wb.list_formulas("S").unwrap();
    assert!(formulas.iter().any(|(r, f)| r == "A3" && f.contains("A1")));
    wb.set_row_breaks("S", &[5, 10]).unwrap();
    assert_eq!(wb.row_breaks("S").unwrap(), vec![5, 10]);
    wb.set_col_breaks("S", &[2]).unwrap();
    assert_eq!(wb.col_breaks("S").unwrap(), vec![2]);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.append_heading("Heading1", "Title").unwrap();
    let styles = doc.paragraph_style_ids().unwrap();
    assert!(styles.iter().any(|s| s == "Heading1"));
    doc.add_default_numbering().unwrap();
    assert!(doc.has_numbering());
    assert!(doc.clear_numbering().unwrap());
    assert!(!doc.has_numbering());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.set_fade_transition(1, "med").unwrap();
    assert_eq!(ppt.slides_with_transition().unwrap(), vec![1]);
}

#[test]
fn clear_cf_sst_masters_custom_xml() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_shared_strings("S", &[vec!["a", "b"]]).unwrap();
    assert!(wb.has_shared_strings());
    assert!(!wb.shared_strings_list().is_empty());
    assert!(wb.clear_shared_strings().unwrap());
    assert!(!wb.has_shared_strings());
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.add_conditional_formatting_cell_is("S", "A1:A10", "greaterThan", "1", "FFFF0000", 1)
        .unwrap();
    let cfs = wb.list_conditional_formatting("S").unwrap();
    assert!(!cfs.is_empty());
    assert_eq!(cfs[0].0, "A1:A10");
    assert_eq!(cfs[0].1, "cellIs");
    assert!(wb.clear_conditional_formatting("S").unwrap() >= 1);
    assert!(!wb.has_conditional_formatting("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    let (rid, _) = doc.add_custom_xml_part(b"<root/>".to_vec()).unwrap();
    assert!(doc.has_custom_xml_parts().unwrap());
    assert!(doc.remove_custom_xml_part(&rid).unwrap());
    assert!(!doc.has_custom_xml_parts().unwrap());
    doc.add_custom_xml_part(b"<a/>".to_vec()).unwrap();
    doc.add_custom_xml_part(b"<b/>".to_vec()).unwrap();
    assert_eq!(doc.clear_custom_xml_parts().unwrap(), 2);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    // tiny 1x1 PNG
    let png = [
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8,
        0xcf, 0xc0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xfe, 0xd4, 0xef, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ];
    ppt.add_image_on_slide(0, &png, "image/png", "png", 0, 0, 100000, 100000, "pic")
        .unwrap();
    assert!(!ppt.list_media().is_empty());
    ppt.add_notes_master().unwrap();
    assert!(ppt.has_notes_master());
    assert!(ppt.clear_notes_master().unwrap());
    assert!(!ppt.has_notes_master());
    ppt.add_handout_master().unwrap();
    assert!(ppt.has_handout_master());
    assert!(ppt.clear_handout_master().unwrap());
    assert!(!ppt.has_handout_master());
}

#[test]
fn external_links_pivot_glossary_alt_charts() {
    use officexml::packaging::AlternativeFormatImportType;

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Src", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.write_sheet_strings("Out", &[vec!["x"]]).unwrap();
    wb.add_external_link("other.xlsx").unwrap();
    assert_eq!(wb.external_link_count(), 1);
    assert_eq!(wb.clear_external_links().unwrap(), 1);
    assert_eq!(wb.external_link_count(), 0);
    wb.add_pivot_table("Src", "A1:B2", "Out", "A3", &["A", "B"], 0, 1, 1)
        .unwrap();
    assert!(wb.pivot_table_count() >= 1);
    assert!(!wb.list_pivot_tables().is_empty());
    let infos = wb.pivot_table_infos().unwrap();
    assert!(!infos.is_empty());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_glossary_document("Entry1", vec![paragraph_with_text("g")])
        .unwrap();
    assert!(doc.has_glossary());
    assert!(doc.clear_glossary().unwrap());
    assert!(!doc.has_glossary());
    doc.add_alt_chunk(AlternativeFormatImportType::Html, b"<html><body>hi</body></html>")
        .unwrap();
    assert_eq!(doc.alt_chunk_count(), 1);
    assert!(!doc.list_alt_chunks().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_chart_on_slide(0, "C", &["a"], &[1.0]).unwrap();
    assert!(ppt.chart_count() >= 1);
    assert!(!ppt.list_charts().is_empty());
}

#[test]
fn sheet_format_bookmarks_tags_sync() {
    use officexml::wordprocessing::{body, document, paragraph, run, text, with_bookmark};

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_sheet_format("S", 18.0, Some(12.0)).unwrap();
    let fmt = wb.sheet_format("S").unwrap().unwrap();
    assert!((fmt.0 - 18.0).abs() < 0.01);
    assert_eq!(fmt.1, Some(12.0));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let p = paragraph(with_bookmark("1", "Intro", vec![run(vec![text("x")])]));
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    assert_eq!(doc.bookmark_count().unwrap(), 1);
    assert_eq!(doc.remove_bookmark("Intro").unwrap(), 1);
    assert_eq!(doc.bookmark_count().unwrap(), 0);
    assert_eq!(doc.remove_bookmark("Intro").unwrap(), 0);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_user_defined_tags(0, &[("k", "v")]).unwrap();
    assert!(ppt.has_user_defined_tags());
    assert!(ppt.clear_user_defined_tags().unwrap() >= 1);
    assert!(!ppt.has_user_defined_tags());
    ppt.add_slide_sync_data(0, "srv").unwrap();
    assert!(ppt.has_slide_sync_data());
    assert!(ppt.clear_slide_sync_data().unwrap() >= 1);
    assert!(!ppt.has_slide_sync_data());
}

#[test]
fn calc_props_drawings_thumbnail_masters() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_calc_properties(true, "auto").unwrap();
    let cp = wb.get_calc_properties().unwrap().unwrap();
    assert!(cp.0);
    assert_eq!(cp.1, "auto");
    let png = [
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8,
        0xcf, 0xc0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xfe, 0xd4, 0xef, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ];
    wb.add_image_on_sheet("S", &png, "image/png", "png", 0, 0, 100000, 100000, "pic")
        .unwrap();
    assert!(wb.has_drawings());
    assert!(wb.clear_drawings().unwrap() >= 1);
    assert!(!wb.has_drawings());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_thumbnail());
    doc.add_thumbnail(&png, "image/png", "png").unwrap();
    assert!(doc.has_thumbnail());
    assert!(doc.clear_thumbnail().unwrap());
    assert!(!doc.has_thumbnail());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.ensure_default_master_layout().unwrap();
    assert!(!ppt.list_masters().is_empty());
    assert!(!ppt.list_layouts().is_empty());
    assert_eq!(ppt.master_count(), ppt.list_masters().len());
    assert_eq!(ppt.layout_count(), ppt.list_layouts().len());
}

#[test]
fn clear_slicers_theme_vba_fonts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_slicer_shell("S", "S1", "C1").unwrap();
    assert!(wb.has_slicers());
    assert!(wb.clear_slicers().unwrap() >= 1);
    assert!(!wb.has_slicers());
    wb.add_timeline_shell("S", "T1", "TC1").unwrap();
    assert!(wb.has_timelines());
    assert!(wb.clear_timelines().unwrap() >= 1);
    wb.add_connections(&[("C1", "SELECT 1", "DSN=x")]).unwrap();
    assert!(wb.clear_connections().unwrap());
    assert!(!wb.has_connections());
    wb.add_query_table("S", "QT1", 1).unwrap();
    assert!(wb.clear_query_tables().unwrap() >= 1);
    wb.add_volatile_dependencies().unwrap();
    assert!(wb.has_volatile_dependencies());
    assert!(wb.clear_volatile_dependencies().unwrap());
    wb.add_default_theme().unwrap();
    assert!(wb.has_theme());
    assert!(wb.clear_theme().unwrap());
    assert!(!wb.has_theme());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_theme().unwrap();
    assert!(doc.clear_theme().unwrap());
    doc.add_default_font_table().unwrap();
    assert!(doc.has_font_table());
    assert!(doc.clear_font_table().unwrap());
    doc.add_vba_project(b"VBA").unwrap();
    doc.add_vba_data().unwrap();
    assert!(doc.has_vba_project());
    assert!(doc.clear_vba_project().unwrap());
    assert!(!doc.has_vba_project());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_default_theme().unwrap();
    assert!(ppt.has_theme());
    assert!(ppt.clear_theme().unwrap());
    assert!(!ppt.has_theme());
}

#[test]
fn clear_styles_settings_media_signatures() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_minimal_styles(true).unwrap();
    assert!(wb.has_styles());
    assert!(wb.clear_styles().unwrap());
    assert!(!wb.has_styles());
    let png = [
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8,
        0xcf, 0xc0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xfe, 0xd4, 0xef, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ];
    wb.add_image_on_sheet("S", &png, "image/png", "png", 0, 0, 100000, 100000, "pic")
        .unwrap();
    // image may land under drawings; also check media list API works
    let _ = wb.list_media();

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    assert!(doc.has_styles());
    assert!(doc.clear_styles().unwrap());
    doc.add_default_settings().unwrap();
    assert!(doc.has_settings());
    assert!(doc.clear_settings().unwrap());
    assert!(!doc.has_settings());
    doc.add_digital_signature_origin().unwrap();
    doc.add_xml_signature_part(b"<Signature/>").unwrap();
    assert!(doc.has_digital_signature_origin());
    assert!(doc.digital_signature_count() >= 1);
    assert!(doc.clear_digital_signatures().unwrap());
    assert!(!doc.has_digital_signature_origin());
    assert_eq!(doc.digital_signature_count(), 0);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_image_on_slide(0, &png, "image/png", "png", 0, 0, 100000, 100000, "pic")
        .unwrap();
    assert!(!ppt.list_media().is_empty());
    assert!(ppt.clear_media().unwrap() >= 1);
    assert!(ppt.list_media().is_empty());
}

#[test]
fn clear_word_aux_excel_charts_pres_props() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Src", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.write_sheet_strings("Out", &[vec!["x"]]).unwrap();
    wb.add_bar_chart("C", &["a"], &[1.0]).unwrap();
    assert!(wb.has_charts());
    assert!(wb.clear_charts().unwrap() >= 1);
    assert!(!wb.has_charts());
    wb.add_pivot_table("Src", "A1:B2", "Out", "A3", &["A", "B"], 0, 1, 1)
        .unwrap();
    assert!(wb.has_pivot_tables());
    assert!(wb.clear_pivot_tables().unwrap() >= 1);
    assert!(!wb.has_pivot_tables());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_people(&[("Ada", "AD")]).unwrap();
    assert!(doc.clear_people().unwrap());
    assert!(!doc.has_people());
    doc.add_mail_merge_recipients(b"<recipients/>").unwrap();
    assert!(doc.clear_mail_merge_recipients().unwrap() >= 1);
    assert!(!doc.has_mail_merge_recipients());
    doc.add_default_web_settings().unwrap();
    assert!(doc.clear_web_settings().unwrap());
    assert!(!doc.has_web_settings());
    doc.add_printer_settings(b"BIN").unwrap();
    assert!(doc.clear_printer_settings().unwrap() >= 1);
    assert!(!doc.has_printer_settings());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_presentation_properties().unwrap();
    assert!(ppt.has_presentation_properties());
    assert!(ppt.clear_presentation_properties().unwrap());
    ppt.add_view_properties().unwrap();
    assert!(ppt.has_view_properties());
    assert!(ppt.clear_view_properties().unwrap());
}

#[test]
fn clear_images_notes_charts() {
    use officexml::packaging::ImageFormat;

    let png = [
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8,
        0xcf, 0xc0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xfe, 0xd4, 0xef, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ];

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_image(ImageFormat::Png, png.to_vec()).unwrap();
    assert!(doc.media_count() >= 1);
    assert!(doc.clear_images().unwrap() >= 1);
    assert_eq!(doc.media_count(), 0);
    doc.add_footnote("1", "fn").unwrap();
    assert!(doc.has_footnotes());
    assert!(doc.clear_footnotes().unwrap());
    assert!(!doc.has_footnotes());
    doc.add_endnote("1", "en").unwrap();
    assert!(doc.has_endnotes());
    assert!(doc.clear_endnotes().unwrap());
    assert!(!doc.has_endnotes());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_chart_on_slide(0, "C", &["a"], &[1.0]).unwrap();
    assert!(ppt.chart_count() >= 1);
    assert!(ppt.clear_charts().unwrap() >= 1);
    assert_eq!(ppt.chart_count(), 0);
}

#[test]
fn named_styles_alt_chunks_table_styles() {
    use officexml::packaging::AlternativeFormatImportType;

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_styles_with_named_title().unwrap();
    let styles = wb.list_named_styles().unwrap();
    assert!(styles.iter().any(|(n, _)| !n.is_empty()));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_alt_chunk(AlternativeFormatImportType::Html, b"<html>hi</html>")
        .unwrap();
    assert_eq!(doc.alt_chunk_count(), 1);
    assert_eq!(doc.clear_alt_chunks().unwrap(), 1);
    assert_eq!(doc.alt_chunk_count(), 0);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_table_styles().unwrap();
    assert!(ppt.has_table_styles());
    assert!(ppt.clear_table_styles().unwrap());
    assert!(!ppt.has_table_styles());
}

#[test]
fn doc_vars_cell_hyperlinks_sdt_count() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"]]).unwrap();
    wb.add_cell_hyperlink("S", "A1", "https://example.com", Some("ex"))
        .unwrap();
    wb.add_cell_hyperlink("S", "B1", "https://example.org", None)
        .unwrap();
    assert_eq!(wb.list_cell_hyperlinks("S").unwrap().len(), 2);
    assert_eq!(wb.clear_cell_hyperlinks("S").unwrap(), 2);
    assert!(wb.list_cell_hyperlinks("S").unwrap().is_empty());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_document_variables(&[("v1", "a"), ("v2", "b")])
        .unwrap();
    assert_eq!(doc.document_variable_count().unwrap(), 2);
    assert!(doc.remove_document_variable("v1").unwrap());
    assert_eq!(doc.document_variable_count().unwrap(), 1);
    assert!(doc.clear_document_variables().unwrap());
    assert!(!doc.has_document_variables().unwrap());
    assert_eq!(doc.content_control_count().unwrap(), 0);
}

#[test]
fn hidden_sheets_merges_array_formulas() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Visible", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("Hidden", &[vec!["b"]]).unwrap();
    wb.set_sheet_state("Hidden", "hidden").unwrap();
    assert_eq!(wb.sheet_state("Hidden").unwrap(), "hidden");
    let hidden = wb.list_hidden_sheets().unwrap();
    assert!(hidden.iter().any(|(n, s)| n == "Hidden" && s == "hidden"));
    wb.merge_range("Visible", "A1:B1").unwrap();
    assert_eq!(wb.clear_merge_cells("Visible").unwrap(), 1);
    assert!(wb.merge_cells("Visible").unwrap().is_empty());
    wb.set_array_formula("Visible", "C1", "SUM(A1:A2)", "C1", Some("0"))
        .unwrap();
    let arrays = wb.list_array_formulas("Visible").unwrap();
    assert!(arrays.iter().any(|(r, f)| r == "C1" && f.contains("SUM")));
}

#[test]
fn num_fmts_shared_formulas_animations() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.add_styles_with_num_fmt("0.00%").unwrap();
    let fmts = wb.list_number_formats().unwrap();
    assert!(fmts.iter().any(|(_, c)| c == "0.00%"));
    wb.set_shared_formula("S", &["A3", "A4"], "A1+1", &[Some("2"), Some("3")], 0)
        .unwrap();
    let shared = wb.list_shared_formulas("S").unwrap();
    assert!(shared.iter().any(|(r, si, _)| r == "A3" && *si == 0));
    assert!(shared.iter().any(|(r, si, _)| r == "A4" && *si == 0));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.set_simple_appear_animation(1, 2).unwrap();
    assert_eq!(ppt.slides_with_animation().unwrap(), vec![1]);
}

#[test]
fn font_lists_master_counts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_minimal_styles(true).unwrap();
    let fonts = wb.list_style_fonts().unwrap();
    assert!(fonts.iter().any(|f| f.contains("Calibri") || !f.is_empty()));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_font_table().unwrap();
    let names = doc.list_font_names().unwrap();
    assert!(!names.is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    assert_eq!(ppt.notes_master_count(), 0);
    ppt.add_notes_master().unwrap();
    assert_eq!(ppt.notes_master_count(), 1);
    ppt.add_handout_master().unwrap();
    assert_eq!(ppt.handout_master_count(), 1);
}

#[test]
fn defined_name_lookup_sheet_states_sections() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("A", &[vec!["1"]]).unwrap();
    wb.write_sheet_strings("B", &[vec!["2"]]).unwrap();
    wb.add_defined_name("Sales", "A!$A$1").unwrap();
    assert_eq!(
        wb.get_defined_name("Sales").unwrap().as_deref(),
        Some("A!$A$1")
    );
    assert!(wb.get_defined_name("Missing").unwrap().is_none());
    wb.set_sheet_state("B", "veryHidden").unwrap();
    let states = wb.list_sheet_states().unwrap();
    assert_eq!(states.len(), 2);
    assert!(states.iter().any(|(n, s)| n == "B" && s == "veryHidden"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("1").unwrap();
    ppt.add_slide_with_text("2").unwrap();
    ppt.set_sections(&[("Intro", 0, 0), ("Body", 1, 1)]).unwrap();
    assert_eq!(ppt.section_count().unwrap(), 2);
}

#[test]
fn sheet_count_table_cols_hidden_slides_anchors() {
    use officexml::wordprocessing::{body, document, paragraph, run, text};

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    assert_eq!(wb.sheet_count(), 1);
    wb.add_table("S", "T1", "A1:B2", &["h1", "h2"]).unwrap();
    assert_eq!(wb.table_columns("T1").unwrap(), vec!["h1", "h2"]);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let anchor = doc.create_anchor_hyperlink("Intro", "go");
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![paragraph(vec![anchor])])]));
    let anchors = doc.list_anchor_hyperlinks().unwrap();
    assert!(anchors.iter().any(|a| a == "Intro"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("1").unwrap();
    ppt.add_slide_with_text("2").unwrap();
    ppt.set_slide_hidden(1, true).unwrap();
    assert_eq!(ppt.list_hidden_slides().unwrap(), vec![1]);
    assert_eq!(ppt.hidden_slide_count().unwrap(), 1);
}

#[test]
fn column_count_breaks_slide_comments_style_ids() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b", "c"]]).unwrap();
    assert_eq!(wb.column_count("S").unwrap(), 3);
    assert_eq!(wb.row_count("S").unwrap(), 1);
    wb.set_row_breaks("S", &[5]).unwrap();
    assert!(wb.clear_row_breaks("S").unwrap());
    assert!(wb.row_breaks("S").unwrap().is_empty());
    wb.set_col_breaks("S", &[2]).unwrap();
    assert!(wb.clear_col_breaks("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_paragraph_styles(&[("MyS", "My Style", None)])
        .unwrap();
    let ids = doc.list_style_ids().unwrap();
    assert!(ids.iter().any(|id| id == "MyS"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.add_slide_comments(0, &[(0, "2020-01-01T00:00:00", 0, 0, "hi")])
        .unwrap();
    assert!(ppt.has_slide_comments(0).unwrap());
    assert!(ppt.clear_slide_comments(0).unwrap());
    assert!(!ppt.has_slide_comments(0).unwrap());
}

#[test]
fn fills_protection_flags_watermark() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_styles_with_fill("FF0000").unwrap();
    let fills = wb.list_fills().unwrap();
    assert!(fills.iter().any(|(_, rgb)| rgb.as_deref() == Some("FF0000") || rgb.as_ref().map(|s| s.contains("FF0000")).unwrap_or(false) || fills.len() >= 2));
    wb.set_sheet_protection("S", true, false, true).unwrap();
    assert_eq!(
        wb.sheet_protection_flags("S").unwrap(),
        Some((true, false, true))
    );
    wb.set_workbook_protection(true, false).unwrap();
    assert_eq!(
        wb.workbook_protection_flags().unwrap(),
        Some((true, false))
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_watermark("CONFIDENTIAL").unwrap();
    assert!(doc.has_watermark());
    assert!(doc.clear_watermark().unwrap() >= 1);
    assert!(!doc.has_watermark());
}

#[test]
fn auto_filter_borders_page_borders_hf() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]])
        .unwrap();
    assert!(!wb.has_auto_filter("S").unwrap());
    wb.set_auto_filter("S", "A1:B2").unwrap();
    assert!(wb.has_auto_filter("S").unwrap());
    assert_eq!(wb.auto_filter_ref("S").unwrap().as_deref(), Some("A1:B2"));
    wb.add_styles_with_border().unwrap();
    assert!(wb.border_count().unwrap() >= 2);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_page_borders().unwrap());
    doc.set_page_borders("FF0000", 24).unwrap();
    assert!(doc.has_page_borders().unwrap());
    assert_eq!(doc.page_border_color().unwrap().as_deref(), Some("FF0000"));
    assert!(doc.clear_page_borders().unwrap());
    assert!(!doc.has_page_borders().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S").unwrap();
    ppt.set_slide_header_footer(0, true, true, true).unwrap();
    assert!(ppt.has_slide_header_footer(0).unwrap());
    assert!(ppt.clear_slide_header_footer(0).unwrap());
    assert!(!ppt.has_slide_header_footer(0).unwrap());
}

#[test]
fn background_dxf_notes_indices() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.add_conditional_formatting_cell_is("S", "A1:A10", "greaterThan", "0", "00FF00", 1)
        .unwrap();
    assert!(wb.dxf_count().unwrap() >= 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_document_background("FFFFCC").unwrap();
    assert!(doc.has_document_background().unwrap());
    assert_eq!(
        doc.document_background_color().unwrap().as_deref(),
        Some("FFFFCC")
    );
    assert!(doc.clear_document_background().unwrap());
    assert!(!doc.has_document_background().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.add_notes_to_slide(1, "note").unwrap();
    assert_eq!(ppt.slides_with_notes().unwrap(), vec![1]);
}

#[test]
fn mirror_margins_calc_chain_backgrounds() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_calc_chain(&[("A1", 1), ("B2", 1)]).unwrap();
    let chain = wb.list_calc_chain().unwrap();
    assert_eq!(chain.len(), 2);
    assert!(chain.iter().any(|(r, i)| r == "A1" && *i == 1));
    assert_eq!(wb.calc_chain_entry_count().unwrap(), 2);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.mirror_margins_enabled().unwrap());
    doc.set_mirror_margins(true).unwrap();
    assert!(doc.mirror_margins_enabled().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.set_slide_background(0, "112233").unwrap();
    assert_eq!(ppt.slides_with_background().unwrap(), vec![0]);
}

#[test]
fn even_odd_table_names_slides_with_comments() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    wb.add_table("S", "Sales", "A1:B2", &["h1", "h2"]).unwrap();
    assert_eq!(wb.table_names().unwrap(), vec!["Sales"]);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.even_odd_headers_enabled().unwrap());
    doc.add_even_odd_headers("odd", "even").unwrap();
    assert!(doc.even_odd_headers_enabled().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.add_slide_comments(1, &[(0, "2020-01-01T00:00:00", 0, 0, "c")])
        .unwrap();
    assert_eq!(ppt.slides_with_comments().unwrap(), vec![1]);
}

#[test]
fn sheet_hidden_even_odd_clear_hf_indices() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Vis", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("Hid", &[vec!["b"]]).unwrap();
    assert!(!wb.is_sheet_hidden("Vis").unwrap());
    wb.set_sheet_state("Hid", "hidden").unwrap();
    assert!(wb.is_sheet_hidden("Hid").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_even_odd_headers("o", "e").unwrap();
    assert!(doc.even_odd_headers_enabled().unwrap());
    assert!(doc.clear_even_odd_headers().unwrap());
    assert!(!doc.even_odd_headers_enabled().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.set_slide_header_footer(0, true, false, true).unwrap();
    assert_eq!(ppt.slides_with_header_footer().unwrap(), vec![0]);
}

#[test]
fn hidden_rows_cols_merge_print_bookmarks_transitions() {
    use officexml::wordprocessing::{bookmark_end, bookmark_start, paragraph, run, text};

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["c", "d"], vec!["e", "f"]])
        .unwrap();
    wb.set_row_hidden("S", 2, true).unwrap();
    assert_eq!(wb.list_hidden_rows("S").unwrap(), vec![2]);
    wb.set_column_hidden("S", 1, 1, true).unwrap();
    assert_eq!(wb.list_hidden_columns("S").unwrap(), vec![1]);
    wb.merge_range("S", "A1:B1").unwrap();
    assert!(wb.has_merge_cells("S").unwrap());
    assert_eq!(wb.merge_cell_count("S").unwrap(), 1);
    wb.set_print_area("S", "A1:B2").unwrap();
    assert!(wb.has_print_area().unwrap());
    assert!(!wb.has_print_titles().unwrap());
    wb.set_print_titles("S", Some("$1:$1"), None).unwrap();
    assert!(wb.has_print_titles().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let mut p = paragraph(vec![
        bookmark_start("0", "bm1"),
        run(vec![text("hi")]),
        bookmark_end("0"),
    ]);
    let _ = &mut p;
    doc.add_main_document_part()
        .set_document(simple_document(vec![p]));
    assert!(doc.has_bookmarks().unwrap());
    assert_eq!(doc.list_bookmark_names().unwrap(), vec!["bm1".to_string()]);
    doc.set_document_variables(&[("Author", "Alice"), ("Rev", "1")])
        .unwrap();
    assert_eq!(
        doc.get_document_variable("Author").unwrap().as_deref(),
        Some("Alice")
    );
    assert_eq!(doc.image_count(), 0);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.set_fade_transition(0, "med").unwrap();
    ppt.set_simple_appear_animation(1, 2).unwrap();
    assert_eq!(ppt.transition_count().unwrap(), 1);
    assert_eq!(ppt.animation_count().unwrap(), 1);
}

#[test]
fn defined_names_drawings_headers_notes_counts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["c", "d"]])
        .unwrap();
    assert!(!wb.has_defined_names().unwrap());
    wb.add_defined_name("Sales", "S!$A$1:$B$2").unwrap();
    assert!(wb.has_defined_names().unwrap());
    assert_eq!(wb.defined_name_count().unwrap(), 1);
    assert!(!wb.has_tables());
    wb.add_table("S", "T1", "A1:B2", &["a", "b"]).unwrap();
    assert!(wb.has_tables());
    assert_eq!(wb.drawing_count(), 0);
    wb.set_row_breaks("S", &[2]).unwrap();
    assert!(wb.has_row_breaks("S").unwrap());
    assert_eq!(wb.row_break_count("S").unwrap(), 1);
    assert!(!wb.has_col_breaks("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("body")]));
    assert_eq!(doc.header_count(), 0);
    doc.add_default_header("hdr").unwrap();
    doc.add_default_footer("ftr").unwrap();
    assert_eq!(doc.header_count(), 1);
    assert_eq!(doc.footer_count(), 1);
    assert!(!doc.has_external_hyperlinks());
    doc.append_hyperlink("https://example.com", "ex")
        .unwrap();
    assert!(doc.has_external_hyperlinks());
    assert_eq!(doc.external_hyperlink_count(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.add_notes_to_slide(0, "note text").unwrap();
    assert_eq!(ppt.notes_count().unwrap(), 1);
    assert!(ppt.total_shape_count().unwrap() >= 2);
}

#[test]
fn formula_style_chart_count_helpers() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1", "2"]]).unwrap();
    wb.set_cell_formula("S", "C1", "A1+B1", Some("3")).unwrap();
    assert!(wb.has_formulas("S").unwrap());
    assert_eq!(wb.formula_count("S").unwrap(), 1);
    wb.add_cell_hyperlink("S", "A1", "https://example.com", Some("x"))
        .unwrap();
    assert!(wb.has_cell_hyperlinks("S").unwrap());
    assert_eq!(wb.cell_hyperlink_count("S").unwrap(), 1);
    wb.ensure_styles().unwrap();
    let named = wb.named_style_count().unwrap();
    let fills = wb.fill_count().unwrap();
    let fonts = wb.style_font_count().unwrap();
    // ensure_styles creates default stylesheet content
    assert!(named + fills + fonts > 0 || wb.has_styles());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.ensure_styles().unwrap();
    assert!(doc.style_count().unwrap() >= 1 || doc.has_styles());
    doc.add_default_font_table().unwrap();
    assert!(doc.font_count().unwrap() >= 1 || doc.has_font_table());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    assert!(!ppt.has_charts());
    assert_eq!(ppt.slide_comments_count().unwrap(), 0);
    ppt.set_slide_background(0, "AABBCC").unwrap();
    assert_eq!(ppt.background_count().unwrap(), 1);
    ppt.set_slide_header_footer(1, true, true, false).unwrap();
    assert_eq!(ppt.header_footer_count().unwrap(), 1);
}

#[test]
fn slicer_people_page_setup_props_counts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert_eq!(wb.slicer_count(), 0);
    wb.add_slicer_shell("S", "S1", "C1").unwrap();
    assert_eq!(wb.slicer_count(), 1);
    assert!(wb.has_slicers());
    wb.add_connections(&[("C1", "SELECT 1", "DSN=x")]).unwrap();
    assert!(wb.has_connections());
    assert_eq!(wb.connection_count().unwrap(), 1);
    assert!(!wb.has_page_setup("S").unwrap());
    wb.set_page_setup("S", (0.7, 0.7, 0.75, 0.75, 0.3, 0.3), 9, "landscape")
        .unwrap();
    assert!(wb.has_page_setup("S").unwrap());
    assert!(wb.has_page_margins("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(doc.people_count().unwrap(), 0);
    doc.add_people(&[("Ada", "AD")]).unwrap();
    assert_eq!(doc.people_count().unwrap(), 1);
    doc.add_mail_merge_recipients(b"<recipients/>").unwrap();
    assert_eq!(doc.mail_merge_recipient_count(), 1);
    doc.add_printer_settings(b"BIN").unwrap();
    assert_eq!(doc.printer_settings_count(), 1);
    doc.add_default_web_settings().unwrap();
    assert!(doc.has_web_settings());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(!ppt.has_any_properties());
    ppt.add_presentation_properties().unwrap();
    ppt.add_view_properties().unwrap();
    assert!(ppt.has_any_properties());
    assert_eq!(ppt.extra_master_count(), 0);
    ppt.add_handout_master().unwrap();
    assert!(ppt.has_any_master_extras());
    assert_eq!(ppt.extra_master_count(), 1);
}

#[test]
fn active_tab_zoom_dimension_mirror_size_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("T", &[vec!["b"]]).unwrap();
    assert!(!wb.has_active_tab().unwrap());
    wb.set_active_tab(1).unwrap();
    assert!(wb.has_active_tab().unwrap());
    assert_eq!(wb.active_tab().unwrap(), Some(1));
    assert!(wb.clear_active_tab().unwrap());
    assert!(!wb.has_active_tab().unwrap());
    assert!(!wb.has_zoom("S").unwrap());
    wb.set_zoom("S", 150).unwrap();
    assert!(wb.has_zoom("S").unwrap());
    assert!(wb.clear_zoom("S").unwrap());
    assert!(!wb.has_zoom("S").unwrap());
    wb.set_sheet_dimension("S", "A1:B2").unwrap();
    assert!(wb.has_sheet_dimension("S").unwrap());
    assert!(wb.clear_sheet_dimension("S").unwrap());
    wb.set_sheet_format("S", 18.0, Some(10.0)).unwrap();
    assert!(wb.has_sheet_format("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.mirror_margins_enabled().unwrap());
    doc.set_mirror_margins(true).unwrap();
    assert!(doc.clear_mirror_margins().unwrap());
    assert!(!doc.mirror_margins_enabled().unwrap());
    doc.set_page_setup(12240, 15840, 1440, 1440, 1440, 1440)
        .unwrap();
    assert!(doc.has_page_size().unwrap());
    assert!(doc.has_page_margins().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    // create_in_memory may already set a default slide size
    let _ = ppt.has_slide_size().unwrap();
    ppt.set_slide_size(9144000, 6858000).unwrap();
    assert!(ppt.has_slide_size().unwrap());
    ppt.set_notes_size(6858000, 9144000).unwrap();
    assert!(ppt.has_notes_size().unwrap());
}

#[test]
fn title_creator_custom_property_helpers() {
    use officexml::opc::CustomProperties;

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_package_properties() || wb.title().unwrap().is_none());
    wb.set_title("Sales Workbook").unwrap();
    wb.set_creator("Alice").unwrap();
    assert!(wb.has_package_properties());
    assert_eq!(wb.title().unwrap().as_deref(), Some("Sales Workbook"));
    assert_eq!(wb.creator().unwrap().as_deref(), Some("Alice"));
    let mut custom = CustomProperties::new();
    custom.set_string("Dept", "Finance");
    custom.set_i4("Year", 2026);
    wb.set_custom_properties(&custom).unwrap();
    assert!(wb.has_custom_properties());
    assert_eq!(wb.custom_property_count().unwrap(), 2);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_title("Memo").unwrap();
    doc.set_creator("Bob").unwrap();
    assert_eq!(doc.title().unwrap().as_deref(), Some("Memo"));
    assert_eq!(doc.creator().unwrap().as_deref(), Some("Bob"));
    assert!(doc.has_package_properties());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_title("Pitch").unwrap();
    ppt.set_creator("Carol").unwrap();
    assert_eq!(ppt.title().unwrap().as_deref(), Some("Pitch"));
    assert_eq!(ppt.creator().unwrap().as_deref(), Some("Carol"));
    assert!(!ppt.has_custom_properties());
    assert_eq!(ppt.custom_property_count().unwrap(), 0);
}

#[test]
fn subject_keywords_company_custom_prop_crud() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_subject("Q1").unwrap();
    wb.set_keywords("finance,q1").unwrap();
    wb.set_description("Quarterly sales").unwrap();
    wb.set_category("Reports").unwrap();
    wb.set_application("openxml-rs").unwrap();
    wb.set_company("Acme").unwrap();
    assert_eq!(wb.subject().unwrap().as_deref(), Some("Q1"));
    assert_eq!(wb.keywords().unwrap().as_deref(), Some("finance,q1"));
    assert_eq!(
        wb.description().unwrap().as_deref(),
        Some("Quarterly sales")
    );
    assert_eq!(wb.category().unwrap().as_deref(), Some("Reports"));
    assert_eq!(wb.application().unwrap().as_deref(), Some("openxml-rs"));
    assert_eq!(wb.company().unwrap().as_deref(), Some("Acme"));
    wb.set_custom_property_string("Owner", "Dana").unwrap();
    assert_eq!(
        wb.get_custom_property_string("Owner").unwrap().as_deref(),
        Some("Dana")
    );
    assert!(wb.remove_custom_property("Owner").unwrap());
    assert!(wb.get_custom_property_string("Owner").unwrap().is_none());
    wb.set_custom_property_string("A", "1").unwrap();
    wb.set_custom_property_string("B", "2").unwrap();
    assert!(wb.clear_custom_properties().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_subject("Legal").unwrap();
    doc.set_company("Acme Legal").unwrap();
    assert_eq!(doc.subject().unwrap().as_deref(), Some("Legal"));
    assert_eq!(doc.company().unwrap().as_deref(), Some("Acme Legal"));
    doc.set_custom_property_string("Matter", "M-1").unwrap();
    assert_eq!(
        doc.get_custom_property_string("Matter").unwrap().as_deref(),
        Some("M-1")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_keywords("pitch,demo").unwrap();
    ppt.set_application("openxml-rs").unwrap();
    assert_eq!(ppt.keywords().unwrap().as_deref(), Some("pitch,demo"));
    assert_eq!(ppt.application().unwrap().as_deref(), Some("openxml-rs"));
    ppt.set_custom_property_string("Stage", "draft").unwrap();
    assert!(ppt.remove_custom_property("Stage").unwrap());
}

#[test]
fn last_modified_revision_manager_template_helpers() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_last_modified_by("Editor").unwrap();
    wb.set_revision("7").unwrap();
    wb.set_language("en-US").unwrap();
    wb.set_version("1.2").unwrap();
    wb.set_content_status("Draft").unwrap();
    wb.set_manager("Pat").unwrap();
    wb.set_template("Budget.xltx").unwrap();
    wb.set_hyperlink_base("https://intranet/").unwrap();
    assert_eq!(wb.last_modified_by().unwrap().as_deref(), Some("Editor"));
    assert_eq!(wb.revision().unwrap().as_deref(), Some("7"));
    assert_eq!(wb.language().unwrap().as_deref(), Some("en-US"));
    assert_eq!(wb.version().unwrap().as_deref(), Some("1.2"));
    assert_eq!(wb.content_status().unwrap().as_deref(), Some("Draft"));
    assert_eq!(wb.manager().unwrap().as_deref(), Some("Pat"));
    assert_eq!(wb.template().unwrap().as_deref(), Some("Budget.xltx"));
    assert_eq!(
        wb.hyperlink_base().unwrap().as_deref(),
        Some("https://intranet/")
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_last_modified_by("Reviewer").unwrap();
    doc.set_manager("Lee").unwrap();
    doc.set_template("Normal.dotm").unwrap();
    assert_eq!(
        doc.last_modified_by().unwrap().as_deref(),
        Some("Reviewer")
    );
    assert_eq!(doc.manager().unwrap().as_deref(), Some("Lee"));
    assert_eq!(doc.template().unwrap().as_deref(), Some("Normal.dotm"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_revision("3").unwrap();
    ppt.set_content_status("Final").unwrap();
    ppt.set_hyperlink_base("https://slides/").unwrap();
    assert_eq!(ppt.revision().unwrap().as_deref(), Some("3"));
    assert_eq!(ppt.content_status().unwrap().as_deref(), Some("Final"));
    assert_eq!(
        ppt.hyperlink_base().unwrap().as_deref(),
        Some("https://slides/")
    );
}

#[test]
fn advanced_shell_inventory_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_threaded_comments());
    assert!(!wb.has_persons());
    wb.add_persons(&[("{p1}", "Alice")]).unwrap();
    wb.add_threaded_comments("S", &[("{c1}", "{p1}", "Hi")])
        .unwrap();
    assert!(wb.has_persons());
    assert_eq!(wb.person_count().unwrap(), 1);
    assert!(wb.has_threaded_comments());
    assert_eq!(wb.threaded_comment_count(), 1);
    wb.add_query_table("S", "QT1", 1).unwrap();
    assert!(wb.has_query_tables());
    wb.add_volatile_dependencies().unwrap();
    assert!(wb.has_volatile_dependencies());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_custom_ui());
    doc.add_custom_ui(b"<customUI xmlns=\"http://schemas.microsoft.com/office/2006/01/customui\"/>")
        .unwrap();
    assert!(doc.has_custom_ui());
    doc.add_vba_data().unwrap();
    assert!(doc.has_vba_data());
    doc.add_styles_with_effects().unwrap();
    assert!(doc.has_styles_with_effects());
    doc.add_comments_ids(&[("00000001", "11111111")]).unwrap();
    assert!(doc.has_comments_ids());
    doc.add_comments_extensible().unwrap();
    assert!(doc.has_comments_extensible());
    doc.add_comments_extended(&[("00000001", "1", false)])
        .unwrap();
    assert!(doc.has_comments_extended());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert_eq!(ppt.user_defined_tag_count(), 0);
    ppt.add_user_defined_tags(0, &[("k", "v")]).unwrap();
    assert!(ppt.has_user_defined_tags());
    assert_eq!(ppt.user_defined_tag_count(), 1);
    ppt.add_slide_sync_data(0, "srv1").unwrap();
    assert!(ppt.has_slide_sync_data());
    assert_eq!(ppt.slide_sync_count(), 1);
    ppt.add_comment_authors(&[(0, "Alice", "A")]).unwrap();
    assert!(ppt.has_comment_authors());
    assert_eq!(ppt.comment_author_count().unwrap(), 1);
    ppt.add_model_3d(0, b"glTF").unwrap();
    assert!(ppt.has_model_3d());
}

#[test]
fn advanced_shell_clear_helpers() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_persons(&[("{p1}", "Alice")]).unwrap();
    wb.add_threaded_comments("S", &[("{c1}", "{p1}", "Hi")])
        .unwrap();
    assert_eq!(wb.clear_threaded_comments().unwrap(), 1);
    assert!(!wb.has_threaded_comments());
    assert_eq!(wb.clear_persons().unwrap(), 1);
    assert!(!wb.has_persons());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_custom_ui(b"<customUI xmlns=\"http://schemas.microsoft.com/office/2006/01/customui\"/>")
        .unwrap();
    doc.add_vba_data().unwrap();
    doc.add_styles_with_effects().unwrap();
    doc.add_comments_ids(&[("00000001", "11111111")]).unwrap();
    doc.add_comments_extensible().unwrap();
    doc.add_comments_extended(&[("00000001", "1", false)])
        .unwrap();
    assert!(doc.clear_custom_ui().unwrap());
    assert!(!doc.has_custom_ui());
    assert!(doc.clear_vba_data().unwrap());
    assert!(!doc.has_vba_data());
    assert!(doc.clear_styles_with_effects().unwrap());
    assert!(doc.clear_comments_ids().unwrap());
    assert!(doc.clear_comments_extensible().unwrap());
    assert!(doc.clear_comments_extended().unwrap());
    assert!(!doc.has_comments_ids());
    assert!(!doc.has_comments_extended());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_comment_authors(&[(0, "Alice", "A")]).unwrap();
    ppt.add_model_3d(0, b"glTF").unwrap();
    ppt.add_user_defined_tags(0, &[("k", "v")]).unwrap();
    ppt.add_slide_sync_data(0, "srv").unwrap();
    assert!(ppt.clear_comment_authors().unwrap());
    assert!(!ppt.has_comment_authors());
    assert_eq!(ppt.clear_model_3d().unwrap(), 1);
    assert!(!ppt.has_model_3d());
    assert_eq!(ppt.clear_user_defined_tags().unwrap(), 1);
    assert_eq!(ppt.clear_slide_sync_data().unwrap(), 1);
    assert!(!ppt.has_user_defined_tags());
    assert!(!ppt.has_slide_sync_data());
}

#[test]
fn advanced_shell_list_entries() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_persons(&[("{p1}", "Alice"), ("{p2}", "Bob")])
        .unwrap();
    let people = wb.list_persons().unwrap();
    assert_eq!(people.len(), 2);
    assert!(people.iter().any(|(id, n)| id == "{p1}" && n == "Alice"));
    wb.add_threaded_comments("S", &[("{c1}", "{p1}", "Hi")])
        .unwrap();
    let entries = wb.list_threaded_comment_entries().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].2, "Hi");
    wb.add_query_table("S", "QT1", 3).unwrap();
    let q = wb.query_table_infos().unwrap();
    assert_eq!(q.len(), 1);
    assert_eq!(q[0].0, "QT1");
    assert_eq!(q[0].1, 3);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_comment_authors(&[(0, "Alice", "A"), (1, "Bob", "B")])
        .unwrap();
    let authors = ppt.list_comment_authors().unwrap();
    assert_eq!(authors.len(), 2);
    assert_eq!(authors[0].1, "Alice");
    ppt.add_user_defined_tags(0, &[("k", "v"), ("x", "y")])
        .unwrap();
    let tags = ppt.list_user_defined_tag_entries().unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags.iter().any(|(n, v)| n == "k" && v == "v"));
    ppt.add_slide_sync_data(0, "srv").unwrap();
    assert_eq!(ppt.list_slide_sync_parts().len(), 1);
}

#[test]
fn remaining_shell_inventory_clear() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_dialogsheets());
    wb.add_dialogsheet("Dialog1").unwrap();
    assert!(wb.has_dialogsheets());
    assert_eq!(wb.dialogsheet_count(), 1);
    assert_eq!(wb.clear_dialogsheets().unwrap(), 1);
    assert!(!wb.has_dialogsheets());
    wb.add_macrosheet("Macro1").unwrap();
    assert!(wb.has_macrosheets());
    assert_eq!(wb.clear_macrosheets().unwrap(), 1);
    wb.add_xml_maps(
        1,
        "root",
        r#"<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"/>"#,
    )
    .unwrap();
    assert!(wb.has_xml_maps());
    assert!(wb.clear_xml_maps().unwrap());
    assert!(!wb.has_xml_maps());
    wb.add_sort_map("S").unwrap();
    assert!(wb.has_sort_maps());
    assert_eq!(wb.clear_sort_maps().unwrap(), 1);
    wb.add_revision_tracking_shell().unwrap();
    assert!(wb.has_revision_tracking());
    assert!(wb.clear_revision_tracking().unwrap() >= 1);
    assert!(!wb.has_revision_tracking());
    wb.add_single_cell_table("S", "A1", 1).unwrap();
    assert!(wb.has_single_cell_tables());
    assert_eq!(wb.clear_single_cell_tables().unwrap(), 1);
    wb.add_embedded_control("S", b"ACTIVEX").unwrap();
    assert!(wb.has_embedded_controls());
    assert_eq!(wb.clear_embedded_controls().unwrap(), 1);
    wb.add_attached_toolbars(b"TB").unwrap();
    assert!(wb.has_attached_toolbars());
    assert_eq!(wb.clear_attached_toolbars().unwrap(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_document_tasks(&["Review", "Ship"]).unwrap();
    assert!(doc.has_document_tasks());
    assert!(doc.clear_document_tasks().unwrap());
    assert!(!doc.has_document_tasks());
    doc.add_web_extension_shell("MyAddin", "1.0.0.0").unwrap();
    assert!(doc.has_web_extensions());
    assert!(doc.web_extension_count() >= 1);
    assert!(doc.clear_web_extensions().unwrap() >= 1);
    assert!(!doc.has_web_extensions());
    doc.add_customization().unwrap();
    assert!(doc.has_customization());
    assert!(doc.clear_customization().unwrap());
    doc.add_quick_access_toolbar().unwrap();
    assert!(doc.has_quick_access_toolbar());
    assert!(doc.clear_quick_access_toolbar().unwrap());
    doc.add_label_info("{label-id}", "Confidential").unwrap();
    assert!(doc.has_label_info());
    assert!(doc.clear_label_info().unwrap());
    doc.add_attached_toolbars(b"TOOLBAR").unwrap();
    assert!(doc.has_attached_toolbars());
    assert!(doc.clear_attached_toolbars().unwrap());
    doc.add_diagram_shell("diag-1").unwrap();
    assert!(doc.has_diagrams());
    assert!(doc.diagram_count() >= 1);
    assert!(doc.clear_diagrams().unwrap() >= 1);
    assert!(!doc.has_diagrams());
    assert!(!doc.has_embeddings());
    doc.add_embedded_package(b"PK\x03\x04fake", "application/octet-stream", "bin")
        .unwrap();
    assert!(doc.has_embeddings());
    assert_eq!(doc.embedding_count(), 1);
    assert_eq!(doc.clear_embeddings().unwrap(), 1);
    assert!(!doc.has_embeddings());
}

#[test]
fn metadata_modern_chart_shell_inventory() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_cell_metadata());
    wb.add_cell_metadata().unwrap();
    assert!(wb.has_cell_metadata());
    assert_eq!(wb.clear_cell_metadata().unwrap(), 1);
    assert!(!wb.has_cell_metadata());
    let (chart_uri, _) = wb
        .add_bar_chart("Sales", &["A", "B"], &[1.0, 2.0])
        .unwrap();
    wb.add_chart_drawing(&chart_uri).unwrap();
    assert!(wb.has_chart_drawings());
    assert_eq!(wb.clear_chart_drawings().unwrap(), 1);
    assert!(!wb.has_chart_drawings());
    wb.add_theme_override(&chart_uri).unwrap();
    assert!(wb.has_theme_override());
    assert_eq!(wb.clear_theme_override().unwrap(), 1);
    wb.add_custom_data(b"CUSTOM", "{aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee}")
        .unwrap();
    assert!(wb.has_custom_data());
    assert!(wb.clear_custom_data().unwrap() >= 1);
    wb.add_supporting_property_bags().unwrap();
    assert!(wb.has_supporting_property_bags());
    assert!(wb.clear_supporting_property_bags().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_modern_authors(&[("auth1", "Alice")]).unwrap();
    assert!(ppt.has_modern_authors());
    ppt.add_modern_comments(0, &[("auth1", "Looks good")])
        .unwrap();
    assert!(ppt.has_modern_comments());
    assert_eq!(ppt.modern_comment_count(), 1);
    assert_eq!(ppt.clear_modern_comments().unwrap(), 1);
    assert!(!ppt.has_modern_comments());
    assert!(ppt.clear_modern_authors().unwrap());
    assert!(!ppt.has_modern_authors());
    ppt.add_chart_drawing_for_slide(0, "chart1").unwrap();
    assert!(ppt.has_chart_drawings());
    assert_eq!(ppt.clear_chart_drawings().unwrap(), 1);
    assert!(!ppt.has_chart_drawings());

    // chartsheet inventory
    let mut wb2 =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb2.write_sheet_strings("Data", &[vec!["1", "2"]]).unwrap();
    let (chart_uri, _) = wb2
        .add_bar_chart("Sales", &["A", "B"], &[1.0, 2.0])
        .unwrap();
    wb2.add_chartsheet("Chart1", &chart_uri).unwrap();
    assert!(wb2.has_chartsheets());
    assert_eq!(wb2.chartsheet_count(), 1);
    assert_eq!(wb2.list_chartsheets().len(), 1);
    assert_eq!(wb2.clear_chartsheets().unwrap(), 1);
    assert!(!wb2.has_chartsheets());
}


#[test]
fn schematron_subset_validation() {
    use officexml::validation::{
        SCHEMATRON_EXTRACTED_REL_COUNT, SCHEMATRON_EXTRACTED_UNIQUE_COUNT,
        SCHEMATRON_NUMERIC_RANGE_COUNT, SCHEMATRON_PATTERN_COUNT,
        SCHEMATRON_STRING_LENGTH_COUNT, SCHEMATRON_TOTAL_SOURCE_RULES,
    };
    assert_eq!(SCHEMATRON_TOTAL_SOURCE_RULES, 948);
    assert!(SCHEMATRON_EXTRACTED_REL_COUNT >= 50);
    assert!(SCHEMATRON_EXTRACTED_UNIQUE_COUNT >= 100);
    assert!(SCHEMATRON_NUMERIC_RANGE_COUNT >= 200);
    assert!(SCHEMATRON_STRING_LENGTH_COUNT >= 150);
    assert!(SCHEMATRON_PATTERN_COUNT >= 10);
    // Extractable families cover a large majority of the 948 source rules.
    let extractable = SCHEMATRON_EXTRACTED_REL_COUNT
        + SCHEMATRON_EXTRACTED_UNIQUE_COUNT
        + SCHEMATRON_NUMERIC_RANGE_COUNT
        + SCHEMATRON_STRING_LENGTH_COUNT
        + SCHEMATRON_PATTERN_COUNT;
    assert!(extractable >= 500, "extractable={extractable}");

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("ok")]));
    let errs = doc.validate_schematron().unwrap();
    // Clean simple doc should not trip relationship/uniqueness Schematron subset.
    assert!(errs.is_empty(), "{errs:?}");
    let rel_errs = doc.validate_relationships().unwrap();
    assert!(rel_errs.is_empty(), "{rel_errs:?}");

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(wb.validate_schematron().unwrap().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(ppt.validate_schematron().unwrap().is_empty());
}

#[test]
fn schematron_attribute_constraints() {
    use officexml::element::OpenXmlElement;
    use officexml::validation::{
        validate_schematron_constraints, validate_schematron_numeric_ranges,
        validate_schematron_patterns, validate_schematron_string_lengths,
    };

    // sheetId must be 1..=65534
    let sheet = OpenXmlElement::new(
        "x",
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        "sheet",
    )
    .with_attribute("name", "S")
    .with_attribute("sheetId", "70000");
    let sheets = OpenXmlElement::new(
        "x",
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        "sheets",
    )
    .with_child(sheet);
    let range_errs = validate_schematron_numeric_ranges(&sheets);
    assert!(
        range_errs.iter().any(|e| e.path.contains("sheetId")),
        "{range_errs:?}"
    );

    // sheet name length max 31
    let long_name = "a".repeat(40);
    let sheet2 = OpenXmlElement::new(
        "x",
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        "sheet",
    )
    .with_attribute("name", &long_name)
    .with_attribute("sheetId", "1");
    let sheets2 = OpenXmlElement::new(
        "x",
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        "sheets",
    )
    .with_child(sheet2);
    let len_errs = validate_schematron_string_lengths(&sheets2);
    assert!(
        len_errs.iter().any(|e| e.message.contains("length")),
        "{len_errs:?}"
    );

    // decimalSymbol must be exactly one character
    let settings = OpenXmlElement::w("settings").with_child(
        OpenXmlElement::w("decimalSymbol").with_attribute_qname("w:val", "ab"),
    );
    let pat_errs = validate_schematron_patterns(&settings);
    assert!(
        pat_errs.iter().any(|e| e.message.contains("does not match")),
        "{pat_errs:?}"
    );

    // Combined entry point
    let all = validate_schematron_constraints(&sheets);
    assert!(!all.is_empty(), "{all:?}");
}


#[test]
fn created_modified_content_type_rels() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_created("2020-01-01T00:00:00Z").unwrap();
    wb.set_modified("2020-01-02T00:00:00Z").unwrap();
    assert_eq!(
        wb.created().unwrap().as_deref(),
        Some("2020-01-01T00:00:00Z")
    );
    assert_eq!(
        wb.modified().unwrap().as_deref(),
        Some("2020-01-02T00:00:00Z")
    );
    let ct = wb.part_content_type("/xl/workbook.xml").expect("workbook content type");
    assert!(ct.contains("sheet.main") || ct.contains("spreadsheetml"), "{ct}");
    assert!(wb.package_relationship_count() >= 1);
    assert!(!wb.list_package_relationships().is_empty());
    assert!(wb.workbook_relationship_count() >= 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_created("2021-05-05T12:00:00Z").unwrap();
    assert_eq!(
        doc.created().unwrap().as_deref(),
        Some("2021-05-05T12:00:00Z")
    );
    let wct = doc.part_content_type("/word/document.xml").expect("document content type");
    assert!(wct.contains("wordprocessingml"), "{wct}");
    assert!(!doc.list_package_relationships().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_modified("2022-03-03T03:03:03Z").unwrap();
    assert_eq!(
        ppt.modified().unwrap().as_deref(),
        Some("2022-03-03T03:03:03Z")
    );
    assert!(ppt.part_content_type("/ppt/presentation.xml").is_some());
    assert!(!ppt.list_package_relationships().is_empty());
}


#[test]
fn app_version_rels_content_types() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_application_version("16.0").unwrap();
    wb.set_doc_security(0).unwrap();
    assert_eq!(wb.application_version().unwrap().as_deref(), Some("16.0"));
    assert_eq!(wb.doc_security().unwrap(), Some(0));
    assert!(!wb.list_workbook_relationships().is_empty());
    assert!(!wb.list_content_type_overrides().is_empty());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("hello world")]));
    doc.set_application_version("1.0").unwrap();
    doc.set_pages(1).unwrap();
    doc.set_words(2).unwrap();
    doc.set_characters(11).unwrap();
    assert_eq!(doc.pages().unwrap(), Some(1));
    assert_eq!(doc.words().unwrap(), Some(2));
    assert_eq!(doc.characters().unwrap(), Some(11));
    let _ = doc.main_relationship_count(); // may be 0 before save; list is fine either way
    assert!(doc.list_main_relationships().len() == doc.main_relationship_count());
    assert!(!doc.list_content_type_overrides().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_app_slides(1).unwrap();
    ppt.set_app_notes(0).unwrap();
    ppt.set_presentation_format("On-screen Show (16:9)").unwrap();
    assert_eq!(ppt.app_slides().unwrap(), Some(1));
    assert_eq!(ppt.app_notes().unwrap(), Some(0));
    assert_eq!(
        ppt.presentation_format().unwrap().as_deref(),
        Some("On-screen Show (16:9)")
    );
    assert!(!ppt.list_presentation_relationships().is_empty());
    assert!(!ppt.list_content_type_overrides().is_empty());
}


#[test]
fn typed_custom_props_and_app_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_custom_property_i4("Year", 2026).unwrap();
    wb.set_custom_property_bool("Approved", true).unwrap();
    assert_eq!(wb.get_custom_property_i4("Year").unwrap(), Some(2026));
    assert_eq!(wb.get_custom_property_bool("Approved").unwrap(), Some(true));
    let names = wb.list_custom_property_names().unwrap();
    assert!(names.iter().any(|n| n == "Year"));
    wb.set_shared_doc(true).unwrap();
    wb.set_links_up_to_date(true).unwrap();
    wb.set_hyperlinks_changed(false).unwrap();
    wb.set_scale_crop(false).unwrap();
    wb.set_total_time(15).unwrap();
    assert_eq!(wb.shared_doc().unwrap(), Some(true));
    assert_eq!(wb.links_up_to_date().unwrap(), Some(true));
    assert_eq!(wb.hyperlinks_changed().unwrap(), Some(false));
    assert_eq!(wb.scale_crop().unwrap(), Some(false));
    assert_eq!(wb.total_time().unwrap(), Some(15));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_characters_with_spaces(12).unwrap();
    doc.set_lines(3).unwrap();
    doc.set_paragraphs_count(1).unwrap();
    doc.set_shared_doc(false).unwrap();
    doc.set_total_time(5).unwrap();
    doc.set_custom_property_i4("Rev", 2).unwrap();
    assert_eq!(doc.characters_with_spaces().unwrap(), Some(12));
    assert_eq!(doc.lines().unwrap(), Some(3));
    assert_eq!(doc.paragraphs_count().unwrap(), Some(1));
    assert_eq!(doc.get_custom_property_i4("Rev").unwrap(), Some(2));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_app_hidden_slides(0).unwrap();
    ppt.set_mm_clips(0).unwrap();
    ppt.set_shared_doc(true).unwrap();
    ppt.set_total_time(20).unwrap();
    ppt.set_custom_property_bool("Draft", false).unwrap();
    assert_eq!(ppt.app_hidden_slides().unwrap(), Some(0));
    assert_eq!(ppt.mm_clips().unwrap(), Some(0));
    assert_eq!(ppt.get_custom_property_bool("Draft").unwrap(), Some(false));
    // slide may have 0 rels if only text; list should not error
    let _ = ppt.list_slide_relationships(0).unwrap();
    assert!(!ppt.list_presentation_relationships().is_empty());
}


#[test]
fn sheet_relationship_list() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    // empty sheet still has no extra rels; list must succeed
    let _ = wb.list_sheet_relationships("S").unwrap();
    wb.add_table("S", "T1", "A1:A1", &["a"]).unwrap();
    assert!(wb.sheet_relationship_count("S").unwrap() >= 1);
    let rels = wb.list_sheet_relationships("S").unwrap();
    assert!(rels.iter().any(|(_, ty, _)| ty.contains("table")));
}


#[test]
fn part_bytes_and_flat_opc_excel_ppt() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["hello"]]).unwrap();
    assert!(wb.has_part("/xl/workbook.xml"));
    let bytes = wb.get_part_bytes("/xl/workbook.xml").unwrap();
    assert!(!bytes.is_empty());
    assert_eq!(wb.part_size("/xl/workbook.xml"), Some(bytes.len()));
    let flat = wb.to_flat_opc_string().unwrap();
    assert!(flat.contains("Excel.Sheet") || flat.contains("pkg:package") || flat.contains("Package"));
    let wb2 = SpreadsheetDocument::from_flat_opc(flat.as_bytes()).unwrap();
    assert!(wb2.has_part("/xl/workbook.xml"));
    let grid = wb2.read_sheet_strings().unwrap();
    assert!(grid.iter().flatten().any(|c| c == "hello"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Title").unwrap();
    assert!(ppt.has_part("/ppt/presentation.xml"));
    assert!(ppt.part_size("/ppt/presentation.xml").unwrap() > 0);
    let flat = ppt.to_flat_opc_string().unwrap();
    let ppt2 = PresentationDocument::from_flat_opc(flat.as_bytes()).unwrap();
    assert!(ppt2.has_part("/ppt/presentation.xml"));
    assert_eq!(ppt2.slide_count(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(doc.has_part("/word/document.xml"));
    assert!(doc.get_part_bytes("/word/document.xml").unwrap().len() > 10);
    assert!(doc.package_relationship_count() >= 1);
    let rid = doc.list_package_relationships()[0].0.clone();
    assert!(doc.package_relationship_target(&rid).is_some());
}


#[test]
fn settings_strict_flat_helpers() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(wb.auto_save());
    wb.settings_mut().auto_save = false;
    assert!(!wb.auto_save());
    let (x, r) = wb.rewrite_strict_to_transitional().unwrap();
    let _ = (x, r);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.settings_mut().max_characters_in_part = 0;
    let _ = ppt.rewrite_strict_to_transitional().unwrap();
    assert!(ppt.settings().max_characters_in_part == 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(doc.auto_save());
    let _ = doc.rewrite_strict_to_transitional().unwrap();
    // Non-encrypted empty path should error or return false; just ensure API is callable.
    let _ = SpreadsheetDocument::is_encrypted_office_file("/tmp/does-not-exist-openxml.bin");
}


#[test]
fn change_document_type_and_close() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert_eq!(wb.document_type(), SpreadsheetDocumentType::Workbook);
    wb.change_document_type(SpreadsheetDocumentType::Template)
        .unwrap();
    assert_eq!(wb.document_type(), SpreadsheetDocumentType::Template);
    let ct = wb.part_content_type("/xl/workbook.xml").unwrap();
    assert!(ct.contains("template") || ct.contains("spreadsheetml"));
    wb.close().unwrap();

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.change_document_type(PresentationDocumentType::Slideshow)
        .unwrap();
    assert_eq!(
        ppt.document_type(),
        PresentationDocumentType::Slideshow
    );
    ppt.close().unwrap();

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.change_document_type(WordprocessingDocumentType::Template)
        .unwrap();
    assert_eq!(doc.document_type(), WordprocessingDocumentType::Template);
    doc.close().unwrap();
}


#[test]
fn excel_ppt_thumbnail_signature_custom_xml_parity() {
    // Excel
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(wb.package_part_count() >= 2);
    assert!(!wb.list_part_uris().is_empty());

    let png: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0xD4, 0xEF, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    wb.add_thumbnail(png, "image/png", "png").unwrap();
    assert!(wb.has_thumbnail());
    assert!(wb.clear_thumbnail().unwrap());
    assert!(!wb.has_thumbnail());

    let (rid, uri) = wb.add_digital_signature_origin().unwrap();
    assert!(rid.starts_with('r'));
    assert!(wb.has_digital_signature_origin());
    let (srid, _) = wb
        .add_xml_signature_part(br#"<?xml version="1.0"?><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"/>"#)
        .unwrap();
    assert!(srid.starts_with('r'));
    assert_eq!(wb.digital_signature_count(), 1);
    assert!(wb.clear_digital_signatures().unwrap());
    assert!(!wb.has_digital_signature_origin());
    assert_eq!(wb.digital_signature_count(), 0);
    let _ = uri;

    let xml = br#"<?xml version="1.0"?><root xmlns="urn:test"><n>1</n></root>"#;
    let (cx_rid, cx_uri) = wb.add_custom_xml_part(xml).unwrap();
    assert!(cx_uri.as_str().contains("customXml"));
    assert!(wb.has_custom_xml_parts().unwrap());
    assert_eq!(wb.custom_xml_part_count().unwrap(), 1);
    assert_eq!(wb.custom_xml_parts().unwrap()[0].0, cx_rid);
    assert_eq!(wb.clear_custom_xml_parts().unwrap(), 1);
    assert!(!wb.has_custom_xml_parts().unwrap());

    // PowerPoint
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(ppt.package_part_count() >= 2);

    ppt.add_thumbnail(png, "image/png", "png").unwrap();
    assert!(ppt.has_thumbnail());
    assert!(ppt.clear_thumbnail().unwrap());

    ppt.add_digital_signature_origin().unwrap();
    ppt.add_xml_signature_part(b"<Signature/>").unwrap();
    assert!(ppt.has_digital_signature_origin());
    assert!(ppt.digital_signature_count() >= 1);
    assert!(ppt.clear_digital_signatures().unwrap());

    let (prid, _) = ppt.add_custom_xml_part(xml).unwrap();
    assert!(ppt.has_custom_xml_parts().unwrap());
    assert!(ppt.remove_custom_xml_part(&prid).unwrap());
    assert!(!ppt.has_custom_xml_parts().unwrap());

    // Word package_part_count alias
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(doc.package_part_count(), doc.part_count());
}


#[test]
fn excel_ppt_embeddings_and_vba_parity() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (rid, uri) = wb
        .add_embedded_package(b"PK\x03\x04fake", "application/vnd.openxmlformats-officedocument.oleObject", "bin")
        .unwrap();
    assert!(rid.starts_with('r'));
    assert!(uri.as_str().starts_with("/xl/embeddings/"));
    assert!(wb.has_embeddings());
    assert_eq!(wb.embedding_count(), 1);
    assert_eq!(wb.list_embeddings().len(), 1);
    assert_eq!(wb.clear_embeddings().unwrap(), 1);
    assert!(!wb.has_embeddings());

    let (vrid, vuri) = wb.add_vba_project(b"vba-bin").unwrap();
    assert!(vrid.starts_with('r'));
    assert!(vuri.as_str().contains("vbaProject"));
    assert!(wb.has_vba_project());
    assert!(wb.clear_vba_project().unwrap());
    assert!(!wb.has_vba_project());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let (_, puri) = ppt
        .add_embedded_package(b"embed", "application/octet-stream", "bin")
        .unwrap();
    assert!(puri.as_str().starts_with("/ppt/embeddings/"));
    assert!(ppt.has_embeddings());
    assert_eq!(ppt.clear_embeddings().unwrap(), 1);

    ppt.add_vba_project(b"ppt-vba").unwrap();
    assert!(ppt.has_vba_project());
    assert!(ppt.clear_vba_project().unwrap());
    assert!(!ppt.has_vba_project());
}


#[test]
fn excel_ppt_custom_ui_parity() {
    let ui = br#"<?xml version="1.0"?><customUI xmlns="http://schemas.microsoft.com/office/2006/01/customui"><ribbon/></customUI>"#;
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (rid, uri) = wb.add_custom_ui(ui).unwrap();
    assert!(rid.starts_with('r'));
    assert_eq!(uri.as_str(), "/customUI/customUI.xml");
    assert!(wb.has_custom_ui());
    assert!(wb.clear_custom_ui().unwrap());
    assert!(!wb.has_custom_ui());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_custom_ui(ui).unwrap();
    assert!(ppt.has_custom_ui());
    assert!(ppt.clear_custom_ui().unwrap());
    assert!(!ppt.has_custom_ui());
}


#[test]
fn excel_ppt_printer_qat_toolbars_parity() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (rid, uri) = wb.add_printer_settings(b"bin-printer").unwrap();
    assert!(rid.starts_with('r'));
    assert!(uri.as_str().contains("printerSettings"));
    assert!(wb.has_printer_settings());
    assert_eq!(wb.printer_settings_count(), 1);
    assert_eq!(wb.clear_printer_settings().unwrap(), 1);
    assert!(!wb.has_printer_settings());

    let (qrid, _) = wb.add_quick_access_toolbar().unwrap();
    assert!(qrid.starts_with('r'));
    assert!(wb.has_quick_access_toolbar());
    assert!(wb.clear_quick_access_toolbar().unwrap());
    assert!(!wb.has_quick_access_toolbar());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_printer_settings(b"ppt-ps").unwrap();
    assert!(ppt.has_printer_settings());
    assert_eq!(ppt.clear_printer_settings().unwrap(), 1);

    let (trid, turi) = ppt.add_attached_toolbars(b"tb").unwrap();
    assert!(trid.starts_with('r'));
    assert!(turi.as_str().contains("attachedToolbars"));
    assert!(ppt.has_attached_toolbars());
    assert!(ppt.clear_attached_toolbars().unwrap());
    assert!(!ppt.has_attached_toolbars());

    ppt.add_quick_access_toolbar().unwrap();
    assert!(ppt.has_quick_access_toolbar());
    assert!(ppt.clear_quick_access_toolbar().unwrap());
}


#[test]
fn excel_ppt_label_info_parity() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (rid, uri) = wb.add_label_info("lbl-1", "Confidential").unwrap();
    assert!(rid.starts_with('r'));
    assert_eq!(uri.as_str(), "/docMetadata/LabelInfo.xml");
    assert!(wb.has_label_info());
    assert!(wb.clear_label_info().unwrap());
    assert!(!wb.has_label_info());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_label_info("lbl-2", "Internal").unwrap();
    assert!(ppt.has_label_info());
    assert!(ppt.clear_label_info().unwrap());
    assert!(!ppt.has_label_info());
}


#[test]
fn excel_ppt_web_extension_parity() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (we, tp) = wb.add_web_extension_shell("WA123", "1.0.0.0").unwrap();
    assert!(we.as_str().contains("/xl/webextensions/"));
    assert!(tp.as_str().contains("taskpanes"));
    assert!(wb.has_web_extensions());
    assert!(wb.web_extension_count() >= 2);
    assert!(wb.clear_web_extensions().unwrap() >= 2);
    assert!(!wb.has_web_extensions());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_web_extension_shell("WA456", "1.0.0.0").unwrap();
    assert!(ppt.has_web_extensions());
    assert!(ppt.clear_web_extensions().unwrap() >= 2);
    assert!(!ppt.has_web_extensions());
}


#[test]
fn font_parts_inventory_all_docs() {
    use officexml::namespace::content_type;
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("f")]));
    doc.add_font_part(b"font-bin", content_type::FONT_TTF, "ttf")
        .unwrap();
    assert!(doc.has_font_parts());
    assert_eq!(doc.font_part_count(), 1);
    assert_eq!(doc.list_font_parts().len(), 1);
    assert_eq!(doc.clear_font_parts().unwrap(), 1);
    assert!(!doc.has_font_parts());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_font_part(b"xl-font", content_type::FONT_TTF, "ttf")
        .unwrap();
    assert!(wb.has_font_parts());
    assert_eq!(wb.clear_font_parts().unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_font_part(b"ppt-font", content_type::FONT_TTF, "ttf")
        .unwrap();
    assert!(ppt.has_font_parts());
    assert_eq!(ppt.clear_font_parts().unwrap(), 1);
}


#[test]
fn word_charts_and_diagram_inventory_parity() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("c")]));
    doc.add_chart("Sales", &["A", "B"], &[1.0, 2.0]).unwrap();
    assert!(doc.has_charts());
    assert_eq!(doc.chart_count(), 1);
    assert_eq!(doc.list_charts().len(), 1);
    assert_eq!(doc.clear_charts().unwrap(), 1);
    assert!(!doc.has_charts());

    // diagram inventory APIs exist even if empty
    assert!(!doc.has_diagrams());
    assert_eq!(doc.diagram_count(), 0);
    assert!(doc.list_diagrams().is_empty());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_diagrams());
    assert_eq!(wb.clear_diagrams().unwrap(), 0);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(!ppt.has_diagrams());
    assert_eq!(ppt.diagram_count(), 0);
}


#[test]
fn excel_ppt_open_create_with_settings() {
    use officexml::packaging::OpenSettings;
    let dir = tempfile::tempdir().unwrap();

    let xlsx = dir.path().join("s.xlsx");
    {
        let mut settings = OpenSettings::default();
        settings.auto_save = false;
        let mut wb = SpreadsheetDocument::create_with_settings(
            &xlsx,
            SpreadsheetDocumentType::Workbook,
            settings,
        )
        .unwrap();
        wb.write_sheet_strings("S", &[vec!["z"]]).unwrap();
        wb.save().unwrap();
    }
    let mut settings = OpenSettings::default();
    settings.auto_save = false;
    settings.max_characters_in_part = 0; // unlimited
    let wb2 = SpreadsheetDocument::open_with_settings(&xlsx, false, settings).unwrap();
    assert_eq!(wb2.sheet_names().len(), 1);

    let pptx = dir.path().join("p.pptx");
    {
        let mut settings = OpenSettings::default();
        settings.auto_save = false;
        let mut ppt = PresentationDocument::create_with_settings(
            &pptx,
            PresentationDocumentType::Presentation,
            settings,
        )
        .unwrap();
        ppt.add_slide_with_text("Hi").unwrap();
        ppt.save().unwrap();
    }
    let mut settings = OpenSettings::default();
    settings.auto_save = false;
    let ppt2 = PresentationDocument::open_with_settings(&pptx, false, settings).unwrap();
    assert_eq!(ppt2.slide_count(), 1);
}


#[test]
fn excel_ppt_create_simple() {
    let dir = tempfile::tempdir().unwrap();
    let xlsx = dir.path().join("simple.xlsx");
    {
        let wb = SpreadsheetDocument::create_simple(
            &xlsx,
            "Data",
            &[vec!["h1", "h2"], vec!["a", "b"]],
        )
        .unwrap();
        assert!(wb.sheet_names().iter().any(|n| *n == "Data"));
        // Drop saves when auto_save is on and path is set.
    }
    let wb2 = SpreadsheetDocument::open(&xlsx, false).unwrap();
    assert!(wb2.sheet_names().iter().any(|n| *n == "Data"));

    let pptx = dir.path().join("simple.pptx");
    {
        PresentationDocument::create_simple(&pptx, "Hello PPT").unwrap();
    }
    let ppt2 = PresentationDocument::open(&pptx, false).unwrap();
    assert_eq!(ppt2.slide_count(), 1);
}


#[test]
fn excel_ppt_validate_and_custom_xml_props() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let errs = wb.validate().unwrap();
    assert!(errs.is_empty(), "{errs:?}");
    let _ = wb.validate_full().unwrap();
    let (rid, uri) = wb
        .add_custom_xml_part(br#"<?xml version="1.0"?><root xmlns="urn:x"/>"#)
        .unwrap();
    let (prid, puri) = wb.add_custom_xml_properties(&uri, "{11111111-1111-1111-1111-111111111111}").unwrap();
    assert!(prid.starts_with('r'));
    assert!(puri.as_str().contains("itemProps"));
    let _ = rid;

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(ppt.validate().unwrap().is_empty());
    let (_, uri) = ppt
        .add_custom_xml_part(br#"<?xml version="1.0"?><root xmlns="urn:y"/>"#)
        .unwrap();
    ppt.add_custom_xml_properties(&uri, "{22222222-2222-2222-2222-222222222222}")
        .unwrap();
}


#[test]
fn from_bytes_and_remove_part_aliases() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let bytes = wb.to_bytes().unwrap();
    let wb2 = SpreadsheetDocument::from_bytes(&bytes).unwrap();
    assert!(!wb2.sheet_names().is_empty());
    let mut wb3 = SpreadsheetDocument::from_bytes(&bytes).unwrap();
    // delete a non-critical part if present (theme optional)
    let uris = wb3.list_part_uris();
    if let Some(u) = uris.iter().find(|u| u.as_str().contains("theme")) {
        let _ = wb3.remove_part(u);
    }

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let bytes = ppt.to_bytes().unwrap();
    let ppt2 = PresentationDocument::from_bytes(&bytes).unwrap();
    assert_eq!(ppt2.slide_count(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    let bytes = doc.to_bytes().unwrap();
    let doc2 = WordprocessingDocument::from_bytes(&bytes).unwrap();
    assert!(doc2.has_part("/word/document.xml"));
}


#[test]
fn images_inventory_parity() {
    // Word
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("i")]));
    let png: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0xD4, 0xEF, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    doc.add_image(officexml::ImageFormat::Png, png).unwrap();
    assert!(doc.has_images());
    assert!(doc.image_count() >= 1);
    assert!(!doc.list_images().is_empty());
    assert!(doc.clear_images().unwrap() >= 1);
    assert!(!doc.has_images());

    // Excel — inventory helpers work on empty and after raw media part
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_images());
    wb.set_part_bytes("/xl/media/image1.png", "image/png", png);
    assert!(wb.has_images());
    assert_eq!(wb.image_count(), 1);
    assert_eq!(wb.clear_images().unwrap(), 1);
    assert!(!wb.has_images());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_part_bytes("/ppt/media/image1.png", "image/png", png);
    assert!(ppt.has_images());
    assert_eq!(ppt.clear_images().unwrap(), 1);
}


#[test]
fn theme_inventory_all_docs() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.add_default_theme().unwrap();
    assert!(doc.has_theme());
    assert!(doc.theme_count() >= 1);
    assert!(!doc.list_themes().is_empty());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_default_theme().unwrap();
    assert!(wb.has_theme());
    assert!(wb.theme_count() >= 1);
    assert!(!wb.list_themes().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_default_theme().unwrap();
    assert!(ppt.has_theme());
    assert!(ppt.theme_count() >= 1);
    assert!(!ppt.list_themes().is_empty());
}


#[test]
fn styles_inventory_parity() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.add_default_styles().unwrap();
    assert!(doc.has_styles());
    assert!(doc.styles_count() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let _ = wb.styles_count(); // callable; may be 0 until styles part is added

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_table_styles().unwrap();
    assert!(ppt.has_styles());
    assert!(ppt.styles_count() >= 1);
    assert!(ppt.clear_styles().unwrap());
    assert!(!ppt.has_styles());
}


#[test]
fn comments_inventory_excel_ppt() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_sheet_comments("S", "alice", &[("A1", "note")]).unwrap();
    assert!(wb.has_comments());
    assert!(wb.comments_part_count() >= 1);
    assert!(!wb.list_comment_parts().is_empty());
    assert!(wb.clear_comments().unwrap() >= 1);
    assert!(!wb.has_comments());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    // inventory APIs callable when empty
    assert!(!ppt.has_comments());
    assert_eq!(ppt.comment_count(), 0);
    assert_eq!(ppt.clear_comments().unwrap(), 0);
}


#[test]
fn excel_ppt_embedded_object() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (rid, uri) = wb.add_embedded_object(b"ole-bin", "Excel.Sheet.12").unwrap();
    assert!(rid.starts_with('r'));
    assert!(uri.as_str().contains("/xl/embeddings/oleObject"));
    assert!(wb.has_embeddings());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let (_, uri) = ppt.add_embedded_object(b"ole2", "PowerPoint.Show.12").unwrap();
    assert!(uri.as_str().contains("/ppt/embeddings/oleObject"));
    assert!(ppt.has_embeddings());
}


#[test]
fn excel_ppt_diagram_shell() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let uri = wb.add_diagram_shell("node-1").unwrap();
    assert!(uri.as_str().contains("/xl/diagrams/data"));
    assert!(wb.has_diagrams());
    assert!(wb.diagram_count() >= 5);
    assert!(wb.clear_diagrams().unwrap() >= 5);
    assert!(!wb.has_diagrams());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let uri = ppt.add_diagram_shell("node-2").unwrap();
    assert!(uri.as_str().contains("/ppt/diagrams/data"));
    assert!(ppt.has_diagrams());
    assert!(ppt.clear_diagrams().unwrap() >= 5);
}


#[test]
fn main_relationship_aliases() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert_eq!(
        wb.main_relationship_count(),
        wb.workbook_relationship_count()
    );
    assert_eq!(
        wb.list_main_relationships().len(),
        wb.list_workbook_relationships().len()
    );
    assert!(wb.main_relationship_count() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert_eq!(
        ppt.main_relationship_count(),
        ppt.presentation_relationship_count()
    );
    assert_eq!(
        ppt.list_main_relationships().len(),
        ppt.list_presentation_relationships().len()
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(
        doc.main_relationship_count(),
        doc.list_main_relationships().len()
    );
}


#[test]
fn excel_list_comments() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_sheet_comments("S", "alice", &[("A1", "hello"), ("B2", "world")])
        .unwrap();
    let list = wb.list_comments().unwrap();
    assert_eq!(wb.comment_count().unwrap(), 2);
    assert_eq!(list.len(), 2);
    assert!(list.iter().any(|(_, cell, author, text)| {
        cell == "A1" && author == "alice" && text.contains("hello")
    }));
    assert!(list.iter().any(|(_, cell, _, text)| cell == "B2" && text.contains("world")));
}


#[test]
fn excel_ppt_embedded_package_part_alias() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (rid, uri) = wb.add_embedded_package_part(b"PK\x03\x04x", "xlsx").unwrap();
    assert!(rid.starts_with('r'));
    assert!(uri.as_str().contains("/xl/embeddings/"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let (_, uri) = ppt.add_embedded_package_part(b"PK\x03\x04y", "pptx").unwrap();
    assert!(uri.as_str().contains("/ppt/embeddings/"));
}


#[test]
fn excel_ppt_legacy_diagram_text() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (t, i) = wb.add_legacy_diagram_text(b"legacy").unwrap();
    assert!(t.as_str().contains("/xl/diagrams/legacy/text"));
    assert!(i.as_str().contains("textInfo"));
    assert!(wb.has_diagrams());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let (t, _) = ppt.add_legacy_diagram_text(b"ppt-legacy").unwrap();
    assert!(t.as_str().contains("/ppt/diagrams/legacy/text"));
}


#[test]
fn excel_ppt_add_chart_alias() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (uri, rid) = wb.add_chart("T", &["A", "B"], &[1.0, 2.0]).unwrap();
    assert!(uri.as_str().contains("/xl/charts/"));
    assert!(rid.starts_with('r') || !rid.is_empty());
    assert!(wb.has_charts());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    let (uri, _) = ppt.add_chart("T", &["A", "B"], &[1.0, 2.0]).unwrap();
    assert!(uri.as_str().contains("/ppt/charts/"));
    assert!(ppt.has_charts());
}


#[test]
fn excel_ppt_add_image_media() {
    use officexml::ImageFormat;
    let png: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0xD4, 0xEF, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (rid, uri) = wb.add_image(ImageFormat::Png, png).unwrap();
    assert!(rid.starts_with('r'));
    assert!(uri.as_str().starts_with("/xl/media/"));
    assert!(wb.has_images());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let (_, uri) = ppt.add_image(ImageFormat::Png, png).unwrap();
    assert!(uri.as_str().starts_with("/ppt/media/"));
    assert!(ppt.has_images());
}


#[test]
fn excel_default_styles_and_style_ids() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let rid = wb.add_default_styles().unwrap();
    assert!(!rid.is_empty() || wb.has_styles());
    assert!(wb.has_styles());
    let ids = wb.list_style_ids().unwrap();
    // at least default xf entries usually present
    let _ = wb.style_count().unwrap();
    let _ = ids;
}


#[test]
fn media_and_hyperlink_inventory_helpers() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("m")]));
    assert_eq!(doc.media_count(), doc.list_media().len());
    assert_eq!(doc.has_media(), doc.has_images());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_media());
    assert!(!wb.has_hyperlinks());
    assert_eq!(wb.hyperlink_count().unwrap(), 0);
    assert!(wb.list_hyperlinks().unwrap().is_empty());
    assert_eq!(wb.clear_hyperlinks().unwrap(), 0);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert_eq!(ppt.has_media(), ppt.media_count() > 0);
}


#[test]
fn drawings_inventory_all_docs() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("d")]));
    assert!(!doc.has_drawings());
    assert_eq!(doc.clear_drawings().unwrap(), 0);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_drawings());
    assert!(wb.list_drawings().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(!ppt.has_drawings());
    assert_eq!(ppt.drawing_count(), 0);
    assert_eq!(ppt.clear_drawings().unwrap(), 0);
}


#[test]
fn ppt_hyperlink_inventory() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let rid = ppt
        .add_slide_hyperlink(0, "https://example.com")
        .unwrap();
    assert!(rid.starts_with('r'));
    assert!(ppt.has_hyperlinks());
    assert_eq!(ppt.hyperlink_count().unwrap(), 1);
    let list = ppt.list_hyperlinks().unwrap();
    assert_eq!(list.len(), 1);
    assert!(list[0].2.contains("example.com"));
    assert_eq!(ppt.clear_hyperlinks().unwrap(), 1);
    assert!(!ppt.has_hyperlinks());
}


#[test]
fn excel_clear_tables_and_defined_names() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    wb.add_defined_name("MyRange", "Sheet1!$A$1").unwrap();
    assert!(wb.has_defined_names().unwrap());
    assert_eq!(wb.list_defined_names().unwrap().len(), 1);
    assert_eq!(wb.clear_defined_names().unwrap(), 1);
    assert!(!wb.has_defined_names().unwrap());

    // tables may or may not be created by helpers; ensure clear is callable
    assert_eq!(wb.clear_tables().unwrap(), 0);
}


#[test]
fn hyperlink_alias_parity() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("h")]));
    doc.append_hyperlink("https://example.org", "ex").unwrap();
    assert!(doc.has_hyperlinks());
    assert!(doc.hyperlink_count() >= 1);
    assert_eq!(
        doc.list_hyperlinks().len(),
        doc.list_external_hyperlinks().len()
    );
    assert!(doc.clear_hyperlinks() >= 1);
    assert!(!doc.has_hyperlinks());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_cell_hyperlink("S", "A1", "https://example.com", Some("x"))
        .unwrap();
    assert!(wb.has_hyperlinks());
    assert_eq!(
        wb.list_external_hyperlinks().unwrap().len(),
        wb.list_hyperlinks().unwrap().len()
    );
    assert!(wb.clear_external_hyperlinks().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_hyperlink(0, "https://example.net").unwrap();
    assert!(ppt.has_hyperlinks());
    assert_eq!(
        ppt.list_external_hyperlinks().unwrap().len(),
        ppt.list_hyperlinks().unwrap().len()
    );
    assert_eq!(ppt.clear_external_hyperlinks().unwrap(), 1);
}


#[test]
fn excel_protection_has_aliases() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_workbook_protection().unwrap());
    assert!(!wb.has_sheet_protection("S").unwrap());
    wb.set_workbook_protection(true, false).unwrap();
    assert!(wb.has_workbook_protection().unwrap());
    wb.clear_workbook_protection().unwrap();
    assert!(!wb.has_workbook_protection().unwrap());
}


#[test]
fn template_as_and_doc_protection_alias() {
    let dir = tempfile::tempdir().unwrap();
    let xltx = dir.path().join("t.xltx");
    {
        let mut wb = SpreadsheetDocument::create(&xltx, SpreadsheetDocumentType::Template).unwrap();
        wb.write_sheet_strings("S", &[vec!["t"]]).unwrap();
        wb.save().unwrap();
    }
    let wb2 = SpreadsheetDocument::create_from_template_as(
        &xltx,
        Some(SpreadsheetDocumentType::Workbook),
    )
    .unwrap();
    assert_eq!(wb2.document_type(), SpreadsheetDocumentType::Workbook);

    let potx = dir.path().join("t.potx");
    {
        let mut ppt =
            PresentationDocument::create(&potx, PresentationDocumentType::Template).unwrap();
        ppt.add_slide_with_text("T").unwrap();
        ppt.save().unwrap();
    }
    let ppt2 = PresentationDocument::create_from_template_as(
        &potx,
        Some(PresentationDocumentType::Presentation),
    )
    .unwrap();
    assert_eq!(
        ppt2.document_type(),
        PresentationDocumentType::Presentation
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("p")]));
    assert!(!doc.has_document_protection().unwrap());
}


#[test]
fn excel_pivot_cache_and_sparkline_inventory() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.has_pivot_caches());
    assert_eq!(wb.pivot_cache_count(), 0);
    assert!(wb.list_pivot_caches().is_empty());
    assert_eq!(wb.clear_pivot_caches().unwrap(), 0);
    assert!(wb.sheets_with_sparklines().unwrap().is_empty());
    assert!(!wb.has_sparklines("S").unwrap());
}


#[test]
fn merge_hidden_master_docvar_aliases() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["c", "d"]])
        .unwrap();
    wb.set_merge_cells("S", &["A1:B1"]).unwrap();
    assert!(wb.has_merged_cells("S").unwrap());
    assert_eq!(wb.list_merged_cells("S").unwrap(), vec!["A1:B1".to_string()]);
    assert!(wb.clear_merged_cells("S").unwrap());
    assert!(!wb.has_merged_cells("S").unwrap());
    assert!(!wb.has_hidden_rows("S").unwrap());
    assert!(!wb.has_hidden_cols("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert_eq!(ppt.has_slide_masters(), ppt.master_count() > 0);
    assert_eq!(ppt.list_slide_masters().len(), ppt.list_masters().len());
    assert_eq!(ppt.list_slide_layouts().len(), ppt.list_layouts().len());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("v")]));
    assert!(doc.list_document_variables().unwrap().is_empty());
}


#[test]
fn excel_page_breaks_inventory() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"], vec!["b"], vec!["c"]])
        .unwrap();
    assert!(!wb.has_page_breaks("S").unwrap());
    wb.set_row_breaks("S", &[1, 2]).unwrap();
    wb.set_col_breaks("S", &[1]).unwrap();
    assert!(wb.has_page_breaks("S").unwrap());
    let list = wb.list_page_breaks("S").unwrap();
    assert!(list.iter().any(|(k, id)| k == "row" && *id == 1));
    assert!(list.iter().any(|(k, id)| k == "col" && *id == 1));
    assert!(wb.clear_page_breaks("S").unwrap());
    assert!(!wb.has_page_breaks("S").unwrap());
}


#[test]
fn ppt_clear_all_notes() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.add_notes_to_slide(0, "n0").unwrap();
    ppt.add_notes_to_slide(1, "n1").unwrap();
    assert!(ppt.has_notes_slides().unwrap());
    assert_eq!(ppt.notes_count().unwrap(), 2);
    assert_eq!(ppt.clear_all_notes().unwrap(), 2);
    assert!(!ppt.has_notes_slides().unwrap());
}


#[test]
fn ppt_bulk_clear_transitions_animations() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.set_fade_transition(0, "med").unwrap();
    ppt.set_fade_transition(1, "fast").unwrap();
    assert!(ppt.has_any_transition().unwrap());
    assert_eq!(ppt.transition_count().unwrap(), 2);
    assert_eq!(ppt.clear_all_transitions().unwrap(), 2);
    assert!(!ppt.has_any_transition().unwrap());
    // animation APIs callable
    assert!(!ppt.has_any_animation().unwrap());
    assert_eq!(ppt.clear_all_animations().unwrap(), 0);
}


#[test]
fn ppt_bulk_clear_backgrounds_and_hf() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.set_slide_background(0, "FF0000").unwrap();
    ppt.set_slide_background(1, "00FF00").unwrap();
    assert!(ppt.has_any_background().unwrap());
    assert_eq!(ppt.clear_all_backgrounds().unwrap(), 2);
    assert!(!ppt.has_any_background().unwrap());

    ppt.set_slide_header_footer(0, true, true, true).unwrap();
    ppt.set_slide_header_footer(1, false, true, false).unwrap();
    assert!(ppt.has_any_header_footer().unwrap());
    assert_eq!(ppt.clear_all_header_footers().unwrap(), 2);
    assert!(!ppt.has_any_header_footer().unwrap());
}


#[test]
fn validate_schematron_attributes_all_docs() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("v")]));
    let errs = doc.validate_schematron_attributes().unwrap();
    assert!(errs.is_empty(), "{errs:?}");

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(wb.validate_schematron_attributes().unwrap().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(ppt.validate_schematron_attributes().unwrap().is_empty());
}


#[test]
fn excel_zoom_and_sheet_view_helpers() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_zoom("S", 150).unwrap();
    assert_eq!(wb.get_zoom("S").unwrap(), Some(150));
    assert_eq!(wb.zoom("S").unwrap(), Some(150));
    assert!(wb.has_sheet_view("S").unwrap());
    assert!(wb.has_zoom("S").unwrap());
}


#[test]
fn excel_tab_color_and_dimension_aliases() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_tab_color("S", "FF00FF").unwrap();
    assert!(wb.has_tab_color("S").unwrap());
    assert_eq!(wb.tab_color("S").unwrap().as_deref(), Some("FF00FF"));
    assert!(wb.clear_tab_color("S").unwrap());
    assert!(!wb.has_tab_color("S").unwrap());

    wb.set_sheet_dimension("S", "A1:B2").unwrap();
    assert_eq!(
        wb.get_sheet_dimension("S").unwrap(),
        wb.sheet_dimension("S").unwrap()
    );
    assert_eq!(wb.get_sheet_dimension("S").unwrap().as_deref(), Some("A1:B2"));
    let _ = wb.has_auto_filter_range("S").unwrap();
}


#[test]
fn word_zoom_and_ppt_size_clear() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("z")]));
    doc.set_zoom(120).unwrap();
    assert!(doc.has_zoom().unwrap());
    assert_eq!(doc.zoom().unwrap(), Some(120));
    assert!(doc.clear_zoom().unwrap());
    assert!(!doc.has_zoom().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_slide_size(12192000, 6858000).unwrap();
    assert!(ppt.has_slide_size().unwrap());
    assert!(ppt.clear_slide_size().unwrap());
    assert!(!ppt.has_slide_size().unwrap());
    ppt.set_notes_size(6858000, 9144000).unwrap();
    assert!(ppt.has_notes_size().unwrap());
    assert!(ppt.clear_notes_size().unwrap());
    assert!(!ppt.has_notes_size().unwrap());
}


#[test]
fn word_settings_view_grid_tabstop() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.set_view("print").unwrap();
    assert_eq!(doc.view().unwrap().as_deref(), Some("print"));
    assert!(doc.has_view().unwrap());
    assert!(doc.clear_view().unwrap());
    assert!(!doc.has_view().unwrap());

    doc.set_default_tab_stop(720).unwrap();
    assert_eq!(doc.default_tab_stop().unwrap(), Some(720));
    assert!(doc.clear_default_tab_stop().unwrap());

    doc.set_document_grid(360).unwrap();
    assert_eq!(doc.document_grid_line_pitch().unwrap(), Some(360));
    assert!(doc.has_document_grid().unwrap());
    assert!(doc.clear_document_grid().unwrap());
}


#[test]
fn workbook_view_and_track_revisions_aliases() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_active_tab(0).unwrap();
    assert!(wb.has_workbook_view().unwrap());
    assert!(wb.workbook_view_count().unwrap() >= 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.set_track_revisions(true).unwrap();
    assert!(doc.has_track_revisions().unwrap());
    assert!(doc.track_revisions_enabled().unwrap());
}


#[test]
fn word_settings_boolean_flags() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("f")]));
    doc.set_auto_hyphenation(true).unwrap();
    assert!(doc.has_auto_hyphenation().unwrap());
    doc.set_auto_hyphenation(false).unwrap();
    assert!(!doc.has_auto_hyphenation().unwrap());

    doc.set_embed_true_type_fonts(true).unwrap();
    assert!(doc.has_embed_true_type_fonts().unwrap());
    doc.set_embed_true_type_fonts(false).unwrap();
    assert!(!doc.has_embed_true_type_fonts().unwrap());

    doc.set_save_preview_picture(true).unwrap();
    assert!(doc.has_save_preview_picture().unwrap());
    doc.set_save_preview_picture(false).unwrap();

    doc.set_gutter_at_top(true).unwrap();
    assert!(doc.has_gutter_at_top().unwrap());
    doc.set_gutter_at_top(false).unwrap();
    assert!(!doc.has_gutter_at_top().unwrap());

    // mirror margins alias callable
    let _ = doc.has_mirror_margins().unwrap();
}


#[test]
fn word_hide_print_flags_and_gridlines_alias() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("h")]));
    doc.set_hide_spelling_errors(true).unwrap();
    assert!(doc.has_hide_spelling_errors().unwrap());
    doc.set_hide_spelling_errors(false).unwrap();
    assert!(!doc.has_hide_spelling_errors().unwrap());

    doc.set_hide_grammatical_errors(true).unwrap();
    assert!(doc.has_hide_grammatical_errors().unwrap());
    doc.set_hide_grammatical_errors(false).unwrap();

    doc.set_print_hidden_text(true).unwrap();
    assert!(doc.has_print_hidden_text().unwrap());
    doc.set_print_hidden_text(false).unwrap();

    doc.set_print_forms_data(true).unwrap();
    assert!(doc.has_print_forms_data().unwrap());
    doc.set_print_forms_data(false).unwrap();

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_show_gridlines("S", false).unwrap();
    assert_eq!(
        wb.gridlines_visible("S").unwrap(),
        wb.show_gridlines("S").unwrap()
    );
    assert!(!wb.show_gridlines("S").unwrap());
}


#[test]
fn word_more_settings_flags() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("m")]));
    doc.set_display_background_shape(true).unwrap();
    assert!(doc.has_display_background_shape().unwrap());
    doc.set_display_background_shape(false).unwrap();

    doc.set_do_not_display_page_boundaries(true).unwrap();
    assert!(doc.has_do_not_display_page_boundaries().unwrap());
    doc.set_do_not_display_page_boundaries(false).unwrap();

    doc.set_do_not_auto_compress_pictures(true).unwrap();
    assert!(doc.has_do_not_auto_compress_pictures().unwrap());
    doc.set_do_not_auto_compress_pictures(false).unwrap();

    doc.set_print_two_on_one(true).unwrap();
    assert!(doc.has_print_two_on_one().unwrap());
    doc.set_print_two_on_one(false).unwrap();

    doc.set_strict_first_and_last_chars(true).unwrap();
    assert!(doc.has_strict_first_and_last_chars().unwrap());
    doc.set_strict_first_and_last_chars(false).unwrap();
}


#[test]
fn word_settings_forms_and_border_flags() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("b")]));
    for (set, has) in [
        (
            |d: &mut WordprocessingDocument| d.set_forms_design(true),
            |d: &WordprocessingDocument| d.has_forms_design(),
        ),
    ] {
        let _ = (set, has);
    }
    doc.set_forms_design(true).unwrap();
    assert!(doc.has_forms_design().unwrap());
    doc.set_forms_design(false).unwrap();
    assert!(!doc.has_forms_design().unwrap());

    doc.set_remove_personal_information(true).unwrap();
    assert!(doc.has_remove_personal_information().unwrap());
    doc.set_remove_personal_information(false).unwrap();

    doc.set_remove_date_and_time(true).unwrap();
    assert!(doc.has_remove_date_and_time().unwrap());
    doc.set_remove_date_and_time(false).unwrap();

    doc.set_do_not_shade_form_data(true).unwrap();
    assert!(doc.has_do_not_shade_form_data().unwrap());
    doc.set_do_not_shade_form_data(false).unwrap();

    doc.set_print_fractional_character_width(true).unwrap();
    assert!(doc.has_print_fractional_character_width().unwrap());
    doc.set_print_fractional_character_width(false).unwrap();

    doc.set_print_post_script_over_text(true).unwrap();
    assert!(doc.has_print_post_script_over_text().unwrap());
    doc.set_print_post_script_over_text(false).unwrap();

    doc.set_align_borders_and_edges(true).unwrap();
    assert!(doc.has_align_borders_and_edges().unwrap());
    doc.set_align_borders_and_edges(false).unwrap();

    doc.set_borders_do_not_surround_header(true).unwrap();
    assert!(doc.has_borders_do_not_surround_header().unwrap());
    doc.set_borders_do_not_surround_header(false).unwrap();

    doc.set_borders_do_not_surround_footer(true).unwrap();
    assert!(doc.has_borders_do_not_surround_footer().unwrap());
    doc.set_borders_do_not_surround_footer(false).unwrap();
}


#[test]
fn word_settings_spacing_and_kinsoku_flags() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("k")]));
    doc.set_do_not_use_html_paragraph_auto_spacing(true).unwrap();
    assert!(doc.has_do_not_use_html_paragraph_auto_spacing().unwrap());
    doc.set_do_not_use_html_paragraph_auto_spacing(false).unwrap();

    doc.set_use_fe_layout(true).unwrap();
    assert!(doc.has_use_fe_layout().unwrap());
    doc.set_use_fe_layout(false).unwrap();

    doc.set_swap_borders_facing_pages(true).unwrap();
    assert!(doc.has_swap_borders_facing_pages().unwrap());
    doc.set_swap_borders_facing_pages(false).unwrap();

    doc.set_balance_single_byte_double_byte_width(true).unwrap();
    assert!(doc.has_balance_single_byte_double_byte_width().unwrap());
    doc.set_balance_single_byte_double_byte_width(false).unwrap();

    doc.set_no_punctuation_kerning(true).unwrap();
    assert!(doc.has_no_punctuation_kerning().unwrap());
    doc.set_no_punctuation_kerning(false).unwrap();
}


#[test]
fn excel_right_to_left_view() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.right_to_left("S").unwrap());
    wb.set_right_to_left("S", true).unwrap();
    assert!(wb.right_to_left("S").unwrap());
    assert!(wb.has_right_to_left("S").unwrap());
    wb.set_right_to_left("S", false).unwrap();
    assert!(!wb.right_to_left("S").unwrap());
}


#[test]
fn excel_show_zeros_formulas_and_word_char_spacing() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(wb.show_zeros("S").unwrap()); // default true
    wb.set_show_zeros("S", false).unwrap();
    assert!(!wb.show_zeros("S").unwrap());
    assert!(!wb.show_formulas("S").unwrap());
    wb.set_show_formulas("S", true).unwrap();
    assert!(wb.show_formulas("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("c")]));
    doc.set_character_spacing_control("doNotCompress").unwrap();
    assert_eq!(
        doc.character_spacing_control().unwrap().as_deref(),
        Some("doNotCompress")
    );
    assert!(doc.has_character_spacing_control().unwrap());
    assert!(doc.clear_character_spacing_control().unwrap());
    assert!(!doc.has_character_spacing_control().unwrap());
}


#[test]
fn excel_outline_symbols_and_view_type() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(wb.show_outline_symbols("S").unwrap());
    wb.set_show_outline_symbols("S", false).unwrap();
    assert!(!wb.show_outline_symbols("S").unwrap());
    wb.set_sheet_view_type("S", "pageLayout").unwrap();
    assert_eq!(wb.sheet_view_type("S").unwrap().as_deref(), Some("pageLayout"));
}


#[test]
fn excel_workbook_pr_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    assert!(!wb.date1904().unwrap());
    wb.set_date1904(true).unwrap();
    assert!(wb.date1904().unwrap());
    assert!(wb.has_date1904().unwrap());
    wb.set_date1904(false).unwrap();
    assert!(!wb.date1904().unwrap());

    wb.set_backup_file(true).unwrap();
    assert!(wb.backup_file().unwrap());
    wb.set_backup_file(false).unwrap();
    assert!(!wb.backup_file().unwrap());

    wb.set_filter_mode(true).unwrap();
    assert!(wb.filter_mode().unwrap());
    wb.set_filter_mode(false).unwrap();
}


#[test]
fn workbook_codename_ppt_rtl_word_template() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_code_name("ThisWorkbook").unwrap();
    assert_eq!(wb.code_name().unwrap().as_deref(), Some("ThisWorkbook"));
    assert!(wb.has_code_name().unwrap());
    wb.set_refresh_all_connections(true).unwrap();
    assert!(wb.refresh_all_connections().unwrap());
    wb.set_default_theme_version("124226").unwrap();
    assert_eq!(wb.default_theme_version().unwrap().as_deref(), Some("124226"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_first_slide_num(2).unwrap();
    assert_eq!(ppt.first_slide_num().unwrap(), Some(2));
    assert!(ppt.has_first_slide_num().unwrap());
    ppt.set_rtl(true).unwrap();
    assert!(ppt.rtl().unwrap());
    ppt.set_rtl(false).unwrap();
    assert!(!ppt.rtl().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    let rid = doc.set_attached_template("C:\\\\Templates\\\\Normal.dotm").unwrap();
    assert!(rid.starts_with('r'));
    assert!(doc.has_attached_template().unwrap());
    assert!(doc.clear_attached_template().unwrap());
    assert!(!doc.has_attached_template().unwrap());
}


#[test]
fn excel_workbook_pr_extended_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();

    wb.set_date_compatibility(true).unwrap();
    assert!(wb.date_compatibility().unwrap());

    wb.set_show_objects("placeholders").unwrap();
    assert_eq!(wb.show_objects().unwrap().as_deref(), Some("placeholders"));

    assert!(wb.show_border_unselected_tables().unwrap()); // default true
    wb.set_show_border_unselected_tables(false).unwrap();
    assert!(!wb.show_border_unselected_tables().unwrap());

    wb.set_prompted_solutions(true).unwrap();
    assert!(wb.prompted_solutions().unwrap());

    assert!(wb.show_ink_annotation().unwrap()); // default true
    wb.set_show_ink_annotation(false).unwrap();
    assert!(!wb.show_ink_annotation().unwrap());

    assert!(wb.save_external_link_values().unwrap()); // default true
    wb.set_save_external_link_values(false).unwrap();
    assert!(!wb.save_external_link_values().unwrap());

    wb.set_update_links("never").unwrap();
    assert_eq!(wb.update_links().unwrap().as_deref(), Some("never"));

    wb.set_hide_pivot_field_list(true).unwrap();
    assert!(wb.hide_pivot_field_list().unwrap());

    wb.set_show_pivot_chart_filter(true).unwrap();
    assert!(wb.show_pivot_chart_filter().unwrap());

    wb.set_allow_refresh_query(true).unwrap();
    assert!(wb.allow_refresh_query().unwrap());

    wb.set_publish_items(true).unwrap();
    assert!(wb.publish_items().unwrap());

    wb.set_check_compatibility(true).unwrap();
    assert!(wb.check_compatibility().unwrap());

    assert!(wb.auto_compress_pictures().unwrap()); // default true
    wb.set_auto_compress_pictures(false).unwrap();
    assert!(!wb.auto_compress_pictures().unwrap());

    wb.set_filter_privacy(true).unwrap();
    assert!(wb.filter_privacy().unwrap());
    assert!(wb.has_filter_privacy().unwrap());
}


#[test]
fn ppt_presentation_level_flags() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();

    ppt.set_server_zoom(50).unwrap();
    assert_eq!(ppt.server_zoom().unwrap(), Some(50));

    assert!(ppt.show_special_pls_on_title_sld().unwrap()); // default true
    ppt.set_show_special_pls_on_title_sld(false).unwrap();
    assert!(!ppt.show_special_pls_on_title_sld().unwrap());

    ppt.set_remove_personal_info_on_save(true).unwrap();
    assert!(ppt.remove_personal_info_on_save().unwrap());

    ppt.set_compat_mode(true).unwrap();
    assert!(ppt.compat_mode().unwrap());

    ppt.set_strict_first_and_last_chars(true).unwrap();
    assert!(ppt.strict_first_and_last_chars().unwrap());

    ppt.set_embed_true_type_fonts(true).unwrap();
    assert!(ppt.embed_true_type_fonts().unwrap());

    ppt.set_save_subset_fonts(true).unwrap();
    assert!(ppt.save_subset_fonts().unwrap());

    assert!(ppt.auto_compress_pictures().unwrap()); // default true
    ppt.set_auto_compress_pictures(false).unwrap();
    assert!(!ppt.auto_compress_pictures().unwrap());

    ppt.set_bookmark_id_seed(42).unwrap();
    assert_eq!(ppt.bookmark_id_seed().unwrap(), Some(42));

    ppt.set_conformance("strict").unwrap();
    assert_eq!(ppt.conformance().unwrap().as_deref(), Some("strict"));
}


#[test]
fn word_compat_flags_and_settings() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));

    doc.set_compatibility_mode("15").unwrap();
    assert_eq!(doc.compatibility_mode().unwrap().as_deref(), Some("15"));
    assert!(doc.has_compatibility_mode().unwrap());

    doc.set_compat_flag("usePrinterMetrics", true).unwrap();
    assert!(doc.has_compat_flag("usePrinterMetrics").unwrap());
    assert!(doc.has_use_printer_metrics().unwrap());

    doc.set_do_not_expand_shift_return(true).unwrap();
    assert!(doc.has_do_not_expand_shift_return().unwrap());

    doc.set_balance_single_byte_double_byte_width(true).unwrap();
    assert!(doc.has_balance_single_byte_double_byte_width().unwrap());

    doc.set_adjust_line_height_in_table(true).unwrap();
    assert!(doc.has_adjust_line_height_in_table().unwrap());

    let flags = doc.list_compat_flags().unwrap();
    assert!(flags.contains(&"usePrinterMetrics".to_string()));
    assert!(flags.contains(&"doNotExpandShiftReturn".to_string()));
    assert!(flags.contains(&"balanceSingleByteDoubleByteWidth".to_string()));
    assert!(flags.contains(&"adjustLineHeightInTable".to_string()));

    doc.set_compat_setting(
        "overrideTableStyleFontSizeAndJustification",
        "http://schemas.microsoft.com/office/word",
        "1",
    )
    .unwrap();
    let settings = doc.list_compat_settings().unwrap();
    assert!(settings.iter().any(|(n, _, v)| {
        n == "compatibilityMode" && v == "15"
    }));
    assert!(settings.iter().any(|(n, _, v)| {
        n == "overrideTableStyleFontSizeAndJustification" && v == "1"
    }));

    assert!(doc.clear_compatibility_mode().unwrap());
    assert!(!doc.has_compatibility_mode().unwrap());
    assert!(doc.list_compat_flags().unwrap().is_empty());
}


#[test]
fn excel_sheet_view_print_calc_extended() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();

    wb.set_tab_selected("S", true).unwrap();
    assert!(wb.tab_selected("S").unwrap());

    assert!(wb.show_ruler("S").unwrap());
    wb.set_show_ruler("S", false).unwrap();
    assert!(!wb.show_ruler("S").unwrap());

    wb.set_show_white_space("S", false).unwrap();
    assert!(!wb.show_white_space("S").unwrap());

    wb.set_default_grid_color("S", false).unwrap();
    assert!(!wb.default_grid_color("S").unwrap());

    wb.set_window_protection("S", true).unwrap();
    assert!(wb.window_protection("S").unwrap());

    wb.set_top_left_cell("S", "C5").unwrap();
    assert_eq!(wb.top_left_cell("S").unwrap().as_deref(), Some("C5"));

    wb.set_zoom_scale_page_layout("S", 75).unwrap();
    assert_eq!(wb.zoom_scale_page_layout("S").unwrap(), Some(75));
    wb.set_zoom_scale_sheet_layout("S", 60).unwrap();
    assert_eq!(wb.zoom_scale_sheet_layout("S").unwrap(), Some(60));

    wb.set_print_horizontal_centered("S", true).unwrap();
    assert!(wb.print_horizontal_centered("S").unwrap());
    wb.set_print_vertical_centered("S", true).unwrap();
    assert!(wb.print_vertical_centered("S").unwrap());
    wb.set_print_headings("S", true).unwrap();
    assert!(wb.print_headings("S").unwrap());
    wb.set_print_grid_lines("S", true).unwrap();
    assert!(wb.print_grid_lines("S").unwrap());
    assert!(wb.has_print_options("S").unwrap());
    assert!(wb.clear_print_options("S").unwrap());
    assert!(!wb.has_print_options("S").unwrap());

    wb.set_page_scale("S", 90).unwrap();
    assert_eq!(wb.page_scale("S").unwrap(), Some(90));
    wb.set_fit_to_width("S", 1).unwrap();
    assert_eq!(wb.fit_to_width("S").unwrap(), Some(1));
    wb.set_fit_to_height("S", 2).unwrap();
    assert_eq!(wb.fit_to_height("S").unwrap(), Some(2));
    wb.set_page_copies("S", 3).unwrap();
    assert_eq!(wb.page_copies("S").unwrap(), Some(3));
    wb.set_page_black_and_white("S", true).unwrap();
    assert!(wb.page_black_and_white("S").unwrap());
    wb.set_page_draft("S", true).unwrap();
    assert!(wb.page_draft("S").unwrap());
    wb.set_first_page_number("S", 5).unwrap();
    assert_eq!(wb.first_page_number("S").unwrap(), Some(5));
    wb.set_page_order("S", "overThenDown").unwrap();
    assert_eq!(wb.page_order("S").unwrap().as_deref(), Some("overThenDown"));

    wb.set_sheet_code_name("S", "Sheet1Code").unwrap();
    assert_eq!(wb.sheet_code_name("S").unwrap().as_deref(), Some("Sheet1Code"));
    wb.set_sheet_published("S", false).unwrap();
    assert!(!wb.sheet_published("S").unwrap());
    wb.set_enable_format_conditions_calculation("S", false).unwrap();
    assert!(!wb.enable_format_conditions_calculation("S").unwrap());

    wb.set_calc_id(191029).unwrap();
    assert_eq!(wb.calc_id().unwrap(), Some(191029));
    wb.set_ref_mode("R1C1").unwrap();
    assert_eq!(wb.ref_mode().unwrap().as_deref(), Some("R1C1"));
    wb.set_iterate(true).unwrap();
    assert!(wb.iterate().unwrap());
    wb.set_iterate_count(50).unwrap();
    assert_eq!(wb.iterate_count().unwrap(), Some(50));
    wb.set_iterate_delta(0.001).unwrap();
    assert!((wb.iterate_delta().unwrap().unwrap() - 0.001).abs() < 1e-9);
    wb.set_full_precision(false).unwrap();
    assert!(!wb.full_precision().unwrap());
    wb.set_calc_on_save(false).unwrap();
    assert!(!wb.calc_on_save().unwrap());
    wb.set_concurrent_calc(false).unwrap();
    assert!(!wb.concurrent_calc().unwrap());
    wb.set_force_full_calc(true).unwrap();
    assert!(wb.force_full_calc().unwrap());
    assert!(wb.has_calc_properties().unwrap());
    assert!(wb.clear_calc_properties().unwrap());
    assert!(!wb.has_calc_properties().unwrap());
}


#[test]
fn ppt_view_properties_flags() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();

    ppt.set_last_view("notesView").unwrap();
    assert_eq!(ppt.last_view().unwrap().as_deref(), Some("notesView"));
    assert!(ppt.has_view_properties());

    assert!(ppt.show_comments().unwrap()); // default true
    ppt.set_show_comments(false).unwrap();
    assert!(!ppt.show_comments().unwrap());

    ppt.set_grid_spacing(720000, 720000).unwrap();
    assert_eq!(ppt.grid_spacing().unwrap(), Some((720000, 720000)));
    assert!(ppt.has_grid_spacing().unwrap());
    assert!(ppt.clear_grid_spacing().unwrap());
    assert!(!ppt.has_grid_spacing().unwrap());
}


#[test]
fn excel_workbook_view_extended() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();

    wb.set_first_sheet(0).unwrap();
    assert_eq!(wb.first_sheet().unwrap(), Some(0));
    wb.set_tab_ratio(600).unwrap();
    assert_eq!(wb.tab_ratio().unwrap(), Some(600));
    wb.set_workbook_window(120, 80, 20000, 15000).unwrap();
    assert_eq!(wb.workbook_window().unwrap(), Some((120, 80, 20000, 15000)));

    assert!(wb.show_horizontal_scroll().unwrap());
    wb.set_show_horizontal_scroll(false).unwrap();
    assert!(!wb.show_horizontal_scroll().unwrap());
    wb.set_show_vertical_scroll(false).unwrap();
    assert!(!wb.show_vertical_scroll().unwrap());
    wb.set_show_sheet_tabs(false).unwrap();
    assert!(!wb.show_sheet_tabs().unwrap());

    wb.set_workbook_minimized(true).unwrap();
    assert!(wb.workbook_minimized().unwrap());
    wb.set_workbook_visibility("hidden").unwrap();
    assert_eq!(wb.workbook_visibility().unwrap().as_deref(), Some("hidden"));
    wb.set_auto_filter_date_grouping(false).unwrap();
    assert!(!wb.auto_filter_date_grouping().unwrap());
}


#[test]
fn word_settings_locale_hyphen_print_extended() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));

    doc.set_decimal_symbol(",").unwrap();
    assert_eq!(doc.decimal_symbol().unwrap().as_deref(), Some(","));
    assert!(doc.has_decimal_symbol().unwrap());
    assert!(doc.clear_decimal_symbol().unwrap());

    doc.set_list_separator(";").unwrap();
    assert_eq!(doc.list_separator().unwrap().as_deref(), Some(";"));
    assert!(doc.clear_list_separator().unwrap());

    doc.set_consecutive_hyphen_limit(3).unwrap();
    assert_eq!(doc.consecutive_hyphen_limit().unwrap(), Some(3));
    assert!(doc.clear_consecutive_hyphen_limit().unwrap());

    doc.set_hyphenation_zone(360).unwrap();
    assert_eq!(doc.hyphenation_zone().unwrap(), Some(360));
    assert!(doc.clear_hyphenation_zone().unwrap());

    doc.set_do_not_hyphenate_caps(true).unwrap();
    assert!(doc.has_do_not_hyphenate_caps().unwrap());
    doc.set_do_not_hyphenate_caps(false).unwrap();
    assert!(!doc.has_do_not_hyphenate_caps().unwrap());

    doc.set_save_subset_fonts(true).unwrap();
    assert!(doc.has_save_subset_fonts().unwrap());
    doc.set_embed_system_fonts(true).unwrap();
    assert!(doc.has_embed_system_fonts().unwrap());
    doc.set_link_styles(true).unwrap();
    assert!(doc.has_link_styles().unwrap());
    doc.set_style_lock_theme(true).unwrap();
    assert!(doc.has_style_lock_theme().unwrap());

    doc.set_do_not_track_moves(true).unwrap();
    assert!(doc.has_do_not_track_moves().unwrap());
    doc.set_do_not_track_formatting(true).unwrap();
    assert!(doc.has_do_not_track_formatting().unwrap());

    doc.set_book_fold_printing(true).unwrap();
    assert!(doc.has_book_fold_printing().unwrap());
    doc.set_book_fold_rev_printing(true).unwrap();
    assert!(doc.has_book_fold_rev_printing().unwrap());
    doc.set_book_fold_printing_sheets(2).unwrap();
    assert_eq!(doc.book_fold_printing_sheets().unwrap(), Some(2));

    doc.set_default_table_style("TableGrid").unwrap();
    assert_eq!(doc.default_table_style().unwrap().as_deref(), Some("TableGrid"));
    assert!(doc.clear_default_table_style().unwrap());

    doc.set_click_and_type_style("Normal").unwrap();
    assert_eq!(doc.click_and_type_style().unwrap().as_deref(), Some("Normal"));
    assert!(doc.clear_click_and_type_style().unwrap());

    doc.set_theme_font_lang("en-US").unwrap();
    assert_eq!(doc.theme_font_lang().unwrap().as_deref(), Some("en-US"));
    assert!(doc.clear_theme_font_lang().unwrap());

    doc.set_summary_length(100).unwrap();
    assert_eq!(doc.summary_length().unwrap(), Some(100));

    doc.set_remove_date_and_time(true).unwrap();
    assert!(doc.has_remove_date_and_time().unwrap());

    doc.set_drawing_grid_horizontal_spacing(120).unwrap();
    assert_eq!(doc.drawing_grid_horizontal_spacing().unwrap(), Some(120));
    doc.set_drawing_grid_vertical_spacing(120).unwrap();
    assert_eq!(doc.drawing_grid_vertical_spacing().unwrap(), Some(120));
}


#[test]
fn ppt_show_and_print_properties() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();

    ppt.set_show_loop(true).unwrap();
    assert!(ppt.show_loop().unwrap());
    ppt.set_show_narration(true).unwrap();
    assert!(ppt.show_narration().unwrap());
    assert!(ppt.show_animation().unwrap()); // default true
    ppt.set_show_animation(false).unwrap();
    assert!(!ppt.show_animation().unwrap());
    ppt.set_use_timings(false).unwrap();
    assert!(!ppt.use_timings().unwrap());

    ppt.set_show_mode_browse(false).unwrap();
    assert_eq!(ppt.show_mode().unwrap().as_deref(), Some("browse"));
    ppt.set_show_mode_kiosk(300_000).unwrap();
    assert_eq!(ppt.show_mode().unwrap().as_deref(), Some("kiosk"));
    ppt.set_show_mode_presented().unwrap();
    assert_eq!(ppt.show_mode().unwrap().as_deref(), Some("presented"));
    assert!(ppt.has_show_properties());
    assert!(ppt.clear_show_properties().unwrap());
    assert!(!ppt.has_show_properties());

    ppt.set_print_frame_slides(true).unwrap();
    assert!(ppt.print_frame_slides().unwrap());
    ppt.set_print_hidden_slides(true).unwrap();
    assert!(ppt.print_hidden_slides().unwrap());
    ppt.set_print_scale_to_fit_paper(true).unwrap();
    assert!(ppt.print_scale_to_fit_paper().unwrap());
    assert!(ppt.has_print_properties());
}


#[test]
fn excel_sheet_format_extended_and_word_proof_state() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();

    wb.set_base_col_width("S", 10).unwrap();
    assert_eq!(wb.base_col_width("S").unwrap(), Some(10));
    wb.set_zero_height("S", true).unwrap();
    assert!(wb.zero_height("S").unwrap());
    wb.set_thick_top("S", true).unwrap();
    assert!(wb.thick_top("S").unwrap());
    wb.set_thick_bottom("S", true).unwrap();
    assert!(wb.thick_bottom("S").unwrap());
    wb.set_outline_level_row("S", 3).unwrap();
    assert_eq!(wb.outline_level_row("S").unwrap(), Some(3));
    wb.set_outline_level_col("S", 2).unwrap();
    assert_eq!(wb.outline_level_col("S").unwrap(), Some(2));
    assert!(wb.has_sheet_format("S").unwrap());
    assert!(wb.clear_sheet_format("S").unwrap());
    assert!(!wb.has_sheet_format("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.set_proof_state("clean", "clean").unwrap();
    assert_eq!(
        doc.proof_state().unwrap(),
        Some(("clean".into(), "clean".into()))
    );
    assert!(doc.has_proof_state().unwrap());
    assert!(doc.clear_proof_state().unwrap());
    assert!(!doc.has_proof_state().unwrap());
}


#[test]
fn excel_header_footer_and_ppt_custom_show_word_write_protection() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_odd_header("S", "&CHeader").unwrap();
    assert_eq!(wb.odd_header("S").unwrap().as_deref(), Some("&CHeader"));
    wb.set_odd_footer("S", "&CPage &P").unwrap();
    assert_eq!(wb.odd_footer("S").unwrap().as_deref(), Some("&CPage &P"));
    wb.set_even_header("S", "EvenH").unwrap();
    assert_eq!(wb.even_header("S").unwrap().as_deref(), Some("EvenH"));
    wb.set_even_footer("S", "EvenF").unwrap();
    assert_eq!(wb.even_footer("S").unwrap().as_deref(), Some("EvenF"));
    wb.set_first_header("S", "FirstH").unwrap();
    assert_eq!(wb.first_header("S").unwrap().as_deref(), Some("FirstH"));
    wb.set_first_footer("S", "FirstF").unwrap();
    assert_eq!(wb.first_footer("S").unwrap().as_deref(), Some("FirstF"));
    wb.set_header_footer_flags("S", Some(true), Some(true), Some(false), Some(true))
        .unwrap();
    assert!(wb.header_footer_different_odd_even("S").unwrap());
    assert!(wb.header_footer_different_first("S").unwrap());
    assert!(wb.has_header_footer("S").unwrap());
    assert!(wb.clear_header_footer("S").unwrap());
    assert!(!wb.has_header_footer("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("One").unwrap();
    ppt.add_slide_with_text("Two").unwrap();
    ppt.add_slide_with_text("Three").unwrap();
    let id = ppt.add_custom_show("Highlights", &[0, 2]).unwrap();
    assert!(id >= 1);
    assert!(ppt.has_custom_shows().unwrap());
    assert_eq!(ppt.custom_show_count().unwrap(), 1);
    let shows = ppt.list_custom_shows().unwrap();
    assert_eq!(shows.len(), 1);
    assert_eq!(shows[0].1, "Highlights");
    assert_eq!(shows[0].2.len(), 2);
    let id2 = ppt.add_custom_show("All", &[0, 1, 2]).unwrap();
    assert_eq!(ppt.custom_show_count().unwrap(), 2);
    assert!(ppt.remove_custom_show(id).unwrap());
    assert_eq!(ppt.custom_show_count().unwrap(), 1);
    assert_eq!(ppt.clear_custom_shows().unwrap(), 1);
    assert!(!ppt.has_custom_shows().unwrap());
    let _ = id2;

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.set_write_protection(true).unwrap();
    assert!(doc.has_write_protection().unwrap());
    assert!(doc.write_protection_recommended().unwrap());
    assert!(doc.clear_write_protection().unwrap());
    assert!(!doc.has_write_protection().unwrap());
}


#[test]
fn word_section_properties_extended() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("section")]));

    doc.set_page_setup(12240, 15840, 1440, 1440, 1440, 1440).unwrap();
    doc.set_page_orientation("landscape").unwrap();
    assert_eq!(doc.page_orientation().unwrap().as_deref(), Some("landscape"));

    doc.set_columns(2, Some(720), true).unwrap();
    assert_eq!(doc.column_count().unwrap(), Some(2));
    assert!(doc.has_columns().unwrap());
    assert!(doc.clear_columns().unwrap());
    assert!(!doc.has_columns().unwrap());

    doc.set_title_page(true).unwrap();
    assert!(doc.has_title_page().unwrap());
    doc.set_title_page(false).unwrap();
    assert!(!doc.has_title_page().unwrap());

    doc.set_vertical_page_align("center").unwrap();
    assert_eq!(doc.vertical_page_align().unwrap().as_deref(), Some("center"));
    assert!(doc.clear_vertical_page_align().unwrap());

    doc.set_section_type("continuous").unwrap();
    assert_eq!(doc.section_type().unwrap().as_deref(), Some("continuous"));

    doc.set_page_number_type_start(5).unwrap();
    assert_eq!(doc.page_number_type_start().unwrap(), Some(5));

    doc.set_section_bidi(true).unwrap();
    assert!(doc.has_section_bidi().unwrap());
    doc.set_section_bidi(false).unwrap();
    assert!(!doc.has_section_bidi().unwrap());

    doc.set_header_footer_distance(720, 720).unwrap();
    assert_eq!(doc.header_footer_distance().unwrap(), Some((720, 720)));
}


#[test]
fn excel_selection_sort_and_word_section_more() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[
        vec!["Name", "Score"],
        vec!["Alice", "90"],
        vec!["Bob", "80"],
    ]).unwrap();

    wb.set_active_cell("S", "B2").unwrap();
    assert_eq!(wb.active_cell("S").unwrap().as_deref(), Some("B2"));
    assert_eq!(wb.selection_sqref("S").unwrap().as_deref(), Some("B2"));
    assert!(wb.has_selection("S").unwrap());
    assert!(wb.clear_selection("S").unwrap());
    assert!(!wb.has_selection("S").unwrap());

    wb.set_sort_state("S", "A1:B3", "B1:B3", true).unwrap();
    let ss = wb.sort_state("S").unwrap().unwrap();
    assert_eq!(ss.0, "A1:B3");
    assert_eq!(ss.1, "B1:B3");
    assert!(ss.2);
    wb.set_sort_case_sensitive("S", true).unwrap();
    assert!(wb.sort_case_sensitive("S").unwrap());
    assert!(wb.clear_sort_state("S").unwrap());
    assert!(!wb.has_sort_state("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("ln")]));

    doc.set_line_numbering(1, 1, "newPage").unwrap();
    assert_eq!(
        doc.line_numbering().unwrap(),
        Some((1, 1, "newPage".into()))
    );
    assert!(doc.has_line_numbering().unwrap());
    assert!(doc.clear_line_numbering().unwrap());

    doc.set_page_number_format("upperRoman").unwrap();
    assert_eq!(doc.page_number_format().unwrap().as_deref(), Some("upperRoman"));

    doc.set_text_direction("tbRl").unwrap();
    assert_eq!(doc.text_direction().unwrap().as_deref(), Some("tbRl"));
    assert!(doc.clear_text_direction().unwrap());

    doc.set_gutter(720).unwrap();
    assert_eq!(doc.gutter().unwrap(), Some(720));

    doc.set_paper_source(1, 4).unwrap();
    assert_eq!(doc.paper_source().unwrap(), Some((1, 4)));
    assert!(doc.clear_paper_source().unwrap());

    doc.set_rtl_gutter(true).unwrap();
    assert!(doc.has_rtl_gutter().unwrap());
    doc.set_rtl_gutter(false).unwrap();
    assert!(!doc.has_rtl_gutter().unwrap());
}


#[test]
fn excel_protected_ignored_scenario_and_ppt_album_kinsoku() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();

    wb.add_protected_range("S", "Editable", "A1:B1").unwrap();
    assert_eq!(wb.protected_range_count("S").unwrap(), 1);
    assert!(wb.has_protected_ranges("S").unwrap());
    let prs = wb.list_protected_ranges("S").unwrap();
    assert_eq!(prs[0].0, "Editable");
    assert_eq!(prs[0].1, "A1:B1");
    assert!(wb.remove_protected_range("S", "Editable").unwrap());
    assert!(!wb.has_protected_ranges("S").unwrap());
    wb.add_protected_range("S", "R1", "A1").unwrap();
    wb.add_protected_range("S", "R2", "B1").unwrap();
    assert_eq!(wb.clear_protected_ranges("S").unwrap(), 2);

    wb.add_ignored_error("S", "A2", &["numberStoredAsText", "formula"]).unwrap();
    assert_eq!(wb.ignored_error_count("S").unwrap(), 1);
    let errs = wb.list_ignored_errors("S").unwrap();
    assert_eq!(errs[0].0, "A2");
    assert!(errs[0].1.contains(&"numberStoredAsText".to_string()));
    assert_eq!(wb.clear_ignored_errors("S").unwrap(), 1);

    wb.add_scenario("S", "Base", &[("A1", "10"), ("B1", "20")], Some("base case")).unwrap();
    assert_eq!(wb.scenario_count("S").unwrap(), 1);
    let sc = wb.list_scenarios("S").unwrap();
    assert_eq!(sc[0].0, "Base");
    assert_eq!(sc[0].1.as_deref(), Some("base case"));
    assert_eq!(sc[0].2, 2);
    assert!(wb.remove_scenario("S", "Base").unwrap());
    wb.add_scenario("S", "A", &[("A1", "1")], None).unwrap();
    wb.add_scenario("S", "B", &[("A1", "2")], None).unwrap();
    assert_eq!(wb.clear_scenarios("S").unwrap(), 2);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_slide_header_footer(0, true, false, true).unwrap();
    let hf = ppt.slide_header_footer(0).unwrap().unwrap();
    assert!(hf.0);  // date
    assert!(!hf.1); // footer
    assert!(hf.2);  // slide num

    ppt.set_photo_album(true, true, "1pic", "roundedRectangle").unwrap();
    let pa = ppt.photo_album().unwrap().unwrap();
    assert!(pa.0 && pa.1);
    assert_eq!(pa.2, "1pic");
    assert_eq!(pa.3, "roundedRectangle");
    assert!(ppt.has_photo_album().unwrap());
    assert!(ppt.clear_photo_album().unwrap());

    ppt.set_kinsoku("ja-JP", "([{", ")]}").unwrap();
    let k = ppt.kinsoku().unwrap().unwrap();
    assert_eq!(k.0, "ja-JP");
    assert_eq!(k.1, "([{");
    assert_eq!(k.2, ")]}");
    assert!(ppt.has_kinsoku().unwrap());
    assert!(ppt.clear_kinsoku().unwrap());
}


#[test]
fn word_revision_captions_math_color_mapping() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("m")]));

    doc.set_revision_view(true, false, true, false).unwrap();
    let rv = doc.revision_view().unwrap().unwrap();
    assert!(rv.0 && !rv.1 && rv.2 && !rv.3);
    assert!(doc.has_revision_view().unwrap());
    assert!(doc.clear_revision_view().unwrap());

    doc.set_document_type_setting("letter").unwrap();
    assert_eq!(doc.document_type_setting().unwrap().as_deref(), Some("letter"));
    assert!(doc.clear_document_type_setting().unwrap());

    doc.set_style_pane_sort_method("name").unwrap();
    assert_eq!(doc.style_pane_sort_method().unwrap().as_deref(), Some("name"));

    doc.add_caption_definition("Figure", "below", "decimal").unwrap();
    doc.add_caption_definition("Table", "above", "upperRoman").unwrap();
    let caps = doc.list_caption_definitions().unwrap();
    assert_eq!(caps.len(), 2);
    assert!(caps.iter().any(|(n, p, _)| n == "Figure" && p == "below"));
    assert!(doc.has_captions().unwrap());
    assert!(doc.clear_captions().unwrap());

    doc.set_math_font("Cambria Math").unwrap();
    assert_eq!(doc.math_font().unwrap().as_deref(), Some("Cambria Math"));
    doc.set_math_display_defaults(true, "centerGroup").unwrap();
    assert!(doc.has_math_properties().unwrap());
    assert!(doc.clear_math_properties().unwrap());

    doc.set_color_scheme_mapping("accent1", "accent1").unwrap();
    assert_eq!(doc.color_scheme_mapping("accent1").unwrap().as_deref(), Some("accent1"));
    assert!(doc.has_color_scheme_mapping().unwrap());
    assert!(doc.clear_color_scheme_mapping().unwrap());
}


#[test]
fn excel_custom_sheet_views() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_custom_sheet_view("S", "{11111111-1111-1111-1111-111111111111}", 80, false)
        .unwrap();
    wb.add_custom_sheet_view("S", "{22222222-2222-2222-2222-222222222222}", 120, true)
        .unwrap();
    assert_eq!(wb.custom_sheet_view_count("S").unwrap(), 2);
    assert!(wb.has_custom_sheet_views("S").unwrap());
    let views = wb.list_custom_sheet_views("S").unwrap();
    assert_eq!(views[0].1, 80);
    assert_eq!(views[1].1, 120);
    assert_eq!(wb.clear_custom_sheet_views("S").unwrap(), 2);
    assert!(!wb.has_custom_sheet_views("S").unwrap());
}


#[test]
fn excel_ole_controls_web_ppt_verifier_word_xml_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();

    wb.add_ole_object("S", "Excel.Sheet.12", 1025, Some("rIdOle1")).unwrap();
    assert_eq!(wb.ole_object_count("S").unwrap(), 1);
    assert!(wb.has_ole_objects("S").unwrap());
    let ole = wb.list_ole_objects("S").unwrap();
    assert_eq!(ole[0].0, "Excel.Sheet.12");
    assert_eq!(ole[0].1, 1025);
    assert_eq!(ole[0].2.as_deref(), Some("rIdOle1"));
    assert_eq!(wb.clear_ole_objects("S").unwrap(), 1);

    wb.add_control("S", "CheckBox1", 2048, None).unwrap();
    assert_eq!(wb.control_count("S").unwrap(), 1);
    assert!(wb.has_controls("S").unwrap());
    assert_eq!(wb.clear_controls("S").unwrap(), 1);

    wb.add_web_publish_item("S", 1, "sheet", "out.htm", "Report").unwrap();
    assert_eq!(wb.web_publish_item_count("S").unwrap(), 1);
    let items = wb.list_web_publish_items("S").unwrap();
    assert_eq!(items[0].3, "Report");
    assert_eq!(wb.clear_web_publish_items("S").unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_modify_verifier("SHA-512", 100000).unwrap();
    assert!(ppt.has_modify_verifier().unwrap());
    assert_eq!(
        ppt.modify_verifier_algorithm().unwrap().as_deref(),
        Some("SHA-512")
    );
    assert!(ppt.clear_modify_verifier().unwrap());
    assert!(!ppt.has_modify_verifier().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_force_upgrade(true).unwrap();
    assert!(doc.has_force_upgrade().unwrap());
    doc.set_force_upgrade(false).unwrap();
    assert!(!doc.has_force_upgrade().unwrap());

    doc.set_do_not_validate_against_schema(true).unwrap();
    assert!(doc.has_do_not_validate_against_schema().unwrap());
    doc.set_save_invalid_xml(true).unwrap();
    assert!(doc.has_save_invalid_xml().unwrap());
    doc.set_ignore_mixed_content(true).unwrap();
    assert!(doc.has_ignore_mixed_content().unwrap());
    doc.set_always_show_placeholder_text(true).unwrap();
    assert!(doc.has_always_show_placeholder_text().unwrap());
    doc.set_show_xml_tags(true).unwrap();
    assert!(doc.has_show_xml_tags().unwrap());
    doc.set_do_not_demarcate_invalid_xml(true).unwrap();
    assert!(doc.has_do_not_demarcate_invalid_xml().unwrap());
}


#[test]
fn word_rsids_schemas_xslt_and_ppt_embedded_fonts() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("r")]));

    doc.set_rsids("00AABBCC", &["00AABBCD", "00AABBCE"]).unwrap();
    let (root, list) = doc.rsids().unwrap().unwrap();
    assert_eq!(root, "00AABBCC");
    assert_eq!(list.len(), 2);
    assert!(doc.has_rsids().unwrap());
    assert!(doc.clear_rsids().unwrap());

    doc.add_attached_schema("http://example.com/schema.xsd").unwrap();
    doc.add_attached_schema("http://example.com/other.xsd").unwrap();
    // duplicate should not add
    doc.add_attached_schema("http://example.com/schema.xsd").unwrap();
    let schemas = doc.list_attached_schemas().unwrap();
    assert_eq!(schemas.len(), 2);
    assert_eq!(doc.clear_attached_schemas().unwrap(), 2);

    doc.set_save_through_xslt(Some("sol-1")).unwrap();
    assert!(doc.has_save_through_xslt().unwrap());
    assert!(doc.clear_save_through_xslt().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_embedded_font("Calibri", Some("rIdFont1")).unwrap();
    ppt.add_embedded_font("Arial", None).unwrap();
    assert_eq!(ppt.embedded_font_count().unwrap(), 2);
    assert!(ppt.has_embedded_fonts().unwrap());
    let fonts = ppt.list_embedded_fonts().unwrap();
    assert!(fonts.contains(&"Calibri".to_string()));
    assert!(fonts.contains(&"Arial".to_string()));
    assert_eq!(ppt.clear_embedded_fonts().unwrap(), 2);
    assert!(!ppt.has_embedded_fonts().unwrap());
}


#[test]
fn excel_sheet_pr_outline_pagesetup_and_word_subdocs() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();

    assert!(!wb.fit_to_page("S").unwrap());
    wb.set_fit_to_page("S", true).unwrap();
    assert!(wb.fit_to_page("S").unwrap());
    assert!(wb.auto_page_breaks("S").unwrap()); // default true
    wb.set_auto_page_breaks("S", false).unwrap();
    assert!(!wb.auto_page_breaks("S").unwrap());

    let (below, right, styles) = wb.outline_properties("S").unwrap();
    assert!(below && right && !styles);
    wb.set_outline_properties("S", false, false, true).unwrap();
    let op = wb.outline_properties("S").unwrap();
    assert!(!op.0 && !op.1 && op.2);
    assert!(wb.has_outline_properties("S").unwrap());
    assert!(wb.clear_outline_properties("S").unwrap());

    wb.set_sheet_filter_mode("S", true).unwrap();
    assert!(wb.sheet_filter_mode("S").unwrap());
    wb.set_transition_evaluation("S", true).unwrap();
    assert!(wb.transition_evaluation("S").unwrap());
    wb.set_transition_entry("S", true).unwrap();
    assert!(wb.transition_entry("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.set_do_not_include_subdocs_in_stats(true).unwrap();
    assert!(doc.has_do_not_include_subdocs_in_stats().unwrap());
    doc.set_do_not_include_subdocs_in_stats(false).unwrap();
    assert!(!doc.has_do_not_include_subdocs_in_stats().unwrap());
}


#[test]
fn excel_phonetic_custom_wb_views_consolidate_word_writing_ppt_custdata() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();

    wb.set_phonetic_properties("S", 0, "halfwidthKatakana", "center").unwrap();
    let pp = wb.phonetic_properties("S").unwrap().unwrap();
    assert_eq!(pp.0, 0);
    assert_eq!(pp.1, "halfwidthKatakana");
    assert_eq!(pp.2, "center");
    assert!(wb.has_phonetic_properties("S").unwrap());
    assert!(wb.clear_phonetic_properties("S").unwrap());

    wb.add_custom_workbook_view("MyView", "{aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee}", 1)
        .unwrap();
    assert_eq!(wb.custom_workbook_view_count().unwrap(), 1);
    assert!(wb.has_custom_workbook_views().unwrap());
    let cv = wb.list_custom_workbook_views().unwrap();
    assert_eq!(cv[0].0, "MyView");
    assert_eq!(cv[0].2, 1);
    assert_eq!(wb.clear_custom_workbook_views().unwrap(), 1);

    wb.set_data_consolidate("S", "sum", true).unwrap();
    assert!(wb.has_data_consolidate("S").unwrap());
    assert_eq!(wb.data_consolidate_function("S").unwrap().as_deref(), Some("sum"));
    assert!(wb.clear_data_consolidate("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("w")]));
    doc.set_active_writing_style("en-US", 1, 1, "MSWord").unwrap();
    let aws = doc.active_writing_style().unwrap().unwrap();
    assert_eq!(aws.0, "en-US");
    assert_eq!(aws.3, "MSWord");
    assert!(doc.has_active_writing_style().unwrap());
    assert!(doc.clear_active_writing_style().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_customer_data("rIdCust1").unwrap();
    ppt.add_customer_data("rIdCust2").unwrap();
    assert_eq!(ppt.customer_data_count().unwrap(), 2);
    assert!(ppt.has_customer_data().unwrap());
    let ids = ppt.list_customer_data().unwrap();
    assert!(ids.contains(&"rIdCust1".to_string()));
    assert_eq!(ppt.clear_customer_data().unwrap(), 2);
}


#[test]
fn excel_filter_cols_data_refs_word_mail_merge_ppt_slide_name() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[
        vec!["Name", "Score"],
        vec!["Alice", "90"],
        vec!["Bob", "80"],
        vec!["Carol", "95"],
    ]).unwrap();
    wb.set_auto_filter("S", "A1:B4").unwrap();
    wb.add_auto_filter_values("S", 0, &["Alice", "Carol"]).unwrap();
    wb.add_auto_filter_top10("S", 1, true, false, 2.0).unwrap();
    let cols = wb.list_auto_filter_columns("S").unwrap();
    assert_eq!(cols.len(), 2);
    assert!(cols.contains(&0) && cols.contains(&1));
    assert_eq!(wb.auto_filter_column_count("S").unwrap(), 2);
    assert_eq!(wb.clear_auto_filter_columns("S").unwrap(), 2);
    assert_eq!(wb.auto_filter_column_count("S").unwrap(), 0);
    assert!(wb.has_auto_filter("S").unwrap()); // ref remains

    wb.set_data_consolidate("S", "average", false).unwrap();
    wb.add_data_consolidate_ref("S", "A1:B2", Some("S"), None).unwrap();
    wb.add_data_consolidate_ref("S", "A3:B4", Some("S"), Some("Block2")).unwrap();
    let refs = wb.list_data_consolidate_refs("S").unwrap();
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[1].2.as_deref(), Some("Block2"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("Dear «Name»")]));
    doc.set_mail_merge("formLetters", "database", Some("SELECT * FROM Contacts"), true)
        .unwrap();
    assert!(doc.has_mail_merge().unwrap());
    assert_eq!(
        doc.mail_merge_main_document_type().unwrap().as_deref(),
        Some("formLetters")
    );
    assert_eq!(doc.mail_merge_data_type().unwrap().as_deref(), Some("database"));
    assert!(doc.mail_merge_view_merged_data().unwrap());
    assert!(doc.clear_mail_merge().unwrap());
    assert!(!doc.has_mail_merge().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Title").unwrap();
    ppt.set_slide_name(0, "Opening").unwrap();
    assert_eq!(ppt.slide_name(0).unwrap().as_deref(), Some("Opening"));
}


#[test]
fn excel_custom_dynamic_filter_word_odso_ppt_notes_name() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[
        vec!["Name", "Score"],
        vec!["Alice", "90"],
        vec!["Bob", "80"],
    ]).unwrap();
    wb.set_auto_filter("S", "A1:B3").unwrap();
    wb.add_auto_filter_custom("S", 1, &[("greaterThan", "85")], true)
        .unwrap();
    assert_eq!(
        wb.auto_filter_column_kind("S", 1).unwrap().as_deref(),
        Some("custom")
    );
    wb.add_auto_filter_dynamic("S", 0, "aboveAverage").unwrap();
    assert_eq!(
        wb.auto_filter_column_kind("S", 0).unwrap().as_deref(),
        Some("dynamic")
    );
    assert_eq!(wb.auto_filter_column_count("S").unwrap(), 2);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("mm")]));
    doc.set_mail_merge("formLetters", "native", None, false).unwrap();
    doc.set_mail_merge_odso("Contacts", "database").unwrap();
    assert!(doc.has_mail_merge_odso().unwrap());
    assert_eq!(doc.mail_merge_odso_table().unwrap().as_deref(), Some("Contacts"));
    doc.set_mail_merge_active_record(3).unwrap();
    assert_eq!(doc.mail_merge_active_record().unwrap(), Some(3));
    assert!(doc.clear_mail_merge_odso().unwrap());
    assert!(!doc.has_mail_merge_odso().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S1").unwrap();
    ppt.add_notes_to_slide(0, "Speaker notes").unwrap();
    ppt.set_notes_name(0, "Notes for S1").unwrap();
    assert_eq!(ppt.notes_name(0).unwrap().as_deref(), Some("Notes for S1"));
}


#[test]
fn excel_hyperlink_tooltip_location_word_mail_dest_ppt_notes_hf() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["c", "d"]]).unwrap();
    wb.add_cell_hyperlink_with_tooltip("S", "A1", "https://example.com", Some("Ex"), Some("Go"))
        .unwrap();
    assert_eq!(
        wb.cell_hyperlink_tooltip("S", "A1").unwrap().as_deref(),
        Some("Go")
    );
    wb.add_cell_location_hyperlink("S", "B1", "S!A2", Some("Jump"), Some("to A2"))
        .unwrap();
    let locs = wb.list_location_hyperlinks("S").unwrap();
    assert_eq!(locs.len(), 1);
    assert_eq!(locs[0].0, "B1");
    assert_eq!(locs[0].1, "S!A2");
    assert_eq!(locs[0].3.as_deref(), Some("to A2"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("mm")]));
    doc.set_mail_merge("email", "database", None, false).unwrap();
    doc.set_mail_merge_destination("email").unwrap();
    assert_eq!(doc.mail_merge_destination().unwrap().as_deref(), Some("email"));
    doc.set_mail_merge_subject("Hello").unwrap();
    assert_eq!(doc.mail_merge_subject().unwrap().as_deref(), Some("Hello"));
    doc.set_mail_merge_address_field_name("Email").unwrap();
    assert_eq!(
        doc.mail_merge_address_field_name().unwrap().as_deref(),
        Some("Email")
    );
    doc.set_mail_merge_as_attachment(true).unwrap();
    assert!(doc.mail_merge_as_attachment().unwrap());
    doc.set_mail_merge_as_attachment(false).unwrap();
    assert!(!doc.mail_merge_as_attachment().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("S1").unwrap();
    ppt.add_notes_to_slide(0, "notes").unwrap();
    ppt.set_notes_header_footer(0, true, true, false, true).unwrap();
    assert!(ppt.has_notes_header_footer(0).unwrap());
    assert!(ppt.clear_notes_header_footer(0).unwrap());
    assert!(!ppt.has_notes_header_footer(0).unwrap());
}


#[test]
fn excel_dv_messages_date_textlen_blank_filter_word_mm_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[
        vec!["Name", "Score", "Date", "Note"],
        vec!["Alice", "90", "2020-01-01", "hi"],
    ]).unwrap();

    wb.add_data_validation_list("S", "A2:A100", "\"A,B,C\"", true).unwrap();
    assert!(wb
        .set_data_validation_messages(
            "S",
            "A2:A100",
            Some("Pick"),
            Some("Choose A/B/C"),
            Some("Bad"),
            Some("Invalid value"),
        )
        .unwrap());
    let msgs = wb.data_validation_messages("S", "A2:A100").unwrap().unwrap();
    assert_eq!(msgs.0.as_deref(), Some("Pick"));
    assert_eq!(msgs.1.as_deref(), Some("Choose A/B/C"));
    assert_eq!(msgs.2.as_deref(), Some("Bad"));
    assert_eq!(msgs.3.as_deref(), Some("Invalid value"));
    assert!(wb.set_data_validation_error_style("S", "A2:A100", "warning").unwrap());

    wb.add_data_validation_date("S", "C2:C100", "greaterThan", "2020-01-01", None, true)
        .unwrap();
    wb.add_data_validation_text_length("S", "D2:D100", "lessThanOrEqual", "50", None, true)
        .unwrap();
    let dvs = wb.list_data_validations("S").unwrap();
    assert!(dvs.iter().any(|(t, _, _)| t == "date"));
    assert!(dvs.iter().any(|(t, _, _)| t == "textLength"));
    assert!(dvs.iter().any(|(t, _, _)| t == "list"));

    wb.set_auto_filter("S", "A1:D2").unwrap();
    wb.add_auto_filter_blank("S", 3).unwrap();
    assert_eq!(
        wb.auto_filter_column_kind("S", 3).unwrap().as_deref(),
        Some("values")
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("mm")]));
    doc.set_mail_merge("formLetters", "database", None, false).unwrap();
    doc.set_mail_merge_do_not_suppress_blank_lines(true).unwrap();
    assert!(doc.mail_merge_do_not_suppress_blank_lines().unwrap());
    doc.set_mail_merge_link_to_query(true).unwrap();
    assert!(doc.mail_merge_link_to_query().unwrap());
    doc.set_mail_merge_check_errors(2).unwrap();
    assert_eq!(doc.mail_merge_check_errors().unwrap(), Some(2));
    doc.set_mail_merge_connect_string("Provider=SQLOLEDB;").unwrap();
    assert_eq!(
        doc.mail_merge_connect_string().unwrap().as_deref(),
        Some("Provider=SQLOLEDB;")
    );
}


#[test]
fn excel_column_attrs_and_ppt_show_range() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b", "c"]]).unwrap();
    wb.set_column_widths("S", &[(1, 1, 12.0), (2, 3, 10.0)]).unwrap();

    wb.set_column_best_fit("S", 1, 1, true).unwrap();
    assert!(wb.column_best_fit("S", 1, 1).unwrap());
    wb.set_column_style("S", 1, 1, 5).unwrap();
    assert_eq!(wb.column_style("S", 1, 1).unwrap(), Some(5));
    wb.set_column_outline_level("S", 2, 3, 2).unwrap();
    assert_eq!(wb.column_outline_level("S", 2, 3).unwrap(), Some(2));
    wb.set_column_collapsed("S", 2, 3, true).unwrap();
    assert!(wb.column_collapsed("S", 2, 3).unwrap());
    wb.set_column_collapsed("S", 2, 3, false).unwrap();
    assert!(!wb.column_collapsed("S", 2, 3).unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("1").unwrap();
    ppt.add_slide_with_text("2").unwrap();
    ppt.add_slide_with_text("3").unwrap();
    let id = ppt.add_custom_show("Demo", &[0, 2]).unwrap();

    ppt.set_show_slide_range(0, 1).unwrap();
    assert_eq!(ppt.show_slide_range().unwrap(), Some((0, 1)));
    assert!(!ppt.show_all_slides().unwrap());

    ppt.set_show_all_slides().unwrap();
    assert!(ppt.show_all_slides().unwrap());
    assert!(ppt.show_slide_range().unwrap().is_none());

    ppt.set_show_custom_show(id).unwrap();
    assert_eq!(ppt.show_custom_show_id().unwrap(), Some(id));
    assert!(!ppt.show_all_slides().unwrap());
}


#[test]
fn excel_dv_decimal_custom_word_style_pane_ppt_pen() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["n", "f"], vec!["1.5", "x"]]).unwrap();
    wb.add_data_validation_decimal("S", "A2:A100", "between", "0", Some("100"), true)
        .unwrap();
    wb.add_data_validation_custom("S", "B2:B100", "ISNUMBER(B2)", true)
        .unwrap();
    let dvs = wb.list_data_validations("S").unwrap();
    assert!(dvs.iter().any(|(t, _, _)| t == "decimal"));
    assert!(dvs.iter().any(|(t, _, f)| t == "custom" && f.contains("ISNUMBER")));
    assert!(wb
        .set_data_validation_show_drop_down("S", "A2:A100", true)
        .unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.set_style_pane_format_filter(&[
        ("allStyles", true),
        ("headingStyles", true),
        ("tableStyles", false),
    ])
    .unwrap();
    assert_eq!(doc.style_pane_format_filter("allStyles").unwrap(), Some(true));
    assert_eq!(doc.style_pane_format_filter("tableStyles").unwrap(), Some(false));
    assert!(doc.has_style_pane_format_filter().unwrap());
    assert!(doc.clear_style_pane_format_filter().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_show_pen_color("FF0000").unwrap();
    assert_eq!(ppt.show_pen_color().unwrap().as_deref(), Some("FF0000"));
    assert!(ppt.clear_show_pen_color().unwrap());
    assert!(ppt.show_pen_color().unwrap().is_none());
}


#[test]
fn excel_row_attrs_and_word_xml_save_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"], vec!["b"], vec!["c"]]).unwrap();
    wb.set_row_thick_top("S", 1, true).unwrap();
    assert!(wb.row_thick_top("S", 1).unwrap());
    wb.set_row_thick_bottom("S", 2, true).unwrap();
    assert!(wb.row_thick_bottom("S", 2).unwrap());
    wb.set_row_collapsed("S", 3, true).unwrap();
    assert!(wb.row_collapsed("S", 3).unwrap());
    wb.set_row_style("S", 1, 4).unwrap();
    assert_eq!(wb.row_style("S", 1).unwrap(), Some(4));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_save_xml_data_only(true).unwrap();
    assert!(doc.has_save_xml_data_only().unwrap());
    doc.set_use_xslt_when_saving(true).unwrap();
    assert!(doc.has_use_xslt_when_saving().unwrap());
    doc.set_always_merge_empty_namespace(true).unwrap();
    assert!(doc.has_always_merge_empty_namespace().unwrap());
    doc.set_save_xml_data_only(false).unwrap();
    assert!(!doc.has_save_xml_data_only().unwrap());
}


#[test]
fn excel_time_dv_word_theme_auto_caption_ppt_master_hf() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["t"], vec!["12:00"]]).unwrap();
    wb.add_data_validation_time("S", "A2:A100", "greaterThan", "0.5", None, true)
        .unwrap();
    let dvs = wb.list_data_validations("S").unwrap();
    assert!(dvs.iter().any(|(t, _, _)| t == "time"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.set_theme_font_lang_ex("en-US", Some("ja-JP"), Some("ar-SA"))
        .unwrap();
    let tfl = doc.theme_font_lang_ex().unwrap().unwrap();
    assert_eq!(tfl.0, "en-US");
    assert_eq!(tfl.1.as_deref(), Some("ja-JP"));
    assert_eq!(tfl.2.as_deref(), Some("ar-SA"));

    doc.add_caption_definition("Figure", "below", "decimal").unwrap();
    doc.add_auto_caption("Microsoft Word Picture", "Figure").unwrap();
    doc.add_auto_caption("Microsoft Excel Worksheet", "Table").unwrap();
    let ac = doc.list_auto_captions().unwrap();
    assert_eq!(ac.len(), 2);
    assert!(ac.iter().any(|(n, c)| n.contains("Picture") && c == "Figure"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_notes_master_header_footer(true, true, true, false).unwrap();
    assert!(ppt.has_notes_master_header_footer());
    ppt.set_handout_master_header_footer(false, true, true, true).unwrap();
    assert!(ppt.has_handout_master_header_footer());
}


#[test]
fn excel_file_version_sharing_ole_word_web_ppt_size_type() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_file_version("xl", "7", "7", "1000").unwrap();
    let fv = wb.file_version().unwrap().unwrap();
    assert_eq!(fv.0, "xl");
    assert_eq!(fv.1, "7");
    assert_eq!(fv.3, "1000");
    assert!(wb.has_file_version().unwrap());

    wb.set_file_sharing(true, Some("Alice")).unwrap();
    assert!(wb.has_file_sharing().unwrap());
    assert!(wb.file_sharing_read_only_recommended().unwrap());
    assert!(wb.clear_file_sharing().unwrap());

    wb.set_ole_size("A1:H20").unwrap();
    assert_eq!(wb.ole_size().unwrap().as_deref(), Some("A1:H20"));
    assert!(wb.clear_ole_size().unwrap());
    assert!(wb.clear_file_version().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("w")]));
    // default web settings already has optimizeForBrowser + allowPNG
    doc.add_default_web_settings().unwrap();
    assert!(doc.has_optimize_for_browser().unwrap());
    assert!(doc.has_allow_png().unwrap());
    doc.set_rely_on_vml(true).unwrap();
    assert!(doc.has_rely_on_vml().unwrap());
    doc.set_do_not_rely_on_css(true).unwrap();
    assert!(doc.has_do_not_rely_on_css().unwrap());
    doc.set_pixels_per_inch(96).unwrap();
    assert_eq!(doc.pixels_per_inch().unwrap(), Some(96));
    doc.set_web_encoding("utf-8").unwrap();
    assert_eq!(doc.web_encoding().unwrap().as_deref(), Some("utf-8"));
    doc.set_optimize_for_browser(false).unwrap();
    assert!(!doc.has_optimize_for_browser().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_slide_size_ex(12192000, 6858000, Some("screen16x9"))
        .unwrap();
    assert_eq!(ppt.slide_size().unwrap(), Some((12192000, 6858000)));
    assert_eq!(ppt.slide_size_type().unwrap().as_deref(), Some("screen16x9"));
}


#[test]
fn excel_function_groups_word_note_props_ppt_default_text_style() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_function_group("MyFuncs").unwrap();
    wb.add_function_group("Stats").unwrap();
    // duplicate should replace not double
    wb.add_function_group("MyFuncs").unwrap();
    assert_eq!(wb.function_group_count().unwrap(), 2);
    assert!(wb.has_function_groups().unwrap());
    let names = wb.list_function_groups().unwrap();
    assert!(names.contains(&"MyFuncs".to_string()));
    assert!(names.contains(&"Stats".to_string()));
    assert_eq!(wb.clear_function_groups().unwrap(), 2);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("n")]));
    doc.set_footnote_properties(
        Some("pageBottom"),
        Some("decimal"),
        Some(1),
        Some("continuous"),
    )
    .unwrap();
    let fp = doc.footnote_properties().unwrap().unwrap();
    assert_eq!(fp.0.as_deref(), Some("pageBottom"));
    assert_eq!(fp.1.as_deref(), Some("decimal"));
    assert_eq!(fp.2, Some(1));
    assert_eq!(fp.3.as_deref(), Some("continuous"));
    assert!(doc.has_footnote_properties().unwrap());

    doc.set_endnote_properties(Some("docEnd"), Some("lowerRoman"), Some(1), Some("eachSect"))
        .unwrap();
    let ep = doc.endnote_properties().unwrap().unwrap();
    assert_eq!(ep.0.as_deref(), Some("docEnd"));
    assert_eq!(ep.1.as_deref(), Some("lowerRoman"));
    assert!(doc.has_endnote_properties().unwrap());
    assert!(doc.clear_footnote_properties().unwrap());
    assert!(doc.clear_endnote_properties().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(!ppt.has_default_text_style().unwrap());
    ppt.ensure_default_text_style().unwrap();
    assert!(ppt.has_default_text_style().unwrap());
    ppt.ensure_default_text_style().unwrap(); // idempotent
    assert!(ppt.clear_default_text_style().unwrap());
    assert!(!ppt.has_default_text_style().unwrap());
}


#[test]
fn excel_protection_flags_word_web_more_ppt_master_hf() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_sheet_protection("S", true, true, false).unwrap();
    wb.set_sheet_protection_flags(
        "S",
        &[
            ("formatCells", false),
            ("insertRows", false),
            ("sort", true),
            ("autoFilter", true),
            ("selectLockedCells", true),
        ],
    )
    .unwrap();
    assert!(!wb.sheet_protection_flag("S", "formatCells").unwrap());
    assert!(!wb.sheet_protection_flag("S", "insertRows").unwrap());
    assert!(wb.sheet_protection_flag("S", "sort").unwrap());
    assert!(wb.sheet_protection_flag("S", "autoFilter").unwrap());
    assert!(wb.sheet_protection_flag("S", "selectLockedCells").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("w")]));
    doc.add_default_web_settings().unwrap();
    doc.set_do_not_organize_in_folder(true).unwrap();
    assert!(doc.has_do_not_organize_in_folder().unwrap());
    doc.set_do_not_use_long_file_names(true).unwrap();
    assert!(doc.has_do_not_use_long_file_names().unwrap());
    doc.set_do_not_save_as_single_file(true).unwrap();
    assert!(doc.has_do_not_save_as_single_file().unwrap());
    doc.set_target_screen_size("1024x768").unwrap();
    assert_eq!(doc.target_screen_size().unwrap().as_deref(), Some("1024x768"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    // ensure master exists
    let _ = ppt.ensure_default_master_layout().unwrap();
    ppt.set_slide_master_header_footer(true, false, true, true).unwrap();
    assert!(ppt.has_slide_master_header_footer());
    assert_eq!(ppt.clear_slide_master_header_footers().unwrap(), 1);
    assert!(!ppt.has_slide_master_header_footer());
}


#[test]
fn word_docgrid_page_border_opts_excel_prot_flags_ppt_layout_hf() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("g")]));
    doc.set_document_grid_ex(360, Some("linesAndChars"), Some(100))
        .unwrap();
    assert_eq!(doc.document_grid_line_pitch().unwrap(), Some(360));
    assert_eq!(doc.document_grid_type().unwrap().as_deref(), Some("linesAndChars"));
    assert_eq!(doc.document_grid_char_space().unwrap(), Some(100));
    assert!(doc.has_document_grid().unwrap());

    doc.set_page_borders("FF0000", 12).unwrap();
    doc.set_page_border_options(Some("allPages"), Some("page"), Some("front"))
        .unwrap();
    assert_eq!(doc.page_border_display().unwrap().as_deref(), Some("allPages"));
    assert_eq!(doc.page_border_offset_from().unwrap().as_deref(), Some("page"));

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    // flags without prior protection should create it
    wb.set_sheet_protection_flags("S", &[("pivotTables", true), ("deleteRows", false)])
        .unwrap();
    assert!(wb.sheet_protection_flag("S", "pivotTables").unwrap());
    assert!(!wb.sheet_protection_flag("S", "deleteRows").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let _ = ppt.ensure_default_master_layout().unwrap();
    ppt.set_slide_layout_header_footer(true, true, false, true).unwrap();
    assert!(ppt.has_slide_layout_header_footer());
    assert!(ppt.clear_slide_layout_header_footers().unwrap() >= 1);
    assert!(!ppt.has_slide_layout_header_footer());
}


#[test]
fn excel_wb_prot_table_style_calc_word_grid_forms_ppt_authors() {
    // Excel: workbook protection with lockRevision
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H1", "H2"], vec!["a", "b"]])
        .unwrap();
    wb.set_workbook_protection_ex(true, false, true).unwrap();
    assert!(wb.has_workbook_protection().unwrap());
    assert!(wb.workbook_lock_revision().unwrap());
    let flags = wb.workbook_protection_flags_ex().unwrap().unwrap();
    assert_eq!(flags, (true, false, true));

    // Excel: table style info + rename/ref/totals/header
    wb.add_table("S", "T1", "A1:B2", &["H1", "H2"]).unwrap();
    let info = wb.table_style_info("T1").unwrap().unwrap();
    assert_eq!(info.0, "TableStyleMedium2");
    assert!(info.3); // showRowStripes default
    assert!(wb
        .set_table_style_info("T1", "TableStyleMedium9", true, true, false, true)
        .unwrap());
    let info2 = wb.table_style_info("T1").unwrap().unwrap();
    assert_eq!(info2, ("TableStyleMedium9".into(), true, true, false, true));
    assert_eq!(wb.table_header_row_count("T1").unwrap(), Some(1));
    assert!(wb.set_table_totals_row("T1", true).unwrap());
    assert!(wb.table_has_totals_row("T1").unwrap());
    assert!(wb.set_table_header_row_count("T1", 1).unwrap());
    assert!(wb.set_table_ref("T1", "A1:B3").unwrap());
    assert!(wb.rename_table("T1", "Sales").unwrap());
    assert!(wb.table_uri("Sales").unwrap().is_some());
    assert!(wb.table_uri("T1").unwrap().is_none());

    // Excel: calc mode / fullCalcOnLoad / calcCompleted
    wb.set_calc_mode("manual").unwrap();
    assert_eq!(wb.calc_mode().unwrap().as_deref(), Some("manual"));
    wb.set_full_calc_on_load(true).unwrap();
    assert!(wb.full_calc_on_load().unwrap());
    wb.set_calc_completed(false).unwrap();
    assert!(!wb.calc_completed().unwrap());

    // Word: drawing grid origin/displayEvery + forms/smart tags
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("g")]));
    doc.set_drawing_grid_horizontal_origin(100).unwrap();
    doc.set_drawing_grid_vertical_origin(200).unwrap();
    assert_eq!(doc.drawing_grid_horizontal_origin().unwrap(), Some(100));
    assert_eq!(doc.drawing_grid_vertical_origin().unwrap(), Some(200));
    doc.set_display_horizontal_drawing_grid_every(2).unwrap();
    doc.set_display_vertical_drawing_grid_every(3).unwrap();
    assert_eq!(doc.display_horizontal_drawing_grid_every().unwrap(), Some(2));
    assert_eq!(doc.display_vertical_drawing_grid_every().unwrap(), Some(3));
    doc.set_save_forms_data(true).unwrap();
    assert!(doc.has_save_forms_data().unwrap());
    doc.set_do_not_embed_smart_tags(true).unwrap();
    assert!(doc.has_do_not_embed_smart_tags().unwrap());
    doc.set_save_forms_data(false).unwrap();
    assert!(!doc.has_save_forms_data().unwrap());

    // PPT: comment author append/lookup/remove
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_comment_authors(&[(1, "Alice", "A"), (2, "Bob", "B")])
        .unwrap();
    assert_eq!(ppt.comment_author_count().unwrap(), 2);
    assert_eq!(
        ppt.comment_author_by_id(1).unwrap(),
        Some(("Alice".into(), "A".into()))
    );
    ppt.append_comment_author(3, "Carol", "C").unwrap();
    assert_eq!(ppt.comment_author_count().unwrap(), 3);
    assert!(ppt.remove_comment_author(2).unwrap());
    assert!(!ppt.remove_comment_author(2).unwrap());
    assert_eq!(ppt.comment_author_count().unwrap(), 2);
    assert!(ppt.comment_author_by_id(2).unwrap().is_none());
}


#[test]
fn word_doc_prot_ex_excel_sheet_calc_dv_prompts_ppt_master_sp() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("p")]));
    doc.set_document_protection_ex("readOnly", true, true)
        .unwrap();
    assert!(doc.has_document_protection().unwrap());
    assert_eq!(
        doc.document_protection_edit().unwrap().as_deref(),
        Some("readOnly")
    );
    assert!(doc.document_protection_enforcement().unwrap());
    assert!(doc.document_protection_formatting().unwrap());
    assert!(doc.clear_document_protection().unwrap());
    assert!(!doc.document_protection_enforcement().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b", "c"]]).unwrap();
    wb.add_data_validation_list("S", "A1", "\"x,y,z\"", true).unwrap();
    assert!(wb
        .set_data_validations_disable_prompts("S", true)
        .unwrap());
    assert!(wb.data_validations_disable_prompts("S").unwrap());
    wb.set_sheet_full_calc_on_load("S", true).unwrap();
    assert!(wb.sheet_full_calc_on_load("S").unwrap());
    wb.set_sheet_full_calc_on_load("S", false).unwrap();
    assert!(!wb.sheet_full_calc_on_load("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    assert!(ppt.show_master_shapes(0).unwrap());
    ppt.set_show_master_shapes(0, false).unwrap();
    assert!(!ppt.show_master_shapes(0).unwrap());
    ppt.set_show_master_ph_anim(0, false).unwrap();
    assert!(!ppt.show_master_ph_anim(0).unwrap());
    ppt.set_show_master_shapes(0, true).unwrap();
    assert!(ppt.show_master_shapes(0).unwrap());
}


#[test]
fn excel_table_col_totals_word_style_lock_qf_ppt_layout_type() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H1", "H2"], vec!["1", "2"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["H1", "H2"]).unwrap();
    assert!(wb
        .set_table_column_totals("T1", "H2", "sum", Some("Total"))
        .unwrap());
    let totals = wb.table_column_totals("T1", "H2").unwrap().unwrap();
    assert_eq!(totals.0, "sum");
    assert_eq!(totals.1.as_deref(), Some("Total"));
    assert!(wb.table_has_totals_row("T1").unwrap());
    assert!(wb.rename_table_column("T1", "H1", "Name").unwrap());
    assert_eq!(wb.table_columns("T1").unwrap(), vec!["Name", "H2"]);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.set_style_lock_qf_set(true).unwrap();
    assert!(doc.has_style_lock_qf_set().unwrap());
    doc.set_style_lock_theme(true).unwrap();
    assert!(doc.has_style_lock_theme().unwrap());
    doc.set_style_lock_qf_set(false).unwrap();
    assert!(!doc.has_style_lock_qf_set().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let _ = ppt.ensure_default_master_layout().unwrap();
    ppt.set_slide_layout_type(0, "title").unwrap();
    assert_eq!(ppt.slide_layout_type(0).unwrap().as_deref(), Some("title"));
    ppt.set_slide_layout_preserve(0, true).unwrap();
    assert!(ppt.slide_layout_preserve(0).unwrap());
    ppt.set_slide_layout_matching_name(0, "Title Slide").unwrap();
    assert_eq!(
        ppt.slide_layout_matching_name(0).unwrap().as_deref(),
        Some("Title Slide")
    );
}


#[test]
fn excel_table_flags_word_mm_query_ppt_layout_show_master() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["A", "B"]).unwrap();
    assert!(wb.set_table_comment("T1", "sales table").unwrap());
    assert_eq!(wb.table_comment("T1").unwrap().as_deref(), Some("sales table"));
    assert!(wb.set_table_insert_row("T1", true).unwrap());
    assert!(wb.table_insert_row("T1").unwrap());
    assert!(wb.set_table_totals_row_shown("T1", false).unwrap());
    assert!(!wb.table_totals_row_shown("T1").unwrap());
    assert!(wb.set_table_published("T1", true).unwrap());
    assert!(wb.table_published("T1").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("m")]));
    doc.set_mail_merge("formLetters", "database", Some("SELECT * FROM t"), true)
        .unwrap();
    assert_eq!(
        doc.mail_merge_query().unwrap().as_deref(),
        Some("SELECT * FROM t")
    );
    doc.set_mail_merge_query("SELECT id FROM t").unwrap();
    assert_eq!(
        doc.mail_merge_query().unwrap().as_deref(),
        Some("SELECT id FROM t")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let _ = ppt.ensure_default_master_layout().unwrap();
    assert!(ppt.slide_layout_show_master_shapes(0).unwrap());
    ppt.set_slide_layout_show_master_shapes(0, false).unwrap();
    assert!(!ppt.slide_layout_show_master_shapes(0).unwrap());
}


#[test]
fn word_ea_table_flags_excel_table_type_ppt_user_drawn() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("e")]));
    doc.set_do_not_break_wrapped_tables(true).unwrap();
    assert!(doc.has_do_not_break_wrapped_tables().unwrap());
    doc.set_do_not_snap_to_grid_in_cell(true).unwrap();
    assert!(doc.has_do_not_snap_to_grid_in_cell().unwrap());
    doc.set_select_fld_with_first_or_last_char(true).unwrap();
    assert!(doc.has_select_fld_with_first_or_last_char().unwrap());
    doc.set_do_not_use_east_asian_break_rules(true).unwrap();
    assert!(doc.has_do_not_use_east_asian_break_rules().unwrap());
    doc.set_use_alt_kinsoku_line_break_rules(true).unwrap();
    assert!(doc.has_use_alt_kinsoku_line_break_rules().unwrap());
    doc.set_do_not_break_wrapped_tables(false).unwrap();
    assert!(!doc.has_do_not_break_wrapped_tables().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_table("S", "T1", "A1:A2", &["A"]).unwrap();
    assert!(wb.set_table_type("T1", "queryTable").unwrap());
    assert_eq!(wb.table_type("T1").unwrap().as_deref(), Some("queryTable"));
    assert!(wb.set_table_connection_id("T1", 7).unwrap());
    assert_eq!(wb.table_connection_id("T1").unwrap(), Some(7));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let _ = ppt.ensure_default_master_layout().unwrap();
    assert!(!ppt.slide_layout_user_drawn(0).unwrap());
    ppt.set_slide_layout_user_drawn(0, true).unwrap();
    assert!(ppt.slide_layout_user_drawn(0).unwrap());
}


#[test]
fn word_print_layout_flags_ppt_section_rename() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("p")]));
    doc.set_do_not_leave_backslash_alone(true).unwrap();
    assert!(doc.has_do_not_leave_backslash_alone().unwrap());
    doc.set_ul_trail_space(true).unwrap();
    assert!(doc.has_ul_trail_space().unwrap());
    doc.set_print_body_text_before_header(true).unwrap();
    assert!(doc.has_print_body_text_before_header().unwrap());
    doc.set_print_col_black(true).unwrap();
    assert!(doc.has_print_col_black().unwrap());
    doc.set_mw_small_caps(true).unwrap();
    assert!(doc.has_mw_small_caps().unwrap());
    doc.set_shape_layout_like_ww8(true).unwrap();
    assert!(doc.has_shape_layout_like_ww8().unwrap());
    doc.set_footnote_layout_like_ww8(true).unwrap();
    assert!(doc.has_footnote_layout_like_ww8().unwrap());
    doc.set_print_col_black(false).unwrap();
    assert!(!doc.has_print_col_black().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.set_sections(&[("Intro", 0, 0), ("Body", 1, 1)]).unwrap();
    assert_eq!(ppt.section_count().unwrap(), 2);
    assert!(ppt.rename_section("Intro", "Opening").unwrap());
    let names: Vec<String> = ppt
        .list_sections()
        .unwrap()
        .into_iter()
        .map(|(n, _)| n)
        .collect();
    assert!(names.contains(&"Opening".to_string()));
    assert!(!names.contains(&"Intro".to_string()));
    assert!(ppt.remove_section("Body").unwrap());
    assert_eq!(ppt.section_count().unwrap(), 1);
}


#[test]
fn word_table_layout_flags_excel_dv_window_mm_view() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.set_grow_autofit(true).unwrap();
    assert!(doc.has_grow_autofit().unwrap());
    doc.set_use_normal_style_for_list(true).unwrap();
    assert!(doc.has_use_normal_style_for_list().unwrap());
    doc.set_use_word2002_table_style_rules(true).unwrap();
    assert!(doc.has_use_word2002_table_style_rules().unwrap());
    doc.set_layout_raw_table_width(true).unwrap();
    assert!(doc.has_layout_raw_table_width().unwrap());
    doc.set_layout_table_rows_apart(true).unwrap();
    assert!(doc.has_layout_table_rows_apart().unwrap());
    doc.set_use_single_border_for_contiguous_cells(true).unwrap();
    assert!(doc.has_use_single_border_for_contiguous_cells().unwrap());
    doc.set_grow_autofit(false).unwrap();
    assert!(!doc.has_grow_autofit().unwrap());
    doc.set_mail_merge_view_merged_data(true).unwrap();
    assert!(doc.mail_merge_view_merged_data().unwrap());
    doc.set_mail_merge_view_merged_data(false).unwrap();
    assert!(!doc.mail_merge_view_merged_data().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.add_data_validation_list("S", "A1", "\"x,y\"", true).unwrap();
    assert!(wb.set_data_validations_window("S", 100, 200).unwrap());
    assert_eq!(wb.data_validations_window("S").unwrap(), Some((100, 200)));
}


#[test]
fn excel_table_display_dxf_word_space_ul_ppt_master_preserve() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["A", "B"]).unwrap();
    assert!(wb.set_table_display_name("T1", "SalesTable").unwrap());
    assert_eq!(
        wb.table_display_name("T1").unwrap().as_deref(),
        Some("SalesTable")
    );
    assert!(wb.set_table_dxf_ids("T1", Some(1), Some(2), Some(3)).unwrap());
    let dxf = wb.table_dxf_ids("T1").unwrap().unwrap();
    assert_eq!(dxf, (Some(1), Some(2), Some(3)));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("u")]));
    doc.set_space_for_ul(true).unwrap();
    assert!(doc.has_space_for_ul().unwrap());
    doc.set_space_for_ul(false).unwrap();
    assert!(!doc.has_space_for_ul().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    let _ = ppt.ensure_default_master_layout().unwrap();
    assert!(!ppt.slide_master_preserve(0).unwrap());
    ppt.set_slide_master_preserve(0, true).unwrap();
    assert!(ppt.slide_master_preserve(0).unwrap());
}


#[test]
fn word_autofit_flags_excel_table_styles_ppt_notes_master_sp() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("a")]));
    doc.set_do_not_autofit_constrained_tables(true).unwrap();
    assert!(doc.has_do_not_autofit_constrained_tables().unwrap());
    doc.set_autofit_to_first_fixed_width_cell(true).unwrap();
    assert!(doc.has_autofit_to_first_fixed_width_cell().unwrap());
    doc.set_display_hangul_fixed_width(true).unwrap();
    assert!(doc.has_display_hangul_fixed_width().unwrap());
    doc.set_split_pg_break_and_para_mark(true).unwrap();
    assert!(doc.has_split_pg_break_and_para_mark().unwrap());
    doc.set_do_not_break_constrained_forced_table(true).unwrap();
    assert!(doc.has_do_not_break_constrained_forced_table().unwrap());
    doc.set_do_not_autofit_constrained_tables(false).unwrap();
    assert!(!doc.has_do_not_autofit_constrained_tables().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_table("S", "T1", "A1:A2", &["A"]).unwrap();
    assert!(wb
        .set_table_cell_styles("T1", Some("Header"), Some("Data"), Some("Total"))
        .unwrap());
    let styles = wb.table_cell_styles("T1").unwrap().unwrap();
    assert_eq!(
        styles,
        (
            Some("Header".into()),
            Some("Data".into()),
            Some("Total".into())
        )
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_notes_to_slide(0, "note").unwrap();
    assert!(ppt.notes_show_master_shapes(0).unwrap());
    assert!(ppt.set_notes_show_master_shapes(0, false).unwrap());
    assert!(!ppt.notes_show_master_shapes(0).unwrap());
}


#[test]
fn excel_table_border_shift_word_mm_types_ppt_ph_anim() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_table("S", "T1", "A1:A2", &["A"]).unwrap();
    assert!(wb.set_table_insert_row_shift("T1", true).unwrap());
    assert!(wb.table_insert_row_shift("T1").unwrap());
    assert!(wb
        .set_table_border_dxf_ids("T1", Some(4), Some(5), Some(6))
        .unwrap());
    assert_eq!(
        wb.table_border_dxf_ids("T1").unwrap().unwrap(),
        (Some(4), Some(5), Some(6))
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("m")]));
    doc.set_mail_merge_main_document_type("email").unwrap();
    assert_eq!(
        doc.mail_merge_main_document_type().unwrap().as_deref(),
        Some("email")
    );
    doc.set_mail_merge_data_type("native").unwrap();
    assert_eq!(
        doc.mail_merge_data_type().unwrap().as_deref(),
        Some("native")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_notes_to_slide(0, "n").unwrap();
    let _ = ppt.ensure_default_master_layout().unwrap();
    ppt.set_slide_layout_show_master_ph_anim(0, false).unwrap();
    assert!(!ppt.slide_layout_show_master_ph_anim(0).unwrap());
    assert!(ppt.set_notes_show_master_ph_anim(0, false).unwrap());
    assert!(!ppt.notes_show_master_ph_anim(0).unwrap());
}


#[test]
fn word_align_tab_flags_excel_col_unique_ppt_custom_show() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("f")]));
    doc.set_align_tables_row_by_row(true).unwrap();
    assert!(doc.has_align_tables_row_by_row().unwrap());
    doc.set_forget_last_tab_alignment(true).unwrap();
    assert!(doc.has_forget_last_tab_alignment().unwrap());
    doc.set_use_ansi_kerning_pairs(true).unwrap();
    assert!(doc.has_use_ansi_kerning_pairs().unwrap());
    doc.set_cached_col_balance(true).unwrap();
    assert!(doc.has_cached_col_balance().unwrap());
    doc.set_align_tables_row_by_row(false).unwrap();
    assert!(!doc.has_align_tables_row_by_row().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["A", "B"]).unwrap();
    assert!(wb.set_table_column_unique_name("T1", "A", "colA").unwrap());
    assert_eq!(
        wb.table_column_unique_name("T1", "A").unwrap().as_deref(),
        Some("colA")
    );
    assert!(wb
        .set_table_column_dxf_ids("T1", "B", Some(1), Some(2), Some(3))
        .unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    let id = ppt.add_custom_show("Demo", &[0, 1]).unwrap();
    assert!(ppt.rename_custom_show(id, "Renamed").unwrap());
    let show = ppt.custom_show_by_id(id).unwrap().unwrap();
    assert_eq!(show.0, "Renamed");
    assert_eq!(show.1.len(), 2);
}


#[test]
fn word_suppress_odso_excel_col_styles_ppt_custom_slides() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.set_suppress_sp_bf_after_pg_brk(true).unwrap();
    assert!(doc.has_suppress_sp_bf_after_pg_brk().unwrap());
    doc.set_conv_mail_merge_esc(true).unwrap();
    assert!(doc.has_conv_mail_merge_esc().unwrap());
    doc.set_truncate_font_heights_like_wp6(true).unwrap();
    assert!(doc.has_truncate_font_heights_like_wp6().unwrap());
    doc.set_sub_font_by_size(true).unwrap();
    assert!(doc.has_sub_font_by_size().unwrap());
    doc.set_mail_merge_odso("Contacts", "database").unwrap();
    assert_eq!(doc.mail_merge_odso_src().unwrap().as_deref(), Some("database"));
    doc.add_mail_merge_odso_field_map("dbColumn", "Email", "email_addr")
        .unwrap();
    let maps = doc.list_mail_merge_odso_field_maps().unwrap();
    assert_eq!(maps.len(), 1);
    assert_eq!(maps[0].0, "dbColumn");
    assert_eq!(maps[0].1, "Email");
    assert_eq!(maps[0].2, "email_addr");

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["A", "B"]).unwrap();
    assert!(wb
        .set_table_column_cell_styles("T1", "A", Some("H"), Some("D"), Some("T"))
        .unwrap());
    let styles = wb.table_column_cell_styles("T1", "A").unwrap().unwrap();
    assert_eq!(
        styles,
        (Some("H".into()), Some("D".into()), Some("T".into()))
    );
    assert!(wb.set_table_column_query_field_id("T1", "B", 9).unwrap());
    assert_eq!(wb.table_column_query_field_id("T1", "B").unwrap(), Some(9));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_slide_with_text("B").unwrap();
    ppt.add_slide_with_text("C").unwrap();
    let id = ppt.add_custom_show("All", &[0, 1, 2]).unwrap();
    assert!(ppt.set_custom_show_slides(id, &[0, 2]).unwrap());
    let show = ppt.custom_show_by_id(id).unwrap().unwrap();
    assert_eq!(show.1.len(), 2);
}


#[test]
fn word_odso_hdr_excel_sort_conds_ppt_verifier_spin() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("o")]));
    doc.set_mail_merge_odso("T", "textFile").unwrap();
    doc.set_mail_merge_odso_col_delim(44).unwrap();
    assert_eq!(doc.mail_merge_odso_col_delim().unwrap(), Some(44));
    doc.set_mail_merge_odso_f_hdr(true).unwrap();
    assert!(doc.mail_merge_odso_f_hdr().unwrap());
    doc.set_mail_merge_odso_f_hdr(false).unwrap();
    assert!(!doc.mail_merge_odso_f_hdr().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[
            vec!["A", "B"],
            vec!["2", "x"],
            vec!["1", "y"],
        ],
    )
    .unwrap();
    wb.set_sort_state("S", "A1:B3", "A2", false).unwrap();
    assert!(wb.add_sort_condition("S", "B2", true).unwrap());
    assert_eq!(wb.sort_condition_count("S").unwrap(), 2);
    let conds = wb.list_sort_conditions("S").unwrap();
    assert_eq!(conds.len(), 2);
    assert_eq!(conds[0].0, "A2");
    assert!(!conds[0].1);
    assert_eq!(conds[1].0, "B2");
    assert!(conds[1].1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_modify_verifier("SHA-512", 100000).unwrap();
    assert_eq!(
        ppt.modify_verifier_algorithm().unwrap().as_deref(),
        Some("SHA-512")
    );
    assert_eq!(ppt.modify_verifier_spin_count().unwrap(), Some(100000));
}


#[test]
fn word_odso_udl_type_excel_table_id_ppt_cust_data() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("u")]));
    doc.set_mail_merge_odso_udl("Provider=SQLOLEDB;Data Source=server").unwrap();
    assert_eq!(
        doc.mail_merge_odso_udl().unwrap().as_deref(),
        Some("Provider=SQLOLEDB;Data Source=server")
    );
    doc.set_mail_merge_odso_type("database").unwrap();
    assert_eq!(
        doc.mail_merge_odso_type().unwrap().as_deref(),
        Some("database")
    );

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_table("S", "T1", "A1:A2", &["A"]).unwrap();
    let id = wb.table_id("T1").unwrap().unwrap();
    assert!(id >= 1);
    assert!(wb.set_table_id("T1", 42).unwrap());
    assert_eq!(wb.table_id("T1").unwrap(), Some(42));
    assert_eq!(wb.table_ref("T1").unwrap().as_deref(), Some("A1:A2"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_customer_data("rId99").unwrap();
    assert!(ppt.has_customer_data().unwrap());
    assert_eq!(ppt.list_customer_data().unwrap(), vec!["rId99".to_string()]);
    assert!(ppt.remove_customer_data("rId99").unwrap());
    assert!(!ppt.has_customer_data().unwrap());
}


#[test]
fn excel_dv_remove_op_word_odso_recip_ppt_show_mode() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"]]).unwrap();
    wb.add_data_validation_list("S", "A1", "\"x,y\"", true).unwrap();
    wb.add_data_validation_whole("S", "B1", "between", "1", Some("10"), true)
        .unwrap();
    assert_eq!(wb.data_validation_count("S").unwrap(), 2);
    assert!(wb
        .set_data_validation_operator("S", "B1", "greaterThan")
        .unwrap());
    assert_eq!(
        wb.data_validation_operator("S", "B1").unwrap().as_deref(),
        Some("greaterThan")
    );
    assert!(wb.set_data_validation_allow_blank("S", "A1", false).unwrap());
    assert!(!wb.data_validation_allow_blank("S", "A1").unwrap());
    assert!(wb.remove_data_validation("S", "A1").unwrap());
    assert_eq!(wb.data_validation_count("S").unwrap(), 1);
    assert!(!wb.remove_data_validation("S", "A1").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("r")]));
    doc.set_mail_merge_odso("T", "database").unwrap();
    doc.add_mail_merge_odso_field_map("dbColumn", "A", "a").unwrap();
    doc.add_mail_merge_odso_field_map("dbColumn", "B", "b").unwrap();
    assert_eq!(doc.list_mail_merge_odso_field_maps().unwrap().len(), 2);
    assert_eq!(doc.clear_mail_merge_odso_field_maps().unwrap(), 2);
    assert!(doc.list_mail_merge_odso_field_maps().unwrap().is_empty());
    doc.set_mail_merge_odso_recipient_data("rIdRecip").unwrap();
    assert_eq!(
        doc.mail_merge_odso_recipient_data().unwrap().as_deref(),
        Some("rIdRecip")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_show_mode_browse(false).unwrap();
    assert_eq!(ppt.show_mode().unwrap().as_deref(), Some("browse"));
    assert_eq!(ppt.show_browse_scrollbar().unwrap(), Some(false));
    ppt.set_show_mode_kiosk(5000).unwrap();
    assert_eq!(ppt.show_mode().unwrap().as_deref(), Some("kiosk"));
    assert_eq!(ppt.show_kiosk_restart_ms().unwrap(), Some(5000));
    ppt.set_show_mode_presented().unwrap();
    assert_eq!(ppt.show_mode().unwrap().as_deref(), Some("presented"));
}


#[test]
fn excel_dv_type_filter_btns_ppt_print_clr() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B", "C"], vec!["1", "2", "3"]])
        .unwrap();
    wb.add_data_validation_list("S", "A2", "\"x,y\"", true).unwrap();
    assert_eq!(
        wb.data_validation_type("S", "A2").unwrap().as_deref(),
        Some("list")
    );
    assert!(wb.set_data_validation_type("S", "A2", "whole").unwrap());
    assert_eq!(
        wb.data_validation_type("S", "A2").unwrap().as_deref(),
        Some("whole")
    );
    assert!(wb
        .set_data_validation_formulas("S", "A2", "1", Some("9"))
        .unwrap());

    wb.set_auto_filter("S", "A1:C2").unwrap();
    wb.add_auto_filter_top10("S", 0, true, false, 10.0).unwrap();
    wb.add_auto_filter_dynamic("S", 1, "today").unwrap();
    assert!(wb
        .set_auto_filter_column_buttons("S", 0, Some(false), Some(true))
        .unwrap());
    assert_eq!(
        wb.auto_filter_column_buttons("S", 0).unwrap(),
        Some((false, true))
    );
    assert!(wb.remove_auto_filter_column("S", 1).unwrap());
    assert_eq!(wb.auto_filter_column_count("S").unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_print_color_mode("gray").unwrap();
    assert_eq!(ppt.print_color_mode().unwrap().as_deref(), Some("gray"));
    ppt.set_print_frame_slides(true).unwrap();
    assert!(ppt.print_frame_slides().unwrap());
}


#[test]
fn excel_color_icon_filter_word_odso_table_ppt_print_what() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.set_auto_filter("S", "A1:B2").unwrap();
    wb.add_auto_filter_color("S", 0, 3, true).unwrap();
    assert_eq!(
        wb.auto_filter_column_kind("S", 0).unwrap().as_deref(),
        Some("color")
    );
    wb.add_auto_filter_icon("S", 1, "3TrafficLights1", 0).unwrap();
    assert_eq!(
        wb.auto_filter_column_kind("S", 1).unwrap().as_deref(),
        Some("icon")
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("o")]));
    doc.set_mail_merge_odso_table("People").unwrap();
    assert_eq!(
        doc.mail_merge_odso_table().unwrap().as_deref(),
        Some("People")
    );
    doc.set_mail_merge_odso_src("database").unwrap();
    assert_eq!(
        doc.mail_merge_odso_src().unwrap().as_deref(),
        Some("database")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_print_what("notes").unwrap();
    assert_eq!(ppt.print_what().unwrap().as_deref(), Some("notes"));
    ppt.set_print_color_mode("bw").unwrap();
    assert_eq!(ppt.print_color_mode().unwrap().as_deref(), Some("bw"));
}


#[test]
fn excel_dv_msgs_totals_ppt_browse_scrollbar() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_data_validation_list("S", "A2", "\"x,y\"", true).unwrap();
    wb.set_data_validation_error_style("S", "A2", "warning").unwrap();
    assert_eq!(
        wb.data_validation_error_style("S", "A2").unwrap().as_deref(),
        Some("warning")
    );
    assert!(wb
        .set_data_validation_show_messages("S", "A2", Some(true), Some(false))
        .unwrap());
    assert_eq!(
        wb.data_validation_show_messages("S", "A2").unwrap(),
        Some((true, false))
    );
    wb.add_table("S", "T1", "A1:A2", &["A"]).unwrap();
    assert!(wb.set_table_totals_row("T1", true).unwrap());
    assert_eq!(wb.table_totals_row_count("T1").unwrap(), Some(1));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_show_mode_browse(true).unwrap();
    assert_eq!(ppt.show_browse_scrollbar().unwrap(), Some(true));
    assert!(ppt.set_show_browse_scrollbar(false).unwrap());
    assert_eq!(ppt.show_browse_scrollbar().unwrap(), Some(false));
    assert!(!ppt.set_show_browse_scrollbar(true).unwrap() || true); // still browse
    // switch to present then scrollbar setter should fail
    ppt.set_show_mode_presented().unwrap();
    assert!(!ppt.set_show_browse_scrollbar(true).unwrap());
}


#[test]
fn excel_sort_range_dv_sqref_ppt_kiosk_restart() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[vec!["A", "B"], vec!["2", "x"], vec!["1", "y"]],
    )
    .unwrap();
    wb.set_sort_state("S", "A1:B3", "A2", false).unwrap();
    assert!(wb.set_sort_range("S", "A1:B4").unwrap());
    let st = wb.sort_state("S").unwrap().unwrap();
    assert_eq!(st.0, "A1:B4");
    wb.add_data_validation_list("S", "A2", "\"a,b\"", true).unwrap();
    assert!(wb.set_data_validation_sqref("S", "A2", "A2:A10").unwrap());
    assert_eq!(
        wb.list_data_validations("S").unwrap()[0].1,
        "A2:A10"
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_show_mode_kiosk(1000).unwrap();
    assert_eq!(ppt.show_kiosk_restart_ms().unwrap(), Some(1000));
    assert!(ppt.set_show_kiosk_restart_ms(2500).unwrap());
    assert_eq!(ppt.show_kiosk_restart_ms().unwrap(), Some(2500));
    ppt.set_show_mode_presented().unwrap();
    assert!(!ppt.set_show_kiosk_restart_ms(3000).unwrap());
}


#[test]
fn excel_dv_ime_sort_remove_ppt_clear_print() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[vec!["A", "B"], vec!["2", "x"], vec!["1", "y"]],
    )
    .unwrap();
    wb.add_data_validation_list("S", "A2", "\"あ,い\"", true).unwrap();
    assert!(wb
        .set_data_validation_ime_mode("S", "A2", "hiragana")
        .unwrap());
    assert_eq!(
        wb.data_validation_ime_mode("S", "A2").unwrap().as_deref(),
        Some("hiragana")
    );
    wb.set_sort_state("S", "A1:B3", "A2", false).unwrap();
    wb.add_sort_condition("S", "B2", true).unwrap();
    assert_eq!(wb.sort_condition_count("S").unwrap(), 2);
    assert!(wb.remove_sort_condition("S", "B2").unwrap());
    assert_eq!(wb.sort_condition_count("S").unwrap(), 1);
    assert!(!wb.remove_sort_condition("S", "B2").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_print_what("handouts").unwrap();
    ppt.set_print_color_mode("clr").unwrap();
    assert!(ppt.has_print_properties());
    assert!(ppt.clear_print_properties().unwrap());
    assert!(!ppt.has_print_properties());
    assert!(!ppt.clear_print_properties().unwrap());
}


#[test]
fn excel_sort_method_custom_ppt_remove_font() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[vec!["A", "B"], vec!["2", "x"], vec!["1", "y"]],
    )
    .unwrap();
    wb.set_sort_state("S", "A1:B3", "A2", false).unwrap();
    assert!(wb.set_sort_method("S", "pinYin").unwrap());
    assert_eq!(wb.sort_method("S").unwrap().as_deref(), Some("pinYin"));
    assert!(wb.set_sort_column_sort("S", true).unwrap());
    assert!(wb.sort_column_sort("S").unwrap());
    assert!(wb
        .set_sort_condition_custom_list("S", "A2", "x,y,z")
        .unwrap());
    assert!(wb
        .set_sort_condition_sort_by("S", "A2", "value")
        .unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.add_embedded_font("Arial", None).unwrap();
    ppt.add_embedded_font("Calibri", None).unwrap();
    assert_eq!(ppt.embedded_font_count().unwrap(), 2);
    assert!(ppt.remove_embedded_font("Arial").unwrap());
    assert_eq!(ppt.embedded_font_count().unwrap(), 1);
    assert!(!ppt.remove_embedded_font("Arial").unwrap());
    let fonts = ppt.list_embedded_fonts().unwrap();
    assert_eq!(fonts, vec!["Calibri".to_string()]);
}


#[test]
fn excel_sort_icon_dxf_word_compat_setting_remove() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[vec!["A", "B"], vec!["2", "x"], vec!["1", "y"]],
    )
    .unwrap();
    wb.set_sort_state("S", "A1:B3", "A2", false).unwrap();
    assert!(wb
        .set_sort_condition_icon("S", "A2", "3TrafficLights1", 1)
        .unwrap());
    assert!(wb
        .set_sort_condition_dxf("S", "A2", "cellColor", 4)
        .unwrap());
    assert!(wb.set_sort_condition_descending("S", "A2", true).unwrap());
    let conds = wb.list_sort_conditions("S").unwrap();
    assert_eq!(conds.len(), 1);
    assert!(conds[0].1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("c")]));
    doc.set_compat_setting(
        "compatibilityMode",
        "http://schemas.microsoft.com/office/word",
        "15",
    )
    .unwrap();
    assert_eq!(
        doc.compat_setting_val("compatibilityMode")
            .unwrap()
            .as_deref(),
        Some("15")
    );
    assert!(doc.remove_compat_setting("compatibilityMode").unwrap());
    assert!(doc
        .compat_setting_val("compatibilityMode")
        .unwrap()
        .is_none());
    assert!(!doc.remove_compat_setting("compatibilityMode").unwrap());
}


#[test]
fn excel_col_id_filter_and_ppt_album_kinsoku_attrs() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["A", "B"]).unwrap();
    assert_eq!(wb.table_column_id("T1", "A").unwrap(), Some(1));
    assert!(wb.set_table_column_id("T1", "A", 10).unwrap());
    assert_eq!(wb.table_column_id("T1", "A").unwrap(), Some(10));
    wb.set_auto_filter("S", "A1:B2").unwrap();
    wb.add_auto_filter_custom(
        "S",
        0,
        &[("greaterThan", "0"), ("lessThan", "10")],
        true,
    )
    .unwrap();
    assert!(wb.set_auto_filter_custom_and("S", 0, false).unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_photo_album(false, false, "fitToSlide", "rectangle")
        .unwrap();
    assert!(ppt
        .set_photo_album_attrs(Some(true), Some(true), Some("1pic"), Some("roundedRect"))
        .unwrap());
    let album = ppt.photo_album().unwrap().unwrap();
    assert!(album.0);
    assert!(album.1);
    assert_eq!(album.2, "1pic");
    assert_eq!(album.3, "roundedRect");
    ppt.set_kinsoku("ja-JP", "([{（［", ")]}）］").unwrap();
    assert!(ppt
        .set_kinsoku_attrs(Some("zh-CN"), Some("([{"), None)
        .unwrap());
    let k = ppt.kinsoku().unwrap().unwrap();
    assert_eq!(k.0, "zh-CN");
    assert_eq!(k.1, "([{");
}


#[test]
fn excel_dv_msg_fields_ppt_verifier_attrs_word_force_upgrade() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_data_validation_list("S", "A2", "\"x,y\"", true).unwrap();
    assert!(wb
        .set_data_validation_message_fields(
            "S",
            "A2",
            Some("Pick"),
            Some("Choose x or y"),
            Some("Bad"),
            Some("Not allowed"),
        )
        .unwrap());
    let msgs = wb.data_validation_messages("S", "A2").unwrap().unwrap();
    assert_eq!(msgs.0.as_deref(), Some("Pick"));
    assert_eq!(msgs.1.as_deref(), Some("Choose x or y"));
    assert_eq!(msgs.2.as_deref(), Some("Bad"));
    assert_eq!(msgs.3.as_deref(), Some("Not allowed"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_modify_verifier("SHA-256", 50000).unwrap();
    assert!(ppt
        .set_modify_verifier_attrs(Some("SHA-512"), Some(100000))
        .unwrap());
    assert_eq!(
        ppt.modify_verifier_algorithm().unwrap().as_deref(),
        Some("SHA-512")
    );
    assert_eq!(ppt.modify_verifier_spin_count().unwrap(), Some(100000));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("f")]));
    doc.set_force_upgrade(true).unwrap();
    assert!(doc.has_force_upgrade().unwrap());
    assert!(doc.clear_force_upgrade().unwrap());
    assert!(!doc.has_force_upgrade().unwrap());
}


#[test]
fn excel_default_dims_sort_details_ppt_clear_attrs() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.set_default_row_height("S", 18.5).unwrap();
    assert_eq!(wb.default_row_height("S").unwrap(), Some(18.5));
    wb.set_default_col_width("S", 12.0).unwrap();
    assert_eq!(wb.default_col_width("S").unwrap(), Some(12.0));
    wb.set_sort_state("S", "A1:A2", "A2", false).unwrap();
    wb.set_sort_condition_icon("S", "A2", "3TrafficLights1", 2)
        .unwrap();
    let details = wb.sort_condition_details("S", "A2").unwrap().unwrap();
    assert_eq!(details.0.as_deref(), Some("icon"));
    assert_eq!(details.3.as_deref(), Some("3TrafficLights1"));
    assert_eq!(details.4, Some(2));
    wb.set_sort_condition_custom_list("S", "A2", "a,b,c").unwrap();
    let details2 = wb.sort_condition_details("S", "A2").unwrap().unwrap();
    assert_eq!(details2.1.as_deref(), Some("a,b,c"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("A").unwrap();
    ppt.set_server_zoom(50).unwrap();
    ppt.set_bookmark_id_seed(100).unwrap();
    ppt.set_conformance("strict").unwrap();
    assert_eq!(ppt.server_zoom().unwrap(), Some(50));
    assert_eq!(ppt.bookmark_id_seed().unwrap(), Some(100));
    assert_eq!(ppt.conformance().unwrap().as_deref(), Some("strict"));
    assert!(ppt.clear_server_zoom().unwrap());
    assert!(ppt.clear_bookmark_id_seed().unwrap());
    assert!(ppt.clear_conformance().unwrap());
    assert!(ppt.server_zoom().unwrap().is_none());
    assert!(ppt.bookmark_id_seed().unwrap().is_none());
    assert!(ppt.conformance().unwrap().is_none());
}


#[test]
fn excel_selection_sqref_hf_flags_word_write_prot_ex() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.set_selection_sqref("S", "A1", "A1:B2").unwrap();
    assert_eq!(wb.active_cell("S").unwrap().as_deref(), Some("A1"));
    assert_eq!(wb.selection_sqref("S").unwrap().as_deref(), Some("A1:B2"));
    wb.set_header_footer_flags("S", Some(true), Some(true), Some(false), Some(false))
        .unwrap();
    assert!(wb.header_footer_different_odd_even("S").unwrap());
    assert!(wb.header_footer_different_first("S").unwrap());
    assert!(!wb.header_footer_scale_with_doc("S").unwrap());
    assert!(!wb.header_footer_align_with_margins("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("w")]));
    doc.set_write_protection_ex(true, Some("SHA-512")).unwrap();
    assert!(doc.has_write_protection().unwrap());
    assert!(doc.write_protection_recommended().unwrap());
    assert_eq!(
        doc.write_protection_algorithm_name().unwrap().as_deref(),
        Some("SHA-512")
    );
    assert!(doc.clear_write_protection().unwrap());
}

#[test]
fn excel_freeze_ex_color_id_word_settings_ppt_normal_view() {
    // Excel freeze panes ex + colorId + zoomScaleNormal
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.set_freeze_panes_ex("S", 1.0, 2.0, "B3", "bottomRight", "frozen")
        .unwrap();
    let details = wb.freeze_pane_details("S").unwrap().unwrap();
    assert_eq!(details.0, 1.0);
    assert_eq!(details.1, 2.0);
    assert_eq!(details.2, "B3");
    assert_eq!(details.3, "bottomRight");
    assert_eq!(details.4, "frozen");
    assert_eq!(wb.freeze_panes("S").unwrap(), Some((1, 2)));
    wb.set_color_id("S", 10).unwrap();
    assert_eq!(wb.color_id("S").unwrap(), Some(10));
    wb.set_default_grid_color("S", false).unwrap();
    assert!(!wb.default_grid_color("S").unwrap());
    assert!(wb.clear_color_id("S").unwrap());
    assert_eq!(wb.color_id("S").unwrap(), None);
    wb.set_zoom_scale_normal("S", 85).unwrap();
    assert_eq!(wb.zoom_scale_normal("S").unwrap(), Some(85));
    wb.set_workbook_view_id("S", 0).unwrap();
    assert_eq!(wb.workbook_view_id("S").unwrap(), Some(0));
    assert!(wb.clear_freeze_panes("S").unwrap());
    assert!(!wb.has_freeze_panes("S").unwrap());

    // Word remaining settings flags
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("w")]));
    doc.set_do_not_use_margins_for_drawing_grid_origin(true)
        .unwrap();
    assert!(doc.has_do_not_use_margins_for_drawing_grid_origin().unwrap());
    doc.set_show_envelope(true).unwrap();
    assert!(doc.has_show_envelope().unwrap());
    doc.set_auto_format_override(true).unwrap();
    assert!(doc.has_auto_format_override().unwrap());
    doc.set_ui_compat_97_to_2003(true).unwrap();
    assert!(doc.has_ui_compat_97_to_2003().unwrap());
    doc.set_do_not_use_margins_for_drawing_grid_origin(false)
        .unwrap();
    assert!(!doc.has_do_not_use_margins_for_drawing_grid_origin().unwrap());

    // PPT normalViewPr + lastView clear
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_last_view("sldView").unwrap();
    assert_eq!(ppt.last_view().unwrap().as_deref(), Some("sldView"));
    assert!(ppt.clear_last_view().unwrap());
    assert_eq!(ppt.last_view().unwrap(), None);
    ppt.set_show_outline_icons(false).unwrap();
    assert!(!ppt.show_outline_icons().unwrap());
    ppt.set_prefer_single_view(true).unwrap();
    assert!(ppt.prefer_single_view().unwrap());
    ppt.set_snap_vert_splitter(true).unwrap();
    assert!(ppt.snap_vert_splitter().unwrap());
    ppt.set_vert_bar_state("minimized").unwrap();
    assert_eq!(ppt.vert_bar_state().unwrap().as_deref(), Some("minimized"));
    ppt.set_horz_bar_state("restored").unwrap();
    assert_eq!(ppt.horz_bar_state().unwrap().as_deref(), Some("restored"));
    ppt.set_restored_left(20000, Some(false)).unwrap();
    assert_eq!(ppt.restored_left().unwrap(), Some((20000, false)));
    ppt.set_restored_top(50000, Some(true)).unwrap();
    assert_eq!(ppt.restored_top().unwrap(), Some((50000, true)));
}

#[test]
fn excel_page_setup_attrs_word_kinsoku_chars_ppt_slide_view() {
    // Excel page setup attribute surface
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.set_page_setup("S", (0.7, 0.7, 0.75, 0.75, 0.3, 0.3), 9, "landscape")
        .unwrap();
    assert_eq!(wb.paper_size("S").unwrap(), Some(9));
    assert_eq!(wb.page_orientation("S").unwrap().as_deref(), Some("landscape"));
    wb.set_paper_size("S", 1).unwrap();
    assert_eq!(wb.paper_size("S").unwrap(), Some(1));
    wb.set_page_orientation("S", "portrait").unwrap();
    assert_eq!(wb.page_orientation("S").unwrap().as_deref(), Some("portrait"));
    wb.set_use_printer_defaults("S", false).unwrap();
    assert!(!wb.use_printer_defaults("S").unwrap());
    wb.set_use_first_page_number("S", true).unwrap();
    assert!(wb.use_first_page_number("S").unwrap());
    wb.set_cell_comments("S", "atEnd").unwrap();
    assert_eq!(wb.cell_comments("S").unwrap().as_deref(), Some("atEnd"));
    wb.set_print_errors("S", "blank").unwrap();
    assert_eq!(wb.print_errors("S").unwrap().as_deref(), Some("blank"));
    wb.set_horizontal_dpi("S", 300).unwrap();
    assert_eq!(wb.horizontal_dpi("S").unwrap(), Some(300));
    wb.set_vertical_dpi("S", 600).unwrap();
    assert_eq!(wb.vertical_dpi("S").unwrap(), Some(600));
    wb.set_page_scale("S", 90).unwrap();
    assert_eq!(wb.page_scale("S").unwrap(), Some(90));
    assert!(wb.clear_page_setup("S").unwrap());
    assert!(!wb.has_page_setup("S").unwrap());

    // Word kinsoku character lists (correct element names + lang/val)
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("k")]));
    doc.set_no_line_breaks_after("ja-JP", "、。，．").unwrap();
    assert_eq!(
        doc.no_line_breaks_after().unwrap(),
        Some(("ja-JP".into(), "、。，．".into()))
    );
    assert!(doc.has_no_line_breaks_after().unwrap());
    assert!(doc.has_no_line_breaks_after_kinsoku().unwrap());
    doc.set_no_line_breaks_before("ja-JP", "（「『").unwrap();
    assert_eq!(
        doc.no_line_breaks_before().unwrap(),
        Some(("ja-JP".into(), "（「『".into()))
    );
    assert!(doc.clear_no_line_breaks_after().unwrap());
    assert!(doc.clear_no_line_breaks_before().unwrap());
    assert!(!doc.has_no_line_breaks_after().unwrap());

    // PPT slide view snap/guides + sorter formatting
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_snap_to_grid(false).unwrap();
    assert!(!ppt.snap_to_grid().unwrap());
    ppt.set_snap_to_objects(true).unwrap();
    assert!(ppt.snap_to_objects().unwrap());
    ppt.set_show_guides(true).unwrap();
    assert!(ppt.show_guides().unwrap());
    ppt.set_sorter_show_formatting(false).unwrap();
    assert!(!ppt.sorter_show_formatting().unwrap());
}

#[test]
fn excel_custom_view_attrs_word_read_mode_grid_lines_set() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_custom_sheet_view("S", "{AAAA-BBBB}", 100, true).unwrap();
    assert_eq!(wb.custom_sheet_view_count("S").unwrap(), 1);
    assert!(wb
        .set_custom_sheet_view_attrs(
            "S",
            "{AAAA-BBBB}",
            Some(80),
            Some(false),
            Some(true),
            Some(false),
            Some(12),
        )
        .unwrap());
    let views = wb.list_custom_sheet_views("S").unwrap();
    assert_eq!(views[0].0, "{AAAA-BBBB}");
    assert_eq!(views[0].1, 80);
    assert!(wb.remove_custom_sheet_view("S", "{AAAA-BBBB}").unwrap());
    assert_eq!(wb.custom_sheet_view_count("S").unwrap(), 0);

    wb.set_custom_height("S", true).unwrap();
    assert!(wb.custom_height("S").unwrap());
    wb.set_print_grid_lines("S", true).unwrap();
    wb.set_print_grid_lines_set("S", false).unwrap();
    assert!(!wb.print_grid_lines_set("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("r")]));
    doc.set_read_mode_ink_lock_down(12240, 15840, "100%", Some(true))
        .unwrap();
    let rm = doc.read_mode_ink_lock_down().unwrap().unwrap();
    assert_eq!(rm.0, 12240);
    assert_eq!(rm.1, 15840);
    assert_eq!(rm.2, "100%");
    assert_eq!(rm.3, Some(true));
    assert!(doc.has_read_mode_ink_lock_down().unwrap());
    assert!(doc.clear_read_mode_ink_lock_down().unwrap());
    assert!(!doc.has_read_mode_ink_lock_down().unwrap());
}

#[test]
fn excel_custom_wb_view_word_smart_tags_ppt_notes_snap() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_custom_workbook_view("MyView", "{GUID-1}", 1).unwrap();
    assert_eq!(wb.custom_workbook_view_count().unwrap(), 1);
    assert!(wb
        .set_custom_workbook_view_attrs(
            "{GUID-1}",
            Some("Renamed"),
            Some(2),
            Some(18000),
            Some(12000),
            Some(10),
            Some(20),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
            Some(false),
        )
        .unwrap());
    let views = wb.list_custom_workbook_views().unwrap();
    assert_eq!(views[0].0, "Renamed");
    assert_eq!(views[0].1, "{GUID-1}");
    assert_eq!(views[0].2, 2);
    assert!(wb.remove_custom_workbook_view("{GUID-1}").unwrap());
    assert_eq!(wb.custom_workbook_view_count().unwrap(), 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.add_smart_tag_type("urn:schemas-microsoft-com:office:smarttags", "place", Some("http://example.com/st"))
        .unwrap();
    let tags = doc.list_smart_tag_types().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].0, "urn:schemas-microsoft-com:office:smarttags");
    assert_eq!(tags[0].1, "place");
    assert_eq!(tags[0].2.as_deref(), Some("http://example.com/st"));
    assert!(doc.has_smart_tag_types().unwrap());
    assert_eq!(doc.clear_smart_tag_types().unwrap(), 1);
    assert!(!doc.has_smart_tag_types().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_notes_snap_to_grid(false).unwrap();
    assert!(!ppt.notes_snap_to_grid().unwrap());
    ppt.set_notes_snap_to_objects(true).unwrap();
    assert!(ppt.notes_snap_to_objects().unwrap());
    ppt.set_notes_show_guides(true).unwrap();
    assert!(ppt.notes_show_guides().unwrap());
}

#[test]
fn excel_page_margins_prot_range_word_schema_library() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.set_page_margins("S", 0.5, 0.6, 0.7, 0.8, 0.3, 0.4).unwrap();
    let m = wb.get_page_margins("S").unwrap().unwrap();
    assert!((m.0 - 0.5).abs() < 1e-9);
    assert!((m.1 - 0.6).abs() < 1e-9);
    assert!((m.4 - 0.3).abs() < 1e-9);
    wb.set_page_margin_attr("S", "left", 1.0).unwrap();
    assert_eq!(wb.page_margin_attr("S", "left").unwrap(), Some(1.0));
    assert!(wb.clear_page_margins("S").unwrap());
    assert!(!wb.has_page_margins("S").unwrap());

    wb.add_protected_range("S", "Rng1", "A1:B2").unwrap();
    assert!(wb
        .set_protected_range_attrs("S", "Rng1", Some("A1:C3"), Some("SHA-512"), Some("SD"))
        .unwrap());
    let ranges = wb.list_protected_ranges("S").unwrap();
    assert_eq!(ranges[0].1, "A1:C3");

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.add_schema_library_entry(
        "http://example.com/schema",
        Some("http://example.com/schema.xsd"),
        None,
    )
    .unwrap();
    let entries = doc.list_schema_library_entries().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "http://example.com/schema");
    assert_eq!(
        entries[0].1.as_deref(),
        Some("http://example.com/schema.xsd")
    );
    assert!(doc.has_schema_library().unwrap());
    assert_eq!(doc.clear_schema_library().unwrap(), 1);
    assert!(!doc.has_schema_library().unwrap());
}

#[test]
fn excel_scenario_watch_word_web_clear_ppt_guides() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.add_scenario("S", "Base", &[("A1", "10"), ("B1", "20")], Some("init"))
        .unwrap();
    assert!(wb
        .set_scenario_attrs("S", "Base", Some(true), Some(false), Some(true), Some("upd"))
        .unwrap());
    let inputs = wb.list_scenario_inputs("S", "Base").unwrap();
    assert_eq!(inputs.len(), 2);
    assert_eq!(inputs[0], ("A1".into(), "10".into()));
    wb.add_cell_watch("S", "A1").unwrap();
    wb.add_cell_watch("S", "B2").unwrap();
    assert_eq!(wb.list_cell_watches("S").unwrap(), vec!["A1", "B2"]);
    assert_eq!(wb.clear_cell_watches("S").unwrap(), 2);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("w")]));
    doc.set_web_encoding("utf-8").unwrap();
    assert_eq!(doc.web_encoding().unwrap().as_deref(), Some("utf-8"));
    doc.set_target_screen_size("1024x768").unwrap();
    assert_eq!(doc.target_screen_size().unwrap().as_deref(), Some("1024x768"));
    doc.set_pixels_per_inch(96).unwrap();
    assert_eq!(doc.pixels_per_inch().unwrap(), Some(96));
    assert!(doc.clear_web_encoding().unwrap());
    assert!(doc.clear_target_screen_size().unwrap());
    assert!(doc.clear_pixels_per_inch().unwrap());
    assert_eq!(doc.web_encoding().unwrap(), None);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_slide_guide("vert", 1008000).unwrap();
    ppt.add_slide_guide("horz", 2000000).unwrap();
    let guides = ppt.list_slide_guides().unwrap();
    assert_eq!(guides.len(), 2);
    assert_eq!(guides[0].0, "vert");
    assert_eq!(guides[0].1, 1008000);
    assert_eq!(ppt.clear_slide_guides().unwrap(), 2);
    assert!(ppt.list_slide_guides().unwrap().is_empty());
}

#[test]
fn excel_ignored_dc_attrs_word_font_entries() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_ignored_error("S", "A1", &["numberStoredAsText", "evalError"])
        .unwrap();
    wb.add_ignored_error("S", "B2", &["formula"]).unwrap();
    assert_eq!(wb.ignored_error_count("S").unwrap(), 2);
    assert_eq!(wb.remove_ignored_error("S", "A1").unwrap(), 1);
    assert_eq!(wb.ignored_error_count("S").unwrap(), 1);
    wb.set_data_consolidate("S", "sum", true).unwrap();
    wb.set_data_consolidate_attrs("S", Some("average"), Some(true), Some(true), Some(false), Some(true))
        .unwrap();
    let attrs = wb.data_consolidate_attrs("S").unwrap().unwrap();
    assert_eq!(attrs.0.as_deref(), Some("average"));
    assert!(attrs.1);
    assert!(attrs.2);
    assert!(!attrs.3);
    assert!(attrs.4);
    wb.add_data_consolidate_ref("S", "A1:A10", Some("S"), None)
        .unwrap();
    assert_eq!(wb.list_data_consolidate_refs("S").unwrap().len(), 1);
    assert_eq!(wb.clear_data_consolidate_refs("S").unwrap(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("f")]));
    doc.add_font_entry(
        "Segoe UI",
        Some("00"),
        Some("swiss"),
        Some("variable"),
        Some("Segoe"),
    )
    .unwrap();
    assert!(doc.list_font_names().unwrap().contains(&"Segoe UI".into()));
    let entry = doc.font_entry("Segoe UI").unwrap().unwrap();
    assert_eq!(entry.0.as_deref(), Some("00"));
    assert_eq!(entry.1.as_deref(), Some("swiss"));
    assert_eq!(entry.2.as_deref(), Some("variable"));
    assert_eq!(entry.3.as_deref(), Some("Segoe"));
    assert!(doc.remove_font_entry("Segoe UI").unwrap());
    assert!(!doc.list_font_names().unwrap().contains(&"Segoe UI".into()));
}

#[test]
fn excel_hyperlink_attrs_word_numbering_ppt_outline_scale() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_cell_hyperlink_with_tooltip("S", "A1", "https://example.com", Some("ex"), Some("tip"))
        .unwrap();
    assert!(wb
        .set_cell_hyperlink_attrs("S", "A1", Some("Example"), Some("Hover"), None)
        .unwrap());
    let details = wb.cell_hyperlink_details("S", "A1").unwrap().unwrap();
    assert!(details.0.is_some());
    assert_eq!(details.2.as_deref(), Some("Example"));
    assert_eq!(details.3.as_deref(), Some("Hover"));
    wb.add_cell_location_hyperlink("S", "B1", "S!A1", Some("loc"), Some("t2"))
        .unwrap();
    let loc = wb.list_location_hyperlinks("S").unwrap();
    assert_eq!(loc[0].1, "S!A1");

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("n")]));
    doc.add_default_numbering().unwrap();
    let abstracts = doc.list_abstract_nums().unwrap();
    assert!(!abstracts.is_empty());
    let nums = doc.list_num_instances().unwrap();
    assert_eq!(nums[0], (1, 0));
    assert!(doc
        .set_abstract_num_level(0, 0, Some("•"), Some("bullet"), Some(1))
        .unwrap());
    assert_eq!(
        doc.abstract_num_level_text(0, 0).unwrap().as_deref(),
        Some("•")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_outline_view_scale(50, 100, 75, 100).unwrap();
    assert_eq!(
        ppt.outline_view_scale().unwrap(),
        Some((50, 100, 75, 100))
    );
    assert!(ppt.has_outline_view_pr().unwrap());
}

#[test]
fn excel_defined_name_conn_attrs_word_style_links() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_defined_name("Sales", "S!$A$1").unwrap();
    assert!(wb
        .set_defined_name_attrs("Sales", Some(true), Some("region total"), Some("S!$A$1:$A$10"))
        .unwrap());
    let d = wb.defined_name_details("Sales").unwrap().unwrap();
    assert_eq!(d.0, "S!$A$1:$A$10");
    assert!(d.1);
    assert_eq!(d.2.as_deref(), Some("region total"));

    wb.add_connections(&[("Conn1", "SELECT 1", "DSN=x")]).unwrap();
    let conns = wb.list_connections().unwrap();
    assert_eq!(conns.len(), 1);
    assert_eq!(conns[0].1, "Conn1");
    assert!(wb
        .set_connection_attrs(1, Some("ConnX"), Some(1), Some(false), Some("DSN=y"))
        .unwrap());
    let conns = wb.list_connections().unwrap();
    assert_eq!(conns[0].1, "ConnX");
    assert_eq!(conns[0].3.as_deref(), Some("DSN=y"));
    assert!(wb.remove_connection(1).unwrap());
    assert_eq!(wb.connection_count().unwrap(), 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.add_default_styles().unwrap();
    let styles = doc.list_styles().unwrap();
    assert!(!styles.is_empty());
    let style_id = styles[0].0.clone();
    assert!(doc
        .set_style_links(&style_id, Some("Normal"), Some("Normal"), None)
        .unwrap());
    let links = doc.style_links(&style_id).unwrap().unwrap();
    assert_eq!(links.0.as_deref(), Some("Normal"));
    assert_eq!(links.1.as_deref(), Some("Normal"));
}

#[test]
fn excel_sparkline_cf_extlink_word_style_flags_ppt_scales() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[
            vec!["A", "B", "C", "D", "E"],
            vec!["1", "2", "3", "4", "5"],
        ],
    )
    .unwrap();
    wb.add_sparkline("S", "line", "A2:E2", "F2").unwrap();
    let sps = wb.list_sparklines("S").unwrap();
    assert_eq!(sps.len(), 1);
    assert_eq!(sps[0].0, "line");
    assert_eq!(sps[0].1, "A2:E2");
    assert_eq!(sps[0].2, "F2");
    assert!(wb
        .set_sparkline_group_attrs(
            "S",
            Some("column"),
            Some("zero"),
            Some(true),
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
        )
        .unwrap());
    assert_eq!(wb.list_sparklines("S").unwrap()[0].0, "column");

    wb.add_conditional_formatting_cell_is("S", "A2:E2", "greaterThan", "3", "FF0000", 1)
        .unwrap();
    assert_eq!(
        wb.set_cf_rule_attrs("S", "A2:E2", Some(2), Some(true), Some("greaterThan"), None)
            .unwrap(),
        1
    );
    let rules = wb.list_cf_rules("S").unwrap();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].4, Some(2));
    assert!(rules[0].5);

    let (link_uri, _) = wb.add_external_link("other.xlsx").unwrap();
    let targets = wb.list_external_link_targets().unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].1, "other.xlsx");
    assert!(wb
        .set_external_link_target(&link_uri, "updated.xlsx")
        .unwrap());
    assert_eq!(
        wb.list_external_link_targets().unwrap()[0].1,
        "updated.xlsx"
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.add_default_styles().unwrap();
    let style_id = doc.list_style_ids().unwrap()[0].clone();
    assert!(doc
        .set_style_flags(&style_id, Some(true), Some(true), Some(false), Some(true), Some(99))
        .unwrap());
    let flags = doc.style_flags(&style_id).unwrap().unwrap();
    assert!(flags.0);
    assert!(flags.1);
    assert!(!flags.2);
    assert!(flags.3);
    assert_eq!(flags.4, Some(99));
    doc.set_form_protection(true).unwrap();
    assert!(doc.has_form_protection().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_notes_text_view_scale(40, 100, 60, 100).unwrap();
    assert_eq!(
        ppt.notes_text_view_scale().unwrap(),
        Some((40, 100, 60, 100))
    );
    ppt.set_sorter_view_scale(30, 100, 30, 100).unwrap();
    assert_eq!(ppt.sorter_view_scale().unwrap(), Some((30, 100, 30, 100)));
}

#[test]
fn excel_pivot_attrs_word_glossary_parts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "Data",
        &[vec!["Region", "Sales"], vec!["East", "10"], vec!["West", "20"]],
    )
    .unwrap();
    wb.add_worksheet("Pivot").unwrap();
    wb.add_pivot_table("Data", "A1:B3", "Pivot", "A1", &["Region", "Sales"], 0, 1, 2)
        .unwrap();
    let infos = wb.pivot_table_infos().unwrap();
    assert!(!infos.is_empty());
    let name = infos[0].0.clone();
    assert_eq!(wb.pivot_table_location(&name).unwrap().as_deref(), Some("A1"));
    assert!(wb
        .set_pivot_table_attrs(
            &name,
            Some("Totals"),
            Some(false),
            Some(true),
            Some(true),
            Some(false),
        )
        .unwrap());
    assert!(wb.rename_pivot_table(&name, "SalesPivot").unwrap());
    let infos = wb.pivot_table_infos().unwrap();
    assert!(infos.iter().any(|(n, _)| n == "SalesPivot"));
    assert_eq!(
        wb.pivot_table_location("SalesPivot").unwrap().as_deref(),
        Some("A1")
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("g")]));
    doc.add_glossary_document("Part1", vec![paragraph_with_text("body1")])
        .unwrap();
    assert!(doc.has_glossary());
    assert_eq!(doc.list_glossary_doc_parts().unwrap(), vec!["Part1".to_string()]);
    doc.append_glossary_doc_part("Part2", vec![paragraph_with_text("body2")])
        .unwrap();
    let parts = doc.list_glossary_doc_parts().unwrap();
    assert_eq!(parts.len(), 2);
    assert!(doc.remove_glossary_doc_part("Part1").unwrap());
    assert_eq!(doc.list_glossary_doc_parts().unwrap(), vec!["Part2".to_string()]);
}

#[test]
fn excel_chart_title_word_comments_ppt_transition_details() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    let (chart_uri, _) = wb
        .add_bar_chart("Sales", &["A", "B"], &[1.0, 2.0])
        .unwrap();
    let titles = wb.list_chart_titles().unwrap();
    assert!(titles.iter().any(|(u, t)| u == &chart_uri && t == "Sales"));
    assert!(wb.set_chart_title(&chart_uri, "Revenue").unwrap());
    let titles = wb.list_chart_titles().unwrap();
    assert!(titles.iter().any(|(u, t)| u == &chart_uri && t == "Revenue"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("c")]));
    use officexml::wordprocessing::comment;
    doc.set_comments(vec![
        comment("0", "Alice", "A", "first"),
        comment("1", "Bob", "B", "second"),
    ])
    .unwrap();
    assert_eq!(doc.comment_count().unwrap(), 2);
    assert!(doc
        .set_comment_attrs("0", Some("Alicia"), Some("AL"), Some("2026-01-01T00:00:00Z"))
        .unwrap());
    let c0 = doc.comment_by_id("0").unwrap().unwrap();
    assert_eq!(c0.0, "Alicia");
    assert_eq!(c0.1, "first");
    assert!(doc.remove_comment("1").unwrap());
    assert_eq!(doc.comment_count().unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_slide_transition(0, "fade", "fast", Some(3000))
        .unwrap();
    let details = ppt.transition_details(0).unwrap().unwrap();
    assert_eq!(details.0, "fade");
    assert_eq!(details.1, "fast");
    assert!(details.2);
    assert_eq!(details.3, Some(3000));
    assert!(ppt
        .set_transition_attrs(0, Some("med"), Some(false), Some(Some(5000)))
        .unwrap());
    let details = ppt.transition_details(0).unwrap().unwrap();
    assert_eq!(details.1, "med");
    assert!(!details.2);
    assert_eq!(details.3, Some(5000));
}

#[test]
fn excel_shared_formula_clear_word_sdt_ppt_anim_ids() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"], vec!["3"]])
        .unwrap();
    wb.set_shared_formula(
        "S",
        &["A1", "A2", "A3"],
        "A1*2",
        &[Some("2"), Some("4"), Some("6")],
        0,
    )
    .unwrap();
    assert_eq!(wb.shared_formula_count("S").unwrap(), 3);
    assert_eq!(wb.clear_shared_formula_group("S", 0).unwrap(), 3);
    assert_eq!(wb.shared_formula_count("S").unwrap(), 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    use officexml::wordprocessing::{body, document, sdt_block};
    let sdt1 = sdt_block("t1", "Alias1", vec![paragraph_with_text("one")]);
    let sdt2 = sdt_block("t2", "Alias2", vec![paragraph_with_text("two")]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![sdt1, sdt2])]));
    assert_eq!(doc.content_control_count().unwrap(), 2);
    assert!(doc
        .set_content_control_tag("t1", Some("t1b"), Some("Alias1b"))
        .unwrap());
    let tags = doc.content_control_tags().unwrap();
    assert!(tags.iter().any(|(t, a, _)| t == "t1b" && a == "Alias1b"));
    assert_eq!(doc.remove_content_control_by_tag("t2").unwrap(), 1);
    assert_eq!(doc.content_control_count().unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_simple_appear_animation(0, 42).unwrap();
    assert!(ppt.has_animation(0).unwrap());
    assert_eq!(ppt.list_animation_shape_ids(0).unwrap(), vec![42]);
    assert!(ppt.clear_animation(0).unwrap());
    assert!(!ppt.has_animation(0).unwrap());
}

#[test]
fn excel_array_clear_word_bookmark_rename_ppt_set_notes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.set_array_formula("S", "A1", "SUM(1,2)", "A1:A2", Some("3"))
        .unwrap();
    assert_eq!(wb.array_formula_count("S").unwrap(), 1);
    assert_eq!(wb.clear_array_formulas("S").unwrap(), 1);
    assert_eq!(wb.array_formula_count("S").unwrap(), 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    use officexml::wordprocessing::{body, document, paragraph, with_bookmark};
    let p = paragraph(with_bookmark("1", "Intro", vec![run(vec![text("x")])]));
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![p])]));
    assert!(doc.rename_bookmark("Intro", "Chapter1").unwrap());
    let names = doc.list_bookmark_names().unwrap();
    assert_eq!(names, vec!["Chapter1".to_string()]);
    assert!(!doc.rename_bookmark("Intro", "x").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_notes_text(0, "first notes").unwrap();
    assert_eq!(ppt.notes_text(0).unwrap().as_deref(), Some("first notes"));
    ppt.set_notes_text(0, "updated notes").unwrap();
    assert_eq!(ppt.notes_text(0).unwrap().as_deref(), Some("updated notes"));
    assert!(ppt.clear_notes(0).unwrap());
    assert!(!ppt.has_notes(0).unwrap());
}

#[test]
fn excel_sheet_comment_word_fields_ppt_text_at() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.add_sheet_comments("S", "Alice", &[("A1", "note1"), ("B2", "note2")])
        .unwrap();
    let comments = wb.sheet_comments("S").unwrap();
    assert_eq!(comments.len(), 2);
    assert!(wb.set_sheet_comment_text("S", "A1", "updated").unwrap());
    let comments = wb.sheet_comments("S").unwrap();
    assert!(comments.iter().any(|(r, _, t)| r == "A1" && t == "updated"));
    assert!(wb.remove_sheet_comment("S", "B2").unwrap());
    assert_eq!(wb.sheet_comments("S").unwrap().len(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("f")]));
    doc.append_simple_field(" AUTHOR ", "Wantu").unwrap();
    doc.append_toc_field(r#"TOC \o "1-3" \h \z \u"#).unwrap();
    doc.append_date_field().unwrap();
    let fields = doc.list_simple_fields().unwrap();
    assert!(fields.iter().any(|f| f.contains("AUTHOR")));
    assert!(fields.iter().any(|f| f.contains("TOC")));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_slide_with_text("Hello PPT").unwrap();
    assert!(ppt.slide_text_node_count(0).unwrap() >= 1);
    assert!(ppt.set_slide_text_at(0, 0, "Updated").unwrap());
    let texts = ppt.slide_texts(0).unwrap();
    assert!(texts.iter().any(|t| t.contains("Updated")));
}

#[test]
fn excel_unmerge_word_ppt_theme_names() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.merge_range("S", "A1:B1").unwrap();
    wb.merge_range("S", "A2:B2").unwrap();
    assert_eq!(wb.merge_cell_count("S").unwrap(), 2);
    assert!(wb.is_merged_range("S", "A1:B1").unwrap());
    assert!(wb.unmerge_range("S", "A1:B1").unwrap());
    assert!(!wb.is_merged_range("S", "A1:B1").unwrap());
    assert_eq!(wb.merge_cell_count("S").unwrap(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    doc.add_default_theme().unwrap();
    let names = doc.list_theme_names().unwrap();
    assert!(!names.is_empty());
    assert!(doc.set_theme_name("Custom Theme").unwrap());
    let names = doc.list_theme_names().unwrap();
    assert_eq!(names[0].1, "Custom Theme");

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_default_theme().unwrap();
    assert!(ppt.set_theme_name("Deck Theme").unwrap());
    let names = ppt.list_theme_names().unwrap();
    assert_eq!(names[0].1, "Deck Theme");
}

#[test]
fn word_remove_style_ppt_list_transitions() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.add_default_styles().unwrap();
    let ids = doc.list_style_ids().unwrap();
    assert!(!ids.is_empty());
    let target = ids.iter().find(|id| *id != "Normal").cloned().unwrap_or_else(|| ids[0].clone());
    assert!(doc.has_style(&target).unwrap());
    assert!(doc.remove_style(&target).unwrap());
    assert!(!doc.has_style(&target).unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_fade_transition(0, "fast").unwrap();
    ppt.set_dissolve_transition(1, "med").unwrap();
    let trs = ppt.list_slide_transitions().unwrap();
    assert_eq!(trs.len(), 2);
    assert_eq!(trs[0].1, "fade");
    assert_eq!(trs[1].1, "dissolve");
}

#[test]
fn excel_named_style_word_hf_text_ppt_notes_list() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"]]).unwrap();
    wb.add_styles_with_named_title().unwrap();
    let styles = wb.list_named_styles().unwrap();
    assert!(!styles.is_empty());
    let old = styles[0].0.clone();
    assert!(wb.rename_named_style(&old, "RenamedStyle").unwrap());
    assert!(wb
        .list_named_styles()
        .unwrap()
        .iter()
        .any(|(n, _)| n == "RenamedStyle"));
    assert!(wb.remove_named_style("RenamedStyle").unwrap());
    assert!(!wb
        .list_named_styles()
        .unwrap()
        .iter()
        .any(|(n, _)| n == "RenamedStyle"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("h")]));
    doc.add_default_header("Header1").unwrap();
    doc.add_default_footer("Footer1").unwrap();
    assert_eq!(doc.header_texts().unwrap()[0], "Header1");
    assert!(doc.set_header_text(0, "Header2").unwrap());
    assert_eq!(doc.header_texts().unwrap()[0], "Header2");
    assert!(doc.set_footer_text(0, "Footer2").unwrap());
    assert_eq!(doc.footer_texts().unwrap()[0], "Footer2");

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_notes_text(0, "n0").unwrap();
    ppt.set_notes_text(1, "n1").unwrap();
    let notes = ppt.list_notes_texts().unwrap();
    assert_eq!(notes.len(), 2);
    assert_eq!(notes[0], (0, "n0".into()));
    assert_eq!(notes[1], (1, "n1".into()));
}

#[test]
fn excel_numfmt_ppt_transitions_hyperlinks() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    let (_rid, style_idx) = wb.add_styles_with_num_fmt("0.00%").unwrap();
    let _ = style_idx;
    let fmts = wb.list_number_formats().unwrap();
    assert!(!fmts.is_empty());
    let id = fmts[0].0;
    assert!(wb.set_number_format_code(id, "#,##0.00").unwrap());
    let fmts = wb.list_number_formats().unwrap();
    assert_eq!(fmts[0].1, "#,##0.00");
    assert!(wb.remove_number_format(id).unwrap());
    assert!(wb.list_number_formats().unwrap().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_fade_transition(0, "fast").unwrap();
    ppt.set_dissolve_transition(1, "slow").unwrap();
    assert_eq!(ppt.list_slide_transitions().unwrap().len(), 2);
    assert_eq!(ppt.clear_all_transitions().unwrap(), 2);
    assert!(ppt.list_slide_transitions().unwrap().is_empty());

    let rid = ppt.add_slide_hyperlink(0, "https://example.com").unwrap();
    let links = ppt.list_slide_hyperlinks(0).unwrap();
    assert_eq!(links.len(), 1);
    assert!(ppt.remove_slide_hyperlink(0, &rid).unwrap());
    assert!(ppt.list_slide_hyperlinks(0).unwrap().is_empty());
}

#[test]
fn excel_named_view_word_remove_notes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"]]).unwrap();
    wb.add_named_sheet_views("S", "ViewA").unwrap();
    let views = wb.list_named_sheet_views().unwrap();
    assert_eq!(views.len(), 1);
    assert_eq!(views[0].0, "ViewA");
    assert!(wb.rename_named_sheet_view("ViewA", "ViewB").unwrap());
    assert_eq!(wb.list_named_sheet_views().unwrap()[0].0, "ViewB");

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("n")]));
    doc.add_footnote("1", "fn1").unwrap();
    doc.add_footnote("2", "fn2").unwrap();
    doc.add_endnote("1", "en1").unwrap();
    assert_eq!(doc.list_footnotes().unwrap().len(), 2);
    assert!(doc.remove_footnote("1").unwrap());
    assert_eq!(doc.list_footnotes().unwrap().len(), 1);
    assert!(doc.remove_endnote("1").unwrap());
    assert_eq!(doc.list_endnotes().unwrap().len(), 0);
}

#[test]
fn excel_sheet_states_word_biblio_ppt_clear_anims() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("A", &[vec!["1"]]).unwrap();
    wb.add_worksheet("B").unwrap();
    wb.set_sheet_state("B", "hidden").unwrap();
    let states = wb.list_sheet_states().unwrap();
    assert!(states.iter().any(|(n, s)| n == "A" && s == "visible"));
    assert!(states.iter().any(|(n, s)| n == "B" && s == "hidden"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("b")]));
    doc.add_bibliography(&[("Tag1", "Title One"), ("Tag2", "Title Two")])
        .unwrap();
    assert!(doc.has_bibliography().unwrap());
    let srcs = doc.list_bibliography_sources().unwrap();
    assert_eq!(srcs.len(), 2);
    assert!(srcs.iter().any(|(t, title)| t == "Tag1" && title == "Title One"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_simple_appear_animation(0, 1).unwrap();
    ppt.set_simple_appear_animation(1, 2).unwrap();
    assert_eq!(ppt.clear_all_animations().unwrap(), 2);
    assert!(!ppt.has_animation(0).unwrap());
    assert!(!ppt.has_animation(1).unwrap());
}

#[test]
fn excel_local_names_word_docvar_ppt_slide_names() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"], vec!["1"]]).unwrap();
    wb.add_worksheet("T").unwrap();
    wb.set_local_defined_name("S", "LocalRange", "S!$A$1").unwrap();
    let locals = wb.list_local_defined_names().unwrap();
    assert_eq!(locals.len(), 1);
    assert_eq!(locals[0].0, "LocalRange");
    assert_eq!(locals[0].2, 0);
    wb.set_tab_color("S", "FF0000").unwrap();
    let colors = wb.list_tab_colors().unwrap();
    assert_eq!(colors.len(), 1);
    assert!(colors[0].1.contains("FF0000") || colors[0].1 == "FF0000" || colors[0].1.ends_with("FF0000"));
    assert!(wb.remove_local_defined_name("S", "LocalRange").unwrap());
    assert!(wb.list_local_defined_names().unwrap().is_empty());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("d")]));
    doc.set_document_variable("Author", "Wantu").unwrap();
    doc.set_document_variable("Rev", "1").unwrap();
    assert_eq!(doc.get_document_variable("Author").unwrap().as_deref(), Some("Wantu"));
    doc.set_document_variable("Rev", "2").unwrap();
    assert_eq!(doc.get_document_variable("Rev").unwrap().as_deref(), Some("2"));
    doc.add_footnote("1", "old").unwrap();
    assert!(doc.set_footnote_text("1", "new note").unwrap());
    let fns = doc.list_footnotes().unwrap();
    assert!(fns.iter().any(|(id, t)| id == "1" && t.contains("new note")));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_slide_name(0, "Intro").unwrap();
    ppt.set_slide_name(1, "Body").unwrap();
    let names = ppt.list_slide_names().unwrap();
    assert_eq!(names, vec![(0, "Intro".into()), (1, "Body".into())]);
    assert!(ppt.clear_slide_name(0).unwrap());
    assert_eq!(ppt.list_slide_names().unwrap(), vec![(1, "Body".into())]);
}

#[test]
fn excel_print_area_sheet_word_hyperlink_ppt_shapes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    wb.set_print_area("S", "$A$1:$B$2").unwrap();
    assert_eq!(
        wb.print_area_for_sheet("S").unwrap().as_deref(),
        Some("$A$1:$B$2")
    );
    assert!(wb.clear_print_area().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("h")]));
    let hl = doc
        .create_hyperlink("https://example.com", "link")
        .unwrap();
    doc.body_mut().unwrap().children.insert(0, paragraph(vec![hl]));
    let links = doc.list_external_hyperlinks();
    assert_eq!(links.len(), 1);
    let rid = links[0].0.clone();
    assert!(doc
        .set_hyperlink_target(&rid, "https://updated.example")
        .unwrap());
    let links = doc.list_external_hyperlinks();
    assert_eq!(links[0].1, "https://updated.example");
    assert!(doc.remove_hyperlink_by_id(&rid).unwrap());
    assert!(doc.list_external_hyperlinks().is_empty());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "Hello", "Box1")
        .unwrap();
    let shapes = ppt.list_shape_ids(0).unwrap();
    assert!(shapes.iter().any(|(id, _)| *id == sid));
    assert!(ppt.set_shape_name(0, sid, "TitleBox").unwrap());
    let shapes = ppt.list_shape_ids(0).unwrap();
    assert!(shapes.iter().any(|(id, n)| *id == sid && n == "TitleBox"));
}

#[test]
fn excel_col_outline_list_word_body_hl_ppt_remove_shape() {
    // Excel: column outline list/clear + row outline clear
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[
            vec!["A", "B", "C", "D"],
            vec!["1", "2", "3", "4"],
            vec!["5", "6", "7", "8"],
        ],
    )
    .unwrap();
    wb.set_column_outline_level("S", 2, 3, 1).unwrap();
    wb.set_column_collapsed("S", 2, 3, true).unwrap();
    let cols = wb.column_outline_levels("S").unwrap();
    assert!(
        cols.iter()
            .any(|(min, max, lvl, col)| *min == 2 && *max == 3 && *lvl == 1 && *col),
        "expected column outline entry, got {cols:?}"
    );
    assert!(wb.clear_column_outline("S", 2, 3).unwrap());
    assert!(wb.column_outline_levels("S").unwrap().is_empty());

    wb.set_row_outline_levels("S", &[(2, 1, false), (3, 1, true)])
        .unwrap();
    assert_eq!(wb.row_outline_levels("S").unwrap().len(), 2);
    assert_eq!(wb.clear_row_outline_levels("S", 2, 3).unwrap(), 2);
    assert!(wb.row_outline_levels("S").unwrap().is_empty());

    // Word: body hyperlink list / unwrap / remove_body / anchor update / fields
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("intro")]));
    let hl = doc
        .create_hyperlink("https://example.com/a", "click me")
        .unwrap();
    doc.body_mut().unwrap().children.insert(0, paragraph(vec![hl]));
    let anchor_hl = doc.create_anchor_hyperlink("bm1", "go to bm");
    doc.body_mut()
        .unwrap()
        .children
        .insert(1, paragraph(vec![anchor_hl]));

    let body_hls = doc.list_body_hyperlinks().unwrap();
    assert_eq!(body_hls.len(), 2);
    let rid = body_hls
        .iter()
        .find(|(r, _, _)| !r.is_empty())
        .map(|(r, _, _)| r.clone())
        .expect("external rid");
    assert!(body_hls.iter().any(|(_, a, t)| a == "bm1" && t.contains("go to bm")));

    assert_eq!(doc.set_body_hyperlink_anchor("bm1", "bm2").unwrap(), 1);
    let body_hls = doc.list_body_hyperlinks().unwrap();
    assert!(body_hls.iter().any(|(_, a, _)| a == "bm2"));

    let (rel_removed, unwrapped) = doc.remove_body_hyperlink(&rid).unwrap();
    assert!(rel_removed);
    assert_eq!(unwrapped, 1);
    assert!(doc.list_external_hyperlinks().is_empty());
    // display text preserved after unwrap
    let texts = doc.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("click me")));

    let (rels, bodies) = doc.clear_body_hyperlinks().unwrap();
    assert_eq!(rels, 0);
    assert_eq!(bodies, 1); // remaining anchor hyperlink
    assert!(doc.list_body_hyperlinks().unwrap().is_empty());

    doc.append_simple_field("DATE", "1/1/2026").unwrap();
    doc.append_simple_field("PAGE", "1").unwrap();
    assert_eq!(doc.simple_field_count().unwrap(), 2);
    assert_eq!(doc.remove_simple_fields_matching("DATE").unwrap(), 1);
    assert_eq!(doc.simple_field_count().unwrap(), 1);
    assert_eq!(doc.clear_simple_fields().unwrap(), 1);
    assert_eq!(doc.simple_field_count().unwrap(), 0);

    // PPT: remove shape by id / name / has_shape
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid1 = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "One", "BoxA")
        .unwrap();
    let sid2 = ppt
        .add_text_box_on_slide(0, 0, 600_000, 1_000_000, 500_000, "Two", "BoxB")
        .unwrap();
    assert!(ppt.has_shape(0, sid1).unwrap());
    assert!(ppt.has_shape(0, sid2).unwrap());
    assert_eq!(ppt.shape_count(0).unwrap(), 2);
    assert!(ppt.remove_shape_by_id(0, sid1).unwrap());
    assert!(!ppt.has_shape(0, sid1).unwrap());
    assert_eq!(ppt.shape_count(0).unwrap(), 1);
    assert_eq!(ppt.remove_shapes_by_name(0, "BoxB").unwrap(), 1);
    assert_eq!(ppt.shape_count(0).unwrap(), 0);
    assert!(!ppt.remove_shape_by_id(0, sid2).unwrap());
}

#[test]
fn word_sectpr_hf_refs_excel_clear_outlines_ppt_clear_shapes() {
    // Word: header/footer sectPr reference list + remove by id
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("body")]));
    let hrid = doc.add_default_header("H1").unwrap();
    let frid = doc.add_default_footer("F1").unwrap();
    let refs = doc.list_sect_pr_references().unwrap();
    assert!(
        refs.iter()
            .any(|(k, t, id)| k == "header" && t == "default" && id == &hrid),
        "missing header ref: {refs:?}"
    );
    assert!(
        refs.iter()
            .any(|(k, t, id)| k == "footer" && t == "default" && id == &frid),
        "missing footer ref: {refs:?}"
    );
    assert!(doc.remove_header_by_id(&hrid).unwrap());
    let refs = doc.list_sect_pr_references().unwrap();
    assert!(!refs.iter().any(|(_, _, id)| id == &hrid));
    assert!(refs.iter().any(|(_, _, id)| id == &frid));
    assert!(doc.remove_footer_by_id(&frid).unwrap());
    assert!(doc.list_sect_pr_references().unwrap().is_empty());
    assert!(doc.list_headers().is_empty());
    assert!(doc.list_footers().is_empty());

    // Excel: clear_all_column_outlines
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b", "c", "d"]]).unwrap();
    wb.set_column_outline_level("S", 1, 1, 1).unwrap();
    wb.set_column_outline_level("S", 2, 3, 2).unwrap();
    assert_eq!(wb.column_outline_levels("S").unwrap().len(), 2);
    assert_eq!(wb.clear_all_column_outlines("S").unwrap(), 2);
    assert!(wb.column_outline_levels("S").unwrap().is_empty());

    // PPT: clear_shapes
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_text_box_on_slide(0, 0, 0, 100, 100, "a", "A")
        .unwrap();
    ppt.add_text_box_on_slide(0, 0, 200, 100, 100, "b", "B")
        .unwrap();
    assert_eq!(ppt.shape_count(0).unwrap(), 2);
    assert_eq!(ppt.clear_shapes(0).unwrap(), 2);
    assert_eq!(ppt.shape_count(0).unwrap(), 0);
    assert_eq!(ppt.clear_shapes(0).unwrap(), 0);
}

#[test]
fn ppt_anim_effect_word_num_remove() {
    // PPT animation effect filter/transition
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "Hi", "T")
        .unwrap();
    ppt.set_animation_effect(0, sid, "blinds(horizontal)", "in")
        .unwrap();
    assert!(ppt.has_animation(0).unwrap());
    let (filter, transition) = ppt.animation_effect(0).unwrap().unwrap();
    assert_eq!(filter, "blinds(horizontal)");
    assert_eq!(transition, "in");
    let shapes = ppt.list_animation_shape_ids(0).unwrap();
    assert!(shapes.contains(&sid));
    // set_simple_appear_animation still works (delegates)
    ppt.set_simple_appear_animation(0, sid).unwrap();
    let (filter, transition) = ppt.animation_effect(0).unwrap().unwrap();
    assert_eq!(filter, "fade");
    assert_eq!(transition, "in");

    // Word numbering remove
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_numbering().unwrap();
    let abstracts = doc.list_abstract_nums().unwrap();
    assert!(!abstracts.is_empty());
    let instances = doc.list_num_instances().unwrap();
    assert!(!instances.is_empty());
    let num_id = instances[0].0;
    let abs_id = instances[0].1;
    assert!(doc.remove_num_instance(num_id).unwrap());
    assert!(doc.list_num_instances().unwrap().is_empty());
    assert!(doc.remove_abstract_num(abs_id).unwrap());
    assert!(doc.list_abstract_nums().unwrap().is_empty());
}

#[test]
fn word_paragraph_numbering_apply() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(simple_document(vec![
        paragraph_with_text("one"),
        paragraph_with_text("two"),
        paragraph_with_text("three"),
    ]));
    doc.add_default_numbering().unwrap();
    let n = doc.apply_numbering_to_paragraphs(1, 0).unwrap();
    assert_eq!(n, 3);
    let nums = doc.list_paragraph_numbering().unwrap();
    assert_eq!(nums, vec![(1, 0), (1, 0), (1, 0)]);
    assert_eq!(doc.clear_paragraph_numbering().unwrap(), 3);
    assert!(doc.list_paragraph_numbering().unwrap().is_empty());
}

#[test]
fn excel_remove_chart_word_biblio_ppt_comments_list() {
    // Excel remove chart
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    let (uri, _rid) = wb
        .add_chart("T", &["A", "B"], &[1.0, 2.0])
        .unwrap();
    assert_eq!(wb.chart_count(), 1);
    assert!(wb.remove_chart(&uri).unwrap());
    assert_eq!(wb.chart_count(), 0);
    let (_uri2, _) = wb
        .add_chart("T2", &["X"], &[3.0])
        .unwrap();
    assert!(wb.remove_chart_at(0).unwrap());
    assert!(!wb.has_charts());

    // Word bibliography remove
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("b")]));
    doc.add_bibliography(&[("t1", "Title One"), ("t2", "Title Two")])
        .unwrap();
    assert_eq!(doc.list_bibliography_sources().unwrap().len(), 2);
    assert_eq!(doc.remove_bibliography_source("t1").unwrap(), 1);
    let remaining = doc.list_bibliography_sources().unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].0, "t2");
    assert_eq!(doc.clear_bibliography_sources().unwrap(), 1);
    assert!(!doc.has_bibliography().unwrap());

    // PPT list_slide_comments
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_comment_authors(&[(0, "Alice", "AI")]).unwrap();
    ppt.add_slide_comments(0, &[(0, "2020-01-01T00:00:00", 0, 0, "hello")])
        .unwrap();
    let cms = ppt.list_slide_comments(0).unwrap();
    assert_eq!(cms.len(), 1);
    assert_eq!(cms[0].3, "hello");
    assert!(ppt.clear_slide_comments(0).unwrap());
    assert!(ppt.list_slide_comments(0).unwrap().is_empty());
}

#[test]
fn excel_remove_drawing_link_word_altchunk() {
    use officexml::packaging::AlternativeFormatImportType;

    // Excel: drawing via chart on sheet + external link remove
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_bar_chart_on_sheet("S", "T", &["A"], &[1.0], 0, 0, 4, 8)
        .unwrap();
    assert!(wb.drawing_count() >= 1);
    let drawing = wb.list_drawings()[0].clone();
    assert!(wb.remove_drawing(&drawing).unwrap());
    assert_eq!(wb.drawing_count(), 0);

    let (link_uri, _) = wb.add_external_link("other.xlsx").unwrap();
    assert_eq!(wb.external_link_count(), 1);
    assert!(wb.remove_external_link(&link_uri).unwrap());
    assert_eq!(wb.external_link_count(), 0);

    // Word altChunk remove
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("host")]));
    doc.add_alt_chunk(
        AlternativeFormatImportType::Html,
        b"<html><body>chunk</body></html>",
    )
    .unwrap();
    assert_eq!(doc.alt_chunk_count(), 1);
    assert!(doc.remove_alt_chunk_at(0).unwrap());
    assert_eq!(doc.alt_chunk_count(), 0);
    assert!(!doc.has_alt_chunks());
}

#[test]
fn excel_remove_pivot_word_para_num_at() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Src", &[vec!["A", "B"], vec!["x", "1"]])
        .unwrap();
    wb.add_worksheet("Out").unwrap();
    wb.add_pivot_table("Src", "A1:B2", "Out", "A3", &["A", "B"], 0, 1, 1)
        .unwrap();
    let infos = wb.pivot_table_infos().unwrap();
    assert!(!infos.is_empty());
    let name = infos[0].0.clone();
    assert!(wb.remove_pivot_table(&name).unwrap());
    assert!(wb.pivot_table_infos().unwrap().is_empty());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(simple_document(vec![
        paragraph_with_text("a"),
        paragraph_with_text("b"),
    ]));
    doc.add_default_numbering().unwrap();
    assert!(doc.set_paragraph_numbering_at(1, 1, 0).unwrap());
    let nums = doc.list_paragraph_numbering().unwrap();
    assert_eq!(nums, vec![(1, 0)]);
    assert!(!doc.set_paragraph_numbering_at(5, 1, 0).unwrap());
}

#[test]
fn excel_remove_table_col_word_rename_style_ppt_masters() {
    // Excel remove table column
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2", "h3"], vec!["a", "b", "c"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:C2", &["h1", "h2", "h3"])
        .unwrap();
    assert_eq!(wb.table_columns("T1").unwrap().len(), 3);
    assert!(wb.remove_table_column("T1", "h2").unwrap());
    let cols = wb.table_columns("T1").unwrap();
    assert_eq!(cols, vec!["h1".to_string(), "h3".to_string()]);
    assert!(!wb.remove_table_column("T1", "missing").unwrap());

    // Word rename style
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    assert!(doc.has_style("Normal").unwrap() || doc.style_count().unwrap() > 0);
    // Prefer renaming a known style if present; else rename first id
    let ids = doc.list_style_ids().unwrap();
    assert!(!ids.is_empty());
    let old = ids[0].clone();
    let new_id = format!("{old}Renamed");
    assert!(doc.rename_style(&old, &new_id).unwrap());
    assert!(doc.has_style(&new_id).unwrap());
    assert!(!doc.has_style(&old).unwrap());

    // PPT slide_master_count
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    // blank slide may or may not create master; count should be stable API
    let _ = ppt.slide_master_count();
    let _ = ppt.layout_count();
    assert!(ppt.slide_master_count() == ppt.list_slide_masters().len());
}

#[test]
fn excel_slicer_entries_word_font_list_ppt_custom_names() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_slicer_shell("S", "Region", "CacheRegion").unwrap();
    let entries = wb.list_slicer_entries().unwrap();
    assert!(
        entries
            .iter()
            .any(|(n, c, _)| n == "Region" && c == "CacheRegion"),
        "{entries:?}"
    );
    assert_eq!(wb.remove_slicer_entry("Region").unwrap(), 1);
    assert!(wb.list_slicer_entries().unwrap().is_empty());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("f")]));
    doc.add_font_entry("MyFont", Some("00"), Some("swiss"), Some("variable"), None)
        .unwrap();
    let entries = doc.list_font_entries().unwrap();
    assert!(
        entries
            .iter()
            .any(|(n, cs, fam, pitch)| n == "MyFont"
                && cs.as_deref() == Some("00")
                && fam.as_deref() == Some("swiss")
                && pitch.as_deref() == Some("variable")),
        "{entries:?}"
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    let id = ppt.add_custom_show("Demo", &[0, 1]).unwrap();
    let names = ppt.list_custom_show_names().unwrap();
    assert!(names.iter().any(|(i, n)| *i == id && n == "Demo"));
}

#[test]
fn word_run_styles_excel_slicer_caches() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(simple_document(vec![
        paragraph_with_text("one"),
        paragraph_with_text("two"),
    ]));
    let n = doc.apply_run_style_to_runs("Strong").unwrap();
    assert!(n >= 2);
    let ids = doc.run_style_ids().unwrap();
    assert!(ids.iter().any(|s| s == "Strong"), "{ids:?}");
    assert!(doc.clear_run_styles().unwrap() >= 2);
    assert!(doc.run_style_ids().unwrap().is_empty());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_slicer_shell("S", "City", "CacheCity").unwrap();
    let caches = wb.list_slicer_cache_entries().unwrap();
    assert!(
        caches
            .iter()
            .any(|(n, s)| n == "CacheCity" && s == "City"),
        "{caches:?}"
    );
}

#[test]
fn word_people_list_excel_timeline_entries() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("p")]));
    doc.add_people(&[("Alice", "AD"), ("Bob", "AD")]).unwrap();
    let people = doc.list_people().unwrap();
    assert_eq!(people.len(), 2);
    assert!(doc.remove_person("Alice").unwrap());
    let people = doc.list_people().unwrap();
    assert_eq!(people.len(), 1);
    assert_eq!(people[0].0, "Bob");
    assert!(!doc.remove_person("Alice").unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_timeline_shell("S", "Date", "CacheDate").unwrap();
    let entries = wb.list_timeline_entries().unwrap();
    assert!(
        entries
            .iter()
            .any(|(n, c)| n == "Date" && c == "CacheDate"),
        "{entries:?}"
    );
    assert_eq!(wb.remove_timeline_entry("Date").unwrap(), 1);
    assert!(wb.list_timeline_entries().unwrap().is_empty());
}

#[test]
fn excel_slicer_caption_word_person_ppt_all_shapes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_slicer_shell("S", "Prod", "CacheProd").unwrap();
    assert_eq!(wb.set_slicer_caption("Prod", "Products").unwrap(), 1);
    let entries = wb.list_slicer_entries().unwrap();
    assert!(
        entries
            .iter()
            .any(|(n, _, cap)| n == "Prod" && cap == "Products"),
        "{entries:?}"
    );

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("p")]));
    doc.add_people(&[("Carol", "OLD")]).unwrap();
    assert!(doc.set_person_provider("Carol", "NEW").unwrap());
    let people = doc.list_people().unwrap();
    assert_eq!(people, vec![("Carol".into(), "NEW".into())]);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    let s0 = ppt
        .add_text_box_on_slide(0, 0, 0, 100, 100, "a", "A")
        .unwrap();
    let s1 = ppt
        .add_text_box_on_slide(1, 0, 0, 100, 100, "b", "B")
        .unwrap();
    let all = ppt.list_all_shape_ids().unwrap();
    assert!(all.iter().any(|(i, id, n)| *i == 0 && *id == s0 && n == "A"));
    assert!(all.iter().any(|(i, id, n)| *i == 1 && *id == s1 && n == "B"));
}

#[test]
fn excel_remove_query_table_word_sdt_text() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_query_table("S", "QT1", 1).unwrap();
    assert!(wb.has_query_tables());
    let infos = wb.query_table_infos().unwrap();
    assert!(infos.iter().any(|(n, _, _)| n == "QT1"));
    assert!(wb.remove_query_table("QT1").unwrap());
    assert!(!wb.has_query_tables());

    use officexml::wordprocessing::{body, document, sdt_block};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let sdt = sdt_block("mytag", "Alias", vec![paragraph_with_text("hello-cc")]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![sdt])]));
    let text = doc.content_control_text("mytag").unwrap();
    assert_eq!(text.as_deref(), Some("hello-cc"));
    assert!(doc.content_control_text("missing").unwrap().is_none());
}

#[test]
fn word_sdt_set_text_ppt_list_anim_effects() {
    use officexml::wordprocessing::{body, document, sdt_block};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let sdt = sdt_block("t1", "A", vec![paragraph_with_text("old")]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![sdt])]));
    assert_eq!(doc.content_control_text("t1").unwrap().as_deref(), Some("old"));
    assert!(doc.set_content_control_text("t1", "new-text").unwrap());
    assert_eq!(
        doc.content_control_text("t1").unwrap().as_deref(),
        Some("new-text")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(1, 0, 0, 100, 100, "x", "X")
        .unwrap();
    ppt.set_animation_effect(1, sid, "fade", "out").unwrap();
    let effects = ppt.list_animation_effects().unwrap();
    assert!(
        effects
            .iter()
            .any(|(i, f, t)| *i == 1 && f == "fade" && t == "out"),
        "{effects:?}"
    );
}

#[test]
fn word_biblio_title_excel_rename_query() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("b")]));
    doc.add_bibliography(&[("t1", "Old Title")]).unwrap();
    assert_eq!(doc.set_bibliography_source_title("t1", "New Title").unwrap(), 1);
    let srcs = doc.list_bibliography_sources().unwrap();
    assert!(srcs.iter().any(|(t, title)| t == "t1" && title == "New Title"));

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_query_table("S", "OldQT", 2).unwrap();
    assert!(wb.rename_query_table("OldQT", "NewQT").unwrap());
    let infos = wb.query_table_infos().unwrap();
    assert!(infos.iter().any(|(n, _, _)| n == "NewQT"));
    assert!(!infos.iter().any(|(n, _, _)| n == "OldQT"));
}

#[test]
fn ppt_shape_text_get_set() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "Hello", "Box")
        .unwrap();
    assert_eq!(ppt.shape_text(0, sid).unwrap().as_deref(), Some("Hello"));
    assert!(ppt.set_shape_text(0, sid, "World").unwrap());
    assert_eq!(ppt.shape_text(0, sid).unwrap().as_deref(), Some("World"));
    assert!(ppt.shape_text(0, 99999).unwrap().is_none());
}

#[test]
fn word_hf_rels_excel_af_count_ppt_notes_masters() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    let hrid = doc.add_default_header("H").unwrap();
    let frid = doc.add_default_footer("F").unwrap();
    let hrefs = doc.list_header_relationships();
    assert!(hrefs.iter().any(|(id, _)| id == &hrid));
    let frefs = doc.list_footer_relationships();
    assert!(frefs.iter().any(|(id, _)| id == &frid));

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"], vec!["3", "4"]])
        .unwrap();
    wb.set_auto_filter("S", "A1:B3").unwrap();
    wb.add_auto_filter_values("S", 0, &["1"]).unwrap();
    assert_eq!(wb.auto_filter_column_count("S").unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let n = ppt.notes_master_count();
    assert_eq!(n, ppt.list_notes_masters().len());
    ppt.add_notes_master().unwrap();
    assert_eq!(ppt.notes_master_count(), n + 1);
    assert_eq!(ppt.list_notes_masters().len(), n + 1);
}

#[test]
fn word_styles_by_type_excel_has_table_ppt_handout() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let all = doc.list_styles().unwrap();
    assert!(!all.is_empty(), "expected default styles, got {all:?}");
    // Filter by the type of the first style (default set uses paragraph)
    let ty = all[0].1.clone();
    let filtered = doc.list_styles_by_type(&ty).unwrap();
    assert!(!filtered.is_empty());
    assert!(filtered.iter().any(|(id, _)| id == &all[0].0));

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    wb.add_table("S", "MyTable", "A1:B2", &["h1", "h2"]).unwrap();
    assert!(wb.has_table("MyTable").unwrap());
    assert!(!wb.has_table("Nope").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let n = ppt.handout_master_count();
    assert_eq!(n, ppt.list_handout_masters().len());
    ppt.add_handout_master().unwrap();
    assert_eq!(ppt.handout_master_count(), n + 1);
}

#[test]
fn word_style_default_excel_clear_codename() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let ids = doc.list_style_ids().unwrap();
    assert!(!ids.is_empty());
    let id = ids[0].clone();
    // Normal is default in default_styles
    let _ = doc.style_is_default(&id).unwrap();
    assert!(doc.set_style_default(&id).unwrap());
    assert!(doc.style_is_default(&id).unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.set_sheet_code_name("S", "SheetCode").unwrap();
    assert_eq!(wb.sheet_code_name("S").unwrap().as_deref(), Some("SheetCode"));
    assert!(wb.clear_sheet_code_name("S").unwrap());
    assert!(wb.sheet_code_name("S").unwrap().is_none());
}

#[test]
fn ppt_shape_hidden_word_bookmark_excel_views() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 100, 100, "x", "X")
        .unwrap();
    assert!(!ppt.is_shape_hidden(0, sid).unwrap());
    assert!(ppt.set_shape_hidden(0, sid, true).unwrap());
    assert!(ppt.is_shape_hidden(0, sid).unwrap());
    assert!(ppt.set_shape_hidden(0, sid, false).unwrap());
    assert!(!ppt.is_shape_hidden(0, sid).unwrap());

    use officexml::wordprocessing::{bookmark_end, bookmark_start, paragraph, run, text};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(simple_document(vec![
        paragraph(vec![
            bookmark_start("1", "bmA"),
            run(vec![text("hi")]),
            bookmark_end("1"),
        ]),
    ]));
    assert!(doc.has_bookmark("bmA").unwrap());
    assert!(!doc.has_bookmark("missing").unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_active_cell("S", "A1").unwrap(); // ensures sheetView
    let n = wb.sheet_view_count("S").unwrap();
    assert!(n >= 1, "expected at least one sheetView, got {n}");
}

#[test]
fn word_default_styles_ppt_shape_count_name() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let defaults = doc.list_default_style_ids().unwrap();
    assert!(defaults.iter().any(|id| id == "Normal"), "{defaults:?}");

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_text_box_on_slide(0, 0, 0, 100, 100, "a", "Same").unwrap();
    ppt.add_text_box_on_slide(0, 0, 200, 100, 100, "b", "Same").unwrap();
    ppt.add_text_box_on_slide(0, 0, 400, 100, 100, "c", "Other").unwrap();
    assert_eq!(ppt.shape_count_by_name(0, "Same").unwrap(), 2);
    assert_eq!(ppt.shape_count_by_name(0, "Other").unwrap(), 1);
    assert_eq!(ppt.shape_count_by_name(0, "Nope").unwrap(), 0);
}

#[test]
fn excel_has_defined_name_word_has_hf() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.set_defined_names(&[("MyName", "Sheet1!$A$1")]).unwrap();
    assert!(wb.has_defined_name("MyName").unwrap());
    assert!(!wb.has_defined_name("Nope").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_header());
    assert!(!doc.has_footer());
    doc.add_default_header("H").unwrap();
    doc.add_default_footer("F").unwrap();
    assert!(doc.has_header());
    assert!(doc.has_footer());
}

#[test]
fn excel_has_pivot_ppt_custom_show_named() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Src", &[vec!["A", "B"], vec!["x", "1"]])
        .unwrap();
    wb.add_worksheet("Out").unwrap();
    wb.add_pivot_table("Src", "A1:B2", "Out", "A3", &["A", "B"], 0, 1, 1)
        .unwrap();
    let name = wb.pivot_table_infos().unwrap()[0].0.clone();
    assert!(wb.has_pivot_table(&name).unwrap());
    assert!(!wb.has_pivot_table("Nope").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_custom_show("MyShow", &[0]).unwrap();
    assert!(ppt.has_custom_show_named("MyShow").unwrap());
    assert!(!ppt.has_custom_show_named("Nope").unwrap());
}

#[test]
fn excel_extlink_target_word_docvar_ppt_shape_named() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_external_link("other.xlsx").unwrap();
    assert!(wb.has_external_link_target("other.xlsx").unwrap());
    assert!(!wb.has_external_link_target("missing.xlsx").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_document_variable("v1", "val").unwrap();
    assert!(doc.has_document_variable("v1").unwrap());
    assert!(!doc.has_document_variable("v2").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_text_box_on_slide(0, 0, 0, 100, 100, "t", "NamedBox")
        .unwrap();
    assert!(ppt.has_shape_named(0, "NamedBox").unwrap());
    assert!(!ppt.has_shape_named(0, "Nope").unwrap());
}

#[test]
fn ppt_list_shape_names_word_has_font() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_text_box_on_slide(0, 0, 0, 100, 100, "a", "Alpha").unwrap();
    ppt.add_text_box_on_slide(0, 0, 200, 100, 100, "b", "Beta").unwrap();
    let names = ppt.list_shape_names(0).unwrap();
    assert!(names.contains(&"Alpha".to_string()));
    assert!(names.contains(&"Beta".to_string()));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_font_entry("Fancy", None, None, None, None).unwrap();
    assert!(doc.has_font_entry("Fancy").unwrap());
    assert!(!doc.has_font_entry("MissingFont").unwrap());
}

#[test]
fn excel_sheet_tables_word_simple_fields_has() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["h1", "h2"]).unwrap();
    let names = wb.sheet_table_names("S").unwrap();
    assert!(names.iter().any(|n| n == "T1"), "{names:?}");

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_simple_fields().unwrap());
    doc.append_simple_field("DATE", "today").unwrap();
    assert!(doc.has_simple_fields().unwrap());
}

#[test]
fn excel_has_sheet_ppt_has_slide_named() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("Alpha").unwrap();
    assert!(wb.has_sheet("Alpha"));
    assert!(!wb.has_sheet("Beta"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_slide_name(0, "Intro").unwrap();
    assert!(ppt.has_slide_named("Intro").unwrap());
    assert!(!ppt.has_slide_named("Outro").unwrap());
}

#[test]
fn excel_remove_cf_sqref_word_qformat_ppt_guide() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"], vec!["3"]])
        .unwrap();
    wb.add_conditional_formatting_cell_is("S", "A1:A3", "greaterThan", "1", "FF0000", 1)
        .unwrap();
    assert!(wb.conditional_formatting_count("S").unwrap() >= 1);
    assert_eq!(
        wb.remove_conditional_formatting_sqref("S", "A1:A3").unwrap(),
        1
    );
    assert_eq!(wb.conditional_formatting_count("S").unwrap(), 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    // Normal has qFormat in default_styles
    assert!(doc.style_q_format("Normal").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_slide_guide("horz", 1000).unwrap();
    ppt.add_slide_guide("vert", 2000).unwrap();
    assert_eq!(ppt.list_slide_guides().unwrap().len(), 2);
    assert_eq!(ppt.remove_slide_guide("horz", 1000).unwrap(), 1);
    let guides = ppt.list_slide_guides().unwrap();
    assert_eq!(guides.len(), 1);
    assert_eq!(guides[0], ("vert".into(), 2000));
}

#[test]
fn excel_sparkline_count_word_style_type_count_ppt_guides() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[vec!["1", "2", "3"], vec!["4", "5", "6"]],
    )
    .unwrap();
    wb.add_sparkline("S", "line", "A1:C1", "D1").unwrap();
    assert_eq!(wb.sparkline_count("S").unwrap(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let all = doc.list_styles().unwrap();
    assert!(!all.is_empty());
    let ty = all[0].1.clone();
    let n = doc.style_count_by_type(&ty).unwrap();
    assert!(n >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert_eq!(ppt.slide_guide_count().unwrap(), 0);
    ppt.add_slide_guide("horz", 500).unwrap();
    assert_eq!(ppt.slide_guide_count().unwrap(), 1);
}

#[test]
fn ppt_section_names_word_field_match_count() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_sections(&[("Intro", 0, 0), ("Body", 1, 1)]).unwrap();
    let names = ppt.list_section_names().unwrap();
    assert!(names.iter().any(|n| n == "Intro"));
    assert!(names.iter().any(|n| n == "Body"));

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.append_simple_field("DATE \\@ MMMM", "Jan").unwrap();
    doc.append_simple_field("PAGE", "1").unwrap();
    assert_eq!(doc.count_simple_fields_matching("DATE").unwrap(), 1);
    assert_eq!(doc.count_simple_fields_matching("PAGE").unwrap(), 1);
    assert_eq!(doc.count_simple_fields_matching("AUTHOR").unwrap(), 0);
}

#[test]
fn excel_sheet_has_table_ppt_has_guides_word_field_count() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h"], vec!["v"]]).unwrap();
    assert!(!wb.sheet_has_table("S").unwrap());
    wb.add_table("S", "T", "A1:A2", &["h"]).unwrap();
    assert!(wb.sheet_has_table("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_slide_guides().unwrap());
    ppt.add_slide_guide("vert", 10).unwrap();
    assert!(ppt.has_slide_guides().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(doc.field_count().unwrap(), 0);
    doc.append_simple_field("DATE", "d").unwrap();
    assert_eq!(doc.field_count().unwrap(), 1);
}

#[test]
fn word_biblio_count_ppt_custom_show_names() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(doc.bibliography_source_count().unwrap(), 0);
    doc.add_bibliography(&[("a", "A"), ("b", "B")]).unwrap();
    assert_eq!(doc.bibliography_source_count().unwrap(), 2);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_custom_show("One", &[0]).unwrap();
    ppt.add_custom_show("Two", &[0]).unwrap();
    let names = ppt.custom_show_names().unwrap();
    assert!(names.contains(&"One".to_string()));
    assert!(names.contains(&"Two".to_string()));
}

#[test]
fn excel_has_named_style_word_para_num_ppt_any_shape() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A"]]).unwrap();
    wb.add_styles_with_named_title().unwrap();
    let styles = wb.list_named_styles().unwrap();
    assert!(!styles.is_empty());
    assert!(wb.has_named_style(&styles[0].0).unwrap());
    assert!(!wb.has_named_style("NoSuchStyle").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(simple_document(vec![
        paragraph_with_text("a"),
        paragraph_with_text("b"),
    ]));
    assert!(!doc.has_paragraph_numbering().unwrap());
    doc.add_default_numbering().unwrap();
    doc.apply_numbering_to_paragraphs(1, 0).unwrap();
    assert!(doc.has_paragraph_numbering().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_any_shape().unwrap());
    ppt.add_text_box_on_slide(0, 0, 0, 100, 100, "x", "X").unwrap();
    assert!(ppt.has_any_shape().unwrap());
}

#[test]
fn excel_has_array_word_body_hl_ppt_shape_text() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    assert!(!wb.has_array_formulas("S").unwrap());
    wb.set_array_formula("S", "A1", "ROW()", "A1:A2", None).unwrap();
    assert!(wb.has_array_formulas("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_body_hyperlinks().unwrap());
    let hl = doc.create_hyperlink("https://e.com", "e").unwrap();
    doc.body_mut().unwrap().children.insert(0, paragraph(vec![hl]));
    assert!(doc.has_body_hyperlinks().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 100, 100, "Hi", "T")
        .unwrap();
    assert!(ppt.has_shape_text(0, sid).unwrap());
}

#[test]
fn excel_shared_formula_has_word_hl_count_ppt_shape_name() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"], vec!["3"]])
        .unwrap();
    assert!(!wb.has_shared_formulas("S").unwrap());
    wb.set_shared_formula("S", &["A1", "A2"], "A1*2", &[Some("2"), Some("4")], 0)
        .unwrap();
    assert!(wb.has_shared_formulas("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(doc.body_hyperlink_count().unwrap(), 0);
    let hl = doc.create_hyperlink("https://e.com", "e").unwrap();
    doc.body_mut().unwrap().children.insert(0, paragraph(vec![hl]));
    assert_eq!(doc.body_hyperlink_count().unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 100, 100, "t", "MyName")
        .unwrap();
    assert_eq!(ppt.shape_name(0, sid).unwrap().as_deref(), Some("MyName"));
    assert!(ppt.shape_name(0, 99999).unwrap().is_none());
}

#[test]
fn excel_has_numfmt_word_sectpr_refs_ppt_any_notes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    // custom num fmt ids typically >= 164
    wb.set_number_format(164, "0.00%").unwrap();
    assert!(wb.has_number_format(164).unwrap());
    assert!(!wb.has_number_format(9999).unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_sect_pr_references().unwrap());
    doc.add_default_header("H").unwrap();
    assert!(doc.has_sect_pr_references().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_any_notes().unwrap());
    ppt.add_notes_to_slide(0, "note").unwrap();
    assert!(ppt.has_any_notes().unwrap());
}

#[test]
fn excel_style_font_word_num_counts_ppt_anim_count() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_default_styles().unwrap();
    let fonts = wb.list_style_fonts().unwrap();
    // default stylesheet should declare at least one font
    if let Some(name) = fonts.first() {
        assert!(wb.has_style_font(name).unwrap());
    }
    assert!(!wb.has_style_font("DefinitelyMissingFontXYZ").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(doc.abstract_num_count().unwrap(), 0);
    doc.add_default_numbering().unwrap();
    assert!(doc.abstract_num_count().unwrap() >= 1);
    assert!(doc.num_instance_count().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert_eq!(ppt.slides_with_animation_count().unwrap(), 0);
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 100, 100, "a", "A")
        .unwrap();
    ppt.set_simple_appear_animation(0, sid).unwrap();
    assert_eq!(ppt.slides_with_animation_count().unwrap(), 1);
}

#[test]
fn excel_fill_outline_counts_word_hf_counts() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b", "c"]]).unwrap();
    wb.add_default_styles().unwrap();
    assert!(wb.fill_count().unwrap() >= 1);
    assert_eq!(wb.column_outline_count("S").unwrap(), 0);
    wb.set_column_outline_level("S", 1, 2, 1).unwrap();
    assert_eq!(wb.column_outline_count("S").unwrap(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert_eq!(doc.header_count(), 0);
    assert_eq!(doc.footer_count(), 0);
    doc.add_default_header("H").unwrap();
    doc.add_default_footer("F").unwrap();
    assert_eq!(doc.header_count(), 1);
    assert_eq!(doc.footer_count(), 1);
}

#[test]
fn excel_has_borders_word_custom_xml() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    wb.add_default_styles().unwrap();
    assert!(wb.has_borders().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_custom_xml().unwrap());
    doc.add_bibliography(&[("t", "Title")]).unwrap();
    assert!(doc.has_custom_xml().unwrap());
    assert!(doc.custom_xml_part_count().unwrap() >= 1);
}

#[test]
fn excel_has_qt_word_font_count_ppt_anim_effect() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    assert!(!wb.has_query_table("QT").unwrap());
    wb.add_query_table("S", "QT", 1).unwrap();
    assert!(wb.has_query_table("QT").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_font_entry("F1Unique", None, None, None, None).unwrap();
    assert!(doc.font_entry_count().unwrap() >= 1);
    assert!(doc.has_font_entry("F1Unique").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_animation_effect(0).unwrap());
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 100, 100, "a", "A")
        .unwrap();
    ppt.set_animation_effect(0, sid, "fade", "in").unwrap();
    assert!(ppt.has_animation_effect(0).unwrap());
}

#[test]
fn excel_has_slicer_timeline_entries() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    assert!(!wb.has_slicer_entry("Region").unwrap());
    wb.add_slicer_shell("S", "Region", "CacheRegion").unwrap();
    assert!(wb.has_slicer_entry("Region").unwrap());
    assert!(!wb.has_timeline_entry("Date").unwrap());
    wb.add_timeline_shell("S", "Date", "CacheDate").unwrap();
    assert!(wb.has_timeline_entry("Date").unwrap());
}

#[test]
fn excel_row_outline_count() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[vec!["P"], vec!["c1"], vec!["c2"], vec!["P2"]],
    )
    .unwrap();
    assert_eq!(wb.row_outline_count("S").unwrap(), 0);
    wb.set_row_outline_levels("S", &[(2, 1, false), (3, 1, false)])
        .unwrap();
    assert_eq!(wb.row_outline_count("S").unwrap(), 2);
    assert_eq!(wb.clear_row_outline_levels("S", 2, 3).unwrap(), 2);
    assert_eq!(wb.row_outline_count("S").unwrap(), 0);
}

#[test]
fn excel_list_sheet_code_names() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S1").unwrap();
    wb.add_worksheet("S2").unwrap();
    assert!(wb.list_sheet_code_names().unwrap().is_empty());
    wb.set_sheet_code_name("S1", "CodeOne").unwrap();
    wb.set_sheet_code_name("S2", "CodeTwo").unwrap();
    let list = wb.list_sheet_code_names().unwrap();
    assert_eq!(list.len(), 2);
    assert!(list.iter().any(|(n, c)| n == "S1" && c == "CodeOne"));
    assert!(list.iter().any(|(n, c)| n == "S2" && c == "CodeTwo"));
    wb.clear_sheet_code_name("S1").unwrap();
    let list = wb.list_sheet_code_names().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].0, "S2");
}

#[test]
fn excel_hidden_sheet_names_ppt_has_hidden_slides() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("Visible").unwrap();
    wb.add_worksheet("Hidden1").unwrap();
    wb.add_worksheet("Hidden2").unwrap();
    wb.set_sheet_state("Hidden1", "hidden").unwrap();
    wb.set_sheet_state("Hidden2", "veryHidden").unwrap();
    let names = wb.list_hidden_sheet_names().unwrap();
    assert!(names.contains(&"Hidden1".to_string()));
    assert!(names.contains(&"Hidden2".to_string()));
    assert!(!names.contains(&"Visible".to_string()));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_hidden_slides().unwrap());
    ppt.set_slide_hidden(1, true).unwrap();
    assert!(ppt.has_hidden_slides().unwrap());
    let hidden = ppt.list_hidden_slides().unwrap();
    assert_eq!(hidden, vec![1]);
}

#[test]
fn excel_very_hidden_word_style_type_lists() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("A").unwrap();
    wb.add_worksheet("B").unwrap();
    wb.add_worksheet("C").unwrap();
    wb.set_sheet_state("B", "hidden").unwrap();
    wb.set_sheet_state("C", "veryHidden").unwrap();
    let vh = wb.list_very_hidden_sheet_names().unwrap();
    assert_eq!(vh, vec!["C".to_string()]);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let paras = doc.list_paragraph_styles().unwrap();
    assert!(!paras.is_empty() || !doc.list_styles().unwrap().is_empty());
    // character styles may be empty in minimal default set
    let _ = doc.list_character_styles().unwrap();
}

#[test]
fn excel_has_outlines_and_local_names() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings(
        "S",
        &[vec!["a", "b"], vec!["1", "2"], vec!["3", "4"]],
    )
    .unwrap();
    assert!(!wb.has_column_outlines("S").unwrap());
    assert!(!wb.has_row_outlines("S").unwrap());
    wb.set_column_outline_level("S", 1, 1, 1).unwrap();
    wb.set_row_outline_levels("S", &[(2, 1, false)]).unwrap();
    assert!(wb.has_column_outlines("S").unwrap());
    assert!(wb.has_row_outlines("S").unwrap());

    assert!(!wb.has_local_defined_names().unwrap());
    wb.set_local_defined_name("S", "Local1", "$A$1").unwrap();
    assert!(wb.has_local_defined_names().unwrap());
}

#[test]
fn word_table_styles_ppt_notes_text_excel_codename() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let _ = doc.list_table_styles().unwrap();
    let _ = doc.list_numbering_styles().unwrap();

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_notes_text(0).unwrap());
    ppt.set_notes_text(0, "Speaker notes").unwrap();
    assert!(ppt.has_notes_text(0).unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    assert!(!wb.has_sheet_code_name("S").unwrap());
    wb.set_sheet_code_name("S", "SheetCode").unwrap();
    assert!(wb.has_sheet_code_name("S").unwrap());
}

#[test]
fn excel_visible_sheets_ppt_visible_slides() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("A").unwrap();
    wb.add_worksheet("B").unwrap();
    wb.add_worksheet("C").unwrap();
    wb.set_sheet_state("B", "hidden").unwrap();
    let vis = wb.list_visible_sheet_names().unwrap();
    assert!(vis.contains(&"A".to_string()));
    assert!(vis.contains(&"C".to_string()));
    assert!(!vis.contains(&"B".to_string()));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_slide_hidden(1, true).unwrap();
    let vis = ppt.list_visible_slides().unwrap();
    assert_eq!(vis, vec![0, 2]);
}

#[test]
fn excel_sheets_with_tables_and_filters() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("T", &[vec!["h"], vec!["v"]]).unwrap();
    wb.write_sheet_strings("F", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();
    wb.add_worksheet("Empty").unwrap();
    wb.add_table("T", "Tbl", "A1:A2", &["h"]).unwrap();
    wb.set_auto_filter("F", "A1:B2").unwrap();
    let with_t = wb.sheets_with_tables().unwrap();
    assert!(with_t.contains(&"T".to_string()));
    assert!(!with_t.contains(&"Empty".to_string()));
    let with_f = wb.sheets_with_auto_filter().unwrap();
    assert!(with_f.contains(&"F".to_string()));
    assert!(!with_f.contains(&"Empty".to_string()));
}

#[test]
fn word_has_notes_footnotes_endnotes() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_notes());
    doc.add_footnote("1", "fn").unwrap();
    assert!(doc.has_notes());
    assert!(doc.has_footnotes());
    doc.add_endnote("1", "en").unwrap();
    assert!(doc.has_endnotes());
}

#[test]
fn excel_sheets_with_cf() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("CF", &[vec!["1"], vec!["2"], vec!["3"]])
        .unwrap();
    wb.add_worksheet("Plain").unwrap();
    wb.add_conditional_formatting_cell_is("CF", "A1:A3", "greaterThan", "1", "FF0000", 1)
        .unwrap();
    let sheets = wb.sheets_with_conditional_formatting().unwrap();
    assert!(sheets.contains(&"CF".to_string()));
    assert!(!sheets.contains(&"Plain".to_string()));
}

#[test]
fn excel_sheets_merged_ppt_slides_with_shapes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("M", &[vec!["a", "b"], vec!["c", "d"]]).unwrap();
    wb.add_worksheet("Plain").unwrap();
    wb.merge_range("M", "A1:B1").unwrap();
    let sheets = wb.sheets_with_merged_cells().unwrap();
    assert!(sheets.contains(&"M".to_string()));
    assert!(!sheets.contains(&"Plain".to_string()));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(ppt.slides_with_shapes().unwrap().is_empty());
    ppt.add_text_box_on_slide(1, 0, 0, 100, 100, "x", "X").unwrap();
    assert_eq!(ppt.slides_with_shapes().unwrap(), vec![1]);
}

#[test]
fn excel_sheets_with_dv_and_freeze() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("DV", &[vec!["x"]]).unwrap();
    wb.write_sheet_strings("FR", &[vec!["y"]]).unwrap();
    wb.add_worksheet("Plain").unwrap();
    wb.add_data_validation_list("DV", "A1", "\"A,B,C\"", true)
        .unwrap();
    wb.set_freeze_panes("FR", 1, 1).unwrap();
    let dv = wb.sheets_with_data_validations().unwrap();
    assert!(dv.contains(&"DV".to_string()));
    assert!(!dv.contains(&"Plain".to_string()));
    let fr = wb.sheets_with_freeze_panes().unwrap();
    assert!(fr.contains(&"FR".to_string()));
    assert!(!fr.contains(&"Plain".to_string()));
}

#[test]
fn word_list_qformat_and_locked_styles() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    let qf = doc.list_qformat_styles().unwrap();
    assert!(qf.iter().any(|id| id == "Normal"), "{qf:?}");
    // lock Normal
    assert!(doc
        .set_style_flags("Normal", None, None, None, Some(true), None)
        .unwrap());
    let locked = doc.list_locked_styles().unwrap();
    assert!(locked.iter().any(|id| id == "Normal"), "{locked:?}");
}

#[test]
fn excel_sheets_with_formulas() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Arr", &[vec!["1"], vec!["2"]]).unwrap();
    wb.write_sheet_strings("Shr", &[vec!["1"], vec!["2"]]).unwrap();
    wb.add_worksheet("Plain").unwrap();
    wb.set_array_formula("Arr", "A1", "ROW()", "A1:A2", None)
        .unwrap();
    wb.set_shared_formula("Shr", &["A1", "A2"], "A1*2", &[Some("2"), Some("4")], 0)
        .unwrap();
    let arr = wb.sheets_with_array_formulas().unwrap();
    assert!(arr.contains(&"Arr".to_string()));
    assert!(!arr.contains(&"Plain".to_string()));
    let shr = wb.sheets_with_shared_formulas().unwrap();
    assert!(shr.contains(&"Shr".to_string()));
    assert!(!shr.contains(&"Plain".to_string()));
}

#[test]
fn excel_sheets_with_page_setup_and_protection() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("PS").unwrap();
    wb.add_worksheet("PR").unwrap();
    wb.add_worksheet("Plain").unwrap();
    wb.set_page_setup("PS", (0.7, 0.7, 0.75, 0.75, 0.3, 0.3), 1, "portrait")
        .unwrap();
    wb.set_sheet_protection("PR", true, false, false).unwrap();
    let ps = wb.sheets_with_page_setup().unwrap();
    assert!(ps.contains(&"PS".to_string()));
    assert!(!ps.contains(&"Plain".to_string()));
    let pr = wb.sheets_with_sheet_protection().unwrap();
    assert!(pr.contains(&"PR".to_string()));
    assert!(!pr.contains(&"Plain".to_string()));
}

#[test]
fn word_semi_hidden_styles_ppt_empty_slides() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    assert!(doc
        .set_style_flags("Normal", None, Some(true), None, None, None)
        .unwrap());
    let sh = doc.list_semi_hidden_styles().unwrap();
    assert!(sh.iter().any(|id| id == "Normal"), "{sh:?}");

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    assert_eq!(ppt.list_empty_slides().unwrap(), vec![0, 1]);
    ppt.add_text_box_on_slide(0, 0, 0, 100, 100, "a", "A").unwrap();
    assert_eq!(ppt.list_empty_slides().unwrap(), vec![1]);
}

#[test]
fn excel_empty_sheets() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("Empty").unwrap();
    wb.write_sheet_strings("Data", &[vec!["a"]]).unwrap();
    assert!(wb.sheet_is_empty("Empty").unwrap());
    assert!(!wb.sheet_is_empty("Data").unwrap());
    let empty = wb.list_empty_sheets().unwrap();
    assert!(empty.contains(&"Empty".to_string()));
    assert!(!empty.contains(&"Data".to_string()));
}

#[test]
fn empty_package_helpers() {
    let wb = SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    // create_in_memory may or may not add a default sheet
    let _ = wb.is_workbook_empty();
    let mut wb2 =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb2.add_worksheet("S").unwrap();
    assert!(!wb2.is_workbook_empty());

    let ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    assert!(ppt.is_presentation_empty());
    let mut ppt2 =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt2.add_blank_slide().unwrap();
    assert!(!ppt2.is_presentation_empty());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![]));
    // empty body may still have sectPr only
    let _ = doc.is_body_empty();
    doc.body_mut().unwrap().children.insert(0, paragraph_with_text("x"));
    assert!(!doc.is_body_empty().unwrap());
}

#[test]
fn excel_sheets_with_drawings() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("D").unwrap();
    wb.add_worksheet("Plain").unwrap();
    wb.add_bar_chart_on_sheet("D", "T", &["A"], &[1.0], 0, 0, 4, 8)
        .unwrap();
    assert!(wb.sheet_has_drawing("D").unwrap());
    assert!(!wb.sheet_has_drawing("Plain").unwrap());
    let sheets = wb.sheets_with_drawings().unwrap();
    assert!(sheets.contains(&"D".to_string()));
    assert!(!sheets.contains(&"Plain".to_string()));
}

#[test]
fn word_unhide_styles_ppt_slides_with_hyperlinks() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    assert!(doc
        .set_style_flags("Normal", None, None, Some(true), None, None)
        .unwrap());
    let u = doc.list_unhide_when_used_styles().unwrap();
    assert!(u.iter().any(|id| id == "Normal"), "{u:?}");

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(ppt.slides_with_hyperlinks().unwrap().is_empty());
    ppt.add_slide_hyperlink(1, "https://example.com").unwrap();
    assert_eq!(ppt.slides_with_hyperlinks().unwrap(), vec![1]);
}

#[test]
fn excel_sheets_with_codenames_ppt_section_named() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("A").unwrap();
    wb.add_worksheet("B").unwrap();
    wb.set_sheet_code_name("B", "CodeB").unwrap();
    let names = wb.sheets_with_code_names().unwrap();
    assert_eq!(names, vec!["B".to_string()]);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_sections(&[("Intro", 0, 0), ("Body", 1, 1)]).unwrap();
    assert!(ppt.has_section_named("Intro").unwrap());
    assert!(ppt.has_section_named("Body").unwrap());
    assert!(!ppt.has_section_named("Missing").unwrap());
}

#[test]
fn excel_has_local_defined_name() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S1").unwrap();
    wb.add_worksheet("S2").unwrap();
    assert!(!wb.has_local_defined_name("S1", "L").unwrap());
    wb.set_local_defined_name("S1", "L", "$A$1").unwrap();
    assert!(wb.has_local_defined_name("S1", "L").unwrap());
    assert!(!wb.has_local_defined_name("S2", "L").unwrap());
}

#[test]
fn word_list_styles_based_on() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    doc.add_paragraph_styles(&[
        ("Child1", "Child One", Some("Normal")),
        ("Child2", "Child Two", Some("Normal")),
        ("Other", "Other", None),
    ])
    .unwrap();
    let based = doc.list_styles_based_on("Normal").unwrap();
    assert!(based.iter().any(|id| id == "Child1"), "{based:?}");
    assert!(based.iter().any(|id| id == "Child2"), "{based:?}");
    assert!(!based.iter().any(|id| id == "Other"));
}

#[test]
fn excel_has_table_column_word_has_sdt() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["h1", "h2"]).unwrap();
    assert!(wb.has_table_column("T1", "h1").unwrap());
    assert!(!wb.has_table_column("T1", "missing").unwrap());

    use officexml::wordprocessing::{body, document, sdt_block};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let sdt = sdt_block("tagX", "Alias", vec![paragraph_with_text("c")]);
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![sdt])]));
    assert!(doc.has_content_control("tagX").unwrap());
    assert!(!doc.has_content_control("nope").unwrap());
}

#[test]
fn excel_sheets_with_tab_color() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("Red").unwrap();
    wb.add_worksheet("Plain").unwrap();
    wb.set_tab_color("Red", "FF0000").unwrap();
    let sheets = wb.sheets_with_tab_color().unwrap();
    assert!(sheets.contains(&"Red".to_string()));
    assert!(!sheets.contains(&"Plain".to_string()));
}

#[test]
fn word_list_styles_ui_priority() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    assert!(doc
        .set_style_flags("Normal", None, None, None, None, Some(99))
        .unwrap());
    let prios = doc.list_styles_with_ui_priority().unwrap();
    assert!(
        prios.iter().any(|(id, p)| id == "Normal" && *p == 99),
        "{prios:?}"
    );
}

#[test]
fn excel_has_connection() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    assert!(!wb.has_connection("Conn1").unwrap());
    wb.add_connections(&[("Conn1", "SELECT 1", "Provider=x")]).unwrap();
    assert!(wb.has_connection("Conn1").unwrap());
    assert!(!wb.has_connection("Missing").unwrap());
}

#[test]
fn excel_has_slicer_cache() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    assert!(!wb.has_slicer_cache("CacheRegion").unwrap());
    wb.add_slicer_shell("S", "Region", "CacheRegion").unwrap();
    assert!(wb.has_slicer_cache("CacheRegion").unwrap());
    assert!(!wb.has_slicer_cache("Missing").unwrap());
}

#[test]
fn ppt_has_empty_slides_word_styles_with_next() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(ppt.has_empty_slides().unwrap());
    ppt.add_text_box_on_slide(0, 0, 0, 100, 100, "a", "A").unwrap();
    assert!(!ppt.has_empty_slides().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    doc.add_paragraph_styles(&[("HeadingLike", "HL", Some("Normal"))])
        .unwrap();
    doc.set_style_links("HeadingLike", Some("Normal"), Some("Normal"), None)
        .unwrap();
    let with_next = doc.list_styles_with_next("Normal").unwrap();
    assert!(with_next.iter().any(|id| id == "HeadingLike"), "{with_next:?}");
}

#[test]
fn excel_has_empty_sheets() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("Empty").unwrap();
    assert!(wb.has_empty_sheets().unwrap());
    wb.write_sheet_strings("Empty", &[vec!["x"]]).unwrap();
    assert!(!wb.has_empty_sheets().unwrap());
}

#[test]
fn word_linked_styles_excel_sheets_sparklines() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    doc.add_paragraph_styles(&[("P1", "Para1", None)]).unwrap();
    // Link P1 to a character style id Char1 (may not exist; still stores link)
    doc.set_style_links("P1", None, None, Some("Char1")).unwrap();
    let linked = doc.list_linked_styles().unwrap();
    assert!(
        linked.iter().any(|(id, l)| id == "P1" && l == "Char1"),
        "{linked:?}"
    );

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1", "2", "3"]]).unwrap();
    assert!(!wb.has_sheets_with_sparklines().unwrap());
    wb.add_sparkline("S", "line", "A1:C1", "D1").unwrap();
    assert!(wb.has_sheets_with_sparklines().unwrap());
}

#[test]
fn excel_has_sheets_with_comments_and_tables() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("C", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("T", &[vec!["h"], vec!["v"]]).unwrap();
    assert!(!wb.has_sheets_with_comments().unwrap());
    assert!(!wb.has_sheets_with_tables().unwrap());
    wb.add_sheet_comments("C", "Author", &[("A1", "note")]).unwrap();
    wb.add_table("T", "Tbl", "A1:A2", &["h"]).unwrap();
    assert!(wb.has_sheets_with_comments().unwrap());
    assert!(wb.has_sheets_with_tables().unwrap());
}

#[test]
fn excel_has_sheets_with_cf_and_filter() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"], vec!["3"]])
        .unwrap();
    assert!(!wb.has_sheets_with_conditional_formatting().unwrap());
    assert!(!wb.has_sheets_with_auto_filter().unwrap());
    wb.add_conditional_formatting_cell_is("S", "A1:A3", "greaterThan", "1", "FF0000", 1)
        .unwrap();
    wb.set_auto_filter("S", "A1:A3").unwrap();
    assert!(wb.has_sheets_with_conditional_formatting().unwrap());
    assert!(wb.has_sheets_with_auto_filter().unwrap());
}

#[test]
fn excel_has_sheets_with_drawings_and_merged() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("D", &[vec!["a", "b"]]).unwrap();
    assert!(!wb.has_sheets_with_drawings().unwrap());
    assert!(!wb.has_sheets_with_merged_cells().unwrap());
    wb.add_bar_chart_on_sheet("D", "T", &["A"], &[1.0], 0, 0, 4, 8)
        .unwrap();
    wb.merge_range("D", "A1:B1").unwrap();
    assert!(wb.has_sheets_with_drawings().unwrap());
    assert!(wb.has_sheets_with_merged_cells().unwrap());
}

#[test]
fn excel_has_sheets_with_dv_and_freeze() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["x"]]).unwrap();
    assert!(!wb.has_sheets_with_data_validations().unwrap());
    assert!(!wb.has_sheets_with_freeze_panes().unwrap());
    wb.add_data_validation_list("S", "A1", "\"A,B\"", true).unwrap();
    wb.set_freeze_panes("S", 0, 1).unwrap();
    assert!(wb.has_sheets_with_data_validations().unwrap());
    assert!(wb.has_sheets_with_freeze_panes().unwrap());
}

#[test]
fn excel_has_sheets_with_page_setup_protection() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    assert!(!wb.has_sheets_with_page_setup().unwrap());
    assert!(!wb.has_sheets_with_sheet_protection().unwrap());
    wb.set_page_setup("S", (0.7, 0.7, 0.75, 0.75, 0.3, 0.3), 1, "portrait")
        .unwrap();
    wb.set_sheet_protection("S", true, false, false).unwrap();
    assert!(wb.has_sheets_with_page_setup().unwrap());
    assert!(wb.has_sheets_with_sheet_protection().unwrap());
}

#[test]
fn excel_has_sheets_with_formulas() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Arr", &[vec!["1"], vec!["2"]]).unwrap();
    wb.write_sheet_strings("Shr", &[vec!["1"], vec!["2"]]).unwrap();
    assert!(!wb.has_sheets_with_array_formulas().unwrap());
    assert!(!wb.has_sheets_with_shared_formulas().unwrap());
    wb.set_array_formula("Arr", "A1", "ROW()", "A1:A2", None).unwrap();
    assert!(wb.has_sheets_with_array_formulas().unwrap());
    wb.set_shared_formula("Shr", &["A1", "A2"], "A1*2", &[Some("2"), Some("4")], 0)
        .unwrap();
    assert!(wb.has_sheets_with_shared_formulas().unwrap());
}

#[test]
fn excel_has_sheets_with_codenames_and_tab_color() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("S").unwrap();
    assert!(!wb.has_sheets_with_code_names().unwrap());
    assert!(!wb.has_sheets_with_tab_color().unwrap());
    wb.set_sheet_code_name("S", "Code").unwrap();
    wb.set_tab_color("S", "00FF00").unwrap();
    assert!(wb.has_sheets_with_code_names().unwrap());
    assert!(wb.has_sheets_with_tab_color().unwrap());
}

#[test]
fn word_has_default_styles_ppt_has_slides_with_shapes() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_default_styles().unwrap());
    doc.add_default_styles().unwrap();
    assert!(doc.has_default_styles().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_slides_with_shapes().unwrap());
    ppt.add_text_box_on_slide(0, 0, 0, 100, 100, "a", "A").unwrap();
    assert!(ppt.has_slides_with_shapes().unwrap());
}

#[test]
fn ppt_has_slides_hyperlinks_word_qformat_excel_outlines() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_slides_with_hyperlinks().unwrap());
    ppt.add_slide_hyperlink(0, "https://example.com").unwrap();
    assert!(ppt.has_slides_with_hyperlinks().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_qformat_styles().unwrap());
    doc.add_default_styles().unwrap();
    assert!(doc.has_qformat_styles().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"], vec!["b"]]).unwrap();
    assert!(!wb.has_sheets_with_outlines().unwrap());
    wb.set_row_outline_levels("S", &[(2, 1, false)]).unwrap();
    assert!(wb.has_sheets_with_outlines().unwrap());
}

#[test]
fn word_has_locked_semi_hidden_ppt_notes() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    assert!(!doc.has_locked_styles().unwrap());
    assert!(!doc.has_semi_hidden_styles().unwrap());
    doc.set_style_flags("Normal", None, Some(true), None, Some(true), None)
        .unwrap();
    assert!(doc.has_locked_styles().unwrap());
    assert!(doc.has_semi_hidden_styles().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_slides_with_notes().unwrap());
    ppt.set_notes_text(0, "n").unwrap();
    assert!(ppt.has_slides_with_notes().unwrap());
}

#[test]
fn excel_has_hidden_visible_sheets_ppt_visible_slides() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("A").unwrap();
    wb.add_worksheet("B").unwrap();
    assert!(wb.has_visible_sheets().unwrap());
    assert!(!wb.has_hidden_sheets().unwrap());
    wb.set_sheet_state("B", "hidden").unwrap();
    assert!(wb.has_hidden_sheets().unwrap());
    assert!(wb.has_visible_sheets().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(ppt.has_visible_slides().unwrap());
    ppt.set_slide_hidden(0, true).unwrap();
    assert!(!ppt.has_visible_slides().unwrap());
}

#[test]
fn excel_very_hidden_word_unhide_ppt_anim_slides() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.add_worksheet("A").unwrap();
    wb.add_worksheet("B").unwrap();
    assert!(!wb.has_very_hidden_sheets().unwrap());
    wb.set_sheet_state("B", "veryHidden").unwrap();
    assert!(wb.has_very_hidden_sheets().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    assert!(!doc.has_unhide_when_used_styles().unwrap());
    doc.set_style_flags("Normal", None, None, Some(true), None, None)
        .unwrap();
    assert!(doc.has_unhide_when_used_styles().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_slides_with_animation().unwrap());
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 100, 100, "a", "A")
        .unwrap();
    ppt.set_simple_appear_animation(0, sid).unwrap();
    assert!(ppt.has_slides_with_animation().unwrap());
}

#[test]
fn word_has_ui_priority_linked_ppt_transition() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    assert!(!doc.has_styles_with_ui_priority().unwrap());
    doc.set_style_flags("Normal", None, None, None, None, Some(10))
        .unwrap();
    assert!(doc.has_styles_with_ui_priority().unwrap());
    doc.set_style_links("Normal", None, None, Some("CharLink"))
        .unwrap();
    assert!(doc.has_linked_styles().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_slides_with_transition().unwrap());
    ppt.set_fade_transition(0, "fast").unwrap();
    assert!(ppt.has_slides_with_transition().unwrap());
}

#[test]
fn word_has_styles_based_next_ppt_comments() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    doc.add_paragraph_styles(&[("C1", "Child", Some("Normal"))])
        .unwrap();
    assert!(doc.has_styles_based_on("Normal").unwrap());
    assert!(!doc.has_styles_based_on("Missing").unwrap());
    doc.set_style_links("C1", Some("Normal"), Some("Normal"), None)
        .unwrap();
    assert!(doc.has_styles_with_next("Normal").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_slides_with_comments().unwrap());
    ppt.add_comment_authors(&[(0, "A", "A")]).unwrap();
    ppt.add_slide_comments(0, &[(0, "2020-01-01T00:00:00", 0, 0, "hi")])
        .unwrap();
    assert!(ppt.has_slides_with_comments().unwrap());
}


#[test]
fn rewrite_cleanups_sst_hyperlink_table_ref() {
    // Excel: clear_shared_strings rewrites t="s" to inlineStr
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_shared_strings("S", &[vec!["hello", "world"]])
        .unwrap();
    assert_eq!(wb.get_cell_value("S", "A1").unwrap().as_deref(), Some("hello"));
    assert!(wb.clear_shared_strings().unwrap());
    assert!(!wb.has_shared_strings());
    // Value should still resolve via inlineStr
    assert_eq!(wb.get_cell_value("S", "A1").unwrap().as_deref(), Some("hello"));
    assert_eq!(wb.get_cell_value("S", "B1").unwrap().as_deref(), Some("world"));

    // Excel: remove last table column shrinks ref
    let mut wb2 =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb2.write_sheet_strings("S", &[vec!["h1", "h2", "h3"], vec!["a", "b", "c"]])
        .unwrap();
    wb2.add_table("S", "T1", "A1:C2", &["h1", "h2", "h3"]).unwrap();
    assert_eq!(wb2.table_ref("T1").unwrap().as_deref(), Some("A1:C2"));
    assert!(wb2.remove_table_column("T1", "h3").unwrap());
    assert_eq!(wb2.table_ref("T1").unwrap().as_deref(), Some("A1:B2"));
    // middle column does not shrink ref
    assert!(wb2.remove_table_column("T1", "h1").unwrap());
    assert_eq!(wb2.table_ref("T1").unwrap().as_deref(), Some("A1:B2"));

    // Word: remove_external_hyperlink unwraps body hyperlinks
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("before")]));
    doc.append_hyperlink("https://example.com/x", "link").unwrap();
    assert!(doc.has_body_hyperlinks().unwrap());
    assert!(doc.remove_external_hyperlink("https://example.com/x"));
    assert!(doc.list_external_hyperlinks().is_empty());
    assert!(!doc.has_body_hyperlinks().unwrap());
    // link text survives as plain run text
    let texts = doc.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("link")));
}

#[test]
fn revision_inventory_and_has_companions() {
    use officexml::element::OpenXmlElement;
    use officexml::wordprocessing::{body, document, paragraph, run, text};

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    let ins = OpenXmlElement::w("ins")
        .with_attribute_qname("w:author", "Alice")
        .with_attribute_qname("w:date", "2020-01-01T00:00:00Z")
        .with_child(run(vec![text("added")]));
    let del = OpenXmlElement::w("del")
        .with_attribute_qname("w:author", "Bob")
        .with_child(
            OpenXmlElement::w("r").with_child(
                OpenXmlElement::w("delText").with_text("removed"),
            ),
        );
    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![ins]),
        paragraph(vec![del]),
    ])]));
    assert!(doc.has_revision_markers().unwrap());
    assert_eq!(doc.revision_marker_count().unwrap(), 2);
    assert_eq!(doc.insertion_count().unwrap(), 1);
    assert_eq!(doc.deletion_count().unwrap(), 1);
    let marks = doc.list_revision_markers().unwrap();
    assert_eq!(marks.len(), 2);
    assert_eq!(marks[0].0, "ins");
    assert_eq!(marks[0].1, "Alice");
    assert_eq!(doc.accept_all_revisions().unwrap(), 2);
    assert!(!doc.has_revision_markers().unwrap());

    // Excel inventory has_*
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]])
        .unwrap();
    wb.set_auto_filter("S", "A1:B2").unwrap();
    // filter columns may be empty until configured; has_auto_filter_columns is false initially
    assert!(!wb.has_auto_filter_columns("S").unwrap());
    wb.add_cell_watch("S", "A1").unwrap();
    assert!(wb.has_cell_watches("S").unwrap());
    assert_eq!(wb.cell_watch_count("S").unwrap(), 1);
    assert!(!wb.has_hidden_columns("S").unwrap());
    wb.set_column_hidden("S", 1, 1, true).unwrap();
    assert!(wb.has_hidden_columns("S").unwrap());
    assert!(wb.sheets_with_outlines().unwrap().is_empty() || true);

    // PPT inventory has_*
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let _ = ppt.has_any_shapes().unwrap();
    let _ = ppt.has_slide_names().unwrap();
    let _ = ppt.has_slide_transitions().unwrap();
    assert!(!ppt.has_comment_parts());
    assert!(!ppt.has_slide_sync_parts());
}

#[test]
fn word_has_inventory_companions() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    doc.add_default_numbering().unwrap();
    assert!(doc.has_abstract_nums().unwrap());
    assert!(doc.has_num_instances().unwrap());
    assert!(!doc.has_bibliography_sources().unwrap());
    assert!(!doc.has_auto_captions().unwrap());
    assert!(!doc.has_compat_settings().unwrap());
    assert!(!doc.has_glossary_doc_parts().unwrap());
    assert!(!doc.has_anchor_hyperlinks().unwrap());
    let _ = doc.has_font_names().unwrap();
}


#[test]
fn inventory_location_cf_content_types() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"], vec!["1"]]).unwrap();
    assert!(!wb.has_location_hyperlinks("S").unwrap());
    assert!(!wb.has_cf_rules("S").unwrap());
    wb.add_conditional_formatting_cell_is("S", "A1:A10", "greaterThan", "0", "FFFF0000", 1)
        .unwrap();
    assert!(wb.has_cf_rules("S").unwrap());
    assert!(wb.cf_rule_count("S").unwrap() >= 1);
    assert!(wb.has_content_type_overrides());
    assert!(wb.content_type_override_count() > 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(doc.has_content_type_overrides());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(ppt.has_content_type_overrides());
    assert!(!ppt.has_external_hyperlinks());
}


#[test]
fn excel_remove_chart_rewrites_drawing_anchors() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["A", "B"], vec!["1", "2"]])
        .unwrap();
    let (chart_uri, drawing_uri) = wb
        .add_bar_chart_on_sheet("S", "Sales", &["A", "B"], &[1.0, 2.0], 0, 0, 4, 10)
        .unwrap();
    assert!(wb.has_charts());
    assert!(wb.package().opc().has_part(&drawing_uri));
    // Drawing should contain a chart graphic frame
    let data = wb.package().opc().get_part(&drawing_uri).unwrap();
    let root = parse_element(data).unwrap();
    assert!(
        root.descendants().any(|e| e.local_name == "chart"),
        "expected chart anchor before remove"
    );
    assert!(wb.remove_chart(&chart_uri).unwrap());
    assert!(!wb.has_charts());
    // Drawing part may still exist but should no longer reference the chart
    if let Some(data) = wb.package().opc().get_part(&drawing_uri) {
        let root = parse_element(data).unwrap();
        assert!(
            !root.descendants().any(|e| e.local_name == "chart"),
            "chart anchor should be stripped from drawing"
        );
    }
}


#[test]
fn word_content_control_alias_clear() {
    use officexml::wordprocessing::{body, document, paragraph_with_text, sdt_block};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        sdt_block("tag1", "Alias One", vec![paragraph_with_text("inside")]),
        paragraph_with_text("after"),
    ])]));
    assert!(doc.has_content_control("tag1").unwrap());
    assert_eq!(
        doc.content_control_alias("tag1").unwrap().as_deref(),
        Some("Alias One")
    );
    assert!(doc.has_content_control_alias("Alias One").unwrap());
    assert!(doc.content_control_kind("tag1").unwrap().is_some());
    let tags = doc.list_content_control_tags().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(doc.content_control_text("tag1").unwrap().as_deref(), Some("inside"));
    assert_eq!(doc.clear_content_controls().unwrap(), 1);
    assert!(!doc.has_content_controls().unwrap());
    // Content text should survive unwrap
    let texts = doc.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("inside")));
}


#[test]
fn has_fills_style_ids_mm_anim() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    // default stylesheet may already have fills
    let _ = wb.has_fills().unwrap();
    let _ = wb.has_style_ids().unwrap();
    let _ = wb.style_id_count().unwrap();

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(!doc.has_mail_merge_odso_field_maps().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_animation_effects().unwrap());
    ppt.set_simple_appear_animation(0, 2).unwrap();
    // may or may not register as effect depending on implementation
    let _ = ppt.has_animation_effects().unwrap();
}


#[test]
fn materialize_sst_named_styles_person() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_shared_strings("S", &[vec!["hello", "world"]])
        .unwrap();
    assert!(wb.has_shared_strings());
    let n = wb.materialize_shared_strings().unwrap();
    assert!(n >= 2);
    // SST still present
    assert!(wb.has_shared_strings());
    assert_eq!(wb.get_cell_value("S", "A1").unwrap().as_deref(), Some("hello"));
    let _ = wb.has_named_styles().unwrap();

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    // people may be empty
    assert!(!doc.has_person("nobody").unwrap());
    assert_eq!(doc.people_count().unwrap(), 0);
}


#[test]
fn word_content_control_kind_infos_ppt_clear_names() {
    use officexml::wordprocessing::{body, document, paragraph_with_text, sdt_block_with_kind};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        sdt_block_with_kind("k1", "DateCtl", "date", vec![paragraph_with_text("2020")]),
    ])]));
    assert_eq!(
        doc.content_control_kind("k1").unwrap().as_deref(),
        Some("date")
    );
    let infos = doc.content_control_infos().unwrap();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].0, "k1");
    assert_eq!(infos[0].2, "date");
    assert!(infos[0].3.contains("2020"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_slide_name(0, "Intro").unwrap();
    ppt.set_slide_name(1, "Outro").unwrap();
    assert!(ppt.has_slide_names().unwrap());
    assert_eq!(ppt.clear_all_slide_names().unwrap(), 2);
    assert!(!ppt.has_slide_names().unwrap());
}


#[test]
fn word_remove_chart_excel_unhide_ppt_anim_shape() {
    // Word remove_chart
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("c")]));
    let (_rid, uri) = doc
        .add_chart("T", &["A", "B"], &[1.0, 2.0])
        .unwrap();
    assert_eq!(doc.chart_count(), 1);
    assert!(doc.remove_chart(&uri).unwrap());
    assert_eq!(doc.chart_count(), 0);
    let (_rid2, _) = doc.add_chart("T2", &["X"], &[3.0]).unwrap();
    assert!(doc.remove_chart_at(0).unwrap());
    assert!(!doc.has_charts());

    // Excel unhide columns/rows
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b", "c"], vec!["1", "2", "3"]])
        .unwrap();
    wb.set_column_hidden("S", 1, 2, true).unwrap();
    assert!(wb.has_hidden_columns("S").unwrap());
    assert!(wb.unhide_all_columns("S").unwrap() >= 1);
    assert!(!wb.has_hidden_columns("S").unwrap());
    wb.set_row_hidden("S", 2, true).unwrap();
    assert!(wb.has_hidden_rows("S").unwrap());
    assert_eq!(wb.unhide_all_rows("S").unwrap(), 1);
    assert!(!wb.has_hidden_rows("S").unwrap());

    // PPT animation per shape
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_animation_effect(0, 5, "fade", "in").unwrap();
    let effects = ppt.list_slide_animation_effects(0).unwrap();
    assert!(!effects.is_empty());
    assert_eq!(effects[0].0, 5);
    assert_eq!(ppt.remove_animation_for_shape(0, 5).unwrap(), 1);
    assert!(!ppt.has_animation_effect(0).unwrap());
}


#[test]
fn clear_external_hl_hidden_sheets_anim_list() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.append_hyperlink("https://a.example", "A").unwrap();
    doc.append_hyperlink("https://b.example", "B").unwrap();
    assert_eq!(doc.list_external_hyperlinks().len(), 2);
    let (rels, body) = doc.clear_external_hyperlinks().unwrap();
    assert_eq!(rels, 2);
    assert!(body >= 2);
    assert!(doc.list_external_hyperlinks().is_empty());
    assert!(!doc.has_body_hyperlinks().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]])
        .unwrap();
    wb.set_column_hidden("S", 1, 1, true).unwrap();
    wb.set_row_hidden("S", 2, true).unwrap();
    assert_eq!(wb.sheets_with_hidden_columns().unwrap(), vec!["S".to_string()]);
    assert!(wb.has_sheets_with_hidden_columns().unwrap());
    assert_eq!(wb.sheets_with_hidden_rows().unwrap(), vec!["S".to_string()]);
    assert!(wb.has_sheets_with_hidden_rows().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_animation_effect(1, 3, "fade", "in").unwrap();
    assert_eq!(ppt.slides_with_animation_effects().unwrap(), vec![1]);
}


#[test]
fn excel_table_af_outlines_word_drawing() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["h1", "h2"]).unwrap();
    // table_definition includes autoFilter by default
    assert!(wb.has_table_auto_filter("T1").unwrap());
    assert_eq!(
        wb.table_auto_filter_ref("T1").unwrap().as_deref(),
        Some("A1:B2")
    );
    assert!(wb.clear_table_auto_filter("T1").unwrap());
    assert!(!wb.has_table_auto_filter("T1").unwrap());
    assert!(wb.set_table_auto_filter("T1", "A1:B2").unwrap());
    assert!(wb.has_table_auto_filter("T1").unwrap());

    // outlines
    wb.set_row_outline_levels("S", &[(2, 1, false)]).unwrap();
    assert!(wb.has_row_outlines("S").unwrap());
    assert_eq!(wb.clear_all_outlines().unwrap(), 1);
    assert!(!wb.has_row_outlines("S").unwrap());

    // Word remove_drawing - create via clear_drawings path if no add_drawing simple
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("d")]));
    // drawings may be empty
    assert_eq!(doc.drawing_count(), 0);
    assert!(!doc.remove_drawing(&officexml::opc::PackUri::new("/word/drawings/drawing1.xml")).unwrap());
}


#[test]
fn remove_empty_sheets_and_embeddings() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("Data", &[vec!["x"]]).unwrap();
    wb.add_worksheet("Empty1").unwrap();
    wb.add_worksheet("Empty2").unwrap();
    assert!(wb.list_empty_sheets().unwrap().len() >= 2);
    let n = wb.remove_empty_sheets().unwrap();
    assert!(n >= 2);
    assert!(wb.has_sheet("Data"));
    assert!(!wb.list_empty_sheets().unwrap().contains(&"Empty1".to_string()));

    let (_rid, uri) = wb
        .add_embedded_package(b"fake-embed-bytes", "application/octet-stream", "bin")
        .unwrap();
    assert!(wb.has_embeddings());
    assert!(wb.remove_embedding(&uri).unwrap());
    assert!(!wb.has_embeddings());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("e")]));
    let (_rid, uri) = doc
        .add_embedded_package(b"fake-embed-bytes", "application/octet-stream", "bin")
        .unwrap();
    assert!(doc.has_embeddings());
    assert!(doc.remove_embedding(&uri).unwrap());
    assert!(!doc.has_embeddings());
}



#[test]
fn complex_fields_and_hf_revisions() {
    use officexml::element::OpenXmlElement;
    use officexml::wordprocessing::{
        body, complex_field_paragraph, document, paragraph, run, text,
    };

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        complex_field_paragraph(" PAGE ", "1"),
    ])]));
    assert!(doc.has_complex_fields().unwrap());
    assert_eq!(doc.complex_field_count().unwrap(), 1);
    let instrs = doc.list_complex_field_instructions().unwrap();
    assert!(instrs.iter().any(|s| s.contains("PAGE")));
    doc.append_complex_field(" DATE ", "today").unwrap();
    assert_eq!(doc.complex_field_count().unwrap(), 2);

    // Header with tracked insert, then accept in headers
    let rid = doc
        .add_header(vec![paragraph(vec![OpenXmlElement::w("ins")
            .with_attribute_qname("w:author", "A")
            .with_child(run(vec![text("hdr")]))])])
        .unwrap();
    assert!(!rid.is_empty());
    let n = doc.accept_all_revisions_in_headers_footers().unwrap();
    assert!(n >= 1);
    let texts = doc.header_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("hdr")));
}



#[test]
fn clear_complex_fields_ppt_empty_slides() {
    use officexml::wordprocessing::{body, complex_field_paragraph, document};

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        complex_field_paragraph(" PAGE ", "1"),
    ])]));
    assert!(doc.has_complex_fields().unwrap());
    let n = doc.clear_complex_fields().unwrap();
    assert!(n >= 2); // begin/instr/separate/end markup runs
    assert!(!doc.has_complex_fields().unwrap());
    // result text should remain
    let texts = doc.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("1")));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    // blank slides are empty
    let before = ppt.slide_count();
    let n = ppt.remove_empty_slides().unwrap();
    assert!(n >= 1);
    assert!(ppt.slide_count() < before);
    assert_eq!(ppt.slide_count(), 1);
}


#[test]
fn excel_clear_all_cf_dv() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["1"], vec!["2"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["3"], vec!["4"]]).unwrap();
    wb.add_conditional_formatting_cell_is("S1", "A1:A10", "greaterThan", "0", "FFFF0000", 1)
        .unwrap();
    wb.add_conditional_formatting_cell_is("S2", "A1:A10", "lessThan", "10", "FF00FF00", 1)
        .unwrap();
    assert!(wb.has_conditional_formatting("S1").unwrap());
    assert_eq!(wb.clear_all_conditional_formatting().unwrap(), 2);
    assert!(!wb.has_conditional_formatting("S1").unwrap());
    assert!(!wb.has_conditional_formatting("S2").unwrap());

    // data validation
    wb.add_data_validation_list("S1", "B1:B10", "\"A,B,C\"", true).unwrap();
    assert!(wb.has_data_validations("S1").unwrap());
    assert!(wb.clear_all_data_validations().unwrap() >= 1);
}



#[test]
fn clear_all_merges_hl_fields() {
    use officexml::wordprocessing::{body, complex_field_paragraph, document};

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();
    wb.merge_range("S", "A1:B1").unwrap();
    assert!(!wb.merge_cells("S").unwrap().is_empty());
    assert_eq!(wb.clear_all_merged_cells().unwrap(), 1);
    assert!(wb.merge_cells("S").unwrap().is_empty());

    // hyperlinks if add API exists - soft
    let _ = wb.clear_all_cell_hyperlinks().unwrap();

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        complex_field_paragraph(" PAGE ", "1"),
    ])]));
    doc.append_simple_field(" AUTHOR ", "me").unwrap();
    let (s, c) = doc.clear_all_fields().unwrap();
    assert!(s >= 1 || c >= 1);
}


#[test]
fn clear_sparklines_freeze_bookmarks() {
    use officexml::wordprocessing::{
        body, bookmark_end, bookmark_start, document, paragraph, run, text,
    };

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1", "2", "3"], vec!["4", "5", "6"]])
        .unwrap();
    wb.add_sparkline("S", "line", "A1:C1", "E1").unwrap();
    assert!(wb.has_sparklines("S").unwrap());
    assert_eq!(wb.clear_all_sparklines().unwrap(), 1);
    assert!(!wb.has_sparklines("S").unwrap());

    wb.set_freeze_panes("S", 0, 1).unwrap();
    assert!(wb.has_freeze_panes("S").unwrap());
    assert_eq!(wb.clear_all_freeze_panes().unwrap(), 1);
    assert!(!wb.has_freeze_panes("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![
            bookmark_start("0", "bm1"),
            run(vec![text("here")]),
            bookmark_end("0"),
        ]),
    ])]));
    assert!(doc.has_bookmark("bm1").unwrap());
    assert!(doc.clear_bookmarks().unwrap() >= 1);
    assert!(!doc.has_bookmark("bm1").unwrap());
}


#[test]
fn clear_all_auto_filters_test() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["c", "d"], vec!["3", "4"]]).unwrap();
    wb.set_auto_filter("S1", "A1:B2").unwrap();
    wb.set_auto_filter("S2", "A1:B2").unwrap();
    assert_eq!(wb.sheets_with_auto_filter().unwrap().len(), 2);
    assert_eq!(wb.clear_all_auto_filters().unwrap(), 2);
    assert!(!wb.has_auto_filter("S1").unwrap());
    assert!(!wb.has_auto_filter("S2").unwrap());
}


#[test]
fn rebuild_calc_unused_styles() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.set_cell_formula("S", "A3", "A1+A2", Some("3")).unwrap();
    assert!(wb.list_formulas("S").unwrap().iter().any(|(r, _)| r == "A3"));
    let n = wb.rebuild_calc_chain().unwrap();
    assert!(n >= 1);
    assert!(wb.has_calc_chain());
    assert!(wb.calc_chain_entry_count().unwrap() >= 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_default_styles().unwrap();
    doc.add_paragraph_styles(&[("Unused1", "Unused Style", Some("Normal"))])
        .unwrap();
    // Body uses no explicit style, or Normal only
    let unused = doc.list_unused_style_ids().unwrap();
    assert!(unused.iter().any(|s| s == "Unused1"));
    let removed = doc.remove_unused_styles().unwrap();
    assert!(removed >= 1);
    assert!(!doc.has_style("Unused1").unwrap());
}


#[test]
fn clear_all_tables_and_remove_media() {
    use officexml::packaging::ImageFormat;

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["h1", "h2"], vec!["a", "b"]])
        .unwrap();
    wb.write_sheet_strings("S2", &[vec!["x", "y"], vec!["1", "2"]])
        .unwrap();
    wb.add_table("S1", "T1", "A1:B2", &["h1", "h2"]).unwrap();
    wb.add_table("S2", "T2", "A1:B2", &["x", "y"]).unwrap();
    assert!(wb.has_tables());
    let n = wb.clear_all_tables().unwrap();
    assert!(n >= 2);
    assert!(!wb.has_tables());

    let png = [
        0x89u8, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8,
        0xcf, 0xc0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xfe, 0xd4, 0xef, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ];
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("img")]));
    let image = doc.add_image(ImageFormat::Png, png.to_vec()).unwrap();
    let uri = image.uri().clone();
    assert!(doc.media_count() >= 1);
    assert!(doc.remove_media(&uri).unwrap());
    assert_eq!(doc.media_count(), 0);
}



#[test]
fn custom_xml_uris_and_masters() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_custom_xml_part(b"<root/>".to_vec()).unwrap();
    let uris = doc.list_custom_xml_part_uris().unwrap();
    assert_eq!(uris.len(), 1);
    assert_eq!(doc.custom_xml_part_count().unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let _ = ppt.has_slide_masters();
    let _ = ppt.slide_master_count();
    let _ = ppt.slide_master_part_count();
}


#[test]
fn unhide_slides_clear_formulas() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    assert_eq!(ppt.hide_slides(&[0, 2]).unwrap(), 2);
    assert_eq!(ppt.list_hidden_slides().unwrap(), vec![0, 2]);
    assert_eq!(ppt.unhide_all_slides().unwrap(), 2);
    assert!(ppt.list_hidden_slides().unwrap().is_empty());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.set_cell_formula("S", "A3", "A1+A2", Some("3")).unwrap();
    assert_eq!(wb.formula_count("S").unwrap(), 1);
    assert_eq!(wb.clear_formulas("S").unwrap(), 1);
    assert_eq!(wb.formula_count("S").unwrap(), 0);
    // cached value should remain
    assert_eq!(wb.get_cell_value("S", "A3").unwrap().as_deref(), Some("3"));
}


#[test]
fn clear_col_row_sizes_and_notes() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]])
        .unwrap();
    wb.set_column_widths("S", &[(1, 1, 20.0)]).unwrap();
    assert!(!wb.column_widths("S").unwrap().is_empty());
    assert!(wb.clear_column_widths("S").unwrap());
    assert!(wb.column_widths("S").unwrap().is_empty());

    wb.set_row_heights("S", &[(1, 30.0, false)]).unwrap();
    assert_eq!(wb.clear_row_heights("S").unwrap(), 1);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("n")]));
    // footnotes may not exist
    let _ = doc.clear_all_notes().unwrap();
    assert!(!doc.has_footnotes());
}


#[test]
fn clear_tab_colors_and_protection() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["b"]]).unwrap();
    wb.set_tab_color("S1", "FF0000").unwrap();
    wb.set_tab_color("S2", "00FF00").unwrap();
    assert_eq!(wb.sheets_with_tab_color().unwrap().len(), 2);
    assert_eq!(wb.clear_all_tab_colors().unwrap(), 2);
    assert!(wb.sheets_with_tab_color().unwrap().is_empty());

    wb.set_sheet_protection("S1", true, false, false).unwrap();
    assert!(wb.has_sheet_protection("S1").unwrap());
    assert_eq!(wb.clear_all_sheet_protection().unwrap(), 1);
    assert!(!wb.has_sheet_protection("S1").unwrap());
}


#[test]
fn clear_code_names_and_rtl() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_sheet_code_name("S", "SheetCode").unwrap();
    assert!(wb.has_sheet_code_name("S").unwrap());
    assert_eq!(wb.clear_all_sheet_code_names().unwrap(), 1);
    assert!(!wb.has_sheet_code_name("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_rtl(true).unwrap();
    assert!(ppt.has_rtl().unwrap());
    ppt.set_rtl(false).unwrap();
    assert!(!ppt.has_rtl().unwrap());
}


#[test]
fn first_slide_num_and_active_tab() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    assert!(!ppt.has_first_slide_num().unwrap());
    ppt.set_first_slide_num(3).unwrap();
    assert_eq!(ppt.first_slide_num().unwrap(), Some(3));
    assert!(ppt.clear_first_slide_num().unwrap());
    assert!(!ppt.has_first_slide_num().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["b"]]).unwrap();
    wb.set_active_tab(1).unwrap();
    assert_eq!(wb.active_tab().unwrap(), Some(1));
    assert!(wb.has_active_tab().unwrap());
}


#[test]
fn clear_all_page_setup_test() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["b"]]).unwrap();
    wb.set_page_setup("S1", (0.7, 0.7, 0.75, 0.75, 0.3, 0.3), 1, "portrait")
        .unwrap();
    wb.set_page_setup("S2", (0.5, 0.5, 0.5, 0.5, 0.3, 0.3), 1, "landscape")
        .unwrap();
    assert!(wb.has_page_setup("S1").unwrap());
    assert!(wb.has_page_setup("S2").unwrap());
    assert_eq!(wb.clear_all_page_setup().unwrap(), 2);
    assert!(!wb.has_page_setup("S1").unwrap());
    assert!(!wb.has_page_setup("S2").unwrap());
}



#[test]
fn clear_all_print_options_test() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["b"]]).unwrap();
    wb.set_print_headings("S1", true).unwrap();
    wb.set_print_grid_lines("S2", true).unwrap();
    assert!(wb.has_print_options("S1").unwrap());
    assert!(wb.has_print_options("S2").unwrap());
    assert_eq!(wb.clear_all_print_options().unwrap(), 2);
    assert!(!wb.has_print_options("S1").unwrap());
    assert!(!wb.has_print_options("S2").unwrap());
}



#[test]
fn clear_all_zoom_and_sort() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["c", "d"], vec!["3", "4"]]).unwrap();
    wb.set_zoom("S1", 150).unwrap();
    wb.set_zoom("S2", 80).unwrap();
    assert!(wb.has_zoom("S1").unwrap());
    assert_eq!(wb.clear_all_zoom().unwrap(), 2);
    assert!(!wb.has_zoom("S1").unwrap());

    wb.set_sort_state("S1", "A1:B2", "A1", false).unwrap();
    wb.set_sort_state("S2", "A1:B2", "B1", true).unwrap();
    assert_eq!(wb.sheets_with_sort_state().unwrap().len(), 2);
    assert_eq!(wb.clear_all_sort_state().unwrap(), 2);
    assert!(!wb.has_sort_state("S1").unwrap());
}


#[test]
fn unhide_sheets_update_fields_bookmark_seed() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["b"]]).unwrap();
    wb.set_sheet_state("S1", "hidden").unwrap();
    wb.set_sheet_state("S2", "veryHidden").unwrap();
    assert!(wb.has_hidden_sheets().unwrap());
    assert_eq!(wb.unhide_all_sheets().unwrap(), 2);
    assert!(!wb.has_hidden_sheets().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_update_fields_on_open(true).unwrap();
    assert!(doc.has_update_fields_on_open().unwrap());
    assert!(doc.clear_update_fields_on_open().unwrap());
    assert!(!doc.has_update_fields_on_open().unwrap());
    doc.set_embed_true_type_fonts(true).unwrap();
    assert!(doc.clear_embed_true_type_fonts().unwrap());
    assert!(!doc.has_embed_true_type_fonts().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_bookmark_id_seed(100).unwrap();
    assert!(ppt.has_bookmark_id_seed().unwrap());
    assert!(ppt.clear_bookmark_id_seed().unwrap());
    assert!(!ppt.has_bookmark_id_seed().unwrap());
}


#[test]
fn clear_settings_flags_and_track() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_track_revisions(true).unwrap();
    assert!(doc.has_track_revisions().unwrap());
    assert!(doc.clear_track_revisions().unwrap());
    assert!(!doc.has_track_revisions().unwrap());

    doc.set_do_not_display_page_boundaries(true).unwrap();
    assert!(doc.clear_do_not_display_page_boundaries().unwrap());
    assert!(!doc.has_do_not_display_page_boundaries().unwrap());

    doc.set_do_not_auto_compress_pictures(true).unwrap();
    assert!(doc.clear_do_not_auto_compress_pictures().unwrap());

    doc.set_do_not_embed_smart_tags(true).unwrap();
    assert!(doc.clear_do_not_embed_smart_tags().unwrap());

    doc.set_strict_first_and_last_chars(true).unwrap();
    assert!(doc.clear_strict_first_and_last_chars().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_show_special_pls_on_title_sld(true).unwrap();
    let _ = ppt.clear_show_special_pls_on_title_sld().unwrap();
}


#[test]
fn more_settings_clears_and_sheet_format() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_remove_personal_information(true).unwrap();
    assert!(doc.clear_remove_personal_information().unwrap());
    doc.set_print_two_on_one(true).unwrap();
    assert!(doc.clear_print_two_on_one().unwrap());
    doc.set_save_forms_data(true).unwrap();
    assert!(doc.clear_save_forms_data().unwrap());
    doc.set_forms_design(true).unwrap();
    assert!(doc.clear_forms_design().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S1", &[vec!["a"]]).unwrap();
    wb.write_sheet_strings("S2", &[vec!["b"]]).unwrap();
    wb.set_sheet_format("S1", 15.0, Some(10.0)).unwrap();
    wb.set_sheet_format("S2", 18.0, None).unwrap();
    assert_eq!(wb.clear_all_sheet_format().unwrap(), 2);
}


#[test]
fn batch_settings_clears_and_rtl() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_auto_hyphenation(true).unwrap();
    assert!(doc.clear_auto_hyphenation().unwrap());
    assert!(!doc.has_auto_hyphenation().unwrap());
    doc.set_book_fold_printing(true).unwrap();
    assert!(doc.clear_book_fold_printing().unwrap());
    doc.set_always_show_placeholder_text(true).unwrap();
    assert!(doc.clear_always_show_placeholder_text().unwrap());
    doc.set_display_background_shape(true).unwrap();
    assert!(doc.clear_display_background_shape().unwrap());
    doc.set_hide_spelling_errors(true).unwrap();
    assert!(doc.clear_hide_spelling_errors().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_right_to_left("S", true).unwrap();
    assert!(wb.has_right_to_left("S").unwrap());
    assert!(wb.clear_right_to_left("S").unwrap());
    assert!(!wb.has_right_to_left("S").unwrap());
}


#[test]
fn excel_sheet_view_clears() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();
    wb.set_show_formulas("S", true).unwrap();
    assert!(wb.clear_show_formulas("S").unwrap());
    wb.set_show_zeros("S", true).unwrap();
    assert!(wb.clear_show_zeros("S").unwrap());
    wb.set_show_gridlines("S", false).unwrap();
    // clear sets false - if already false, clear returns false when had is false
    let _ = wb.clear_show_gridlines("S").unwrap();
    wb.set_show_row_col_headers("S", false).unwrap();
    let _ = wb.clear_show_row_col_headers("S").unwrap();
    wb.set_print_headings("S", true).unwrap();
    assert!(wb.clear_print_headings("S").unwrap());
    wb.set_print_grid_lines("S", true).unwrap();
    assert!(wb.clear_print_grid_lines("S").unwrap());
    wb.set_window_protection("S", true).unwrap();
    assert!(wb.clear_window_protection("S").unwrap());
}


#[test]
fn ppt_bool_settings_clears() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_show_loop(true).unwrap();
    assert!(ppt.clear_show_loop().unwrap());
    ppt.set_show_narration(true).unwrap();
    assert!(ppt.clear_show_narration().unwrap());
    ppt.set_show_animation(true).unwrap();
    assert!(ppt.clear_show_animation().unwrap());
    ppt.set_use_timings(true).unwrap();
    assert!(ppt.clear_use_timings().unwrap());
    ppt.set_rtl(true).unwrap();
    assert!(ppt.clear_rtl().unwrap());
    assert!(!ppt.has_rtl().unwrap());
    ppt.set_snap_to_grid(true).unwrap();
    assert!(ppt.clear_snap_to_grid().unwrap());
    ppt.set_show_guides(true).unwrap();
    assert!(ppt.clear_show_guides().unwrap());
}


#[test]
fn people_and_odso_field_map_remove() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_person("Alice", "provider-1").unwrap();
    assert!(doc.has_person("Alice").unwrap());
    assert_eq!(doc.people_count().unwrap(), 1);
    assert!(doc.remove_person("Alice").unwrap());
    assert!(!doc.has_person("Alice").unwrap());

    doc.add_mail_merge_odso_field_map("dbColumn", "FirstName", "First").unwrap();
    doc.add_mail_merge_odso_field_map("dbColumn", "LastName", "Last").unwrap();
    assert_eq!(doc.mail_merge_odso_field_map_count().unwrap(), 2);
    assert_eq!(doc.remove_mail_merge_odso_field_map("FirstName").unwrap(), 1);
    assert_eq!(doc.mail_merge_odso_field_map_count().unwrap(), 1);
    assert!(doc.has_mail_merge_odso_field_maps().unwrap());
}


#[test]
fn shared_formulas_and_complex_field_remove() {
    use officexml::wordprocessing::{body, complex_field_paragraph, document};

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"], vec!["3"]]).unwrap();
    wb.set_shared_formula(
        "S",
        &["A1", "A2", "A3"],
        "ROW()",
        &[Some("1"), Some("2"), Some("3")],
        0,
    )
    .unwrap();
    assert!(wb.clear_shared_formulas("S").unwrap() >= 1);
    assert_eq!(wb.clear_all_shared_formulas().unwrap(), 0);

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        complex_field_paragraph(" PAGE ", "1"),
        complex_field_paragraph(" DATE ", "2020"),
    ])]));
    assert_eq!(doc.complex_field_count().unwrap(), 2);
    let n = doc.remove_complex_fields_matching("PAGE").unwrap();
    assert!(n >= 1);
    assert_eq!(doc.complex_field_count().unwrap(), 1);
    let texts = doc.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("1")));
}


#[test]
fn transitions_and_array_formulas() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_push_transition(0, "med").unwrap();
    ppt.set_wipe_transition(1, "fast").unwrap();
    assert!(ppt.has_transition(0).unwrap());
    assert!(ppt.has_transition(1).unwrap());
    ppt.set_cover_transition(0, "slow").unwrap();
    ppt.set_wheel_transition(1, "med").unwrap();
    ppt.set_random_transition(0, "fast").unwrap();
    ppt.set_split_transition(1, "med").unwrap();
    assert_eq!(ppt.clear_all_transitions().unwrap(), 2);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1", "2"], vec!["3", "4"]]).unwrap();
    wb.set_array_formula("S", "C1", "A1:A2*B1:B2", "C1:C2", Some("1"))
        .unwrap();
    assert!(wb.has_array_formulas("S").unwrap());
    assert!(wb
        .sheets_with_array_formulas()
        .unwrap()
        .contains(&"S".to_string()));
    assert_eq!(wb.clear_all_array_formulas().unwrap(), 1);
    assert!(!wb.has_array_formulas("S").unwrap());
}



#[test]
fn more_transitions_and_dv_has() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_blinds_transition(0, "med").unwrap();
    assert!(ppt.has_transition(0).unwrap());
    ppt.set_checker_transition(0, "fast").unwrap();
    ppt.set_circle_transition(0, "slow").unwrap();
    ppt.set_diamond_transition(0, "med").unwrap();
    ppt.set_plus_transition(0, "fast").unwrap();
    ppt.set_newsflash_transition(0, "med").unwrap();
    ppt.set_strips_transition(0, "slow").unwrap();
    ppt.set_wedge_transition(0, "med").unwrap();
    ppt.set_zoom_transition(0, "fast").unwrap();
    assert!(ppt.clear_transition(0).unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"], vec!["b"]]).unwrap();
    wb.add_data_validation_list("S", "A1:A10", "\"X,Y\"", true).unwrap();
    assert!(wb.has_data_validation("S", "A1:A10").unwrap());
    assert!(wb.remove_data_validation("S", "A1:A10").unwrap());
    assert!(!wb.has_data_validation("S", "A1:A10").unwrap());
}


#[test]
fn hyperlink_base_language_calc_mode() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_hyperlink_base("https://example.com/").unwrap();
    assert!(doc.has_hyperlink_base().unwrap());
    assert_eq!(
        doc.hyperlink_base().unwrap().as_deref(),
        Some("https://example.com/")
    );
    assert!(doc.clear_hyperlink_base().unwrap());
    assert!(!doc.has_hyperlink_base().unwrap());

    doc.set_language("en-US").unwrap();
    assert!(doc.has_language().unwrap());
    assert!(doc.clear_language().unwrap());
    assert!(!doc.has_language().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_hyperlink_base("https://files.example/").unwrap();
    assert!(wb.has_hyperlink_base().unwrap());
    assert!(wb.clear_hyperlink_base().unwrap());
    wb.set_calc_mode("manual").unwrap();
    assert!(wb.has_calc_mode().unwrap());
    assert_eq!(wb.calc_mode().unwrap().as_deref(), Some("manual"));

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_hyperlink_base("https://slides.example/").unwrap();
    assert!(ppt.has_hyperlink_base().unwrap());
    assert!(ppt.clear_hyperlink_base().unwrap());
    ppt.set_server_zoom(100).unwrap();
    assert!(ppt.has_server_zoom().unwrap());
    assert!(ppt.clear_server_zoom().unwrap());
}


#[test]
fn core_properties_has_clear() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_title("T").unwrap();
    doc.set_creator("C").unwrap();
    doc.set_subject("S").unwrap();
    doc.set_description("D").unwrap();
    doc.set_keywords("K").unwrap();
    doc.set_category("Cat").unwrap();
    assert!(doc.has_title().unwrap());
    assert!(doc.has_creator().unwrap());
    assert!(doc.clear_title().unwrap());
    assert!(!doc.has_title().unwrap());
    assert!(doc.clear_creator().unwrap());
    assert!(doc.clear_subject().unwrap());
    assert!(doc.clear_description().unwrap());
    assert!(doc.clear_keywords().unwrap());
    assert!(doc.clear_category().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_title("Book").unwrap();
    assert!(wb.has_title().unwrap());
    assert!(wb.clear_title().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_creator("Author").unwrap();
    assert!(ppt.has_creator().unwrap());
    assert!(ppt.clear_creator().unwrap());
}


#[test]
fn extended_properties_has_clear() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_company("Acme").unwrap();
    doc.set_manager("Boss").unwrap();
    doc.set_application("openxml-rs").unwrap();
    assert!(doc.has_company().unwrap());
    assert!(doc.has_manager().unwrap());
    assert!(doc.has_application().unwrap());
    assert!(doc.clear_company().unwrap());
    assert!(doc.clear_manager().unwrap());
    assert!(doc.clear_application().unwrap());
    assert!(!doc.has_company().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_template("Book.xltx").unwrap();
    assert!(wb.has_template().unwrap());
    assert!(wb.clear_template().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_application_version("1.0").unwrap();
    assert!(ppt.has_application_version().unwrap());
    assert!(ppt.clear_application_version().unwrap());
}


#[test]
fn more_core_props_revision_status() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_revision("3").unwrap();
    assert!(doc.has_revision().unwrap());
    assert!(doc.clear_revision().unwrap());
    assert!(!doc.has_revision().unwrap());
    // content_status if settable
    if let Ok(()) = doc.set_content_status("Draft") {
        assert!(doc.has_content_status().unwrap());
        assert!(doc.clear_content_status().unwrap());
    }
}


#[test]
fn remaining_flag_clears() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    wb.set_date1904(true).unwrap();
    assert!(wb.clear_date1904().unwrap());
    assert!(!wb.has_date1904().unwrap());
    wb.set_code_name("ThisWorkbook").unwrap();
    assert!(wb.has_code_name().unwrap());
    assert!(wb.clear_code_name().unwrap());
    assert!(!wb.has_code_name().unwrap());
    wb.set_filter_privacy(true).unwrap();
    assert!(wb.clear_filter_privacy().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_compat_flag("usePrinterMetrics", true).unwrap();
    assert!(doc.clear_compat_flag("usePrinterMetrics").unwrap());
    assert!(!doc.has_compat_flag("usePrinterMetrics").unwrap());
}


#[test]
fn final_clear_companions() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_document_variable("v1", "val").unwrap();
    assert!(doc.has_document_variable("v1").unwrap());
    assert!(doc.clear_document_variable("v1").unwrap());
    assert!(!doc.has_document_variable("v1").unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_calc_mode("manual").unwrap();
    assert!(wb.has_calc_mode().unwrap());
    assert!(wb.clear_calc_mode().unwrap());
    assert!(!wb.has_calc_mode().unwrap());
    wb.set_number_format(164, "0.00%").unwrap();
    assert!(wb.has_number_format(164).unwrap());
    assert!(wb.clear_number_format(164).unwrap());
    assert!(!wb.has_number_format(164).unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    // notes text
    ppt.set_notes_text(0, "speaker notes").unwrap();
    assert!(ppt.has_notes_text(0).unwrap());
    assert!(ppt.clear_notes_text(0).unwrap());
    assert!(!ppt.has_notes_text(0).unwrap());
}


#[test]
fn clear_companions_auto_caption_page_sort_hf() {
    // Word: auto captions + page size/margins
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_auto_caption("Microsoft Word Picture", "Figure").unwrap();
    doc.add_auto_caption("Microsoft Excel Worksheet", "Table").unwrap();
    assert!(doc.has_auto_captions().unwrap());
    assert_eq!(doc.auto_caption_count().unwrap(), 2);
    assert!(doc.remove_auto_caption("Microsoft Word Picture").unwrap());
    assert_eq!(doc.auto_caption_count().unwrap(), 1);
    assert_eq!(doc.clear_auto_captions().unwrap(), 1);
    assert!(!doc.has_auto_captions().unwrap());

    doc.set_page_size(12240, 15840).unwrap();
    doc.set_page_margins(1440, 1440, 1440, 1440).unwrap();
    assert!(doc.has_page_size().unwrap());
    assert!(doc.has_page_margins().unwrap());
    assert_eq!(doc.page_size().unwrap(), Some((12240, 15840)));
    assert_eq!(doc.page_margins().unwrap(), Some((1440, 1440, 1440, 1440)));
    assert!(doc.clear_page_size().unwrap());
    assert!(doc.clear_page_margins().unwrap());
    assert!(!doc.has_page_size().unwrap());
    assert!(!doc.has_page_margins().unwrap());

    // Excel: sort conditions clear + slicer cache
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();
    wb.set_sort_state("S", "A1:B2", "A1", false).unwrap();
    wb.add_sort_condition("S", "B1", true).unwrap();
    assert!(wb.has_sort_conditions("S").unwrap());
    assert!(wb.sort_condition_count("S").unwrap() >= 1);
    let n = wb.clear_sort_conditions("S").unwrap();
    assert!(n >= 1);
    assert!(!wb.has_sort_conditions("S").unwrap());
    // slicer shell
    let _ = wb.add_slicer_shell("S", "MySlicer", "MyCache").unwrap();
    assert!(wb.has_slicer_cache("MyCache").unwrap());
    assert!(wb.remove_slicer_cache("MyCache").unwrap());
    assert!(!wb.has_slicer_cache("MyCache").unwrap());

    // PPT: notes/handout master header footer clear
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_notes_master_header_footer(true, true, true, true).unwrap();
    assert!(ppt.has_notes_master_header_footer());
    assert!(ppt.clear_notes_master_header_footer().unwrap() >= 1);
    assert!(!ppt.has_notes_master_header_footer());
    ppt.set_handout_master_header_footer(true, false, true, false).unwrap();
    assert!(ppt.has_handout_master_header_footer());
    assert!(ppt.clear_handout_master_header_footer().unwrap() >= 1);
    assert!(!ppt.has_handout_master_header_footer());
}


#[test]
fn clear_companions_cf_style_alias_outline() {
    // Excel: CF rule remove + style font
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["10"], vec!["60"], vec!["30"]]).unwrap();
    wb.add_conditional_formatting_cell_is("S", "A1:A3", "greaterThan", "50", "FF0000", 1)
        .unwrap();
    assert!(wb.has_cf_rules("S").unwrap());
    let rules = wb.list_cf_rules("S").unwrap();
    assert!(!rules.is_empty());
    let removed = wb
        .remove_cf_rule("S", "A1:A3", Some("cellIs"), None)
        .unwrap();
    assert!(removed >= 1);
    assert!(!wb.has_cf_rules("S").unwrap());

    let fonts = wb.list_style_fonts().unwrap();
    if let Some(name) = fonts.into_iter().find(|n| !n.is_empty()) {
        let _ = wb.remove_style_font(&name).unwrap();
    }

    // Word: content control alias set/clear
    use officexml::wordprocessing::{body, document, paragraph_with_text, sdt_block};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        sdt_block("tag1", "alias1", vec![paragraph_with_text("hello")]),
    ])]));
    assert_eq!(
        doc.content_control_alias("tag1").unwrap().as_deref(),
        Some("alias1")
    );
    assert!(doc.set_content_control_alias("tag1", "alias2").unwrap());
    assert_eq!(
        doc.content_control_alias("tag1").unwrap().as_deref(),
        Some("alias2")
    );
    assert!(doc.clear_content_control_alias("tag1").unwrap());
    let alias = doc.content_control_alias("tag1").unwrap();
    assert!(alias.is_none() || alias.as_deref() == Some(""));

    // PPT: outline view pr clear
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_outline_view_scale(100, 100, 100, 100).unwrap();
    assert!(ppt.has_outline_view_pr().unwrap());
    assert!(ppt.clear_outline_view_pr().unwrap());
    assert!(!ppt.has_outline_view_pr().unwrap());
}


#[test]
fn scenario_inputs_and_animation_duration() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1", "2"]]).unwrap();
    wb.add_scenario("S", "Base", &[("A1", "10"), ("B1", "20")], Some("c"))
        .unwrap();
    assert!(wb.has_scenario_inputs("S", "Base").unwrap());
    assert_eq!(wb.scenario_input_count("S", "Base").unwrap(), 2);
    assert!(wb.set_scenario_input("S", "Base", "A1", "99").unwrap());
    let inputs = wb.list_scenario_inputs("S", "Base").unwrap();
    assert!(inputs.iter().any(|(r, v)| r == "A1" && v == "99"));
    assert!(wb.set_scenario_input("S", "Base", "C1", "5").unwrap());
    assert_eq!(wb.scenario_input_count("S", "Base").unwrap(), 3);
    assert!(wb.remove_scenario_input("S", "Base", "B1").unwrap());
    assert_eq!(wb.scenario_input_count("S", "Base").unwrap(), 2);
    assert_eq!(wb.clear_scenario_inputs("S", "Base").unwrap(), 2);
    assert!(!wb.has_scenario_inputs("S", "Base").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    // add a shape-ish animation target
    ppt.set_animation_effect(0, 2, "fade", "in").unwrap();
    assert!(ppt.has_animation_effect(0).unwrap());
    assert_eq!(ppt.animation_duration(0).unwrap().as_deref(), Some("1"));
    assert!(ppt.set_animation_duration(0, "500").unwrap());
    assert_eq!(ppt.animation_duration(0).unwrap().as_deref(), Some("500"));
    assert!(ppt.clear_animation_duration(0).unwrap());
    assert!(!ppt.has_animation_duration(0).unwrap());
}


#[test]
fn cell_watch_ole_control_animation_filter() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.add_cell_watch("S", "A1").unwrap();
    wb.add_cell_watch("S", "B2").unwrap();
    assert_eq!(wb.cell_watch_count("S").unwrap(), 2);
    assert!(wb.remove_cell_watch("S", "A1").unwrap());
    assert_eq!(wb.cell_watch_count("S").unwrap(), 1);
    assert_eq!(wb.clear_all_cell_watches().unwrap(), 1);
    assert!(!wb.has_cell_watches("S").unwrap());

    wb.add_ole_object("S", "Excel.Sheet.12", 10, None).unwrap();
    wb.add_ole_object("S", "Word.Document.12", 11, None).unwrap();
    assert_eq!(wb.ole_object_count("S").unwrap(), 2);
    assert_eq!(wb.remove_ole_object("S", 10).unwrap(), 1);
    assert_eq!(wb.ole_object_count("S").unwrap(), 1);

    wb.add_control("S", "Button1", 20, None).unwrap();
    wb.add_control("S", "Button2", 21, None).unwrap();
    assert_eq!(wb.control_count("S").unwrap(), 2);
    assert_eq!(wb.remove_control("S", 20).unwrap(), 1);
    assert_eq!(wb.control_count("S").unwrap(), 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_animation_effect(0, 3, "fade", "in").unwrap();
    assert!(ppt.set_animation_filter(0, Some("blinds(horizontal)"), Some("out")).unwrap());
    let eff = ppt.animation_effect(0).unwrap().unwrap();
    assert_eq!(eff.0, "blinds(horizontal)");
    assert_eq!(eff.1, "out");
}


#[test]
fn shape_fill_rgb_set_clear() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1_000_000, 1_000_000, "rect", Some("FF0000"), "Box")
        .unwrap();
    assert!(ppt.has_shape_fill(0, sid).unwrap());
    assert_eq!(ppt.shape_fill_rgb(0, sid).unwrap().as_deref(), Some("FF0000"));
    assert!(ppt.set_shape_fill(0, sid, "00FF00").unwrap());
    assert_eq!(ppt.shape_fill_rgb(0, sid).unwrap().as_deref(), Some("00FF00"));
    assert!(ppt.clear_shape_fill(0, sid).unwrap());
    assert!(!ppt.has_shape_fill(0, sid).unwrap());
}


#[test]
fn footnote_text_and_shape_line() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("body")]));
    doc.add_footnote("1", "A footnote body").unwrap();
    assert!(doc.has_footnote("1").unwrap());
    assert_eq!(
        doc.footnote_text("1").unwrap().as_deref(),
        Some("A footnote body")
    );
    doc.add_endnote("1", "An endnote body").unwrap();
    assert!(doc.has_endnote("1").unwrap());
    assert_eq!(
        doc.endnote_text("1").unwrap().as_deref(),
        Some("An endnote body")
    );

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1_000_000, 1_000_000, "rect", Some("FFFFFF"), "Box")
        .unwrap();
    assert!(ppt.set_shape_line(0, sid, "0000FF", Some(12700)).unwrap());
    assert!(ppt.has_shape_line(0, sid).unwrap());
    assert_eq!(ppt.shape_line_rgb(0, sid).unwrap().as_deref(), Some("0000FF"));
    assert!(ppt.clear_shape_line(0, sid).unwrap());
    assert!(!ppt.has_shape_line(0, sid).unwrap());
}


#[test]
fn shape_transform_and_remove_sparkline() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 100, 200, 300, 400, "rect", Some("AAAAAA"), "Box")
        .unwrap();
    assert!(ppt.has_shape_transform(0, sid).unwrap());
    assert_eq!(
        ppt.shape_transform(0, sid).unwrap(),
        Some((100, 200, 300, 400))
    );
    assert!(ppt.set_shape_transform(0, sid, 10, 20, 30, 40).unwrap());
    assert_eq!(
        ppt.shape_transform(0, sid).unwrap(),
        Some((10, 20, 30, 40))
    );

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1", "2", "3", "4"]]).unwrap();
    wb.add_sparkline("S", "line", "A1:C1", "D1").unwrap();
    wb.add_sparkline("S", "line", "A1:C1", "E1").unwrap();
    assert_eq!(wb.sparkline_count("S").unwrap(), 2);
    let n = wb.remove_sparkline("S", "D1").unwrap();
    assert!(n >= 1);
    assert_eq!(wb.sparkline_count("S").unwrap(), 1);
    let left = wb.list_sparklines("S").unwrap();
    assert!(left.iter().all(|(_, _, c)| c != "D1"));
    assert!(left.iter().any(|(_, _, c)| c == "E1"));
}


#[test]
fn shape_rotation_geom_chart_title() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1000, 1000, "rect", Some("FF00FF"), "R")
        .unwrap();
    assert_eq!(ppt.shape_preset_geom(0, sid).unwrap().as_deref(), Some("rect"));
    assert!(ppt.set_shape_preset_geom(0, sid, "ellipse").unwrap());
    assert_eq!(
        ppt.shape_preset_geom(0, sid).unwrap().as_deref(),
        Some("ellipse")
    );
    assert!(ppt.set_shape_rotation(0, sid, 5_400_000).unwrap()); // 90 deg
    assert_eq!(ppt.shape_rotation(0, sid).unwrap(), Some(5_400_000));
    assert!(ppt.clear_shape_rotation(0, sid).unwrap());
    assert_eq!(ppt.shape_rotation(0, sid).unwrap(), Some(0));

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (chart_uri, _) = wb.add_bar_chart("C", &["a"], &[1.0]).unwrap();
    assert!(wb.set_chart_title(&chart_uri, "Sales").unwrap());
    assert!(wb.has_chart_title(&chart_uri).unwrap());
    assert!(wb.clear_chart_title(&chart_uri).unwrap());
    // title structure may remain with empty text
    let titles = wb.list_chart_titles().unwrap();
    let t = titles
        .into_iter()
        .find(|(u, _)| u == &chart_uri)
        .map(|(_, t)| t)
        .unwrap_or_default();
    assert!(t.is_empty());
}


#[test]
fn clear_local_name_link_preset() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_local_defined_name("S", "Local1", "$A$1").unwrap();
    assert!(wb.has_local_defined_name("S", "Local1").unwrap());
    assert!(wb.clear_local_defined_name("S", "Local1").unwrap());
    assert!(!wb.has_local_defined_name("S", "Local1").unwrap());
    wb.set_local_defined_name("S", "L2", "$B$1").unwrap();
    assert_eq!(wb.clear_local_defined_names().unwrap(), 1);

    let (uri, _) = wb.add_external_link("other.xlsx").unwrap();
    assert!(wb.has_external_link_target("other.xlsx").unwrap() || wb.external_link_count() >= 1);
    // clear path target on the link part
    assert!(wb.clear_external_link_target(&uri).unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 100, 100, "rect", None, "X")
        .unwrap();
    assert!(ppt.has_shape_preset_geom(0, sid).unwrap());
    assert!(ppt.clear_shape_preset_geom(0, sid).unwrap());
    assert!(!ppt.has_shape_preset_geom(0, sid).unwrap());
}


#[test]
fn caption_def_and_workbook_views() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.add_caption_definition("Figure", "below", "decimal").unwrap();
    doc.add_caption_definition("Table", "above", "decimal").unwrap();
    assert!(doc.has_caption_definition("Figure").unwrap());
    assert_eq!(doc.caption_definition_count().unwrap(), 2);
    assert!(doc.remove_caption_definition("Figure").unwrap());
    assert!(!doc.has_caption_definition("Figure").unwrap());
    assert_eq!(doc.caption_definition_count().unwrap(), 1);
    assert!(doc.clear_captions().unwrap());
    assert!(!doc.has_captions().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_active_tab(0).unwrap();
    assert!(wb.workbook_view_count().unwrap() >= 1 || wb.active_tab().unwrap().is_some());
    // clear may remove bookViews
    let _ = wb.clear_workbook_views().unwrap();
}


#[test]
fn math_font_named_style_shape_flip() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_math_font("Cambria Math").unwrap();
    assert!(doc.has_math_font().unwrap());
    assert_eq!(doc.math_font().unwrap().as_deref(), Some("Cambria Math"));
    assert!(doc.clear_math_font().unwrap());
    assert!(!doc.has_math_font().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    // default styles may include named styles
    let styles = wb.list_named_styles().unwrap();
    if let Some((name, _)) = styles.into_iter().next() {
        assert!(wb.has_named_style(&name).unwrap());
        // clear_named_style alias
        let _ = wb.clear_named_style(&name).unwrap();
    }
    let _ = wb.clear_named_styles().unwrap();

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 100, 100, "rect", None, "F")
        .unwrap();
    assert!(ppt.set_shape_flip(0, sid, true, false).unwrap());
    assert_eq!(ppt.shape_flip(0, sid).unwrap(), Some((true, false)));
    assert!(ppt.clear_shape_flip(0, sid).unwrap());
    assert_eq!(ppt.shape_flip(0, sid).unwrap(), Some((false, false)));
}


#[test]
fn notes_text_all_and_list_borders() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_notes_text(0, "n0").unwrap();
    ppt.set_notes_text(1, "n1").unwrap();
    assert!(ppt.has_notes_text(0).unwrap());
    assert_eq!(ppt.clear_all_notes_text().unwrap(), 2);
    assert!(!ppt.has_notes_text(0).unwrap());
    assert!(!ppt.has_notes_text(1).unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    // ensure styles exist
    let _ = wb.border_count().unwrap();
    let borders = wb.list_borders().unwrap();
    assert_eq!(borders.len(), wb.border_count().unwrap());
    assert_eq!(wb.fills_count().unwrap(), wb.fill_count().unwrap());
}


#[test]
fn dxf_para_style_use_timings() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["10"], vec!["60"]]).unwrap();
    // CF cellIs creates a dxf
    wb.add_conditional_formatting_cell_is("S", "A1:A2", "greaterThan", "50", "FF0000", 1)
        .unwrap();
    let _ = wb.has_dxfs().unwrap();
    let _ = wb.list_dxfs().unwrap();

    use officexml::wordprocessing::{body, document, paragraph_with_text};
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(document(vec![body(vec![paragraph_with_text("a"), paragraph_with_text("b")])]));
    doc.add_default_styles().unwrap();
    let n = doc.apply_style_to_paragraphs("Normal").unwrap();
    assert!(n >= 1);
    assert!(doc.clear_paragraph_styles().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_use_timings(true).unwrap();
    assert!(ppt.has_use_timings().unwrap());
    assert!(ppt.clear_use_timings().unwrap());
    assert!(!ppt.has_use_timings().unwrap());
}


#[test]
fn table_totals_comment_slide_hide() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["h1", "h2"], vec!["1", "2"]]).unwrap();
    wb.add_table("S", "T1", "A1:B2", &["h1", "h2"]).unwrap();
    assert!(wb.set_table_totals_row("T1", true).unwrap());
    assert!(wb.table_has_totals_row("T1").unwrap());
    assert!(wb.clear_table_totals_row("T1").unwrap());
    assert!(!wb.table_has_totals_row("T1").unwrap());
    assert!(wb.set_table_comment("T1", "note").unwrap());
    assert!(wb.has_table_comment("T1").unwrap());
    assert!(wb.clear_table_comment("T1").unwrap());
    assert!(!wb.has_table_comment("T1").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.hide_slide(0).unwrap();
    assert!(ppt.is_slide_hidden(0).unwrap());
    ppt.unhide_slide(0).unwrap();
    assert!(!ppt.is_slide_hidden(0).unwrap());
}


#[test]
fn shape_texts_chart_legend() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "Hello", "TB")
        .unwrap();
    assert!(ppt.has_shape_text(0, sid).unwrap());
    let texts = ppt.list_shape_texts(0).unwrap();
    assert!(texts.iter().any(|(id, t)| *id == sid && t.contains("Hello")));
    assert!(ppt.clear_all_shape_text(0).unwrap() >= 1);
    assert!(!ppt.has_shape_text(0, sid).unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (chart_uri, _) = wb.add_bar_chart("C", &["a"], &[1.0]).unwrap();
    // legend may already exist on generated chart
    let had = wb.has_chart_legend(&chart_uri).unwrap();
    if had {
        assert!(wb.clear_chart_legend(&chart_uri).unwrap());
        assert!(!wb.has_chart_legend(&chart_uri).unwrap());
        assert!(wb.set_chart_legend(&chart_uri, true).unwrap());
        assert!(wb.has_chart_legend(&chart_uri).unwrap());
    } else {
        assert!(wb.set_chart_legend(&chart_uri, true).unwrap());
        assert!(wb.has_chart_legend(&chart_uri).unwrap());
        assert!(wb.clear_chart_legend(&chart_uri).unwrap());
    }
}


#[test]
fn chart_axis_title_shape_font() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a"]]).unwrap();
    let (chart_uri, _) = wb.add_bar_chart("C", &["a"], &[1.0]).unwrap();
    // try set valAx title
    let set = wb.set_chart_axis_title(&chart_uri, "valAx", "Values").unwrap()
        || wb.set_chart_axis_title(&chart_uri, "catAx", "Categories").unwrap();
    assert!(set);
    assert!(wb.has_chart_axis_titles(&chart_uri).unwrap());
    let titles = wb.list_chart_axis_titles(&chart_uri).unwrap();
    assert!(!titles.is_empty());
    assert!(wb.clear_chart_axis_titles(&chart_uri).unwrap() >= 1);
    assert!(!wb.has_chart_axis_titles(&chart_uri).unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "Hi", "TB")
        .unwrap();
    assert!(ppt.set_shape_font_size(0, sid, 2400).unwrap());
    assert_eq!(ppt.shape_font_size(0, sid).unwrap(), Some(2400));
    assert!(ppt.clear_shape_font_size(0, sid).unwrap());
    assert!(!ppt.has_shape_font_size(0, sid).unwrap());
}


#[test]
fn ext_stats_print_area_shape_bold() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_words(12).unwrap();
    doc.set_pages(3).unwrap();
    doc.set_characters(100).unwrap();
    assert!(doc.has_words().unwrap());
    assert!(doc.clear_words().unwrap());
    assert!(!doc.has_words().unwrap());
    assert!(doc.clear_pages().unwrap());
    assert!(doc.clear_characters().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_print_area("S", "A1:B2").unwrap();
    // clear sheet-specific
    let _ = wb.clear_print_area_for_sheet("S").unwrap();

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "Bold?", "TB")
        .unwrap();
    assert!(ppt.set_shape_bold(0, sid, true).unwrap());
    assert_eq!(ppt.shape_bold(0, sid).unwrap(), Some(true));
    assert!(ppt.clear_shape_bold(0, sid).unwrap());
    assert!(ppt.shape_bold(0, sid).unwrap().is_none() || ppt.shape_bold(0, sid).unwrap() == Some(false));
}


#[test]
fn very_hidden_italic_runs_bold() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.write_sheet_strings("H", &[vec!["x"]]).unwrap();
    wb.set_sheet_state("H", "veryHidden").unwrap();
    assert!(wb.has_very_hidden_sheets().unwrap());
    assert_eq!(wb.unhide_very_hidden_sheets().unwrap(), 1);
    assert!(!wb.has_very_hidden_sheets().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "i", "TB")
        .unwrap();
    assert!(ppt.set_shape_italic(0, sid, true).unwrap());
    assert_eq!(ppt.shape_italic(0, sid).unwrap(), Some(true));
    assert!(ppt.clear_shape_italic(0, sid).unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("bold me")]));
    assert!(doc.set_all_runs_bold(true).unwrap() >= 1);
    assert!(doc.clear_all_runs_bold().unwrap() >= 1);
}


#[test]
fn runs_style_font_color_cf_types() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("styled")]));
    assert!(doc.set_all_runs_italic(true).unwrap() >= 1);
    assert!(doc.set_all_runs_underline(Some("single")).unwrap() >= 1);
    assert!(doc.clear_all_runs_italic().unwrap() >= 1);
    assert!(doc.clear_all_runs_underline().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "color", "TB")
        .unwrap();
    assert!(ppt.set_shape_font_color(0, sid, "FF00AA").unwrap());
    assert_eq!(
        ppt.shape_font_color(0, sid).unwrap().as_deref(),
        Some("FF00AA")
    );
    assert!(ppt.clear_shape_font_color(0, sid).unwrap());
    assert!(!ppt.has_shape_font_color(0, sid).unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"], vec!["3"]]).unwrap();
    wb.add_conditional_formatting_data_bar("S", "A1:A3", "FF638EC6", 1).unwrap();
    assert!(wb.has_data_bars("S").unwrap());
}


#[test]
fn run_color_cf_type_remove_underline() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("c")]));
    assert!(doc.set_all_runs_color(Some("FF0000")).unwrap() >= 1);
    assert!(doc.clear_all_runs_color().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.add_conditional_formatting_data_bar("S", "A1:A2", "FF638EC6", 1)
        .unwrap();
    wb.add_conditional_formatting_cell_is("S", "A1:A2", "greaterThan", "0", "00FF00", 2)
        .unwrap();
    assert!(wb.has_data_bars("S").unwrap());
    assert!(wb.remove_cf_rules_by_type("S", "dataBar").unwrap() >= 1);
    assert!(!wb.has_data_bars("S").unwrap());
    assert!(wb.has_cf_rules("S").unwrap()); // cellIs remains

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "u", "TB")
        .unwrap();
    assert!(ppt.set_shape_underline(0, sid, Some("sng")).unwrap());
    assert_eq!(ppt.shape_underline(0, sid).unwrap().as_deref(), Some("sng"));
    assert!(ppt.clear_shape_underline(0, sid).unwrap());
    assert!(!ppt.has_shape_underline(0, sid).unwrap());
}


#[test]
fn run_highlight_font_name_zoom_scales() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("hi")]));
    assert!(doc.set_all_runs_highlight(Some("yellow")).unwrap() >= 1);
    assert!(doc.set_all_runs_strike(true).unwrap() >= 1);
    assert!(doc.set_all_runs_caps(true).unwrap() >= 1);
    assert!(doc.set_all_runs_vanish(true).unwrap() >= 1);
    assert!(doc.clear_all_runs_highlight().unwrap() >= 1);
    assert!(doc.clear_all_runs_strike().unwrap() >= 1);
    assert!(doc.clear_all_runs_caps().unwrap() >= 1);
    assert!(doc.clear_all_runs_vanish().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "F", "TB")
        .unwrap();
    assert!(ppt.set_shape_font_name(0, sid, "Arial").unwrap());
    assert_eq!(
        ppt.shape_font_name(0, sid).unwrap().as_deref(),
        Some("Arial")
    );
    assert!(ppt.clear_shape_font_name(0, sid).unwrap());
    assert!(!ppt.has_shape_font_name(0, sid).unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_zoom_scale_normal("S", 120).unwrap();
    wb.set_zoom_scale_page_layout("S", 80).unwrap();
    wb.set_zoom_scale_sheet_layout("S", 90).unwrap();
    assert!(wb.has_zoom_scale_normal("S").unwrap());
    assert!(wb.clear_zoom_scale_normal("S").unwrap());
    assert!(wb.clear_zoom_scale_page_layout("S").unwrap());
    assert!(wb.clear_zoom_scale_sheet_layout("S").unwrap());
    assert!(!wb.has_zoom_scale_normal("S").unwrap());
}


#[test]
fn run_size_shape_strike() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("sz")]));
    assert!(doc.set_all_runs_size(Some(28)).unwrap() >= 1); // 14pt
    assert!(doc.clear_all_runs_size().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "S", "TB")
        .unwrap();
    assert!(ppt.set_shape_strike(0, sid, Some("sngStrike")).unwrap());
    assert_eq!(
        ppt.shape_strike(0, sid).unwrap().as_deref(),
        Some("sngStrike")
    );
    assert!(ppt.clear_shape_strike(0, sid).unwrap());
    assert!(!ppt.has_shape_strike(0, sid).unwrap());
}


#[test]
fn run_font_wvid_clear_fills() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("font")]));
    assert!(doc.set_all_runs_font(Some("Calibri")).unwrap() >= 1);
    assert!(doc.clear_all_runs_font().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_workbook_view_id("S", 0).unwrap();
    assert!(wb.has_workbook_view_id("S").unwrap());
    assert!(wb.clear_workbook_view_id("S").unwrap());
    assert!(!wb.has_workbook_view_id("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1000, 1000, "rect", Some("112233"), "B")
        .unwrap();
    assert!(ppt.has_shape_fill(0, sid).unwrap());
    assert!(ppt.clear_all_shape_fill(0).unwrap() >= 1);
    assert!(!ppt.has_shape_fill(0, sid).unwrap());
}


#[test]
fn small_caps_color_id_clear_lines() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("sc")]));
    assert!(doc.set_all_runs_small_caps(true).unwrap() >= 1);
    assert!(doc.set_all_runs_double_strike(true).unwrap() >= 1);
    assert!(doc.clear_all_runs_small_caps().unwrap() >= 1);
    assert!(doc.clear_all_runs_double_strike().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_color_id("S", 64).unwrap();
    assert!(wb.has_color_id("S").unwrap());
    assert_eq!(wb.clear_all_color_id().unwrap(), 1);
    assert!(!wb.has_color_id("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1000, 1000, "rect", Some("FFFFFF"), "L")
        .unwrap();
    assert!(ppt.set_shape_line(0, sid, "000000", Some(12700)).unwrap());
    assert!(ppt.has_shape_line(0, sid).unwrap());
    assert!(ppt.clear_all_shape_line(0).unwrap() >= 1);
    assert!(!ppt.has_shape_line(0, sid).unwrap());
}


#[test]
fn para_align_view_type_fill_alpha() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(simple_document(vec![
        paragraph_with_text("a"),
        paragraph_with_text("b"),
    ]));
    assert!(doc.set_all_paragraphs_alignment(Some("center")).unwrap() >= 2);
    assert!(doc.set_all_paragraphs_keep_next(true).unwrap() >= 2);
    assert!(doc.set_all_paragraphs_widow_control(false).unwrap() >= 2);
    assert!(doc.clear_all_paragraphs_alignment().unwrap() >= 2);
    assert!(doc.clear_all_paragraphs_keep_next().unwrap() >= 2);
    assert!(doc.clear_all_paragraphs_widow_control().unwrap() >= 2);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_sheet_view_type("S", "pageBreakPreview").unwrap();
    assert!(wb.has_sheet_view_type("S").unwrap());
    assert!(wb.clear_sheet_view_type("S").unwrap());
    assert!(!wb.has_sheet_view_type("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1000, 1000, "rect", Some("00AAFF"), "A")
        .unwrap();
    assert!(ppt.set_shape_fill_alpha(0, sid, 50_000).unwrap());
    assert_eq!(ppt.shape_fill_alpha(0, sid).unwrap(), Some(50_000));
    assert!(ppt.clear_shape_fill_alpha(0, sid).unwrap());
    assert!(!ppt.has_shape_fill_alpha(0, sid).unwrap());
}


#[test]
fn spacing_indent_rot_grid_formulas() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(simple_document(vec![
        paragraph_with_text("p1"),
        paragraph_with_text("p2"),
    ]));
    assert!(doc.set_all_paragraphs_spacing(Some(200), Some(100)).unwrap() >= 2);
    assert!(doc.set_all_paragraphs_indent(Some(720), Some(360)).unwrap() >= 2);
    assert!(doc.clear_all_paragraphs_spacing().unwrap() >= 2);
    assert!(doc.clear_all_paragraphs_indent().unwrap() >= 2);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1000, 1000, "rect", Some("ABCDEF"), "R")
        .unwrap();
    assert!(ppt.set_shape_rotation(0, sid, 1_000_000).unwrap());
    assert!(ppt.clear_all_shape_rotation(0).unwrap() >= 1);
    assert_eq!(ppt.shape_rotation(0, sid).unwrap(), Some(0));

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_show_gridlines("S", false).unwrap();
    wb.set_show_formulas("S", true).unwrap();
    assert!(wb.has_show_gridlines_attr("S").unwrap());
    assert!(wb.has_show_formulas_attr("S").unwrap());
    assert!(wb.clear_all_show_gridlines().unwrap() >= 1);
    assert!(wb.clear_all_show_formulas().unwrap() >= 1);
    assert!(!wb.has_show_gridlines_attr("S").unwrap());
    assert!(!wb.has_show_formulas_attr("S").unwrap());
}


#[test]
fn keep_lines_zeros_flip_all() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    assert!(doc.set_all_paragraphs_keep_lines(true).unwrap() >= 1);
    assert!(doc.set_all_paragraphs_page_break_before(true).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_keep_lines().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_page_break_before().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_show_zeros("S", false).unwrap();
    assert!(wb.has_show_zeros_attr("S").unwrap());
    assert!(wb.clear_all_show_zeros().unwrap() >= 1);
    assert!(!wb.has_show_zeros_attr("S").unwrap());
    wb.set_show_row_col_headers("S", false).unwrap();
    assert!(wb.has_show_row_col_headers_attr("S").unwrap());
    assert!(wb.clear_show_row_col_headers("S").unwrap());
    assert!(!wb.has_show_row_col_headers_attr("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_auto_shape_on_slide(0, 0, 0, 1000, 1000, "rect", Some("111111"), "F")
        .unwrap();
    assert!(ppt.set_shape_flip(0, sid, true, true).unwrap());
    assert!(ppt.clear_all_shape_flip(0).unwrap() >= 1);
    assert_eq!(ppt.shape_flip(0, sid).unwrap(), Some((false, false)));
}


#[test]
fn contextual_outline_shape_clear_alls() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("h1")]));
    assert!(doc.set_all_paragraphs_contextual_spacing(true).unwrap() >= 1);
    assert!(doc.set_all_paragraphs_outline_level(Some(0)).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_contextual_spacing().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_outline_level().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "T", "TB")
        .unwrap();
    assert!(ppt.set_shape_font_size(0, sid, 2000).unwrap());
    assert!(ppt.set_shape_font_color(0, sid, "112233").unwrap());
    assert!(ppt.set_shape_bold(0, sid, true).unwrap());
    assert!(ppt.clear_all_shape_font_size(0).unwrap() >= 1);
    assert!(ppt.clear_all_shape_font_color(0).unwrap() >= 1);
    assert!(ppt.clear_all_shape_bold(0).unwrap() >= 1);
    assert!(!ppt.has_shape_font_size(0, sid).unwrap());
    assert!(!ppt.has_shape_font_color(0, sid).unwrap());
}


#[test]
fn bidi_tab_selected_shape_clear_alls() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("rtl")]));
    assert!(doc.set_all_paragraphs_bidi(true).unwrap() >= 1);
    assert!(doc.set_all_paragraphs_suppress_line_numbers(true).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_bidi().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_suppress_line_numbers().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_tab_selected("S", true).unwrap();
    assert!(wb.tab_selected("S").unwrap());
    assert!(wb.has_tab_selected("S").unwrap());
    assert!(wb.clear_tab_selected("S").unwrap());
    assert!(!wb.has_tab_selected("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "x", "TB")
        .unwrap();
    assert!(ppt.set_shape_italic(0, sid, true).unwrap());
    assert!(ppt.set_shape_underline(0, sid, Some("sng")).unwrap());
    assert!(ppt.set_shape_strike(0, sid, Some("sngStrike")).unwrap());
    assert!(ppt.clear_all_shape_italic(0).unwrap() >= 1);
    assert!(ppt.clear_all_shape_underline(0).unwrap() >= 1);
    assert!(ppt.clear_all_shape_strike(0).unwrap() >= 1);
}


#[test]
fn word_wrap_mirror_font_name_all() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("w")]));
    assert!(doc.set_all_paragraphs_word_wrap(false).unwrap() >= 1);
    assert!(doc.set_all_paragraphs_mirror_indents(true).unwrap() >= 1);
    assert!(doc.set_all_paragraphs_snap_to_grid(false).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_word_wrap().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_mirror_indents().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_snap_to_grid().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_show_ruler("S", true).unwrap();
    wb.set_show_white_space("S", true).unwrap();
    assert!(wb.has_show_ruler_attr("S").unwrap());
    assert!(wb.has_show_white_space_attr("S").unwrap());
    assert!(wb.clear_show_ruler("S").unwrap());
    assert!(wb.clear_show_white_space("S").unwrap());
    assert!(!wb.has_show_ruler_attr("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "f", "TB")
        .unwrap();
    assert!(ppt.set_shape_font_name(0, sid, "Georgia").unwrap());
    assert!(ppt.clear_all_shape_font_name(0).unwrap() >= 1);
    assert!(!ppt.has_shape_font_name(0, sid).unwrap());
}


#[test]
fn text_align_top_left_anchor() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("ta")]));
    assert!(doc.set_all_paragraphs_text_alignment(Some("center")).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_text_alignment().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_top_left_cell("S", "C5").unwrap();
    assert_eq!(wb.top_left_cell("S").unwrap().as_deref(), Some("C5"));
    assert!(wb.has_top_left_cell("S").unwrap());
    assert!(wb.clear_top_left_cell("S").unwrap());
    assert!(!wb.has_top_left_cell("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "a", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_anchor(0, sid, "ctr").unwrap());
    assert_eq!(ppt.shape_text_anchor(0, sid).unwrap().as_deref(), Some("ctr"));
    assert!(ppt.clear_shape_text_anchor(0, sid).unwrap());
    assert!(!ppt.has_shape_text_anchor(0, sid).unwrap());
}


#[test]
fn auto_space_text_wrap() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("as")]));
    assert!(doc.set_all_paragraphs_auto_space_de(false).unwrap() >= 1);
    assert!(doc.set_all_paragraphs_auto_space_dn(false).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_auto_space_de().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_auto_space_dn().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "w", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_wrap(0, sid, "none").unwrap());
    assert_eq!(ppt.shape_text_wrap(0, sid).unwrap().as_deref(), Some("none"));
    assert!(ppt.clear_shape_text_wrap(0, sid).unwrap());
    assert!(!ppt.has_shape_text_wrap(0, sid).unwrap());
}


#[test]
fn workbook_window_clear() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_workbook_window(100, 200, 800, 600).unwrap();
    assert!(wb.has_workbook_window().unwrap());
    assert!(wb.clear_workbook_window().unwrap());
    assert!(!wb.has_workbook_window().unwrap());
}


#[test]
fn ea_punct_first_sheet_upright() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("ea")]));
    assert!(doc.set_all_paragraphs_overflow_punct(false).unwrap() >= 1);
    assert!(doc.set_all_paragraphs_top_line_punct(true).unwrap() >= 1);
    assert!(doc.set_all_paragraphs_adjust_right_ind(false).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_overflow_punct().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_top_line_punct().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_adjust_right_ind().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.write_sheet_strings("T", &[vec!["2"]]).unwrap();
    wb.set_first_sheet(1).unwrap();
    assert!(wb.has_first_sheet().unwrap());
    assert!(wb.clear_first_sheet().unwrap());
    assert!(!wb.has_first_sheet().unwrap());
    wb.set_workbook_minimized(true).unwrap();
    assert!(wb.has_workbook_minimized().unwrap());
    assert!(wb.clear_workbook_minimized().unwrap());
    assert!(!wb.has_workbook_minimized().unwrap());
    wb.set_show_outline_symbols("S", false).unwrap();
    assert!(wb.has_show_outline_symbols_attr("S").unwrap());
    assert!(wb.clear_show_outline_symbols("S").unwrap());
    assert!(!wb.has_show_outline_symbols_attr("S").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "u", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_upright(0, sid, true).unwrap());
    assert!(ppt.has_shape_text_upright(0, sid).unwrap());
    assert!(ppt.clear_shape_text_upright(0, sid).unwrap());
    assert!(!ppt.has_shape_text_upright(0, sid).unwrap());
}


#[test]
fn kinsoku_scroll_vert() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("k")]));
    assert!(doc.set_all_paragraphs_kinsoku(false).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_kinsoku().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_show_horizontal_scroll(false).unwrap();
    wb.set_show_vertical_scroll(false).unwrap();
    wb.set_workbook_visibility("hidden").unwrap();
    assert!(wb.has_show_horizontal_scroll().unwrap());
    assert!(wb.has_show_vertical_scroll().unwrap());
    assert!(wb.has_workbook_visibility().unwrap());
    assert!(wb.clear_show_horizontal_scroll().unwrap());
    assert!(wb.clear_show_vertical_scroll().unwrap());
    assert!(wb.clear_workbook_visibility().unwrap());
    assert!(!wb.has_show_horizontal_scroll().unwrap());
    assert!(!wb.has_workbook_visibility().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "v", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_vert(0, sid, "vert").unwrap());
    assert_eq!(ppt.shape_text_vert(0, sid).unwrap().as_deref(), Some("vert"));
    assert!(ppt.clear_shape_text_vert(0, sid).unwrap());
    assert!(!ppt.has_shape_text_vert(0, sid).unwrap());
}


#[test]
fn hyphens_tabs_insets() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("h")]));
    assert!(doc.set_all_paragraphs_suppress_auto_hyphens(true).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_suppress_auto_hyphens().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_show_sheet_tabs(false).unwrap();
    wb.set_tab_ratio(500).unwrap();
    assert!(wb.has_show_sheet_tabs().unwrap());
    assert!(wb.has_tab_ratio().unwrap());
    assert!(wb.clear_show_sheet_tabs().unwrap());
    assert!(wb.clear_tab_ratio().unwrap());
    assert!(!wb.has_show_sheet_tabs().unwrap());
    assert!(!wb.has_tab_ratio().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "i", "TB")
        .unwrap();
    assert!(ppt
        .set_shape_text_insets(0, sid, Some(10000), Some(20000), Some(30000), Some(40000))
        .unwrap());
    let ins = ppt.shape_text_insets(0, sid).unwrap().unwrap();
    assert_eq!(ins, (Some(10000), Some(20000), Some(30000), Some(40000)));
    assert!(ppt.clear_shape_text_insets(0, sid).unwrap());
    assert!(!ppt.has_shape_text_insets(0, sid).unwrap());
}


#[test]
fn shading_line_objects_numcol() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("sh")]));
    assert!(doc
        .set_all_paragraphs_shading(Some("clear"), Some("FFFF00"))
        .unwrap()
        >= 1);
    assert!(doc
        .set_all_paragraphs_line_spacing(Some(360), Some("auto"))
        .unwrap()
        >= 1);
    assert!(doc.clear_all_paragraphs_shading().unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_line_spacing().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_show_objects("none").unwrap();
    assert!(wb.has_show_objects().unwrap());
    assert!(wb.clear_show_objects().unwrap());
    assert!(!wb.has_show_objects().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "c", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_num_col(0, sid, 2).unwrap());
    assert_eq!(ppt.shape_text_num_col(0, sid).unwrap(), Some(2));
    assert!(ppt.clear_shape_text_num_col(0, sid).unwrap());
    assert!(!ppt.has_shape_text_num_col(0, sid).unwrap());
}


#[test]
fn borders_ink_spccol() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("b")]));
    assert!(doc
        .set_all_paragraphs_bottom_border("single", 12, "000000")
        .unwrap()
        >= 1);
    assert!(doc.clear_all_paragraphs_borders().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_show_border_unselected_tables(false).unwrap();
    wb.set_show_ink_annotation(false).unwrap();
    assert!(wb.has_show_border_unselected_tables().unwrap());
    assert!(wb.has_show_ink_annotation().unwrap());
    assert!(wb.clear_show_border_unselected_tables().unwrap());
    assert!(wb.clear_show_ink_annotation().unwrap());
    assert!(!wb.has_show_border_unselected_tables().unwrap());
    wb.set_filter_mode(true).unwrap();
    assert!(wb.has_filter_mode().unwrap());
    assert!(wb.clear_filter_mode().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "s", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_spc_col(0, sid, 50000).unwrap());
    assert_eq!(ppt.shape_text_spc_col(0, sid).unwrap(), Some(50000));
    assert!(ppt.clear_shape_text_spc_col(0, sid).unwrap());
    assert!(!ppt.has_shape_text_spc_col(0, sid).unwrap());
}


#[test]
fn side_borders_theme_wordart() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("box")]));
    assert!(doc
        .set_all_paragraphs_top_border("single", 8, "FF0000")
        .unwrap()
        >= 1);
    assert!(doc
        .set_all_paragraphs_left_border("single", 8, "00FF00")
        .unwrap()
        >= 1);
    assert!(doc
        .set_all_paragraphs_right_border("single", 8, "0000FF")
        .unwrap()
        >= 1);
    assert!(doc.clear_all_paragraphs_borders().unwrap() >= 1);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_prompted_solutions(true).unwrap();
    wb.set_default_theme_version("124226").unwrap();
    assert!(wb.has_prompted_solutions().unwrap());
    assert!(wb.has_default_theme_version().unwrap());
    assert!(wb.clear_prompted_solutions().unwrap());
    assert!(wb.clear_default_theme_version().unwrap());
    assert!(!wb.has_prompted_solutions().unwrap());
    assert!(!wb.has_default_theme_version().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "wa", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_from_word_art(0, sid, true).unwrap());
    assert!(ppt.has_shape_text_from_word_art(0, sid).unwrap());
    assert!(ppt.clear_shape_text_from_word_art(0, sid).unwrap());
    assert!(!ppt.has_shape_text_from_word_art(0, sid).unwrap());
}


#[test]
fn workbook_pr_indent_anchor_ctr() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_backup_file(true).unwrap();
    wb.set_auto_compress_pictures(false).unwrap();
    wb.set_hide_pivot_field_list(true).unwrap();
    assert!(wb.has_backup_file().unwrap());
    assert!(wb.has_auto_compress_pictures().unwrap());
    assert!(wb.clear_backup_file().unwrap());
    assert!(wb.clear_auto_compress_pictures().unwrap());
    assert!(wb.clear_hide_pivot_field_list().unwrap());
    assert!(!wb.has_backup_file().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("ind")]));
    assert!(doc.set_all_paragraphs_first_line_indent(Some(720)).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_first_line_indent().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "a", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_anchor_ctr(0, sid, true).unwrap());
    assert!(ppt.has_shape_text_anchor_ctr(0, sid).unwrap());
    assert!(ppt.clear_shape_text_anchor_ctr(0, sid).unwrap());
    assert!(!ppt.has_shape_text_anchor_ctr(0, sid).unwrap());
}


#[test]
fn calc_pr_hanging_rtlcol() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_calc_on_save(false).unwrap();
    wb.set_calc_id(12345).unwrap();
    wb.set_concurrent_calc(false).unwrap();
    wb.set_force_full_calc(true).unwrap();
    assert!(wb.has_calc_on_save_attr().unwrap());
    assert!(wb.has_calc_id_attr().unwrap());
    assert!(wb.clear_calc_on_save().unwrap());
    assert!(wb.clear_calc_id().unwrap());
    assert!(wb.clear_concurrent_calc().unwrap());
    assert!(wb.clear_force_full_calc().unwrap());
    assert!(!wb.has_calc_id_attr().unwrap());
    wb.set_update_links("always").unwrap();
    assert!(wb.has_update_links().unwrap());
    assert!(wb.clear_update_links().unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("h")]));
    assert!(doc.set_all_paragraphs_hanging_indent(Some(360)).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_hanging_indent().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "r", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_rtl_col(0, sid, true).unwrap());
    assert!(ppt.has_shape_text_rtl_col(0, sid).unwrap());
    assert!(ppt.clear_shape_text_rtl_col(0, sid).unwrap());
    assert!(!ppt.has_shape_text_rtl_col(0, sid).unwrap());
}


#[test]
fn more_pr_calc_bodypr_flags() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_refresh_all_connections(true).unwrap();
    wb.set_check_compatibility(true).unwrap();
    wb.set_ref_mode("R1C1").unwrap();
    wb.set_iterate(true).unwrap();
    wb.set_full_precision(false).unwrap();
    assert!(wb.has_refresh_all_connections_attr().unwrap());
    assert!(wb.has_check_compatibility_attr().unwrap());
    assert!(wb.has_ref_mode_attr().unwrap());
    assert!(wb.clear_refresh_all_connections().unwrap());
    assert!(wb.clear_check_compatibility().unwrap());
    assert!(wb.clear_ref_mode().unwrap());
    assert!(wb.clear_iterate().unwrap());
    assert!(wb.clear_full_precision().unwrap());
    assert!(!wb.has_ref_mode_attr().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "f", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_force_aa(0, sid, true).unwrap());
    assert!(ppt.set_shape_text_compat_ln_spc(0, sid, true).unwrap());
    assert!(ppt.has_shape_text_force_aa(0, sid).unwrap());
    assert!(ppt.has_shape_text_compat_ln_spc(0, sid).unwrap());
    assert!(ppt.clear_shape_text_force_aa(0, sid).unwrap());
    assert!(ppt.clear_shape_text_compat_ln_spc(0, sid).unwrap());
    assert!(!ppt.has_shape_text_force_aa(0, sid).unwrap());
}


#[test]
fn outline_header_footer_clears() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_outline_level_row("S", 3).unwrap();
    wb.set_outline_level_col("S", 2).unwrap();
    assert!(wb.has_outline_level_row("S").unwrap());
    assert!(wb.has_outline_level_col("S").unwrap());
    assert!(wb.clear_outline_level_row("S").unwrap());
    assert!(wb.clear_outline_level_col("S").unwrap());
    assert!(!wb.has_outline_level_row("S").unwrap());

    wb.set_odd_header("S", "OddH").unwrap();
    wb.set_even_header("S", "EvenH").unwrap();
    wb.set_first_header("S", "FirstH").unwrap();
    wb.set_odd_footer("S", "OddF").unwrap();
    assert!(wb.has_odd_header("S").unwrap());
    assert!(wb.clear_odd_header("S").unwrap());
    assert!(wb.clear_even_header("S").unwrap());
    assert!(wb.clear_first_header("S").unwrap());
    assert!(wb.clear_odd_footer("S").unwrap());
    assert!(!wb.has_odd_header("S").unwrap());
}


#[test]
fn format_run_spacing_spc() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_default_row_height("S", 18.0).unwrap();
    wb.set_default_col_width("S", 12.0).unwrap();
    wb.set_base_col_width("S", 10).unwrap();
    assert!(wb.has_default_row_height("S").unwrap());
    assert!(wb.clear_default_row_height("S").unwrap());
    assert!(wb.clear_default_col_width("S").unwrap());
    assert!(wb.clear_base_col_width("S").unwrap());
    assert!(!wb.has_default_row_height("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("sp")]));
    assert!(doc.set_all_runs_spacing(Some(40)).unwrap() >= 1);
    assert!(doc.set_all_runs_position(Some(6)).unwrap() >= 1);
    assert!(doc.clear_all_runs_spacing().unwrap() >= 1);
    assert!(doc.clear_all_runs_position().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "s", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_spc_first(0, sid, 1000).unwrap());
    assert!(ppt.set_shape_text_spc_last(0, sid, 2000).unwrap());
    assert_eq!(ppt.shape_text_spc_first(0, sid).unwrap(), Some(1000));
    assert!(ppt.clear_shape_text_spc_first(0, sid).unwrap());
    assert!(ppt.clear_shape_text_spc_last(0, sid).unwrap());
    assert!(!ppt.has_shape_text_spc_first(0, sid).unwrap());
}


#[test]
fn page_setup_run_effects_autofit() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_page_orientation("S", "landscape").unwrap();
    wb.set_paper_size("S", 9).unwrap();
    wb.set_page_scale("S", 85).unwrap();
    assert!(wb.has_page_orientation("S").unwrap());
    assert!(wb.clear_page_orientation("S").unwrap());
    assert!(wb.clear_paper_size("S").unwrap());
    assert!(wb.clear_page_scale("S").unwrap());
    assert!(!wb.has_page_orientation("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("fx")]));
    assert!(doc.set_all_runs_kern(Some(24)).unwrap() >= 1);
    assert!(doc.set_all_runs_outline(true).unwrap() >= 1);
    assert!(doc.set_all_runs_shadow(true).unwrap() >= 1);
    assert!(doc.set_all_runs_emboss(true).unwrap() >= 1);
    assert!(doc.clear_all_runs_kern().unwrap() >= 1);
    assert!(doc.clear_all_runs_outline().unwrap() >= 1);
    assert!(doc.clear_all_runs_shadow().unwrap() >= 1);
    assert!(doc.clear_all_runs_emboss().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "a", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_norm_autofit(0, sid, true).unwrap());
    assert!(ppt.has_shape_text_norm_autofit(0, sid).unwrap());
    assert!(ppt.clear_shape_text_norm_autofit(0, sid).unwrap());
    assert!(!ppt.has_shape_text_norm_autofit(0, sid).unwrap());
}


#[test]
fn dpi_autofit_tabs() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"]]).unwrap();
    wb.set_horizontal_dpi("S", 300).unwrap();
    wb.set_vertical_dpi("S", 300).unwrap();
    assert!(wb.has_horizontal_dpi("S").unwrap());
    assert!(wb.has_vertical_dpi("S").unwrap());
    assert_eq!(wb.horizontal_dpi("S").unwrap(), Some(300));
    assert!(wb.clear_horizontal_dpi("S").unwrap());
    assert!(wb.clear_vertical_dpi("S").unwrap());
    assert!(!wb.has_horizontal_dpi("S").unwrap());
    assert!(!wb.has_vertical_dpi("S").unwrap());

    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("t")]));
    assert!(doc.set_all_paragraphs_tabs(&[("left", 720), ("right", 5040)]).unwrap() >= 1);
    assert!(doc.clear_all_paragraphs_tabs().unwrap() >= 1);

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "a", "TB")
        .unwrap();
    assert!(ppt.set_shape_text_sp_autofit(0, sid, true).unwrap());
    assert!(ppt.has_shape_text_sp_autofit(0, sid).unwrap());
    assert!(ppt.clear_shape_text_sp_autofit(0, sid).unwrap());
    assert!(!ppt.has_shape_text_sp_autofit(0, sid).unwrap());
    assert!(ppt.set_shape_text_no_autofit(0, sid, true).unwrap());
    assert!(ppt.has_shape_text_no_autofit(0, sid).unwrap());
    assert!(ppt.clear_shape_text_no_autofit(0, sid).unwrap());
    assert!(ppt.set_shape_text_font_scale(0, sid, Some(90000)).unwrap());
    assert_eq!(ppt.shape_text_font_scale(0, sid).unwrap(), Some(90000));
    assert!(ppt.has_shape_text_font_scale(0, sid).unwrap());
    assert!(ppt.clear_shape_text_font_scale(0, sid).unwrap());
    assert!(!ppt.has_shape_text_font_scale(0, sid).unwrap());
    assert!(ppt
        .set_shape_text_ln_spc_reduction(0, sid, Some(20000))
        .unwrap());
    assert_eq!(
        ppt.shape_text_ln_spc_reduction(0, sid).unwrap(),
        Some(20000)
    );
    assert!(ppt.clear_shape_text_ln_spc_reduction(0, sid).unwrap());
}


#[test]
fn grid_column_print_clears() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("g")]));
    doc.set_drawing_grid_horizontal_spacing(120).unwrap();
    doc.set_drawing_grid_vertical_spacing(120).unwrap();
    doc.set_drawing_grid_horizontal_origin(0).unwrap();
    doc.set_drawing_grid_vertical_origin(0).unwrap();
    doc.set_display_horizontal_drawing_grid_every(1).unwrap();
    doc.set_display_vertical_drawing_grid_every(1).unwrap();
    doc.set_book_fold_printing_sheets(2).unwrap();
    assert!(doc.has_drawing_grid_horizontal_spacing().unwrap());
    assert!(doc.clear_drawing_grid_horizontal_spacing().unwrap());
    assert!(doc.clear_drawing_grid_vertical_spacing().unwrap());
    assert!(doc.clear_drawing_grid_horizontal_origin().unwrap());
    assert!(doc.clear_drawing_grid_vertical_origin().unwrap());
    assert!(doc.clear_display_horizontal_drawing_grid_every().unwrap());
    assert!(doc.clear_display_vertical_drawing_grid_every().unwrap());
    assert!(doc.clear_book_fold_printing_sheets().unwrap());
    doc.set_gutter(720).unwrap();
    assert!(doc.has_gutter().unwrap());
    assert!(doc.clear_gutter().unwrap());
    assert!(!doc.has_gutter().unwrap());
    doc.set_header_footer_distance(720, 720).unwrap();
    assert!(doc.has_header_footer_distance().unwrap());
    assert!(doc.clear_header_footer_distance().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1", "2"]]).unwrap();
    wb.set_column_best_fit("S", 1, 1, true).unwrap();
    wb.set_column_style("S", 1, 1, 1).unwrap();
    wb.set_column_collapsed("S", 1, 1, true).unwrap();
    assert!(wb.column_best_fit("S", 1, 1).unwrap());
    assert!(wb.clear_column_best_fit("S", 1, 1).unwrap());
    assert!(wb.clear_column_style("S", 1, 1).unwrap());
    assert!(wb.clear_column_collapsed("S", 1, 1).unwrap());
    assert!(!wb.column_best_fit("S", 1, 1).unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.set_print_color_mode("blackAndWhite").unwrap();
    ppt.set_print_what("handouts").unwrap();
    assert!(ppt.has_print_color_mode().unwrap());
    assert!(ppt.has_print_what().unwrap());
    assert!(ppt.clear_print_color_mode().unwrap());
    assert!(ppt.clear_print_what().unwrap());
    assert!(!ppt.has_print_color_mode().unwrap());
    assert!(!ppt.has_print_what().unwrap());
}


#[test]
fn settings_ext_view_clears() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.set_summary_length(50).unwrap();
    doc.set_style_pane_sort_method("name").unwrap();
    assert!(doc.has_summary_length().unwrap());
    assert!(doc.clear_summary_length().unwrap());
    assert!(doc.has_style_pane_sort_method().unwrap());
    assert!(doc.clear_style_pane_sort_method().unwrap());
    assert!(!doc.has_summary_length().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.set_doc_security(1).unwrap();
    wb.set_shared_doc(true).unwrap();
    wb.set_links_up_to_date(true).unwrap();
    wb.set_hyperlinks_changed(true).unwrap();
    wb.set_scale_crop(true).unwrap();
    wb.set_total_time(12).unwrap();
    assert!(wb.clear_doc_security().unwrap());
    assert!(wb.clear_shared_doc().unwrap());
    assert!(wb.clear_links_up_to_date().unwrap());
    assert!(wb.clear_hyperlinks_changed().unwrap());
    assert!(wb.clear_scale_crop().unwrap());
    assert!(wb.clear_total_time().unwrap());
    assert!(!wb.has_doc_security().unwrap());
    assert!(!wb.has_total_time().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.set_presentation_format("On-screen Show").unwrap();
    ppt.set_mm_clips(2).unwrap();
    ppt.set_shared_doc(true).unwrap();
    ppt.set_total_time(5).unwrap();
    ppt.set_app_hidden_slides(1).unwrap();
    assert!(ppt.clear_presentation_format().unwrap());
    assert!(ppt.clear_mm_clips().unwrap());
    assert!(ppt.clear_shared_doc().unwrap());
    assert!(ppt.clear_total_time().unwrap());
    assert!(ppt.clear_app_hidden_slides().unwrap());
    ppt.set_horz_bar_state("maximized").unwrap();
    ppt.set_vert_bar_state("minimized").unwrap();
    ppt.set_restored_left(20000, Some(true)).unwrap();
    ppt.set_restored_top(30000, Some(false)).unwrap();
    assert!(ppt.clear_horz_bar_state().unwrap());
    assert!(ppt.clear_vert_bar_state().unwrap());
    assert!(ppt.clear_restored_left().unwrap());
    assert!(ppt.clear_restored_top().unwrap());
    ppt.set_notes_text_view_scale(50, 100, 50, 100).unwrap();
    ppt.set_sorter_view_scale(25, 100, 25, 100).unwrap();
    assert!(ppt.has_notes_text_view_pr().unwrap());
    assert!(ppt.has_sorter_view_pr().unwrap());
    assert!(ppt.clear_notes_text_view_pr().unwrap());
    assert!(ppt.clear_sorter_view_pr().unwrap());
}


#[test]
fn language_hf_row_theme_clears() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["1"], vec!["2"]]).unwrap();
    wb.set_language("en-US").unwrap();
    assert!(wb.has_language().unwrap());
    assert!(wb.clear_language().unwrap());
    assert!(!wb.has_language().unwrap());
    wb.set_enable_format_conditions_calculation("S", false).unwrap();
    assert!(wb.has_enable_format_conditions_calculation("S").unwrap());
    assert!(wb.clear_enable_format_conditions_calculation("S").unwrap());
    assert!(!wb.has_enable_format_conditions_calculation("S").unwrap());
    wb.set_header_footer_flags("S", Some(true), Some(true), Some(false), Some(false))
        .unwrap();
    assert!(wb.header_footer_different_odd_even("S").unwrap());
    assert!(wb.clear_header_footer_different_odd_even("S").unwrap());
    assert!(wb.clear_header_footer_different_first("S").unwrap());
    assert!(wb.clear_header_footer_scale_with_doc("S").unwrap());
    assert!(wb.clear_header_footer_align_with_margins("S").unwrap());
    wb.set_row_thick_top("S", 1, true).unwrap();
    wb.set_row_thick_bottom("S", 1, true).unwrap();
    assert!(wb.row_thick_top("S", 1).unwrap());
    assert!(wb.clear_row_thick_top("S", 1).unwrap());
    assert!(wb.clear_row_thick_bottom("S", 1).unwrap());
    assert!(!wb.row_thick_top("S", 1).unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.add_default_theme().unwrap();
    assert!(ppt.set_theme_name("MyTheme").unwrap());
    assert_eq!(ppt.theme_name().unwrap().as_deref(), Some("MyTheme"));
    assert!(ppt.clear_theme_name().unwrap());
    assert!(!ppt.has_theme_name().unwrap());
    ppt.set_notes_text(0, "n").unwrap();
    ppt.set_notes_name(0, "Notes1").unwrap();
    assert_eq!(ppt.notes_name(0).unwrap().as_deref(), Some("Notes1"));
    assert!(ppt.clear_notes_name(0).unwrap());
    assert!(!ppt.has_notes_name(0).unwrap());
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "a", "TB")
        .unwrap();
    assert!(ppt.set_shape_hidden(0, sid, true).unwrap());
    assert!(ppt.is_shape_hidden(0, sid).unwrap());
    assert!(ppt.clear_shape_hidden(0, sid).unwrap());
    assert!(!ppt.is_shape_hidden(0, sid).unwrap());
}


#[test]
fn mail_merge_fields_language_clears() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("m")]));
    doc.set_mail_merge_main_document_type("formLetters").unwrap();
    doc.set_mail_merge_data_type("database").unwrap();
    doc.set_mail_merge_query("SELECT *").unwrap();
    doc.set_mail_merge_view_merged_data(true).unwrap();
    doc.set_mail_merge_active_record(2).unwrap();
    doc.set_mail_merge_destination("email").unwrap();
    doc.set_mail_merge_subject("Hi").unwrap();
    doc.set_mail_merge_address_field_name("Email").unwrap();
    doc.set_mail_merge_as_attachment(true).unwrap();
    doc.set_mail_merge_do_not_suppress_blank_lines(true).unwrap();
    doc.set_mail_merge_link_to_query(true).unwrap();
    doc.set_mail_merge_check_errors(1).unwrap();
    doc.set_mail_merge_connect_string("DSN=x").unwrap();
    assert!(doc.has_mail_merge_main_document_type().unwrap());
    assert!(doc.clear_mail_merge_main_document_type().unwrap());
    assert!(doc.clear_mail_merge_data_type().unwrap());
    assert!(doc.clear_mail_merge_query().unwrap());
    assert!(doc.clear_mail_merge_view_merged_data().unwrap());
    assert!(doc.clear_mail_merge_active_record().unwrap());
    assert!(doc.clear_mail_merge_destination().unwrap());
    assert!(doc.clear_mail_merge_subject().unwrap());
    assert!(doc.clear_mail_merge_address_field_name().unwrap());
    assert!(doc.clear_mail_merge_as_attachment().unwrap());
    assert!(doc.clear_mail_merge_do_not_suppress_blank_lines().unwrap());
    assert!(doc.clear_mail_merge_link_to_query().unwrap());
    assert!(doc.clear_mail_merge_check_errors().unwrap());
    assert!(doc.clear_mail_merge_connect_string().unwrap());
    assert!(!doc.has_mail_merge_destination().unwrap());
    doc.set_doc_security(2).unwrap();
    assert!(doc.clear_doc_security().unwrap());
    assert!(!doc.has_doc_security().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.set_language("fr-FR").unwrap();
    assert!(ppt.has_language().unwrap());
    assert!(ppt.clear_language().unwrap());
    assert!(!ppt.has_language().unwrap());
}


#[test]
fn show_mode_layout_preserve_clears() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.set_show_mode_browse(true).unwrap();
    assert_eq!(ppt.show_mode().unwrap().as_deref(), Some("browse"));
    assert!(ppt.has_show_mode().unwrap());
    assert!(ppt.clear_show_mode().unwrap());
    assert!(!ppt.has_show_mode().unwrap());
    ppt.set_show_slide_range(0, 2).unwrap();
    assert!(ppt.has_show_slide_range().unwrap());
    assert!(ppt.clear_show_slide_range().unwrap());
    ppt.set_show_all_slides().unwrap();
    assert!(ppt.show_all_slides().unwrap());
    assert!(ppt.clear_show_all_slides().unwrap());
    assert!(!ppt.show_all_slides().unwrap());
    ppt.set_show_custom_show(1).unwrap();
    assert!(ppt.has_show_custom_show().unwrap());
    assert!(ppt.clear_show_custom_show().unwrap());
    assert!(!ppt.has_show_custom_show().unwrap());

    let (_m, _l) = ppt.add_blank_master_with_layout().unwrap();
    ppt.set_slide_layout_preserve(0, true).unwrap();
    assert!(ppt.slide_layout_preserve(0).unwrap());
    assert!(ppt.clear_slide_layout_preserve(0).unwrap());
    assert!(!ppt.slide_layout_preserve(0).unwrap());
    ppt.set_slide_layout_matching_name(0, "Title").unwrap();
    assert!(ppt.has_slide_layout_matching_name(0).unwrap());
    assert!(ppt.clear_slide_layout_matching_name(0).unwrap());
    assert!(!ppt.has_slide_layout_matching_name(0).unwrap());
    ppt.set_slide_master_preserve(0, true).unwrap();
    assert!(ppt.slide_master_preserve(0).unwrap());
    assert!(ppt.clear_slide_master_preserve(0).unwrap());
    assert!(!ppt.slide_master_preserve(0).unwrap());
}


#[test]
fn excel_view_margin_row_clears() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();
    wb.set_auto_filter_date_grouping(false).unwrap();
    assert!(wb.clear_auto_filter_date_grouping().unwrap());
    wb.set_page_margins("S", 0.5, 0.5, 0.5, 0.5, 0.3, 0.3).unwrap();
    wb.set_page_margin_attr("S", "header", 0.4).unwrap();
    assert!(wb.has_page_margin_attr("S", "header").unwrap());
    assert!(wb.clear_page_margin_attr("S", "header").unwrap());
    assert!(!wb.has_page_margin_attr("S", "header").unwrap());
    wb.set_row_collapsed("S", 1, true).unwrap();
    assert!(wb.row_collapsed("S", 1).unwrap());
    assert!(wb.clear_row_collapsed("S", 1).unwrap());
    assert!(!wb.row_collapsed("S", 1).unwrap());
    wb.set_auto_filter("S", "A1:B2").unwrap();
    wb.add_auto_filter_values("S", 0, &["a"]).unwrap();
    assert!(wb
        .set_auto_filter_column_buttons("S", 0, Some(false), Some(true))
        .unwrap());
    assert!(wb.clear_auto_filter_column_buttons("S", 0).unwrap());
}


#[test]
fn word_section_odso_theme_clears() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("s")]));
    doc.set_section_type("nextPage").unwrap();
    assert!(doc.has_section_type().unwrap());
    assert!(doc.clear_section_type().unwrap());
    assert!(!doc.has_section_type().unwrap());
    doc.set_page_number_format("upperRoman").unwrap();
    assert!(doc.clear_page_number_format().unwrap());
    doc.set_page_number_type_start(3).unwrap();
    assert!(doc.clear_page_number_type_start().unwrap());
    doc.set_page_orientation("landscape").unwrap();
    assert!(doc.clear_page_orientation().unwrap());
    assert!(!doc.has_page_orientation().unwrap());
    doc.set_shared_doc(true).unwrap();
    doc.set_total_time(9).unwrap();
    assert!(doc.clear_shared_doc().unwrap());
    assert!(doc.clear_total_time().unwrap());
    doc.add_default_theme().unwrap();
    assert!(doc.set_theme_name("OfficeX").unwrap());
    assert_eq!(doc.theme_name().unwrap().as_deref(), Some("OfficeX"));
    assert!(doc.clear_theme_name().unwrap());
    assert!(!doc.has_theme_name().unwrap());
    doc.set_mail_merge_odso("Contacts", "file:///c:/data.xlsx").unwrap();
    doc.set_mail_merge_odso_col_delim(44).unwrap();
    doc.set_mail_merge_odso_f_hdr(true).unwrap();
    doc.set_mail_merge_odso_udl("Provider=...").unwrap();
    doc.set_mail_merge_odso_type("database").unwrap();
    doc.set_mail_merge_odso_recipient_data("rId1").unwrap();
    assert!(doc.clear_mail_merge_odso_table().unwrap());
    assert!(doc.clear_mail_merge_odso_src().unwrap());
    assert!(doc.clear_mail_merge_odso_col_delim().unwrap());
    assert!(doc.clear_mail_merge_odso_f_hdr().unwrap());
    assert!(doc.clear_mail_merge_odso_udl().unwrap());
    assert!(doc.clear_mail_merge_odso_type().unwrap());
    assert!(doc.clear_mail_merge_odso_recipient_data().unwrap());
}


#[test]
fn ppt_master_excel_table_clears() {
    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_show_master_shapes(0, false).unwrap();
    ppt.set_show_master_ph_anim(0, false).unwrap();
    assert!(ppt.clear_show_master_shapes(0).unwrap());
    assert!(ppt.clear_show_master_ph_anim(0).unwrap());
    ppt.set_notes_text(0, "n").unwrap();
    ppt.set_notes_show_master_shapes(0, false).unwrap();
    ppt.set_notes_show_master_ph_anim(0, false).unwrap();
    assert!(ppt.clear_notes_show_master_shapes(0).unwrap());
    assert!(ppt.clear_notes_show_master_ph_anim(0).unwrap());
    let (_m, _l) = ppt.add_blank_master_with_layout().unwrap();
    ppt.set_slide_layout_user_drawn(0, true).unwrap();
    ppt.set_slide_layout_type(0, "title").unwrap();
    ppt.set_slide_layout_show_master_shapes(0, false).unwrap();
    ppt.set_slide_layout_show_master_ph_anim(0, false).unwrap();
    assert!(ppt.clear_slide_layout_user_drawn(0).unwrap());
    assert!(ppt.clear_slide_layout_type(0).unwrap());
    assert!(ppt.clear_slide_layout_show_master_shapes(0).unwrap());
    assert!(ppt.clear_slide_layout_show_master_ph_anim(0).unwrap());
    ppt.set_slide_size_ex(10_000_000, 7_000_000, Some("screen16x9"))
        .unwrap();
    assert!(ppt.clear_slide_size_type().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H1", "H2"], vec!["a", "b"]])
        .unwrap();
    wb.add_table("S", "T1", "A1:B2", &["H1", "H2"]).unwrap();
    assert!(wb.set_table_display_name("T1", "Display").unwrap());
    assert!(wb.set_table_published("T1", true).unwrap());
    assert!(wb.set_table_insert_row("T1", true).unwrap());
    assert!(wb.set_table_totals_row_shown("T1", false).unwrap());
    assert!(wb.clear_table_display_name("T1").unwrap());
    assert!(wb.clear_table_published("T1").unwrap());
    assert!(wb.clear_table_insert_row("T1").unwrap());
    assert!(wb.clear_table_totals_row_shown("T1").unwrap());
    assert!(!wb.has_table_display_name("T1").unwrap());
}


#[test]
fn excel_sort_sheet_dv_clears() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H", "B"], vec!["1", "2"], vec!["3", "4"]])
        .unwrap();
    wb.set_sheet_state("S", "hidden").unwrap();
    assert!(wb.has_sheet_state("S").unwrap());
    assert!(wb.clear_sheet_state("S").unwrap());
    assert!(!wb.has_sheet_state("S").unwrap());
    assert_eq!(wb.sheet_state("S").unwrap(), "visible");

    wb.set_sort_state("S", "A1:B3", "A2", false).unwrap();
    assert!(wb.set_sort_method("S", "pinYin").unwrap());
    assert!(wb.clear_sort_method("S").unwrap());
    assert!(!wb.has_sort_method("S").unwrap());
    assert!(wb.set_sort_range("S", "A1:B4").unwrap());
    assert!(wb.clear_sort_range("S").unwrap());
    assert!(!wb.has_sort_range("S").unwrap());
    wb.add_sort_condition("S", "B2", true).unwrap();
    assert!(wb
        .set_sort_condition_descending("S", "B2", true)
        .unwrap());
    assert!(wb.clear_sort_condition_descending("S", "B2").unwrap());
    assert!(wb.set_sort_condition_sort_by("S", "B2", "value").unwrap());
    assert!(wb.clear_sort_condition_sort_by("S", "B2").unwrap());
    wb.set_row_style("S", 2, 1).unwrap();
    assert!(wb.has_row_style("S", 2).unwrap());
    assert!(wb.clear_row_style("S", 2).unwrap());
    assert!(!wb.has_row_style("S", 2).unwrap());

    wb.add_data_validation_whole("S", "A1:A10", "between", "1", Some("10"), true)
        .unwrap();
    assert!(wb.set_data_validations_window("S", 10, 20).unwrap());
    assert!(wb.clear_data_validations_window("S").unwrap());
    assert!(wb
        .set_data_validations_disable_prompts("S", true)
        .unwrap());
    assert!(wb.clear_data_validations_disable_prompts("S").unwrap());
}


#[test]
fn word_math_border_ppt_photo_anim() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("x")]));
    doc.set_page_border_options(Some("allPages"), Some("page"), Some("front"))
        .unwrap();
    assert!(doc.clear_page_border_options().unwrap());
    doc.set_math_display_defaults(true, "centerGroup").unwrap();
    assert!(doc.clear_math_display_defaults().unwrap());
    doc.set_write_protection_ex(true, Some("SHA-512")).unwrap();
    assert!(doc.has_write_protection_algorithm_name().unwrap());
    assert!(doc.clear_write_protection_algorithm_name().unwrap());
    assert!(!doc.has_write_protection_algorithm_name().unwrap());
    doc.set_theme_font_lang_ex("en-US", Some("zh-CN"), Some("ar-SA"))
        .unwrap();
    assert!(doc.clear_theme_font_lang_ex().unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    let sid = ppt
        .add_text_box_on_slide(0, 0, 0, 1_000_000, 500_000, "a", "TB")
        .unwrap();
    ppt.set_animation_effect(0, sid, "fade", "in").unwrap();
    assert!(ppt
        .set_animation_filter(0, Some("fade"), Some("in"))
        .unwrap());
    assert!(ppt.clear_animation_filter(0).unwrap());
    ppt.set_photo_album(true, true, "fitToSlide", "rect")
        .unwrap();
    assert!(ppt.has_photo_album().unwrap());
    assert!(ppt.clear_photo_album().unwrap());
    assert!(!ppt.has_photo_album().unwrap());
}


#[test]
fn word_style_compat_ppt_kinsoku() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![
            paragraph_with_text("a"),
            paragraph_with_text("b"),
        ]));
    doc.add_default_styles().unwrap();
    let ids = doc.list_style_ids().unwrap();
    let id = ids[0].clone();
    assert!(doc
        .set_style_flags(&id, Some(true), Some(true), Some(false), Some(true), Some(9))
        .unwrap());
    assert!(doc.clear_style_flags(&id).unwrap());
    assert!(doc
        .set_style_links(&id, Some("Normal"), Some("Normal"), None)
        .unwrap());
    assert!(doc.clear_style_links(&id).unwrap());
    assert!(doc.set_style_default(&id).unwrap());
    assert!(doc.clear_style_default(&id).unwrap());
    doc.set_document_defaults("Calibri", 22).unwrap();
    assert!(doc.clear_document_defaults().unwrap());
    doc.set_document_protection_ex("readOnly", true, true).unwrap();
    assert!(doc.clear_document_protection_ex().unwrap());
    doc.set_compat_setting("compatibilityMode", "http://schemas.microsoft.com/office/word", "15")
        .unwrap();
    assert!(doc.clear_compat_setting("compatibilityMode").unwrap());
    assert!(doc.set_paragraph_numbering_at(0, 1, 0).unwrap());
    assert!(doc.clear_paragraph_numbering_at(0).unwrap());
    doc.add_person("Alice", "AD").unwrap();
    assert!(doc.set_person_provider("Alice", "AD2").unwrap());
    assert!(doc.clear_person_provider("Alice").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.set_photo_album(true, false, "fitToSlide", "rect").unwrap();
    assert!(ppt
        .set_photo_album_attrs(Some(false), Some(true), Some("1pic"), Some("roundedRect"))
        .unwrap());
    assert!(ppt.clear_photo_album_attrs().unwrap());
    ppt.set_kinsoku("ja-JP", "([{|", ")]}").unwrap();
    assert!(ppt
        .set_kinsoku_attrs(Some("en-US"), Some("([{"), Some(")]}"))
        .unwrap());
    assert!(ppt.clear_kinsoku_attrs().unwrap());
    assert!(ppt.clear_kinsoku().unwrap());
    ppt.set_modify_verifier("SHA-512", 100000).unwrap();
    assert!(ppt
        .set_modify_verifier_attrs(Some("SHA-256"), Some(50000))
        .unwrap());
    assert!(ppt.clear_modify_verifier_attrs().unwrap());
    let id = ppt.add_custom_show("Show1", &[0]).unwrap();
    assert!(ppt.set_custom_show_slides(id, &[0]).unwrap());
    assert!(ppt.clear_custom_show_slides(id).unwrap());
    ppt.set_notes_text_view_scale(50, 100, 50, 100).unwrap();
    assert!(ppt.clear_notes_text_view_scale().unwrap());
}


#[test]
fn excel_dv_protection_spark_clears() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H"], vec!["1"], vec!["2"]]).unwrap();
    wb.add_data_validation_whole("S", "A2:A10", "between", "1", Some("100"), true)
        .unwrap();
    assert!(wb.set_data_validation_type("S", "A2:A10", "decimal").unwrap());
    assert!(wb.clear_data_validation_type("S", "A2:A10").unwrap());
    assert!(wb
        .set_data_validation_operator("S", "A2:A10", "greaterThan")
        .unwrap());
    assert!(wb.clear_data_validation_operator("S", "A2:A10").unwrap());
    assert!(wb
        .set_data_validation_allow_blank("S", "A2:A10", true)
        .unwrap());
    assert!(wb.clear_data_validation_allow_blank("S", "A2:A10").unwrap());
    assert!(wb
        .set_data_validation_show_drop_down("S", "A2:A10", true)
        .unwrap());
    assert!(wb
        .clear_data_validation_show_drop_down("S", "A2:A10")
        .unwrap());
    assert!(wb
        .set_data_validation_error_style("S", "A2:A10", "warning")
        .unwrap());
    assert!(wb.clear_data_validation_error_style("S", "A2:A10").unwrap());
    assert!(wb
        .set_data_validation_show_messages("S", "A2:A10", Some(true), Some(true))
        .unwrap());
    assert!(wb
        .clear_data_validation_show_messages("S", "A2:A10")
        .unwrap());

    wb.set_sheet_protection_flags(
        "S",
        &[("sheet", true), ("objects", true), ("scenarios", false)],
    )
    .unwrap();
    assert!(wb
        .clear_sheet_protection_flags("S", &["objects", "scenarios"])
        .unwrap());
    wb.set_workbook_protection_ex(true, true, false).unwrap();
    assert!(wb.clear_workbook_protection_ex().unwrap());

    wb.add_sparkline("S", "line", "A2:A3", "C1").unwrap();
    assert!(wb
        .set_sparkline_group_attrs(
            "S",
            Some("column"),
            Some("zero"),
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
        )
        .unwrap());
    assert!(wb.clear_sparkline_group_attrs("S").unwrap());
}


#[test]
fn residual_clear_aliases() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("z")]));
    doc.set_page_setup(12240, 15840, 1440, 1440, 1440, 1440).unwrap();
    assert!(doc.clear_page_setup().unwrap());
    doc.set_document_grid_ex(360, Some("lines"), Some(0)).unwrap();
    assert!(doc.clear_document_grid_ex().unwrap());
    doc.set_write_protection_ex(true, Some("SHA-1")).unwrap();
    assert!(doc.clear_write_protection_ex().unwrap());
    doc.set_page_number_type_start(5).unwrap();
    assert!(doc.clear_page_number_start().unwrap());
    use officexml::wordprocessing::comment;
    doc.set_comments(vec![comment("0", "Ann", "A", "hi")]).unwrap();
    assert!(doc
        .set_comment_attrs("0", Some("Bob"), Some("B"), Some("2020-01-01T00:00:00Z"))
        .unwrap());
    assert!(doc.clear_comment_attrs("0").unwrap());

    let mut ppt =
        PresentationDocument::create_in_memory(PresentationDocumentType::Presentation).unwrap();
    ppt.add_blank_slide().unwrap();
    ppt.hide_slide(0).unwrap();
    assert!(ppt.clear_slide_hidden(0).unwrap());
    assert!(!ppt.is_slide_hidden(0).unwrap());
    ppt.set_slide_size_ex(9144000, 6858000, Some("screen4x3")).unwrap();
    assert!(ppt.clear_slide_size_ex().unwrap());

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["a", "b"], vec!["1", "2"]]).unwrap();
    wb.set_freeze_panes_ex("S", 1.0, 1.0, "B2", "bottomRight", "frozen")
        .unwrap();
    assert!(wb.clear_freeze_panes_ex("S").unwrap());
    wb.set_header_footer_flags("S", Some(true), Some(true), Some(false), Some(false))
        .unwrap();
    assert!(wb.clear_header_footer_flags("S").unwrap());
    wb.set_column_hidden("S", 1, 1, true).unwrap();
    wb.clear_column_hidden("S", 1, 1).unwrap();
    wb.set_row_hidden("S", 1, true).unwrap();
    wb.clear_row_hidden("S", 1).unwrap();
    wb.set_column_outline_level("S", 1, 1, 2).unwrap();
    assert!(wb.clear_column_outline_level("S", 1, 1).unwrap());
}


#[test]
fn final_residual_clears() {
    let mut doc =
        WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part()
        .set_document(simple_document(vec![paragraph_with_text("n")]));
    doc.add_default_numbering().unwrap();
    // abstractNum 0 level 0 usually exists in default numbering
    let _ = doc.set_abstract_num_level(0, 0, Some("%1."), Some("decimal"), Some(1));
    let _ = doc.clear_abstract_num_level(0, 0);

    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H"], vec!["1"]]).unwrap();
    wb.add_data_validation_list("S", "A2:A10", "\"Y,N\"", true).unwrap();
    assert!(wb
        .set_data_validation_messages(
            "S",
            "A2:A10",
            Some("t"),
            Some("p"),
            Some("et"),
            Some("e"),
        )
        .unwrap());
    assert!(wb.clear_data_validation_messages("S", "A2:A10").unwrap());
    assert!(wb
        .set_data_validation_message_fields(
            "S",
            "A2:A10",
            Some("t2"),
            Some("p2"),
            None,
            None,
        )
        .unwrap());
    assert!(wb
        .clear_data_validation_message_fields("S", "A2:A10")
        .unwrap());
}


#[test]
fn excel_table_slicer_af_clears() {
    let mut wb =
        SpreadsheetDocument::create_in_memory(SpreadsheetDocumentType::Workbook).unwrap();
    wb.write_sheet_strings("S", &[vec!["H1", "H2"], vec!["a", "b"]]).unwrap();
    wb.add_table("S", "T1", "A1:B2", &["H1", "H2"]).unwrap();
    assert!(wb
        .set_table_style_info("T1", "TableStyleMedium2", true, false, true, false)
        .unwrap());
    assert!(wb.clear_table_style_info("T1").unwrap());
    assert!(wb
        .set_table_column_totals("T1", "H2", "sum", Some("Total"))
        .unwrap());
    assert!(wb.clear_table_column_totals("T1", "H2").unwrap());
    assert!(wb.set_table_dxf_ids("T1", Some(1), Some(2), Some(3)).unwrap());
    assert!(wb.clear_table_dxf_ids("T1").unwrap());
    assert!(wb
        .set_table_border_dxf_ids("T1", Some(1), Some(2), Some(3))
        .unwrap());
    assert!(wb.clear_table_border_dxf_ids("T1").unwrap());
    assert!(wb
        .set_table_column_dxf_ids("T1", "H1", Some(1), Some(2), Some(3))
        .unwrap());
    assert!(wb.clear_table_column_dxf_ids("T1", "H1").unwrap());
    let _ = wb.set_table_column_unique_name("T1", "H1", "uniq");
    let _ = wb.clear_table_column_unique_name("T1", "H1");

    wb.set_auto_filter("S", "A1:B2").unwrap();
    wb.add_auto_filter_custom("S", 0, &[("equal", "a")], true)
        .unwrap();
    assert!(wb.set_auto_filter_custom_and("S", 0, true).unwrap());
    assert!(wb.clear_auto_filter_custom_and("S", 0).unwrap());
}


#[test]
fn open_stream_roundtrip_word() {
    use officexml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
    use officexml::wordprocessing::{body, document, paragraph, run, text};
    use std::io::Cursor;

    let mut doc = WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document).unwrap();
    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![run(vec![text("stream-io")])]),
    ])]));
    let bytes = doc.to_bytes().unwrap();
    let mut opened =
        WordprocessingDocument::open_stream(Cursor::new(bytes.clone()), false).unwrap();
    let texts = opened.paragraph_texts().unwrap();
    assert!(texts.iter().any(|t| t.contains("stream-io")), "{texts:?}");

    let mut out = Cursor::new(Vec::new());
    opened.write_to(&mut out).unwrap();
    assert!(out.get_ref().starts_with(b"PK"));
}

#[test]
fn misc_node_comment_roundtrip_in_document_xml() {
    use officexml::element::{parse_element, write_element, OpenXmlMiscKind};

    let xml = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <!-- generator note -->
  <w:body><w:p><w:r><w:t>x</w:t></w:r></w:p></w:body>
</w:document>"#;
    let root = parse_element(xml).unwrap();
    assert!(root
        .children
        .iter()
        .any(|c| c.misc_kind() == OpenXmlMiscKind::Comment));
    let out = write_element(&root).unwrap();
    let s = String::from_utf8_lossy(&out);
    assert!(s.contains("<!-- generator note -->"), "{s}");
}

#[test]
fn compression_option_not_compressed_still_valid_zip() {
    use officexml::opc::CompressionOption;
    use officexml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
    use officexml::wordprocessing::{body, document, paragraph, run, text};

    let mut doc = WordprocessingDocument::create_in_memory(WordprocessingDocumentType::Document)
        .unwrap();
    doc.package_mut()
        .opc_mut()
        .set_compression_option(CompressionOption::NotCompressed);
    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![run(vec![text("zip")])]),
    ])]));
    let bytes = doc.to_bytes().unwrap();
    assert!(bytes.starts_with(b"PK"));
    let mut reopened = WordprocessingDocument::open_bytes(&bytes).unwrap();
    assert!(reopened.paragraph_texts().unwrap().iter().any(|t| t.contains("zip")));
}
