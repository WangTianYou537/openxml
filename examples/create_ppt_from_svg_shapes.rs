//! Convert an SVG into native PowerPoint DrawingML shapes (no SVG media embed).
//!
//! Usage:
//!   create_ppt_from_svg_shapes [options] [out.pptx] [in.svg]
//!   create_ppt_from_svg_shapes [options] [in.svg] [out.pptx]
//!
//! Options (mutually exclusive font modes; last one wins):
//!   --font-shape         Outline glyphs as shapes (no text boxes, no font embed)
//!   --embed-font         Editable text boxes + subset EOT of used fonts
//!   --embed-font-fully   Editable text boxes + full EOT of used fonts
//!
//! Default (no flag): editable text boxes, no font embed (Windows system faces).
//! Extensions decide which positional arg is which; defaults are slide-1.pptx / slide-1.svg.

use openxml::packaging::{
    PresentationDocument, PresentationDocumentType, SvgFontEmbedMode, SvgShapesOnSlideOptions,
};

fn print_usage() {
    eprintln!(
        "usage: create_ppt_from_svg_shapes [--font-shape|--embed-font|--embed-font-fully] [out.pptx] [in.svg]"
    );
}

fn main() -> openxml::Result<()> {
    let mut font_shape = false;
    let mut embed_mode = SvgFontEmbedMode::None;
    let mut positionals: Vec<String> = Vec::new();

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--font-shape" | "-font-shape" => {
                font_shape = true;
                embed_mode = SvgFontEmbedMode::None;
            }
            "--embed-font" | "-embed-font" => {
                font_shape = false;
                embed_mode = SvgFontEmbedMode::Subset;
            }
            "--embed-font-fully" | "-embed-font-fully" | "--embed-font-full" => {
                font_shape = false;
                embed_mode = SvgFontEmbedMode::Full;
            }
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            a if a.starts_with('-') => {
                eprintln!("unknown option: {a}");
                print_usage();
                std::process::exit(2);
            }
            _ => positionals.push(arg),
        }
    }

    let (out, svg_path) = match (positionals.get(0).map(String::as_str), positionals.get(1).map(String::as_str)) {
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

    let options = SvgShapesOnSlideOptions {
        editable_text: !font_shape,
        font_embed: if font_shape {
            SvgFontEmbedMode::None
        } else {
            embed_mode
        },
    };

    let mut ppt = PresentationDocument::create(&out, PresentationDocumentType::Presentation)?;
    ppt.add_blank_slide()?;
    let n = ppt.add_svg_shapes_on_slide_ex(0, &svg_bytes, 0, 0, SLIDE_CX, SLIDE_CY, options)?;

    let mode_label = if font_shape {
        "font-shape"
    } else {
        match embed_mode {
            SvgFontEmbedMode::None => "text-box (no embed)",
            SvgFontEmbedMode::Subset => "embed-font (subset)",
            SvgFontEmbedMode::Full => "embed-font-fully",
        }
    };

    ppt.set_title(&format!("slide-1 ({mode_label})"))?;
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
    println!("  mode: {mode_label}");
    println!("  native shapes: {n}");
    println!(
        "  masters={} layouts={}",
        ppt.masters().len(),
        ppt.layouts().len()
    );
    Ok(())
}
