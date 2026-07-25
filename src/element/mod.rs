//! Open XML element DOM.
//!
//! Mirrors the C# hierarchy:
//! - `OpenXmlElement` — base for all elements
//! - leaf vs composite children
//! - attribute bag + optional text content

mod element;
mod dom_reader;
mod element_context;
mod equality;
mod linq;
mod reader;
mod streaming;
mod typed;
mod part_reader;
mod part_writer;
mod writer;
mod xml_path;

pub use element::{OpenXmlAttribute, OpenXmlContent, OpenXmlElement, OpenXmlMiscKind, OpenXmlQualifiedName};
pub use dom_reader::OpenXmlDomReader;
pub use element_context::{
    ElementEvent, ElementEventKind, OpenXmlElementContext, OpenXmlLoadMode, LAZY_STEPS, XMLNS_PREFIX,
    XMLNS_URI,
};
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
pub use part_reader::{ElementState, OpenXmlPartReader, OpenXmlPartReaderOptions};
pub use part_writer::{OpenXmlPartWriter, OpenXmlPartWriterSettings};
pub use writer::{write_element, write_element_fragment, write_element_to};
pub use xml_path::{xpath_index_among_siblings, OpenXmlUnknownMarker, XmlLineInfo, XmlPath};
