use rowan::{GreenNode, NodeCache};

use crate::{
    kind::{PySyntaxKind, PyTokenKind},
    parser::MarkEvent,
    text::SourceRange,
};

use super::py_green_builder::PyGreenNodeBuilder;

#[derive(Debug)]
pub struct PyTreeBuilder<'a> {
    text: &'a str,
    events: Vec<MarkEvent>,
    green_builder: PyGreenNodeBuilder<'a>,
}

impl<'a> PyTreeBuilder<'a> {
    pub fn new(
        text: &'a str,
        events: Vec<MarkEvent>,
        node_cache: Option<&'a mut NodeCache>,
    ) -> Self {
        match node_cache {
            Some(cache) => PyTreeBuilder {
                text,
                events,
                green_builder: PyGreenNodeBuilder::with_cache(cache),
            },
            None => PyTreeBuilder {
                text,
                events,
                green_builder: PyGreenNodeBuilder::new(),
            },
        }
    }

    pub fn build(&mut self) {
        self.start_node(PySyntaxKind::Chunk);
        let mut parents: Vec<PySyntaxKind> = Vec::new();
        for i in 0..self.events.len() {
            match std::mem::replace(&mut self.events[i], MarkEvent::none()) {
                MarkEvent::NodeStart {
                    kind: PySyntaxKind::None,
                    ..
                }
                | MarkEvent::Trivia => {}
                MarkEvent::NodeStart { kind, parent } => {
                    parents.push(kind);
                    let mut parent_position = parent;
                    while parent_position > 0 {
                        match std::mem::replace(
                            &mut self.events[parent_position],
                            MarkEvent::none(),
                        ) {
                            MarkEvent::NodeStart { kind, parent } => {
                                parents.push(kind);
                                parent_position = parent;
                            }
                            _ => unreachable!(),
                        }
                    }

                    for kind in parents.drain(..).rev() {
                        self.start_node(kind);
                    }
                }
                MarkEvent::NodeEnd => {
                    self.finish_node();
                }
                MarkEvent::EatToken { kind, range } => {
                    self.token(kind, range);
                }
            }
        }

        self.finish_node();
    }

    fn token(&mut self, kind: PyTokenKind, range: SourceRange) {
        self.green_builder.token(kind, range);
    }

    fn start_node(&mut self, kind: PySyntaxKind) {
        self.green_builder.start_node(kind);
    }

    fn finish_node(&mut self) {
        self.green_builder.finish_node();
    }

    pub fn finish(self) -> GreenNode {
        self.green_builder.finish(self.text)
    }
}
