use crate::core::graph::graph::*;
use std::collections::HashMap;

/// A directed graph structure that doesn't contain any information concerning the vertex or the
/// edge attributes
pub struct DirectedGraph {
    // Each edge is indexed for by of both its vertices => 1 edge appears twice in the map
    edge_map: HashMap<VertexId, Vec<Edge>>,
}

impl DirectedGraph {

    /// Creates an empty `DirectedGraph`.
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    ///
    /// let mut graph = DirectedGraph::new();
    /// ```
    pub fn new() -> DirectedGraph {
        DirectedGraph {
            edge_map: HashMap::new(),
        }
    }

    /// Returns `true` if the graph contains no vertices nor edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::VertexId;
    ///
    /// let mut g = DirectedGraph::new();
    /// assert!(g.is_empty());
    /// g.add_vertex(VertexId(1));
    /// assert!(!g.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vertex_count() == 0
    }

    /// Returns the number of vertices in the graph
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::VertexId;
    ///
    /// let mut g = DirectedGraph::new();
    /// assert_eq!(g.vertex_count(), 0);
    /// g.add_vertex(VertexId(1));
    /// assert_eq!(g.vertex_count(), 1);
    /// ```
    pub fn vertex_count(&self) -> usize {
        self.edge_map.len()
    }

    /// Returns the number of edges in the graph
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::{VertexId, Edge};
    ///
    /// let mut g = DirectedGraph::new();
    /// assert_eq!(g.edge_count(), 0);
    /// g.add_edge(Edge(VertexId(1), VertexId(2)));
    /// assert_eq!(g.edge_count(), 1);
    /// ```
    pub fn edge_count(&self) -> usize {
        let mut count: usize = 0;
        for (_, edges) in &self.edge_map {
            count += edges.len()
        }
        // each edge is saved twice => the count should be a multiple of 2, and divided by 2
        count / 2
    }

    /// Returns true if the graph contains the `vertex_id`
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::VertexId;
    ///
    /// let mut g = DirectedGraph::new();
    /// assert!(!g.contains_vertex(VertexId(1)));
    /// g.add_vertex(VertexId(1));
    /// assert!(g.contains_vertex(VertexId(1)));
    /// ```
    pub fn contains_vertex(&self, vertex_id: VertexId) -> bool {
        self.edge_map.contains_key(&vertex_id)
    }

    /// An iterator visiting all vertices of the graph in arbitrary order.
    /// The iterator element type is `&VertexId`
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::{VertexId, Edge};
    ///
    /// let mut g = DirectedGraph::new();
    /// g.add_edge(Edge(VertexId(1), VertexId(2)));
    ///
    /// for &v in g.vertices() {
    ///     println!("{:?}", v);
    /// }
    /// ```
    pub fn vertices(&self) -> impl Iterator<Item=&VertexId> {
        self.edge_map.keys()
    }

    /// Returns true if the graph contains the `edge`.
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::{VertexId, Edge};
    ///
    /// let mut g = DirectedGraph::new();
    /// assert!(!g.contains_edge(Edge(VertexId(1), VertexId(2))));
    /// g.add_edge(Edge(VertexId(1), VertexId(2)));
    /// assert!(g.contains_edge(Edge(VertexId(1), VertexId(2))));
    /// ```
    pub fn contains_edge(&self, edge: Edge) -> bool {
        let Edge(v1, v2) = edge;
        if self.contains_vertex(v1) && self.contains_vertex(v2) {
            // We need to look-up only for one of the vertices
            self.edge_map
                .get(&v1)
                .unwrap()
                .iter()
                .position(|x| *x == edge)
                .is_some()
        } else {
            false
        }
    }

    /// An iterator visiting all the edges of the graph in arbitrary order.
    /// The iterator element type is `&Edge`
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::{VertexId, Edge};
    ///
    /// let mut g = DirectedGraph::new();
    /// g.add_edge(Edge(VertexId(1), VertexId(2)));
    /// g.add_edge(Edge(VertexId(2), VertexId(3)));
    ///
    /// for &e in g.edges() {
    ///     println!("{:?}", e);
    /// }
    ///
    /// ```
    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edge_map.values().flatten()
    }

    /// An iterator visiting all the outbound edges of `vertex_id`.
    /// The iterator element type is `&Edge`
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::{VertexId, Edge};
    ///
    /// let mut g = DirectedGraph::new();
    /// g.add_edge(Edge(VertexId(1), VertexId(2)));
    /// g.add_edge(Edge(VertexId(1), VertexId(3)));
    ///
    /// for &e in g.outbound_edges(VertexId(1)) {
    ///     println!("{:?}", e);
    /// }
    ///
    /// ```
    pub fn outbound_edges(&self, vertex_id: VertexId) -> impl Iterator<Item = &Edge> {
        self.edge_map
            .get(&vertex_id)
            .into_iter()
            .flat_map(|edges| edges.iter())
            .filter(move |e| e.0 == vertex_id)
    }

    /// An iterator visiting all the inbound edges of `vertex_id`.
    /// The iterator element type is `&Edge`
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::{VertexId, Edge};
    ///
    /// let mut g = DirectedGraph::new();
    /// g.add_edge(Edge(VertexId(1), VertexId(3)));
    /// g.add_edge(Edge(VertexId(2), VertexId(3)));
    ///
    /// for &e in g.inbound_edges(VertexId(3)) {
    ///     println!("{:?}", e);
    /// }
    ///
    /// ```
    pub fn inbound_edges(&self, vertex_id: VertexId) -> impl Iterator<Item = &Edge> {
        self.edge_map
            .get(&vertex_id)
            .into_iter()
            .flat_map(|edges| edges.iter())
            .filter(move |e| e.1 == vertex_id)
    }

    /// Returns the number of outbound edges of `vertex_id`.
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::{VertexId, Edge};
    ///
    /// let mut g = DirectedGraph::new();
    /// assert_eq!(g.degree_out(VertexId(1)), 0);
    /// g.add_edge(Edge(VertexId(1), VertexId(2)));
    /// assert_eq!(g.degree_out(VertexId(1)), 1);
    /// ```
    pub fn degree_out(&self, vertex_id: VertexId) -> usize {
        self.outbound_edges(vertex_id).count()
    }

    /// Returns the number of inbound edges of `vertex_id`.
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::{VertexId, Edge};
    ///
    /// let mut g = DirectedGraph::new();
    /// assert_eq!(g.degree_in(VertexId(1)), 0);
    /// g.add_edge(Edge(VertexId(1), VertexId(2)));
    /// assert_eq!(g.degree_in(VertexId(2)), 1);
    /// ```
    pub fn degree_in(&self, vertex_id: VertexId) -> usize {
        self.inbound_edges(vertex_id).count()
    }


    /// Adds a vertex to the graph. Returns true if the graph already contained `vertex_id `.
    ///
    /// # Examples
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::VertexId;
    ///
    /// let mut g = DirectedGraph::new();
    /// assert!(!g.add_vertex(VertexId(1)));
    /// assert!(g.add_vertex(VertexId(1)));
    /// ```
    pub fn add_vertex(&mut self, vertex_id: VertexId) -> bool {
        let mut contains_vertex = true;
        self.edge_map
            .entry(vertex_id)
            .or_insert_with(|| {
                contains_vertex = false;
                Vec::new()
            });
        contains_vertex
    }

    /// Removes a vertex from the graph.
    /// Returns true if the graph contained `vertex_id` before the removal.
    ///
    /// ```
    /// use histo_graph::core::graph::directed_graph::DirectedGraph;
    /// use histo_graph::core::graph::graph::VertexId;
    ///
    /// let mut g = DirectedGraph::new();
    /// g.add_vertex(VertexId(1));
    /// assert!(g.remove_vertex(VertexId(1)));
    /// assert!(!g.remove_vertex(VertexId(1)));
    /// ```
    pub fn remove_vertex(&mut self, vertex_id: VertexId) -> bool {
        // We need to remove all edges containing the vertex
        if let Some((_, edges)) = self.edge_map.remove_entry(&vertex_id) {
            for edge in edges {
                let Edge(v1, v2) = edge;
                if v1 != vertex_id {
                    self.remove_edge(edge);
                }
                if v2 != vertex_id {
                    self.remove_edge(edge);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn add_edge(&mut self, edge: Edge) -> bool {
        if self.contains_edge(edge) {
            false
        } else {
            let Edge(v1, v2) = edge;
            self.add_vertex(v1);
            self.add_vertex(v2);
            self.edge_map.get_mut(&v1).unwrap().push(edge);
            self.edge_map.get_mut(&v2).unwrap().push(edge);
            true
        }
    }

    pub fn remove_edge(&mut self, edge: Edge) -> bool {
        let Edge(v1, v2) = edge;
        let mut found = false;
        if let Some(found_v1) = self.edge_map.get_mut(&v1) {
            found |= remove_item(found_v1, &edge);
        }
        if let Some(found_v2) = self.edge_map.get_mut(&v2) {
            found |= remove_item(found_v2, &edge);
        }
        found
    }
}

// Helpers

// Not available on Vec<A> ... for some reason ...
fn remove_item<A: PartialEq>(xs: &mut Vec<A>, item: &A) -> bool {
    if let Some(i) = xs.iter().position(|x| x == item) {
        xs.remove(i);
        true
    } else {
        false
    }
}
