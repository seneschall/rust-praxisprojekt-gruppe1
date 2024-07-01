use crate::traits::{Directed, Graph, Unlabeled, Unweighted};
use std::collections::HashMap;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct Digraph {

    // vector of all vertice's whom had been deleted
    pub(crate) deleted_vertices: Vec<usize>,
    // the recent number of vertices, deleted_vertices + v_count == adj.len()
    pub(crate) v_count: usize,
    // number of edges              
    pub(crate) e_count: usize,       
    // adjacency list of indices
    pub(crate) adj: Vec<Vec<usize>>,  
}

/// An indexed, mutable graph with directed edges. 
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type.
impl Digraph {

    /// this function instantiiates a new empty digraph, that must be manually filled with vertices and edges
    pub fn new() -> Self {
        Digraph {
            deleted_vertices: Vec::new(),
            v_count: 0,
            e_count: 0,
            adj: vec![vec![]; 0],
        }
    }
    
    /// this function can read in from a vector, but doesn't check that it's valid input, panices instead
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

    /// use at own risk!
    /// adds a new empty vertex to the graph,
    /// by adding an empty vector at the given index, or overwriting the entry with the same key if existant.  
    /// adds several new empty Vertices if the given index exceeds the current v_count.
    /// returns the index of the new vertex
    // todo ! why does this return a usize?
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

    /// return the recent number of edges in the graph
    fn e_count(&self) -> usize {
        self.e_count
    }

    /// return the recent number of vertices in the graph
    fn v_count(&self) -> usize {
        self.v_count
    }

    /// deletes the given edge by deleting the entry by looking up 'from's vector in the adj-list, then search for the index of 'to' in it. 
    /// stores that index in i_of_w, and then removes the entry at that index in 'from's vector.
    /// changes the indices of the edges in the vertex-vertices, but doesn't change the indices of the vertex-vectors, thus preserves indexing.
    /// panics if vertex 'from' or edge 'from'->'to' doens't exists. decreases e_count
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


    /// deletes the vertex at the given index
    /// panics if the vertex doesn't exist - should eventually return a Result type
    /// if the vertex exists, we mark it in the deletec-vertices-Vector, then delete all it's incoming and all it's outgoing mentions.
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

    /// checks if the vertex at the given index exists, by checking if it is smaller than the first unused index in the adj-list.
    fn vertex_exists(&self, vertex: usize) -> bool {
        if !self.deleted_vertices.contains(&vertex) && vertex < self.v_count + self.deleted_vertices.len()
    }

    /// it removes all vertices in deleted_vertices from the graph, thus altering the adj-list and changing indexing.
    /// this lowers adj.len() and resets it to v_count. returns a list comparing the new and old indices.
    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    /// returns if there is an edge between index `from` and index `to` 
    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if self.adj[from].contains(&to) {
            true
        } else {
            false
        }
    }
}
impl Directed<usize> for Digraph {

    /// returns all outgoing edges of the given vertex in a vector, by returning its entry in the adj-list.
    /// should probably be changed to return an iterator instead
    // todo ! catch non-existing vertice as input
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        // todo ! catch non-existing vertice as input ; out-of-bound-error
        self.adj[vertex].clone()
    }

    /// computes the incoming edges of a vertex by looping over all vertices and checking in their adjacency-vector, 
    /// if they have an edge pointing to the given index. returns a vector with the found edges.
    // todo ! catch non-existing vertice as input
    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        let mut incoming_edges: Vec<usize> = Vec::new();
        for i in 0..self.v_count {
            if self.adj[i].contains(&vertex) {
                incoming_edges.push(i);
            }
        }
        incoming_edges
    }

    
    /// deletes all outgoing edges by computing them and then deleting them in a loop.
    fn delete_outgoing_edges(&mut self, vertex: usize) {
        for to in self.outgoing_edges(vertex) {
            self.delete_edge(vertex, to)
        }
    }

    /// deletes all incoming edges by computing them and then deleting them in a loop.
    fn delete_incoming_edges(&mut self, vertex: usize) {
        for from in self.incoming_edges(vertex) {
            self.delete_edge(from, vertex)
        }
    }
}
impl Unlabeled<usize> for Digraph {

    /// adds a new empty vertex at either the index following the last or at (the lowest available) previously freed index.
    /// preserves indexing and never overwrites vertices, increases v_count
    /// append_vertex() is not defined for labeled graphs
    /// returns the index of the new vertex
    fn append_vertex(&mut self) -> usize {
        self.adj.push(vec![]);
        self.v_count += 1;
        self.v_count - 1
    }
}
impl Unweighted<usize> for Digraph {

    /// adds an edge between the vertices `from` and `to`  
    /// panics if either doesn't exist
    /// increases e_count  
    fn add_edge(&mut self, from: usize, to: usize) {
        if !(self.vertex_exists(from) && self.vertex_exists(to)) {
            panic!("One of vertices {}, {} doesn't exist", from, to)
        }
        self.e_count += 1;
        self.adj[from].push(to);
    }
}
