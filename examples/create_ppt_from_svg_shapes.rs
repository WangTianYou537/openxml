//! Convert an SVG into native PowerPoint DrawingML shapes (no SVG media embed).
//!
//! Usage:
//!   create_ppt_from_svg_shapes [out.pptx] [in.svg]
//!   create_ppt_from_svg_shapes [in.svg] [out.pptx]
//! Extensions decide which is which; defaults are slide-1.pptx / slide-1.svg.

use openxml::packaging::{PresentationDocument, PresentationDocumentType};

fn main() -> openxml::Result<()> {
    let a1 = std::env::args().nth(1);
    let a2 = std::env::args().nth(2);

    let (out, svg_path) = match (a1.as_deref(), a2.as_deref()) {
        (Some(x), Some(y)) if x.ends_with(".svg") && y.ends_with(".pptx") => {
            (y.to_string(), x.to_string())
        }
        (Some(x), Some(y)) if x.ends_with(".pptx") && y.ends_with(".svg") => {
            (x.to_string(), y.to_string())
        }
        (Some(x), Some(y)) => (x.to_string(), y.to_string()),
        (Some(x), None) if x.ends_with(".svg") => {
            ("/opt/wp/openxml/slide-1.pptx".into(), x.to_string())
        }
        (Some(x), None) => (x.to_string(), "/opt/wp/openxml/slide-1.svg".into()),
        _ => (
            "/opt/wp/openxml/slide-1.pptx".into(),
            "/opt/wp/openxml/slide-1.svg".into(),
        ),
    };

    let svg_bytes = std::fs::read(&svg_path)?;
    if svg_bytes.starts_with(b"PK") {
        return Err(openxml::Error::Xml(format!(
            "input looks like a ZIP/PPTX, not SVG: {svg_path}"
        )));
    }

    // Full-bleed 16:9 slide (EMUs)
    const SLIDE_CX: i64 = 12_192_000;
    const SLIDE_CY: i64 = 6_858_000;

    let mut ppt =
        PresentationDocument::create(&out, PresentationDocumentType::Presentation)?;
    ppt.add_blank_slide()?;
    let n = ppt.add_svg_shapes_on_slide(0, &svg_bytes, 0, 0, SLIDE_CX, SLIDE_CY)?;

    ppt.set_title("slide-1 (native shapes)")?;
    ppt.set_creator("openxml-rs")?;
    ppt.set_created("2026-07-22T00:00:00Z")?;
    ppt.set_modified("2026-07-22T00:00:00Z")?;
    ppt.set_application("Microsoft Office PowerPoint")?;
    ppt.set_application_version("16.0000")?;
    ppt.set_presentation_format("Widescreen")?;
    ppt.set_app_slides(1)?;
    ppt.save()?;

    println!("wrote {out}");
    println!("  from svg: {svg_path}");
    println!("  native shapes: {n}");
    println!(
        "  masters={} layouts={}",
        ppt.masters().len(),
        ppt.layouts().len()
    );
    Ok(())
}
