//! This module defines the basic building blocks of a graph.
//!
//! [`VertexId`] to identify a vertex, and [`Edge`], a connection between two vertices.
//!
//! [`VertexId`]: struct.VertexId.html
//! [`Edge`]: struct.Edge.html

use serde::Serialize;

/// Identifies and represents a vertex in a graph.
#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Debug, Serialize)]
pub struct VertexId(pub u64);

/// Represents a directed edge in a graph.
/// The first `VertexId` is the start of the edge, the second `VertexId` is its end.
#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Debug, Serialize)]
pub struct Edge(pub VertexId, pub VertexId);

impl Edge {

    pub fn reverse(&self) -> Edge {
        let Edge(v1, v2) = *self;
        Edge(v2, v1)
    }
}

impl From<(VertexId, VertexId)> for Edge {
    fn from(pair: (VertexId, VertexId)) -> Edge {
        Edge(pair.0, pair.1)
    }
}
