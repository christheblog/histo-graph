use serde::Serialize;
use histo_graph_core::graph::directed_graph::DirectedGraph;
use histo_graph_core::graph::graph::{VertexId, Edge};

#[derive(Serialize)]
pub struct DirectedGraphSer {
    vertices: Vec<u64>,
    edges: Vec<(u64, u64)>,
}

impl From<&DirectedGraph> for DirectedGraphSer {
    fn from(graph: &DirectedGraph) -> DirectedGraphSer {
        DirectedGraphSer {
            vertices: graph
                .vertices()
                .map(|&VertexId(id)| id)
                .collect(),
            edges: graph
                .edges()
                .map(|&Edge(VertexId(id_1), VertexId(id_2))| (id_1, id_2))
                .collect()
        }
    }
}
