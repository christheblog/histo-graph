use serde_json;

use histo_graph_core::graph::directed_graph::DirectedGraph;
use histo_graph_core::graph::graph::{Edge, VertexId};
use histo_graph_serde::directed_graph_serde::DirectedGraphSer;

fn main() -> Result<(), serde_json::error::Error>{
    let mut graph = DirectedGraph::new();
    graph.add_edge(Edge(VertexId(0), VertexId(1)));
    graph.add_edge(Edge(VertexId(0), VertexId(2)));
    graph.add_edge(Edge(VertexId(0), VertexId(0)));

    let ser: DirectedGraphSer = (&graph).into();

    let str = serde_json::to_string(&ser)?;

    println!("{}", str);

    Ok(())
}
