use crate::core::graph::graph::{VertexId, Edge};
use crate::core::graph::directed_graph::DirectedGraph;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum GraphCommand {
    AddVertex(VertexId),
    RemoveVertex(VertexId),
    AddEdge(VertexId, VertexId),
    RemoveEdge(VertexId, VertexId),
}

// Not sure this should be here ...
impl GraphCommand {
    /// return a command to revert the one provided in argument
    pub fn revert(command: GraphCommand) -> GraphCommand {
        use GraphCommand::*;
        match command {
            AddVertex(v) => RemoveVertex(v),
            RemoveVertex(v) => AddVertex(v),
            AddEdge(v1, v2) => RemoveEdge(v1, v2),
            RemoveEdge(v1, v2) => AddEdge(v1, v2),
        }
    }

    /// Apply a list of commands on a MutableGraph
    pub fn apply_commands(commands: Vec<GraphCommand>, graph: &mut DirectedGraph) -> () {
        for command in commands.iter() {
            GraphCommand::apply_command(*command, graph);
        }
    }

    /// Apply a command on a MutableGraph
    pub fn apply_command(command: GraphCommand, graph: &mut DirectedGraph) -> bool {
        use GraphCommand::*;
        match command {
            AddVertex(v) => graph.add_vertex(v),
            RemoveVertex(v) => graph.remove_vertex(v),
            AddEdge(v1, v2) => graph.add_edge(Edge(v1, v2)),
            RemoveEdge(v1, v2) => graph.remove_edge(Edge(v1, v2)),
        }
    }

    /// Extracts a graph as a list of commands
    pub fn as_commands(graph: &DirectedGraph) -> Vec<GraphCommand> {
        use GraphCommand::*;
        let mut res: Vec<GraphCommand> = vec![];
        for vertex_id in graph.vertices() {
            res.push(AddVertex(vertex_id))
        }
        for Edge(v1, v2) in graph.edges() {
            res.push(AddEdge(v1, v2))
        }
        res
    }
}