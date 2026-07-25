//! Walk an existing DOM tree with the OpenXmlReader cursor model
//! (C# `OpenXmlDomReader` shell).

use super::element::{OpenXmlAttribute, OpenXmlElement};
use super::part_reader::ElementState;

#[derive(Clone, Copy)]
enum Phase {
    /// About to emit Start for stack top.
    Enter,
    /// Visiting children; `child_index` is next child.
    Children { child_index: usize },
    /// About to emit End for stack top.
    Leave,
}

struct Frame<'a> {
    elem: &'a OpenXmlElement,
    phase: Phase,
}

/// Depth-first cursor over an [`OpenXmlElement`] tree.
pub struct OpenXmlDomReader<'a> {
    stack: Vec<Frame<'a>>,
    state: ElementState,
    read_misc_nodes: bool,
    eof: bool,
    /// Element mirrored for End/Start queries.
    current: Option<&'a OpenXmlElement>,
}

impl<'a> OpenXmlDomReader<'a> {
    pub fn new(root: &'a OpenXmlElement) -> Self {
        Self {
            stack: vec![Frame {
                elem: root,
                phase: Phase::Enter,
            }],
            state: ElementState::Null,
            read_misc_nodes: false,
            eof: false,
            current: None,
        }
    }

    pub fn with_read_misc_nodes(mut self, yes: bool) -> Self {
        self.read_misc_nodes = yes;
        self
    }

    pub fn element_state(&self) -> ElementState {
        self.state
    }

    pub fn is_eof(&self) -> bool {
        self.eof || self.state == ElementState::EOF
    }

    /// Line info is unavailable for pure DOM walks (C# DomReader → Empty).
    pub fn get_line_info(&self) -> super::xml_path::XmlLineInfo {
        let _ = self;
        super::xml_path::XmlLineInfo::EMPTY
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn current(&self) -> Option<&'a OpenXmlElement> {
        self.current
    }

    pub fn local_name(&self) -> Option<&str> {
        self.current.map(|e| e.local_name.as_str())
    }

    pub fn prefix(&self) -> Option<&str> {
        self.current
            .map(|e| e.prefix.as_str())
            .filter(|p| !p.is_empty())
    }

