#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct VertexId(pub u64);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Edge(pub VertexId, pub VertexId);

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
