use serde::Serialize;
use serde_json;

use histo_graph_core::graph::directed_graph::DirectedGraph;
use histo_graph_core::graph::graph::{Edge, VertexId};
use histo_graph_core::util::b_tree_bag::BTreeBag;

#[derive(Serialize)]
struct DirectedGraphSer {
    vertices: Vec<u64>,
    edges: Vec<(u64, u64)>,
}

fn main() -> Result<(), serde_json::error::Error>{
    let mut graph = DirectedGraph::new();
    graph.add_edge(Edge(VertexId(0), VertexId(1)));
    graph.add_edge(Edge(VertexId(0), VertexId(2)));
    graph.add_edge(Edge(VertexId(0), VertexId(0)));

    let ser = DirectedGraphSer {
        vertices: graph
            .vertices()
            .map(|&VertexId(id)| id)
            .collect(),
        edges: graph
            .edges()
            .map(|&Edge(VertexId(id_1), VertexId(id_2))| (id_1, id_2))
            .collect()
    };

    let str = serde_json::to_string(&ser)?;

    println!("{}", str);

    Ok(())
}
