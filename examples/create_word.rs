//! Create a simple Word document.

use openxml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
use openxml::wordprocessing::{body, document, paragraph, paragraph_with_bold_text, run, text};

fn main() -> openxml::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "hello.docx".into());

    let mut doc =
        WordprocessingDocument::create(&path, WordprocessingDocumentType::Document)?;

    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph(vec![run(vec![text("Hello from the Open XML Rust SDK!")])]),
        paragraph_with_bold_text("This line is bold."),
        paragraph(vec![run(vec![text("第三段：中文内容。")])]),
    ])]));

    doc.set_title("Hello Doc")?;
    doc.set_creator("openxml-rs")?;
    doc.save()?;
    println!("Wrote {path}");
    Ok(())
}
