//! Read paragraphs from a Word document.

use officexml::packaging::WordprocessingDocument;

fn main() -> officexml::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "hello.docx".into());

    let mut doc = WordprocessingDocument::open(&path, false)?;
    println!("Document type: {:?}", doc.document_type());

    for (i, p) in doc.paragraph_texts()?.into_iter().enumerate() {
        println!("[{i}] {p}");
    }
    Ok(())
}
