use histo_graph_core::graph::directed_graph::DirectedGraph;

use serde_json;
use histo_graph_core::graph::graph::{VertexId, Edge};
use histo_graph_core::util::b_tree_bag::BTreeBag;

fn main() -> Result<(), serde_json::error::Error>{
    let mut bag: BTreeBag<Edge> = BTreeBag::new();

    bag.insert(Edge(VertexId(0), VertexId(1)));
    bag.insert(Edge(VertexId(1), VertexId(0)));

    let vec: Vec<(u64, u64)> = bag
        .iter()
        .map(|&Edge(VertexId(id1), VertexId(id2))| (id1, id2))
        .collect();

    let str = serde_json::to_string(&vec)?;

    println!("{}", str);

    Ok(())
}
