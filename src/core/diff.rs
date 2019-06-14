use crate::core::graph::*;

/// Diff between 2 graphs
pub struct StructureDiff {
    extra_vertices: Vec<VertexId>,
    missing_vertices: Vec<VertexId>,
    extra_edges: Vec<Edge>,
    missing_edges: Vec<Edge>,
}

// Compute the diff between 2 graphs, from the point of view of the first one
pub fn diff<G1, G2>(g1: G1, g2: G2) -> StructureDiff
where
    G1: Graph,
    G2: Graph,
{
    unimplemented!()
}

impl StructureDiff {
    /// Reverse the diff : provides the point of view of the second graph
    pub fn reverse(&self) -> StructureDiff {
        // Note : inefficient implementation which is using cloning.
        // It would probably be better to provide a view ?
        StructureDiff {
            extra_vertices: self.missing_vertices.clone(),
            missing_vertices: self.extra_vertices.clone(),
            extra_edges: self.missing_edges.clone(),
            missing_edges: self.extra_edges.clone(),
        }
    }

    /// Compute a patch between the 2 graphs
    pub fn as_commands(&self) -> Vec<GraphCommand> {
        use GraphCommand::*;
        let mut res: Vec<GraphCommand> = Vec::new();
        for vertex_id in self.extra_vertices.iter() {
            res.push(RemoveVertex(*vertex_id))
        }
        for vertex_id in self.missing_vertices.iter() {
            res.push(AddVertex(*vertex_id))
        }
        for Edge(v1, v2) in self.extra_edges.iter() {
            res.push(RemoveEdge(*v1, *v2))
        }
        for Edge(v1, v2) in self.missing_edges.iter() {
            res.push(AddEdge(*v1, *v2))
        }
        res
    }
}
