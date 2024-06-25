use crate::traits::{Directed, Graph, UnLabeled, Unweighted};
use std::collections::HashMap;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct Digraph {
    pub(crate) deleted_vertices: Vec<usize>,
    pub(crate) v_count: usize,       // number of vertices
    pub(crate) e_count: usize,       // number of edges
    pub(crate) adj: Vec<Vec<usize>>, // adjacency list of indices -- note from group: should we set this to pub(crate)?
}
impl Digraph {
    pub fn new() -> Self {
        Digraph {
            deleted_vertices: Vec::new(),
            v_count: 0,
            e_count: 0,
            adj: vec![vec![]; 0],
        }
    }
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<usize>>) -> Self {
        // doesn't check valid input
        if v_count == adj.len() {
            Digraph {
                deleted_vertices: Vec::new(),
                v_count,
                e_count,
                adj,
            }
        } else {
            panic!("Digraph: from_adjacency_list v_count != adj.len()");
        }
    }
}
impl Graph<usize> for Digraph {
    fn add_vertex(&mut self, vertex: usize) -> usize {
        if vertex >= self.v_count {
            for i in 0..vertex - self.v_count + 1 {
                self.adj.insert(self.v_count + i, vec![]);
            }
            self.v_count += vertex - self.v_count + 1;
        } else {
            self.adj.insert(vertex, vec![]);
            // self.v_count += 1; // this line is wrong
        }
        self.v_count - 1
    }

    fn e_count(&self) -> usize {
        self.e_count
    }

    fn v_count(&self) -> usize {
        self.v_count
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        if self.deleted_vertices.contains(&vertex) {
            true
        } else {
            false
        }
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        let i_of_w: usize; // -- note from celine: could we use index_of_w for clarity?
        match self.adj.get(from) {
            Some(vs) => {
                let i_of_w_opt = vs.iter().position(|&x| x == to);
                match i_of_w_opt {
                    Some(i) => {
                        i_of_w = i;
                    } // swap_remove more efficient than remove because the order is not important
                    None => {
                        panic!("There was no edge from {from} to {to}.");
                    }
                }
            }
            None => {
                panic!("Vertex {from} doesn't exist."); // Should be replaced by Result type
            }
        }
        self.adj[from].swap_remove(i_of_w);
        self.e_count -= 1;
    }

    fn delete_vertex(&mut self, vertex: usize) {
        if vertex < self.v_count {
            self.deleted_vertices.push(vertex);
            self.delete_incoming_edges(vertex);
            self.delete_outgoing_edges(vertex);
            self.v_count -= 1;
        } else {
            panic!("delete_vertex : Can't delete Vertex : vertex >= self.v_count")
        }
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        (!self.deleted_vertices.contains(&vertex))
            && vertex < self.v_count + self.deleted_vertices.len()
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if self.adj[from].contains(&to) {
            true
        } else {
            false
        }
    }
}
impl Directed<usize> for Digraph {
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        self.adj[vertex].clone()
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        let mut incoming_edges: Vec<usize> = Vec::new();
        for i in 0..self.v_count {
            if self.adj[i].contains(&vertex) {
                incoming_edges.push(i);
            }
        }
        incoming_edges
    }

    fn delete_outgoing_edges(&mut self, vertex: usize) {
        for to in self.outgoing_edges(vertex) {
            self.delete_edge(vertex, to)
        }
    }

    fn delete_incoming_edges(&mut self, vertex: usize) {
        for from in self.incoming_edges(vertex) {
            self.delete_edge(from, vertex)
        }
    }
}
impl UnLabeled<usize> for Digraph {
    fn append_vertex(&mut self) -> usize {
        self.adj.push(vec![]);
        self.v_count += 1;
        self.v_count - 1
    }
}
impl Unweighted<usize> for Digraph {
    fn add_edge(&mut self, from: usize, to: usize) {
        if !(self.vertex_exists(from) && self.vertex_exists(to)) {
            panic!("One of vertices {}, {} doesn't exist", from, to)
        }
        self.e_count += 1;
        self.adj[from].push(to);
    }
}
