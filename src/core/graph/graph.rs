#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct VertexId(pub u64);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Edge(pub VertexId, pub VertexId);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum GraphCommand {
    AddVertex(VertexId),
    RemoveVertex(VertexId),
    AddEdge(VertexId, VertexId),
    RemoveEdge(VertexId, VertexId),
}

pub trait Graph {
    fn is_empty(&self) -> bool;
    fn vertex_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn contains_vertex(&self, vertex_id: VertexId) -> bool;
    fn vertices(&self) -> Vec<VertexId>;
    fn contains_edge(&self, edge: Edge) -> bool;
    fn edges(&self) -> Vec<Edge>;
}

pub trait MutableGraph {
    fn add_vertex(&mut self, vertex_id: VertexId) -> bool;
    fn remove_vertex(&mut self, vertex_id: VertexId) -> bool;
    fn add_edge(&mut self, edge: Edge) -> bool;
    fn remove_edge(&mut self, edge: Edge) -> bool;
}


impl Edge {

    pub fn from_pair(vertices: (VertexId, VertexId)) -> Edge {
        let (v1, v2) = vertices;
        Edge(v1, v2)
    }

    pub fn as_pair(&self) -> (VertexId, VertexId) {
        let Edge(v1, v2) = *self;
        (v1,v2)
    }

    pub fn reverse(&self) -> Edge {
        let Edge(v1, v2) = *self;
        Edge(v2, v1)
    }
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
    pub fn apply_commands<MG>(commands: Vec<GraphCommand>, graph: &mut MG) -> ()
    where
        MG: MutableGraph,
    {
        for command in commands.iter() {
            GraphCommand::apply_command(*command, graph);
        }
    }

    /// Apply a command on a MutableGraph
    pub fn apply_command<MG>(command: GraphCommand, graph: &mut MG) -> bool
    where
        MG: MutableGraph,
    {
        use GraphCommand::*;
        match command {
            AddVertex(v) => graph.add_vertex(v),
            RemoveVertex(v) => graph.remove_vertex(v),
            AddEdge(v1, v2) => graph.add_edge(Edge(v1, v2)),
            RemoveEdge(v1, v2) => graph.remove_edge(Edge(v1, v2)),
        }
    }

    /// Extracts a graph as a list of commands
    pub fn as_commands<G>(graph: &G) -> Vec<GraphCommand>
    where
        G: Graph,
    {
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
