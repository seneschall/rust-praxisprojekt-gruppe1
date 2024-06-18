use num::{ToPrimitive, Unsigned};
use std::collections::HashMap;

// this trait applies to all graph structures
pub trait Graph<L> {
    fn add_edge(&mut self, from: usize, to: usize);

    fn add_ledge(&mut self, from: L, to: L); // get indices of `from` and `to` and call add_edge on those

    fn add_vertex(&mut self, vertex: usize); // adds vertex at given index; use at users own risk; if vertex exists (i.e. vertex is less than wt_adj.len()), it just adds it, if it does, it must not have incoming or outgoing edges

    fn add_lvertex(&mut self, label: L); // shortcut for `append_vertex` and `add_vertex_label`

    fn add_vertex_label(&mut self, vertex: usize, label: L); // adds a label for the vertex with index `vertex`; every label must be unique! Method should check this and return a Result type

    fn append_vertex(&mut self, vertex: usize) -> usize; // adds vertex at position wt_adj.len() or at index of lowest deleted vertex (if that change hasn't been committed)

    fn e_count(&self) -> usize; // should eventually be changed to return a Result type

    fn edit_label(&mut self, vertex: usize, label: L); // true if last item in uncommitted edits for v is Edit::DeleteSelf

    fn get_label(&self, vertex: usize) -> Option<&L>;

    fn get_index(&self, label: L) -> Option<usize>; // returns the index of the vertex with the given label

    fn v_count(&self) -> usize; // returns the number of vertices in graph
}

pub trait Delete<L> {
    fn delete_edge(&mut self, from: usize, to: usize);

    fn delete_and_shift(&mut self, vertex: usize); // deletes vertex at index and shifts all following indices 1 to the left

    // fn delete_lvertex_and_shift(&mut self, label: L); // deletes vertex at index and shifts all following indices 1 to the left
}

pub trait WTDelete<L> {
    fn delete_edge(&mut self, from: usize, to: usize);

    fn delete_ledge(&mut self, from: L, to: L); // get indices of `from` and `to` and call delete_edge on those

    fn delete_vertex(&mut self, vertex: usize); // should eventually be changed to return a Result type

    // fn delete_lvertex(&mut self, label: L);

    fn vertex_deleted(&self, vertex: usize) -> bool;
}

// this trait applies to undirected graph structures
pub trait Undirected<L> {
    fn edges(&self, vertex: usize) -> Vec<usize>; // returns all edges connected to vertex

    // fn edges_lvertex(label: L) -> Vec<L>; // returns the labels of all edges connected to vertex with label

    fn delete_edges(&self, vertex: usize); // deletes all edges connected to vertex
}

// this trait applies to directed graph structures
pub trait Directed<L> {
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize>; // should probably be changed to return an iterator instead

    fn incoming_edges(&self, vertex: usize) -> Vec<usize>; // likewise here

    // fn outgoing_ledges(&self, label: L) -> Vec<L>; // should probably be changed to return an iterator instead

    // fn incoming_ledges(&self, label: L) -> Vec<L>; // likewise here

    fn delete_outgoing_edges(&self, vertex: usize); // deletes all outgoing edges of vertex; should return a Result

    fn delete_incoming_edges(&self, vertex: usize); // deletes all incoming edges of vertex; should return a Result
}

// this trait applies to weighted graph structures
pub trait Weighted {
    fn weight_of_edge(&self, from: usize, to: usize) -> f64;
}

//  this trait applies to all WT graph structures
pub trait WT<L> {
    fn commit_edits(&self);

    fn get_uncommitted_edits(&self) -> Option<HashMap<usize, L>>;

    fn discard_edits(&self);

    fn shrink(&mut self) -> HashMap<usize, usize>; // removes all unconnected vertices from bitmap; only allowed, if has_uncommitted_edits == false; returns a Hashmap with old indices as keys and new indices as values
}

// this trait applies to undirected WT graph structures
pub trait WTGraph {
    fn updated_edges(&self, vertex: usize) -> Option<Vec<usize>>;
}

// this trait applies to directed WT structures
pub trait WTDigraph {
    fn updated_outgoing_edges(&self, vertex: usize) -> Option<Vec<usize>>;

    fn updated_incoming_edges(&self, vertex: usize) -> Option<Vec<usize>>;
}

// are we missing WTWeighted?

// additional graph functionality
pub enum ShortestPathAlgorithm {
    Dijkstra,
    BFS,
    BellmanFord,
    AStar,
}

pub trait GraphSearch {
    fn shortest_path(&self, from: usize, to: usize, mode: ShortestPathAlgorithm) -> Vec<usize>; // returns the shortest path from `from` to `to` using breadth first search

    fn shortest_paths(&self, mode: ShortestPathAlgorithm) -> Vec<Vec<usize>>; // shortest paths from all vertices to all other vertices

    fn connected_components(&self) -> Vec<Vec<usize>>; // returns all groups of vertices that are connected; makes no sense for directed graphs; default: bfs

    fn connected(&self, from: usize, to: usize) -> bool; // is a connected to b? default: bfs
}
