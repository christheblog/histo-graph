use histo_graph_core::graph::directed_graph::DirectedGraph;

use serde_json;
use histo_graph_core::graph::graph::{VertexId, Edge};

fn main() -> Result<(), serde_json::error::Error>{
    let mut graph = DirectedGraph::new();

    graph.add_edge(Edge(VertexId(0), VertexId(1)));

    let str = serde_json::to_string(&graph)?;

    println!("{}", str);

    Ok(())
}
