//! Corpus-level smoke and structure checks for SVG -> native DrawingML conversion.
//!
//! The fixtures intentionally exercise the static SVG vocabulary documented by MDN. The
//! converter's output is native `p:sp` geometry, so text and unsupported browser-only content
//! must not leak a `p:txBody` or cause the conversion to panic.

use officexml::element::{parse_element, write_element, OpenXmlElement};
use officexml::presentation::svg_to_shapes::svg_to_shapes;
use std::fs;
use std::path::{Path, PathBuf};

const TARGET_CX: i64 = 12_192_000;
const TARGET_CY: i64 = 6_858_000;
const START_ID: u32 = 2;

fn fixture_paths() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/svg");
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read SVG fixture directory {}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("svg"))
        .collect();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "SVG conformance corpus is empty: {}",
        dir.display()
    );
    paths
}

fn fixture_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unnamed>")
}

fn child<'a>(element: &'a OpenXmlElement, name: &str) -> &'a OpenXmlElement {
    element
        .children
        .iter()
        .find(|candidate| candidate.local_name == name)
        .unwrap_or_else(|| panic!("{} lacks child {name}", element.qualified_name()))
}

fn attribute_i64(element: &OpenXmlElement, name: &str) -> i64 {
    element
        .get_attribute(name)
        .unwrap_or_else(|| panic!("{} lacks @{name}", element.qualified_name()))
        .parse::<i64>()
        .unwrap_or_else(|e| {
            panic!(
                "{} @{name} is not an integer: {e}",
                element.qualified_name()
            )
        })
}

#[test]
fn svg_fixture_corpus_converts_to_native_shapes() {
    let paths = fixture_paths();
    let mut total_shapes = 0;
    let mut total_fonts = 0;

    for path in paths {
        let name = fixture_name(&path);
        let svg = fs::read(&path).unwrap_or_else(|e| panic!("cannot read {name}: {e}"));

        // catch_unwind makes the corpus's no-panic guarantee explicit instead of allowing one
        // malformed fixture to turn into an opaque test panic.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            svg_to_shapes(&svg, TARGET_CX, TARGET_CY, START_ID)
        }));
        let conversion = result
            .unwrap_or_else(|_| panic!("SVG fixture {name} panicked during conversion"))
            .unwrap_or_else(|e| panic!("SVG fixture {name} failed conversion: {e}"));

        assert!(
            conversion.view_width.is_finite() && conversion.view_width > 0.0,
            "{name}: invalid view width"
        );
        assert!(
            conversion.view_height.is_finite() && conversion.view_height > 0.0,
            "{name}: invalid view height"
        );
        assert!(
            !conversion.shapes.is_empty(),
            "{name}: conversion emitted no native shapes"
        );
        total_shapes += conversion.shapes.len();
        total_fonts += conversion.used_fonts.len();

        let mut ids = Vec::with_capacity(conversion.shapes.len());
        for (index, shape) in conversion.shapes.iter().enumerate() {
            assert_eq!(
                shape.prefix, "p",
                "{name}: shape {index} is not PresentationML"
            );
            assert_eq!(shape.local_name, "sp", "{name}: shape {index} is not p:sp");
            assert!(
                shape.namespace_uri.contains("presentationml"),
                "{name}: shape {index} has wrong namespace"
            );
            assert!(
                !shape
                    .descendants()
                    .any(|element| element.local_name == "txBody"),
                "{name}: shape {index} contains p:txBody"
            );

            let nv_sp_pr = child(shape, "nvSpPr");
            let c_nv_pr = child(nv_sp_pr, "cNvPr");
            let id = attribute_i64(c_nv_pr, "id");
            assert!(
                id >= START_ID as i64,
                "{name}: shape {index} has invalid id {id}"
            );
            ids.push(id);

            let sp_pr = child(shape, "spPr");
            let xfrm = child(sp_pr, "xfrm");
            let off = child(xfrm, "off");
            let ext = child(xfrm, "ext");
            let _x = attribute_i64(off, "x");
            let _y = attribute_i64(off, "y");
            assert!(
                attribute_i64(ext, "cx") > 0,
                "{name}: shape {index} has non-positive width"
            );
            assert!(
                attribute_i64(ext, "cy") > 0,
                "{name}: shape {index} has non-positive height"
            );
            assert!(
                sp_pr
                    .children
                    .iter()
                    .any(|element| element.local_name == "prstGeom"
                        || element.local_name == "custGeom"),
                "{name}: shape {index} has no native geometry"
            );

            // Exercise the XML writer/parser too: every emitted shape must be serializable as a
            // standalone native shape, not merely valid in the in-memory DOM.
            let xml = write_element(shape)
                .unwrap_or_else(|e| panic!("{name}: shape {index} did not serialize: {e}"));
            assert!(
                !xml.is_empty(),
                "{name}: shape {index} serialized to empty XML"
            );
            parse_element(&xml).unwrap_or_else(|e| {
                panic!("{name}: shape {index} did not parse after serialization: {e}")
            });
        }

        ids.sort_unstable();
        ids.dedup();
        assert_eq!(
            ids.len(),
            conversion.shapes.len(),
            "{name}: duplicate cNvPr shape ids"
        );
        assert!(
            conversion.next_shape_id > START_ID,
            "{name}: shape id counter did not advance"
        );
    }

    assert!(
        total_shapes >= fixture_paths().len(),
        "corpus emitted fewer shapes than fixtures"
    );
    assert!(
        total_fonts > 0,
        "corpus exercised no embedded font metadata"
    );
}