    pub fn attributes(&self) -> &'a [OpenXmlAttribute] {
        self.current.map(|e| e.get_attributes()).unwrap_or(&[])
    }

    /// Number of attributes on the current element.
    pub fn attribute_count(&self) -> usize {
        self.attributes().len()
    }

    pub fn get_text(&self) -> Option<&str> {
        self.current.and_then(|e| e.text_value())
    }

    pub fn is_start_element(&self) -> bool {
        self.state == ElementState::Start
    }

    pub fn is_end_element(&self) -> bool {
        self.state == ElementState::End
    }

    pub fn is_misc_node(&self) -> bool {
        self.state == ElementState::Misc
    }

    /// Move to the next node (C# `Read`). Returns false at EOF.
    pub fn read(&mut self) -> bool {
        if self.eof {
            self.state = ElementState::EOF;
            return false;
        }
        loop {
            let Some(frame) = self.stack.last_mut() else {
                self.eof = true;
                self.state = ElementState::EOF;
                self.current = None;
                return false;
            };
            match frame.phase {
                Phase::Enter => {
                    let elem = frame.elem;
                    frame.phase = Phase::Children { child_index: 0 };
                    self.current = Some(elem);
                    if elem.is_misc_node() {
                        if !self.read_misc_nodes {
                            // skip: go straight to leave without reporting
                            frame.phase = Phase::Leave;
                            continue;
                        }
                        self.state = ElementState::Misc;
                    } else {
                        self.state = ElementState::Start;
                    }
                    return true;
                }
                Phase::Children { child_index } => {
                    let elem = frame.elem;
                    let idx = child_index;
                    if idx < elem.children.len() {
                        frame.phase = Phase::Children {
                            child_index: idx + 1,
                        };
                        let child = &elem.children[idx];
                        self.stack.push(Frame {
                            elem: child,
                            phase: Phase::Enter,
                        });
                        continue;
                    }
                    frame.phase = Phase::Leave;
                    continue;
                }
                Phase::Leave => {
                    let elem = frame.elem;
                    self.stack.pop();
                    self.current = Some(elem);
                    if elem.is_misc_node() && !self.read_misc_nodes {
                        // was skipped; don't report end either
                        continue;
                    }
                    self.state = ElementState::End;
                    return true;
                }
            }
        }
    }

    /// Skip the subtree under the current Start (C# `Skip`).
    pub fn skip(&mut self) {
        if self.state != ElementState::Start {
            return;
        }
        // Drop children visitation: mark top as Leave
        if let Some(frame) = self.stack.last_mut() {
            frame.phase = Phase::Leave;
        }
        let _ = self.read(); // consume End
    }

    /// Move to the first child of the current start element (C# `ReadFirstChild`).
    ///
    /// Must be called on a start element. If there is no child, moves to the
    /// matching end element and returns `false`.
    pub fn read_first_child(&mut self) -> bool {
        if self.state != ElementState::Start {
            return false;
        }
        loop {
            let Some(frame) = self.stack.last_mut() else {
                return false;
            };
            // Ensure we are positioned to read children from index 0.
            let elem = frame.elem;
            if elem.children.is_empty() {
                frame.phase = Phase::Leave;
                self.current = Some(elem);
                self.state = ElementState::End;
                return false;
            }
            // Push first child (or next non-misc if skipping misc)
            let mut idx = 0usize;
            while idx < elem.children.len() {
                let child = &elem.children[idx];
                if child.is_misc_node() && !self.read_misc_nodes {
                    idx += 1;
                    continue;
                }
                frame.phase = Phase::Children {
                    child_index: idx + 1,
                };
                self.stack.push(Frame {
                    elem: child,
                    phase: Phase::Enter,
                });
                // Emit Enter for child
                return self.read();
            }
            // only misc children skipped
            frame.phase = Phase::Leave;
            self.current = Some(elem);
            self.state = ElementState::End;
            return false;
        }
    }

    /// Move to the next sibling element (C# `ReadNextSibling`).
    ///
    /// Skips the remainder of the current subtree and advances to the next
    /// sibling start. If none, positions on the parent end element and returns
    /// `false`.
    pub fn read_next_sibling(&mut self) -> bool {
        if self.is_eof() {
            return false;
        }
        if self.stack.is_empty() {
            self.eof = true;
            self.state = ElementState::EOF;
            return false;
        }

        // If on Start, skip children then treat as after End of current.
        if self.state == ElementState::Start {
            if let Some(frame) = self.stack.last_mut() {
                frame.phase = Phase::Leave;
            }
            // pop current without requiring consumer to see End
            let _ = self.stack.pop();
        } else if self.state == ElementState::End || self.state == ElementState::Misc {
            let _ = self.stack.pop();
        } else {
            return false;
        }

        // Parent frame should be in Children phase with next index.
        loop {
            if self.stack.is_empty() {
                self.eof = true;
                self.state = ElementState::EOF;
                self.current = None;
                return false;
            }
            let Some(frame) = self.stack.last_mut() else {
                return false;
            };
            match frame.phase {
                Phase::Children { child_index } => {
                    let elem = frame.elem;
                    let mut idx = child_index;
                    while idx < elem.children.len() {
                        let child = &elem.children[idx];
                        if child.is_misc_node() && !self.read_misc_nodes {
                            idx += 1;
                            continue;
                        }
                        frame.phase = Phase::Children {
                            child_index: idx + 1,
                        };
                        self.stack.push(Frame {
                            elem: child,
                            phase: Phase::Enter,
                        });
                        return self.read();
                    }
                    // no more siblings → parent End
                    frame.phase = Phase::Leave;
                    self.current = Some(elem);
                    if elem.is_misc_node() && !self.read_misc_nodes {
                        // continue unwinding
                        let _ = self.stack.pop();
                        continue;
                    }
                    self.state = ElementState::End;
                    return false;
                }
                Phase::Enter => {
                    // Shouldn't happen often; force children scan
                    frame.phase = Phase::Children { child_index: 0 };
                    continue;
                }
                Phase::Leave => {
                    // already leaving parent
                    self.current = Some(frame.elem);
                    self.state = ElementState::End;
                    return false;
                }
            }
        }
    }

    /// Whether the current element has attributes (C# `HasAttributes`).
    pub fn has_attributes(&self) -> bool {
        self.current.map(|e| e.has_attributes()).unwrap_or(false)
    }

    /// Namespace URI of the current element.
    pub fn namespace_uri(&self) -> Option<&str> {
        self.current
            .map(|e| e.namespace_uri.as_str())
            .filter(|u| !u.is_empty())
    }

    /// Load a clone of the element at the current start cursor and advance to its end
    /// (C# `LoadCurrentElement` subset — returns owned clone).
    pub fn load_current_element(&mut self) -> Option<OpenXmlElement> {
        if self.state != ElementState::Start && self.state != ElementState::Misc {
            return None;
        }
        let cloned = self.current?.clone_node();
        self.skip();
        Some(cloned)
    }

    /// Namespace declarations on the current element (C# `NamespaceDeclarations`).
    pub fn namespace_declarations(&self) -> &[(String, String)] {
        self.current
            .map(|e| e.namespace_declarations())
            .unwrap_or(&[])
    }

    /// Attribute value by local name on the current element.
    pub fn get_attribute(&self, local_name: &str) -> Option<&str> {
        self.current.and_then(|e| e.get_attribute(local_name))
    }

    /// Close the reader (C# `Close` shell).
    pub fn close(&mut self) {
        self.eof = true;
        self.state = ElementState::EOF;
        self.current = None;
        self.stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlElement;

    #[test]
    fn dom_reader_walk() {
        let root = OpenXmlElement::w("document").with_child(
            OpenXmlElement::w("body").with_child(
                OpenXmlElement::w("p").with_child(
                    OpenXmlElement::w("r")
                        .with_child(OpenXmlElement::w("t").with_text("Hi")),
                ),
            ),
        );
        let mut r = OpenXmlDomReader::new(&root);
        let mut starts = Vec::new();
        while r.read() {
            if r.is_start_element() {
                starts.push(r.local_name().unwrap().to_string());
            }
        }
        assert_eq!(
            starts,
            vec!["document", "body", "p", "r", "t"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn dom_reader_skip_body() {
        let root = OpenXmlElement::w("document").with_child(
            OpenXmlElement::w("body")
                .with_child(OpenXmlElement::w("p"))
                .with_child(OpenXmlElement::w("p")),
        );
        let mut r = OpenXmlDomReader::new(&root);
        assert!(r.read());
        assert_eq!(r.local_name(), Some("document"));
        assert!(r.read());
        assert_eq!(r.local_name(), Some("body"));
        r.skip(); // skip body subtree end
        // after skip we consumed body's End; next should be document End
        assert!(r.read());
        assert!(r.is_end_element());
        assert_eq!(r.local_name(), Some("document"));
        assert!(!r.read());
    }

    #[test]
    fn read_first_child_and_siblings() {
        let root = OpenXmlElement::w("body")
            .with_child(OpenXmlElement::w("p1"))
            .with_child(OpenXmlElement::w("p2"))
            .with_child(OpenXmlElement::w("p3"));
        let mut r = OpenXmlDomReader::new(&root);
        assert!(r.read());
        assert_eq!(r.local_name(), Some("body"));
        assert!(r.read_first_child());
        assert_eq!(r.local_name(), Some("p1"));
        assert!(r.is_start_element());
        assert!(r.read_next_sibling());
        assert_eq!(r.local_name(), Some("p2"));
        assert!(r.read_next_sibling());
        assert_eq!(r.local_name(), Some("p3"));
        assert!(!r.read_next_sibling());
        assert!(r.is_end_element());
        assert_eq!(r.local_name(), Some("body"));
    }

    #[test]
    fn load_current_element_clones() {
        let root = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_text("x"));
        let mut r = OpenXmlDomReader::new(&root);
        assert!(r.read());
        let loaded = r.load_current_element().unwrap();
        assert_eq!(loaded.local_name, "p");
        assert_eq!(loaded.children.len(), 1);
        // cursor should be past the element (on end consumed by skip)
        assert!(r.is_end_element() || r.is_eof());
    }

    #[test]
    fn namespace_declarations_and_get_attribute() {
        let root = OpenXmlElement::w("p")
            .with_attribute("rsidR", "Z")
            .with_ns_decl("w", "http://schemas.openxmlformats.org/wordprocessingml/2006/main");
        let mut r = OpenXmlDomReader::new(&root);
        assert!(r.read());
        assert_eq!(r.get_attribute("rsidR"), Some("Z"));
        assert!(r
            .namespace_declarations()
            .iter()
            .any(|(p, _)| p == "w"));
        r.close();
        assert!(r.is_eof());
    }
}
