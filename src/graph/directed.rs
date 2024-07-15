use crate::traits::{Directed, Graph, Unlabeled, Unweighted};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod test;
// An indexed, mutable graph with directed edges. (digraph, dg)
// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Digraph {
    pub(crate) deleted_vertices: HashMap<usize, bool>,
    pub(crate) adj_len: usize, // number of vertices, deleted_vertices + v_count() == adj.len()
    pub(crate) e_count: usize, // number of edges
    pub(crate) adj: Vec<Vec<usize>>, // adjacency list of indices -- note from group: should we set this to pub(crate)?
}

impl Digraph {
    // this function instantiiates a new empty digraph, that must be manually filled with vertices and edges
    pub fn new() -> Self {
        Digraph {
            deleted_vertices: HashMap::new(),
            adj_len: 0,
            e_count: 0,
            adj: vec![vec![]; 0],
        }
    }

    // this function can read in from a vector, but doesn't check that it's valid input, panices instead
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<usize>>) -> Self {
        // doesn't check valid input
        if v_count == adj.len() {
            Digraph {
                deleted_vertices: HashMap::new(),
                adj_len: v_count,
                e_count,
                adj,
            }
        } else {
            panic!("Digraph: from_adjacency_list v_count != adj.len()");
        }
    }
}
impl Graph<usize> for Digraph {
    // use at own risk!
    // adds a new empty vertex to the graph,
    // by adding an empty vector at the given index, or overwriting the entry with the same key if existant.
    // adds several new empty Vertices if the given index exceeds the current v_count.
    // returns the index of the new vertex
    // todo ! why does this return a usize?
    fn add_vertex(&mut self, vertex: usize) -> usize {
        // case 1 vertex exist
        // case 2 vertex does not exist, deleted_vertices contains vertex
        // case 3 vertex does not exist, deleted_vertices does not contain vertex
        // in this case check if vertex > adj_len

        if self.vertex_exists(vertex) {
            // case 1
            self.delete_outgoing_edges(vertex);
            self.delete_incoming_edges(vertex);
        } else {
            if self.deleted_vertices.contains_key(&vertex) {
                // case 2
                self.deleted_vertices.remove(&vertex);
            } else {
                // case 3
                for _i in 0..vertex - self.adj_len + 1 {
                    self.adj.push(vec![]);
                }
                self.adj_len += vertex - self.adj_len + 1;
            }
        }
        return vertex;
    }

    // return the recent number of edges in the graph
    fn e_count(&self) -> usize {
        self.e_count
    }

    // return the recent number of vertices in the graph
    fn v_count(&self) -> usize {
        self.adj_len - self.deleted_vertices.len()
    }

    // deletes the given edge by deleting the entry by looking up 'from's vector in the adj-list, then search for the index of 'to' in it.
    // stores that index in i_of_w, and then removes the entry at that index in 'from's vector.
    // changes the indices of the edges in the vertex-vertices, but doesn't change the indices of the vertex-vectors, thus preserves indexing.
    // panics if vertex 'from' or edge 'from'->'to' doens't exists. decreases e_count
    fn delete_edge(&mut self, from: usize, to: usize) {
        let i_of_w: usize;
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

    // deletes the vertex at the given index
    // panics if the vertex doesn't exist - should eventually return a Result type
    // if the vertex exists, we mark it in the deletec-vertices-Vector, then delete all it's incoming and all it's outgoing mentions.
    fn delete_vertex(&mut self, vertex: usize) {
        if vertex < self.adj_len {
            self.deleted_vertices.insert(vertex, true);
            self.delete_incoming_edges(vertex);
            self.delete_outgoing_edges(vertex);
        } else {
            panic!("dg delete_vertex : Vertex doesn't exist")
        }
    }

    // checks if the vertex at the given index exists, by checking if it is smaller than the first unused index in the adj-list.
    fn vertex_exists(&self, vertex: usize) -> bool {
        if self.deleted_vertices.contains_key(&vertex) {
            return false;
        }
        if vertex < self.adj_len {
            return true;
        }
        return false;
    }

    // returns if there is an edge between index `from` and index `to`
    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if self.adj[from].contains(&to) {
            true
        } else {
            false
        }
    }
}
impl Directed<usize> for Digraph {
    // returns all outgoing edges of the given vertex in a vector, by returning its entry in the adj-list.
    // should probably be changed to return an iterator instead
    // todo ! catch non-existing vertice as input
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        // todo ! catch non-existing vertice as input ; out-of-bound-error
        self.adj[vertex].clone()
    }

    // computes the incoming edges of a vertex by looping over all vertices and checking in their adjacency-vector,
    // if they have an edge pointing to the given index. returns a vector with the found edges.
    // todo ! catch non-existing vertice as input
    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        let mut incoming_edges: Vec<usize> = Vec::new();
        for i in 0..self.adj.len() {
            if self.adj[i].contains(&vertex) {
                incoming_edges.push(i);
            }
        }
        incoming_edges
    }

    // deletes all outgoing edges by computing them and then deleting them in a loop.
    fn delete_outgoing_edges(&mut self, vertex: usize) {
        for to in self.outgoing_edges(vertex) {
            self.delete_edge(vertex, to)
        }
    }

    // deletes all incoming edges by computing them and then deleting them in a loop.
    fn delete_incoming_edges(&mut self, vertex: usize) {
        for from in self.incoming_edges(vertex) {
            self.delete_edge(from, vertex)
        }
    }
}
impl Unlabeled<usize> for Digraph {
    // adds a new empty vertex at either the index following the last or at (the lowest available) previously freed index.
    // preserves indexing and never overwrites vertices, increases v_count
    // append_vertex() is not defined for labeled graphs
    // returns the index of the new vertex
    fn append_vertex(&mut self) -> usize {
        self.adj.push(vec![]);
        self.adj_len += 1;
        self.adj_len - 1
    }

    // it removes all vertices in deleted_vertices from the graph, thus altering the adj-list and changing indexing.
    // this lowers adj.len() and resets it to v_count. returns a list comparing the new and old indices.
    fn shrink(&mut self) -> Vec<Option<usize>> {
        todo!()
    }
}
impl Unweighted<usize> for Digraph {
    // adds an edge between the vertices `from` and `to`
    // panics if either doesn't exist
    // increases e_count
    fn add_edge(&mut self, from: usize, to: usize) {
        if !(self.vertex_exists(from) && self.vertex_exists(to)) {
            panic!("One of vertices {}, {} doesn't exist", from, to)
        }
        self.e_count += 1;
        self.adj[from].push(to);
    }
}
