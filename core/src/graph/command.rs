use crate::graph::graph::{VertexId, Edge};
use crate::graph::directed_graph::DirectedGraph;

/// A command to manipulate a [`DirectedGraph`]
///
/// [`DirectedGraph`]: ../directed_graph/struct.DirectedGraph.html
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum GraphCommand {

    /// Adds a vertex to a graph.
    AddVertex(VertexId),

    /// Removes a vertex from a graph.
    RemoveVertex(VertexId),

    /// Adds an edge to a graph.
    AddEdge(VertexId, VertexId),

    /// Removes an edge from a graph.
    RemoveEdge(VertexId, VertexId),
}

impl GraphCommand {

    /// Returns a command that reverts the provided `command`.
    pub fn revert(command: GraphCommand) -> GraphCommand {
        use GraphCommand::*;
        match command {
            AddVertex(v) => RemoveVertex(v),
            RemoveVertex(v) => AddVertex(v),
            AddEdge(v1, v2) => RemoveEdge(v1, v2),
            RemoveEdge(v1, v2) => AddEdge(v1, v2),
        }
    }

    /// Applies a vector of commands to a MutableGraph
    pub fn apply_commands(commands: Vec<GraphCommand>, graph: &mut DirectedGraph) -> () {
        for command in commands.iter() {
            command.apply_to(graph);
        }
    }

    /// Applies the command to a MutableGraph.
    pub fn apply_to(&self, graph: &mut DirectedGraph) -> bool {
        use GraphCommand::*;
        match self {
            AddVertex(v) => graph.add_vertex(*v),
            RemoveVertex(v) => graph.remove_vertex(*v),
            AddEdge(v1, v2) => graph.add_edge(Edge(*v1, *v2)),
            RemoveEdge(v1, v2) => graph.remove_edge(Edge(*v1, *v2)),
        }
    }

    /// Extracts a graph as a vector of commands.
    pub fn as_commands(graph: &DirectedGraph) -> Vec<GraphCommand> {
        use GraphCommand::*;
        let mut res: Vec<GraphCommand> = vec![];
        for &vertex_id in graph.vertices() {
            res.push(AddVertex(vertex_id))
        }
        for &Edge(v1, v2) in graph.edges() {
            res.push(AddEdge(v1, v2))
        }
        res
    }
}