use rowan::GreenNode;

use crate::{
    PySyntaxNode,
    parser_error::{PyParseError, PyParseErrorKind},
    syntax::{node::PyChunk, traits::PyAstNode},
};

#[derive(Debug, Clone)]
pub struct PySyntaxTree {
    // store GreenNode instead of SyntaxNode, because SyntaxNode is not send and sync
    root: GreenNode,
    errors: Vec<PyParseError>,
}

impl PySyntaxTree {
    pub fn new(root: GreenNode, errors: Vec<PyParseError>) -> Self {
        PySyntaxTree { root, errors }
    }

    // get root node
    pub fn get_red_root(&self) -> PySyntaxNode {
        PySyntaxNode::new_root(self.root.clone())
    }

    // get chunk node, only can cast to LuaChunk
    pub fn get_chunk_node(&self) -> PyChunk {
        PyChunk::cast(self.get_red_root()).unwrap()
    }

    pub fn get_errors(&self) -> &[PyParseError] {
        &self.errors
    }

    pub fn has_syntax_errors(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.kind == PyParseErrorKind::SyntaxError)
    }
}
