use crate::graph::graph::*;
use crate::graph::directed_graph::*;
use crate::graph::command::*;
use crate::history::hashlist::*;
use crate::history::history::*;

type Commands = Vec<GraphCommand>;

pub struct CommandHasher { }

impl Hasher<Commands> for CommandHasher {
    fn hash(&self, item: &Commands, previous: Option<NodeHash>) -> NodeHash {
        unimplemented!()
    }
}

pub struct HistorizedGraph {
    repository: Repository<Commands, CommandHasher>,
    graph: DirectedGraph,
}

impl HistorizedGraph {

    fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }
    fn vertex_count(&self) -> usize {
        self.graph.vertex_count()
    }
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
    fn contains_vertex(&self, vertex_id: VertexId) -> bool {
        self.graph.contains_vertex(vertex_id)
    }
    fn vertices(&self) -> impl Iterator<Item=&VertexId> {
        self.graph.vertices()
    }
    fn contains_edge(&self, edge: Edge) -> bool {
        self.graph.contains_edge(edge)
    }
    fn edges(&self) -> impl Iterator<Item=&Edge> {
        self.graph.edges()
    }

    fn outbound_edges(&self, vertex_id: VertexId) -> impl Iterator<Item=&Edge> {
        self.graph.outbound_edges(vertex_id)
    }
    fn inbound_edges(&self, vertex_id: VertexId) -> impl Iterator<Item=&Edge> {
        self.graph.inbound_edges(vertex_id)
    }
    fn degree_out(&self, vertex_id: VertexId) -> usize {
        self.graph.degree_out(vertex_id)
    }
    fn degree_in(&self, vertex_id: VertexId) -> usize {
        self.graph.degree_in(vertex_id)
    }

    fn add_vertex(&mut self, vertex_id: VertexId) -> bool {
        match commit_command(self, GraphCommand::AddVertex(vertex_id)) {
            Err(_) => false,
            Ok(_) => self.graph.add_vertex(vertex_id),
        }
    }
    fn remove_vertex(&mut self, vertex_id: VertexId) -> bool {
        match commit_command(self, GraphCommand::RemoveVertex(vertex_id)) {
            Err(_) => false,
            Ok(_) => self.graph.remove_vertex(vertex_id),
        }
    }
    fn add_edge(&mut self, edge: Edge) -> bool {
        let Edge(v1, v2) = edge;
        match commit_command(self, GraphCommand::AddEdge(v1, v2)) {
            Err(_) => false,
            Ok(_) => self.graph.add_edge(edge),
        }
    }
    fn remove_edge(&mut self, edge: Edge) -> bool {
        let Edge(v1, v2) = edge;
        match commit_command(self, GraphCommand::RemoveEdge(v1, v2)) {
            Err(_) => false,
            Ok(_) => self.graph.remove_edge(edge),
        }
    }
}

// Helpers

fn commit_command(repo: &mut HistorizedGraph, command: GraphCommand) -> Result<Ref, String> {
    repo.repository.commit(
        vec![command],
        Author("auto".to_string()),
        Comment("auto".to_string()),
    )
}
