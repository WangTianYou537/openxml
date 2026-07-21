//! Create a Word document with a table, header, and hyperlink.

use openxml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
use openxml::wordprocessing::{
    body, document, paragraph, paragraph_with_text, run, table_from_strings, text,
};

fn main() -> openxml::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "report.docx".into());

    let mut doc =
        WordprocessingDocument::create(&path, WordprocessingDocumentType::Document)?;

    let table = table_from_strings(
        &[
            vec!["Item", "Qty", "Price"],
            vec!["Widgets", "10", "$2.50"],
            vec!["Gadgets", "3", "$9.99"],
        ],
        None,
    );

    doc.add_main_document_part().set_document(document(vec![body(vec![
        paragraph_with_text("Quarterly Report"),
        table,
        paragraph(vec![run(vec![text("See also: ")])]),
    ])]));

    doc.add_default_header("Confidential")?;
    doc.add_default_footer("Page footer")?;
    doc.add_default_styles()?;

    let link = doc.create_hyperlink("https://example.com", "example.com")?;
    doc.body_mut()?.append_child(paragraph(vec![link]));

    doc.save()?;
    println!("Wrote {path}");
    Ok(())
}
