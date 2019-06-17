## Histo-Graph

This is a small pet project to teach myself rust (or at least start ...)

Idea is to have a mutable graph structure, where each mutable operation is a command applied to a Vertex or an Edge.
```rust
pub struct VertexId(pub u64);

pub struct Edge(pub VertexId, pub VertexId);

pub enum GraphCommand {
    AddVertex(VertexId),
    RemoveVertex(VertexId),
    AddEdge(VertexId, VertexId),
    RemoveEdge(VertexId, VertexId),
}
```

The graph is just representing the underlying graph structure, and no properties can be attached to the nodes / edges themselves.
A separate data-structure mapping vertex ids, and edge to properties will be added.

History is a list of commits and hashes (in a git fashion). It can be branched, tagged, rebased, checkout-ed.
