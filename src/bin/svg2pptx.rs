//! Convert one or more SVGs to native PowerPoint DrawingML shapes (no SVG media embed).
//!
//! Each input SVG becomes its own slide (16:9 widescreen).
//!
//! ```text
//! svg2pptx [options] -o out.pptx a.svg b.svg c.svg
//! svg2pptx [options] out.pptx a.svg b.svg
//! svg2pptx [options] a.svg b.svg          # writes a.pptx (stem of first SVG)
//! ```
//!
//! Font modes (mutually exclusive; last flag wins):
//!   --font-shape         Outline glyphs as shapes (no text boxes, no font embed)
//!   --embed-font         Editable text boxes + subset EOT of used fonts
//!   --embed-font-fully   Editable text boxes + full EOT of used fonts
//!
//! Default: editable text boxes, no font embed (Windows system faces:
//! Times New Roman + Microsoft YaHei when SVG omits font-family).

use officexml::packaging::{
    PresentationDocument, PresentationDocumentType, SvgFontEmbedMode, SvgShapesOnSlideOptions,
};

fn print_usage() {
    eprintln!(
        "\
svg2pptx — SVG(s) → native PowerPoint shapes (.pptx)

USAGE:
  svg2pptx [OPTIONS] -o OUT.pptx IN.svg [IN.svg ...]
  svg2pptx [OPTIONS] OUT.pptx IN.svg [IN.svg ...]
  svg2pptx [OPTIONS] IN.svg [IN.svg ...]          # OUT = <first-stem>.pptx

Each input SVG is placed on its own 16:9 slide (full-bleed).

OPTIONS:
  -o, --output <file>   Output .pptx path
  --font-shape          Text as outline shapes (no text boxes / font embed)
  --embed-font          Editable text + subset-embed used fonts (EOT .fntdata)
  --embed-font-fully    Editable text + full-embed used fonts
  -h, --help            Show this help

Default (no font flag): editable text boxes, system faces only (no embed)."
    );
}

fn main() -> officexml::Result<()> {
    let mut font_shape = false;
    let mut embed_mode = SvgFontEmbedMode::None;
    let mut output: Option<String> = None;
    let mut positionals: Vec<String> = Vec::new();

    let mut args = std::env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
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
            "-o" | "--output" => {
                let path = args.next().unwrap_or_else(|| {
                    eprintln!("missing path after {arg}");
                    print_usage();
                    std::process::exit(2);
                });
                output = Some(path);
            }
            a if a.starts_with("--output=") => {
                output = Some(a["--output=".len()..].to_string());
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

    // Split positionals into optional leading .pptx and the rest as SVGs.
    let (out, svg_paths) = if let Some(o) = output {
        if positionals.is_empty() {
            eprintln!("error: at least one input .svg is required");
            print_usage();
            std::process::exit(2);
        }
        (o, positionals)
    } else if positionals.is_empty() {
        eprintln!("error: at least one input .svg is required");
        print_usage();
        std::process::exit(2);
    } else if positionals[0].ends_with(".pptx") {
        if positionals.len() < 2 {
            eprintln!("error: provide at least one .svg after the output .pptx");
            print_usage();
            std::process::exit(2);
        }
        let o = positionals.remove(0);
        (o, positionals)
    } else {
        // All SVGs (or first is SVG-like); derive output from first path stem.
        let first = &positionals[0];
        if !first.ends_with(".svg") && positionals.len() == 1 {
            // legacy: single non-svg positional treated as output, default svg
            eprintln!("error: expected .svg input(s); got {first}");
            print_usage();
            std::process::exit(2);
        }
        let stem = std::path::Path::new(first)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("out");
        (format!("{stem}.pptx"), positionals)
    };

    if svg_paths.is_empty() {
        eprintln!("error: at least one input .svg is required");
        print_usage();
        std::process::exit(2);
    }

    let options = SvgShapesOnSlideOptions {
        editable_text: !font_shape,
        font_embed: if font_shape {
            SvgFontEmbedMode::None
        } else {
            embed_mode
        },
    };

    let mode_label = if font_shape {
        "font-shape"
    } else {
        match embed_mode {
            SvgFontEmbedMode::None => "text-box (no embed)",
            SvgFontEmbedMode::Subset => "embed-font (subset)",
            SvgFontEmbedMode::Full => "embed-font-fully",
        }
    };

    // Full-bleed 16:9 slide (EMUs)
    const SLIDE_CX: i64 = 12_192_000;
    const SLIDE_CY: i64 = 6_858_000;

    let mut ppt = PresentationDocument::create(&out, PresentationDocumentType::Presentation)?;
    let mut total_shapes = 0usize;

    for (i, svg_path) in svg_paths.iter().enumerate() {
        let svg_bytes = std::fs::read(svg_path).map_err(|e| {
            officexml::Error::Xml(format!("failed to read SVG {svg_path}: {e}"))
        })?;
        if svg_bytes.starts_with(b"PK") {
            return Err(officexml::Error::Xml(format!(
                "input looks like a ZIP/PPTX, not SVG: {svg_path}"
            )));
        }

        ppt.add_blank_slide()?;
        let n = ppt.add_svg_shapes_on_slide_ex(
            i,
            &svg_bytes,
            0,
            0,
            SLIDE_CX,
            SLIDE_CY,
            options.clone(),
        )?;
        total_shapes += n;
        println!(
            "  slide {}: {} → {n} shapes",
            i + 1,
            svg_path
        );
    }

    ppt.set_title(&format!("svg2pptx ({mode_label}, {} slides)", svg_paths.len()))?;
    ppt.set_creator("officexml")?;
    ppt.set_application("Microsoft Office PowerPoint")?;
    ppt.set_application_version("16.0000")?;
    ppt.set_presentation_format("Widescreen")?;
    ppt.set_app_slides(svg_paths.len() as i32)?;
    ppt.save()?;

    println!("wrote {out}");
    println!("  mode: {mode_label}");
    println!("  slides: {}", svg_paths.len());
    println!("  native shapes: {total_shapes}");
    Ok(())
}