#[test]
fn semantic_fixture_expectations_hold() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/svg");

    let percent = fs::read(dir.join("percent-units-nested-viewports.svg")).unwrap();
    let percent_conversion = svg_to_shapes(&percent, TARGET_CX, TARGET_CY, START_ID).unwrap();
    assert!(percent_conversion.shapes.len() >= 3);
    let percent_extents: Vec<i64> = percent_conversion
        .shapes
        .iter()
        .filter_map(|shape| {
            shape
                .descendants()
                .find(|element| element.local_name == "ext")
                .and_then(|ext| ext.get_attribute("cx"))
                .and_then(|value| value.parse().ok())
        })
        .collect();
    assert!(percent_extents.iter().any(|cx| *cx > 5_000_000));

    let clip = fs::read(dir.join("clip-objectbbox-intersections.svg")).unwrap();
    let clip_conversion = svg_to_shapes(&clip, TARGET_CX, TARGET_CY, START_ID).unwrap();
    assert!(clip_conversion.shapes.len() >= 2);
    assert!(clip_conversion.shapes.iter().all(|shape| {
        shape
            .descendants()
            .any(|element| element.local_name == "custGeom" || element.local_name == "prstGeom")
    }));

    let colors = fs::read(dir.join("css-color-alpha-strokes.svg")).unwrap();
    let color_conversion = svg_to_shapes(&colors, TARGET_CX, TARGET_CY, START_ID).unwrap();
    assert!(color_conversion.shapes.len() >= 4);
    let color_xml = color_conversion
        .shapes
        .iter()
        .map(|shape| String::from_utf8_lossy(&write_element(shape).unwrap()).into_owned())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(color_xml.contains("alpha"));
    assert!(color_xml.to_ascii_lowercase().contains("334155"));

    let graphs = fs::read(dir.join("mask-filter-graphs.svg")).unwrap();
    let graph_conversion = svg_to_shapes(&graphs, TARGET_CX, TARGET_CY, START_ID).unwrap();
    assert!(
        graph_conversion.shapes.len() >= 4,
        "mask/filter graph fixture should emit each painted shape"
    );
    let graph_xml = graph_conversion
        .shapes
        .iter()
        .map(|shape| String::from_utf8_lossy(&write_element(shape).unwrap()).into_owned())
        .collect::<Vec<_>>()
        .join("\n");
    // Drop-shadow graph → outerShdw; alpha matrix → no glow misclassification.
    assert!(graph_xml.contains("outerShdw"), "{graph_xml}");
    assert!(
        graph_xml.contains("alpha") || graph_xml.to_ascii_lowercase().contains("a855f7"),
        "alpha matrix / mask should leave opacity evidence: {graph_xml}"
    );

    let stroke_css = fs::read(dir.join("stroke-text-css-siblings.svg")).unwrap();
    let stroke_conv = svg_to_shapes(&stroke_css, TARGET_CX, TARGET_CY, START_ID).unwrap();
    assert!(
        stroke_conv.shapes.len() >= 4,
        "stroke/text/css sibling fixture should emit several shapes"
    );
    let stroke_xml = stroke_conv
        .shapes
        .iter()
        .map(|shape| String::from_utf8_lossy(&write_element(shape).unwrap()).into_owned())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        stroke_xml.contains("lim=\"200000\"") || stroke_xml.contains("lim=\"400000\""),
        "miter limit should map into DrawingML: {stroke_xml}"
    );
}
