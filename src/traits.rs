// todo: change T to usize
use std::collections::HashMap;

use num::{ToPrimitive, Unsigned};

pub trait WTGraph<T>
where
    T: Unsigned + ToPrimitive,
{
    fn updated_edges(&self, v: T) -> Option<Vec<T>>;
}

pub trait WT<T>
where
    T: Unsigned + ToPrimitive,
{
    fn commit_edits(&self);

    fn get_uncommitted_edits(&self) -> Option<HashMap<T, L>>;

    fn discard_edits(&self);

    fn shrink(&mut self) -> HashMap<T, T>; // removes all unconnected vertices from bitmap; only allowed, if has_uncommitted_edits == false; returns a Hashmap with old indices as keys and new indices as values
}

pub trait WTDigraph<T>
where
    T: Unsigned + ToPrimitive,
{
    fn updated_outgoing_edges(&self, v: T) -> Option<Vec<T>>;

    fn updated_incoming_edges(&self, v: T) -> Option<Vec<T>>;
}

pub trait Graph<T, L>
where
    T: Unsigned + ToPrimitive,
{
    fn add_edge(&mut self, v: T, w: T);

    fn add_vertex(&mut self, v: T); // adds vertex at given index; use at users own risk; if vertex exists (i.e. vertex is less than wt_adj.len()), it just adds it, if it does, it must not have incoming or outgoing edges

    fn add_vertex_label(&mut self, v: T, label: L);

    fn append_vertex(&mut self, v: T) -> T; // adds vertex at position wt_adj.len() or at index of lowest deleted vertex (if that change hasn't been committed)

    fn delete_edge(&mut self, v: T, w: T);

    fn delete_vertex(&mut self, v: T); // should eventually be changed to return a Result type

    fn e_count(&self) -> T; // should eventually be changed to return a Result type

    fn edit_label(&mut self, v: T, change: L); // true if last item in uncommitted edits for v is Edit::DeleteSelf

    fn get_label(&self, v: T) -> Option<&L>;

    fn v_count(&self) -> T;

    fn vertex_deleted(&self, v: T) -> bool;
}

pub trait Directed<T>
where
    T: Unsigned + ToPrimitive,
{
    fn outgoing_edges(&self, vertex: T) -> Vec<T>; // should probably be changed to return an iterator instead

    fn incoming_edges(&self, vertex: T) -> Vec<T>; // likewise here
}

pub trait Undirected<T>
where
    T: Unsigned + ToPrimitive,
{
    fn edges(&self, vertex: T) -> Vec<T>;
}

pub trait Weighted<T>
where
    T: Unsigned + ToPrimitive,
{
    fn weight_of_edge(&self, from: T, to: T) -> f64;
}

pub enum ShortestPathAlgorithm {
    Dijkstra,
    BFS,
    BellmanFord,
    AStar,
}

pub trait GraphSearch<T>
where
    T: Unsigned + ToPrimitive,
{
    fn shortest_path(&self, from: T, to: T, mode: ShortestPathAlgorithm) -> Vec<T>; // returns the shortest path from `from` to `to` using breadth first search

    fn shortest_paths(&self, mode: ShortestPathAlgorithm) -> Vec<Vec<T>>; // shortest paths from all vertices to all other vertices

    fn connected_components(&self) -> Vec<Vec<T>>; // returns all groups of vertices that are connected; makes no sense for directed graphs; default: bfs

    fn connected(&self, a: T, b: T) -> bool; // is a connected to b? default: bfs
}
