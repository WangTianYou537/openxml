//! Create a simple PowerPoint presentation with two slides.

use openxml::packaging::{PresentationDocument, PresentationDocumentType};

fn main() -> openxml::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/hello.pptx".into());
    let mut ppt =
        PresentationDocument::create(&path, PresentationDocumentType::Presentation)?;
    ppt.add_slide_with_text("Hello from Rust")?;
    ppt.add_slide_with_text("Open XML SDK port")?;
    ppt.set_title("Demo Deck")?;
    ppt.set_creator("openxml-rs")?;
    ppt.set_app_slides(2)?;
    ppt.save()?;
    println!("wrote {path}");
    Ok(())
}
