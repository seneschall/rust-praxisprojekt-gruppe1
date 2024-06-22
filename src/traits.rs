use num::{ToPrimitive, Unsigned};
use std::collections::HashMap;

// The user only has two options: use labels for all vertices or for none.

// this trait applies to all graph structures
pub trait Graph<T> {
    fn add_edge(&mut self, from: T, to: T);

    fn add_vertex(&mut self, vertex: T) -> usize; // adds vertex at given index; use at users own risk; if vertex doesn't exist (i.e. vertex is less than wt_adj.len()), it just adds it, if it does, it must not have incoming or outgoing edges

    fn e_count(&self) -> usize;

    fn v_count(&self) -> usize; // returns the number of vertices in graph

    fn vertex_exists(&self, vertex: T) -> bool;
    // append can be added additionaly for unlabeled graph
    //fn append_vertex(&mut self) -> usize; // adds vertex at position wt_adj.len() or at index of lowest deleted vertex (if that change hasn't been committed)
}
pub trait Labeled<L> {
    fn edit_label(&mut self, old_label: L, new_label: L); // true if last item in uncommitted edits for v is Edit::DeleteSelf; should return a Result

    fn get_label(&self, vertex: usize) -> Option<&L>; // O(N) complexity

    fn get_index(&self, label: L) -> Option<&usize>; // returns the index of the vertex with the given label
}
pub trait Delete<T> {
    // decide later whether to implement delete_ledge etc.
    // this trait must not be implemented by WT graphs

    fn delete_vertex(&mut self, vertex: T); // should eventually be changed to return a Result type

    fn vertex_deleted(&self, vertex: T) -> bool;
}

// this trait applies to undirected graph structures
pub trait Undirected<T> {
    fn edges(&self, vertex: T) -> Vec<T>; // returns all edges connected to vertex

    fn delete_edge(&mut self, vertex: T);

    fn delete_edges_from(&self, vertex: T); // deletes all edges connected to vertex
}

// this trait applies to directed graph structures
pub trait Directed<T> {
    fn delete_edge(&mut self, from: T, to: T);

    fn outgoing_edges(&self, vertex: T) -> Vec<T>; // should probably be changed to return an iterator instead

    fn incoming_edges(&self, vertex: T) -> Vec<T>; // likewise here

    fn delete_outgoing_edges(&mut self, vertex: T); // deletes all outgoing edges of vertex; should return a Result

    fn delete_incoming_edges(&mut self, vertex: T); // deletes all incoming edges of vertex; should return a Result
}

// this trait applies to weighted graph structures
pub trait Weighted<T, W> {
    // Weights implemented as generic, certain functions only possible if W is number
    fn add_edge(&mut self, from: T, to: T, weight: W);

    fn add_vertex(&mut self, vertex: T) -> usize; // adds vertex at given index; use at users own risk; if vertex doesn't exist (i.e. vertex is less than wt_adj.len()), it just adds it, if it does, it must not have incoming or outgoing edges

    fn e_count(&self) -> usize;

    fn v_count(&self) -> usize; // returns the number of vertices in graph

    fn edit_weight(&mut self, from: T, to: T, weight: W); // todo: Result; only possible if has_uncommitted_changes == false

    fn get_weight(&mut self, from: T, to: T) -> W;
}

//  this trait applies to all WT graph structures
pub trait WT<T> {
    fn commit_edits(&mut self);

    fn get_uncommitted_edits(&self) -> Option<HashMap<usize, T>>;

    fn discard_edits(&mut self);

    fn shrink(&mut self) -> HashMap<usize, usize>; // removes all unconnected vertices from bitmap; only allowed, if has_uncommitted_edits == false; returns a Hashmap with old indices as keys and new indices as values
}

// this trait applies to undirected WT graph structures
pub trait WTUndirected<T> {
    fn updated_edges(&self, vertex: usize) -> Vec<T>;
}

// this trait applies to directed WT structures
pub trait WTDirected<T> {
    fn updated_outgoing_edges(&self, vertex: T) -> Vec<T>; // if there are no outgoing edges, this returns an empty list

    fn updated_incoming_edges(&self, vertex: T) -> Vec<T>; // if there are no outgoing edges, this returns an empty list
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
