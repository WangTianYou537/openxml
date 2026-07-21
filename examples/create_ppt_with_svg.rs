//! Create a PowerPoint presentation with pure SVG (no PNG fallback), matching Office.

use openxml::packaging::{PresentationDocument, PresentationDocumentType};

fn main() -> openxml::Result<()> {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/opt/wp/openxml/slide-1.pptx".into());
    let svg_path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "/opt/wp/openxml/slide-1.svg".into());

    let svg_bytes = std::fs::read(&svg_path)?;

    // Full-bleed 16:9 slide (EMUs)
    const SLIDE_CX: i64 = 12_192_000;
    const SLIDE_CY: i64 = 6_858_000;

    let mut ppt =
        PresentationDocument::create(&out, PresentationDocumentType::Presentation)?;
    ppt.add_blank_slide()?;
    let svg_uri = ppt.add_svg_on_slide(0, &svg_bytes, 0, 0, SLIDE_CX, SLIDE_CY, "slide-1")?;
    ppt.set_title("slide-1.svg")?;
    ppt.set_creator("openxml-rs")?;
    ppt.set_created("2026-07-22T00:00:00Z")?;
    ppt.set_modified("2026-07-22T00:00:00Z")?;
    ppt.set_application("Microsoft Office PowerPoint")?;
    ppt.set_application_version("16.0000")?;
    ppt.set_presentation_format("Widescreen")?;
    ppt.set_app_slides(1)?;
    ppt.set_app_notes(0)?;
    ppt.set_app_hidden_slides(0)?;
    ppt.save()?;

    println!("wrote {out}");
    println!("  SVG: {}", svg_uri.as_str());
    println!(
        "  masters={} layouts={}",
        ppt.masters().len(),
        ppt.layouts().len()
    );
    Ok(())
}
