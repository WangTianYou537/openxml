//! Open XML element DOM.
//!
//! Mirrors the C# hierarchy:
//! - `OpenXmlElement` — base for all elements
//! - leaf vs composite children
//! - attribute bag + optional text content

mod element;
mod equality;
mod linq;
mod reader;
mod streaming;
mod typed;
mod part_reader;
mod part_writer;
mod writer;

pub use element::{OpenXmlAttribute, OpenXmlElement, OpenXmlMiscKind};
pub use equality::{
    element_hash, element_hash_with, elements_equal, elements_equal_with, ElementComparer,
    EqualityOptions,
};
pub use linq::{
    descendant_attr, descendants_of, elements_of, first_descendant, ElementQuery, NamedQuery,
};
pub use reader::parse_element;
pub use streaming::{write_xml_events, OpenXmlStreamReader, XmlEvent};
pub use typed::{
    Body, Cell, Comment, Document, Footer, Header, Hyperlink, Notes, Paragraph, Run, Slide, Style,
    Table, TableCell, TableRow, Text, Worksheet,
};
pub use part_reader::{ElementState, OpenXmlPartReader};
pub use part_writer::OpenXmlPartWriter;
pub use writer::write_element;
