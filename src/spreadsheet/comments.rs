//! Spreadsheet cell comments (`x:comments`).

use crate::element::OpenXmlElement;
use crate::namespace::ns;

const X: &str = ns::SPREADSHEETML.uri;

/// `x:comments` root with authors and comment list.
pub fn comments_root(authors: &[&str], comments: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let author_els: Vec<_> = authors
        .iter()
        .map(|a| OpenXmlElement::new("x", X, "author").with_text(*a))
        .collect();
    OpenXmlElement::new("x", X, "comments")
        .with_ns_decl("x", X)
        .with_child(
            OpenXmlElement::new("x", X, "authors").with_children(author_els),
        )
        .with_child(
            OpenXmlElement::new("x", X, "commentList").with_children(comments),
        )
}

/// A single `x:comment` for cell `ref` (e.g. `"B2"`) by `author_id` (0-based index into authors).
pub fn comment(cell_ref: &str, author_id: u32, text: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "comment")
        .with_attribute("ref", cell_ref)
        .with_attribute("authorId", author_id.to_string())
        .with_child(
            OpenXmlElement::new("x", X, "text").with_child(
                OpenXmlElement::new("x", X, "t").with_text(text),
            ),
        )
}

/// Build a complete comments part for one author and a list of `(cell_ref, text)`.
pub fn comments_for_author(
    author: &str,
    notes: &[(&str, &str)],
) -> OpenXmlElement {
    let comment_els: Vec<_> = notes
        .iter()
        .map(|(r, t)| comment(r, 0, t))
        .collect();
    comments_root(&[author], comment_els)
}
