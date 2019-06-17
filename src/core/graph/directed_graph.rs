use crate::core::graph::graph::*;
use std::collections::HashMap;

pub trait Directed {
    /// Returns an iterator on the outbound edges
    fn outbound_edges(&self, vertex_id: VertexId) -> Vec<Edge>;
    /// Returns an iterator on the inbound edges
    fn inbound_edges(&self, vertex_id: VertexId) -> Vec<Edge>;
    /// Count the directed edges going-out of the given vertex
    fn degree_out(&self, vertex_id: VertexId) -> usize;
    /// Count the directed edges arriving to the given vertex
    fn degree_in(&self, vertex_id: VertexId) -> usize;
    /// Returns parent vertices
    fn parents(&self, vertex_id: VertexId) -> Vec<VertexId> {
        self.inbound_edges(vertex_id)
            .iter()
            .map(|Edge(v1, _)| *v1)
            .collect()
    }
    /// Returns children vertices
    fn children(&self, vertex_id: VertexId) -> Vec<VertexId> {
        self.outbound_edges(vertex_id)
            .iter()
            .map(|Edge(_, v2)| *v2)
            .collect()
    }
}

/// Directed graph structure
/// It doesn't contain any information concerning the vertex or the edge attributes
pub struct DirectedGraph {
    // Each edge is indexed for by of both its vertices => 1 edge appears twice in the map
    edge_map: HashMap<VertexId, Vec<Edge>>,
}

impl Graph for DirectedGraph {
    fn is_empty(&self) -> bool {
        self.vertex_count() == 0
    }

    fn vertex_count(&self) -> usize {
        self.edge_map.len()
    }

    fn edge_count(&self) -> usize {
        let mut count: usize = 0;
        for (_, edges) in &self.edge_map {
            // each edge is saved twice => the count should be a multiple of 2, and divided by 2
            count += edges.len() / 2
        }
        count
    }

    fn contains_vertex(&self, vertex_id: VertexId) -> bool {
        self.edge_map.contains_key(&vertex_id)
    }

    fn vertices(&self) -> Vec<VertexId> {
        self.edge_map.keys().map(|k| *k).collect()
    }

    fn contains_edge(&self, edge: Edge) -> bool {
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

    fn edges(&self) -> Vec<Edge> {
        self.edge_map.values().flatten().map(|k| *k).collect()
    }
}

impl Directed for DirectedGraph {
    fn outbound_edges(&self, vertex_id: VertexId) -> Vec<Edge> {
        self.edge_map
            .get(&vertex_id)
            .map(|edges| {
                edges
                    .iter()
                    .filter(|Edge(src, _)| *src == vertex_id)
                    .map(|edge| *edge)
                    .collect()
            })
            .unwrap_or_else(|| vec![])
    }

    fn inbound_edges(&self, vertex_id: VertexId) -> Vec<Edge> {
        self.edge_map
            .get(&vertex_id)
            .map(|edges| {
                edges
                    .iter()
                    .filter(|Edge(_, dest)| *dest == vertex_id)
                    .map(|edge| *edge)
                    .collect()
            })
            .unwrap_or_else(|| vec![])
    }

    fn degree_out(&self, vertex_id: VertexId) -> usize {
        self.outbound_edges(vertex_id).len()
    }

    fn degree_in(&self, vertex_id: VertexId) -> usize {
        self.inbound_edges(vertex_id).len()
    }
}

impl MutableGraph for DirectedGraph {
    fn add_vertex(&mut self, vertex_id: VertexId) -> bool {
        let contains_vertex = self.edge_map.contains_key(&vertex_id);
        if !contains_vertex {
            self.edge_map.insert(vertex_id, Vec::new());
        }
        contains_vertex
    }

    fn remove_vertex(&mut self, vertex_id: VertexId) -> bool {
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

    fn add_edge(&mut self, edge: Edge) -> bool {
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

    fn remove_edge(&mut self, edge: Edge) -> bool {
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
