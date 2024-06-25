use crate::graph::directed::Digraph;
use crate::traits::{Graph, UnLabeled, Undirected, Unweighted};
use std::collections::HashMap;

#[cfg(test)]
mod test;
pub struct UGraph {
    dg: Digraph,
}

impl UGraph {
    pub fn new() -> Self {
        UGraph { dg: Digraph::new() }
    }

    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<usize>>) -> Self {
        UGraph {
            dg: Digraph::from_adjacency_list(v_count, e_count, adj),
        }
    }
}

impl Graph<usize> for UGraph {
    fn add_vertex(&mut self, vertex: usize) -> usize {
        self.dg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.dg.e_count()
    }

    fn v_count(&self) -> usize {
        self.dg.v_count()
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        self.dg.vertex_deleted(vertex)
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.dg.delete_edge(from, to);
        } else {
            self.dg.delete_edge(to, from);
        }
    }

    fn delete_vertex(&mut self, vertex: usize) {
        if vertex < self.v_count() {
            self.dg.deleted_vertices.push(vertex);
            self.delete_edges_from(vertex);
            self.dg.v_count -= 1;
        } else {
            panic!("delete_vertex : Can't delete Vertex : vertex >= self.v_count")
        }
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        self.dg.vertex_exists(vertex)
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        todo!()
    }
}

impl Undirected<usize> for UGraph {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        let mut edges: Vec<usize> = Vec::new();
        for i in 0..vertex {
            if self.dg.adj[i].contains(&vertex) {
                edges.push(i);
            }
        }
        edges.append(&mut self.dg.adj[vertex].clone());
        edges
    }

    fn delete_edges_from(&mut self, vertex: usize) {
        for from in 0..vertex {
            if self.dg.adj[from].contains(&vertex) {
                self.delete_edge(from, vertex);
            }
        }
        for to in self.dg.adj[vertex].clone() {
            self.delete_edge(vertex, to);
        }
    }
}

impl UnLabeled<usize> for UGraph {
    fn append_vertex(&mut self) -> usize {
        self.dg.append_vertex()
    }
}

impl Unweighted<usize> for UGraph {
    fn add_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.dg.add_edge(from, to);
        } else {
            self.dg.add_edge(to, from);
        }
    }
}
