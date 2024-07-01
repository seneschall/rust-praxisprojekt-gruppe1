use crate::graph::directed::Digraph;
use crate::traits::{Directed, Graph, Unlabeled, Unweighted, WTDirected, WT};
use crate::Edit;
use core::panic;
use num::Zero; //todo: brauchen wir das noch?
use qwt::{AccessUnsigned, RankUnsigned, SelectUnsigned, QWT256};
use std::collections::{HashMap, VecDeque};
use vers_vecs::{BitVec, RsVec};
// 1 MAJOR if WTGraph has no edges, subtract overflow in qwt crate

#[cfg(test)]
mod test;

/// An indexed Wavelet-Tree-digraph with directed edges. (wt-digraph)
/// The wt-digraph holds data encoding an indexed, mutable digraph, from which we can build a QW-Tree (from QWT-crate), which is the core of the QT-graph.
/// The QW-Tree is immutable and enables us to perform traditional graph algorithms very fast.
/// Users can perform changes on the digraph, which will be recorded in the graph. Users can perfom fast operations on the original graph and slower operations on the recent state of the graph.
/// Users can integrate the recent state of the graph into the QW-Tree by rebuilding it using the commit_edits-function.
/// See more documentation on function-level and in the crate introduction.
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type.
pub struct WTDigraph {
    pub(crate) wt_adj_len: usize,           // last index + 1
    pub(crate)e_count: usize,                         // number of edges
    pub(crate) wt_adj_len_updated: usize,   // last index + 1 updated
    pub(crate) e_count_updated: usize,      // number of edges
    pub(crate) wt_adj: QWT256<usize>,                  // the wavelet tree adjacency list
    pub(crate) starting_indices: RsVec,                // starting indices of each
    pub(crate) deleted_vertices: HashMap<usize, bool>, // key: index of vertex, value true (meaning, vertex is deleted); can never be false (gets reset when shrinking but not when committing)
    // todo: change every function that uses this
    pub(crate) deleted_vertices_uncommitted: HashMap<usize, bool>, // saves only changes to deleted_vertices; true means vertex is deleted, false means it got readded
    // todo: change every function that uses this
    pub(crate) adj_uncommitted: HashMap<usize, Vec<Edit<usize>>>, // changes to outgoing edges
    pub(crate) has_uncommitted_edits: bool,
}

impl WTDigraph {
    
    /// this function instantiiates a wt-digraph from a given digraph   
    pub fn from_digraph(dg: Digraph) -> Self {
        let mut bv = BitVec::new();
        let mut e_count: usize = 0;
        let v_count = dg.adj.len();
        let mut sequence: Vec<usize> = Vec::new();

        for (v, v_adj) in dg.adj.iter().enumerate() {
            // iterate over all vertices (v) in adj
            bv.append(true);
            for val in v_adj.iter() {
                // iterate over the values in the adjacency list of v
                sequence.push(*val);
                bv.append(false); // append 0 to bv for each element in adjacency list of v
                e_count += 1;
            }
        }

        let starting_indices = RsVec::from_bit_vec(bv);

        let wt_adj: QWT256<usize> = QWT256::from(sequence);

        return WTDigraph {
            wt_adj_len: v_count,
            e_count,
            wt_adj_len_updated: v_count,
            e_count_updated: e_count,
            wt_adj,
            starting_indices,
            deleted_vertices: dg.deleted_vertices,
            adj_uncommitted: HashMap::new(),
            deleted_vertices_uncommitted: HashMap::new(), // changed from HashMap::new() to Vec::new()
            has_uncommitted_edits: false,
        };
    }

    /// this function builds a wt-digraph from a Vector sequence of bits, where each 1 represents a vertex, and each 0 another vertex adjacent to the first one (they are connected though an edge)
    /// and a usize-vector representing the id's (indices) of the adjacent vertices.
    pub fn from(sequence: Vec<usize>, starting_indices: RsVec) -> Self {
        let length = starting_indices.len();

        let v_count = starting_indices.rank1(length);

        let e_count = starting_indices.rank0(length);

        let wt_adj: QWT256<usize> = QWT256::from(sequence);

        return WTDigraph {
            wt_adj_len: v_count,
            e_count,
            wt_adj_len_updated: v_count,
            e_count_updated: e_count,
            wt_adj,
            starting_indices,
            deleted_vertices: HashMap::new(),
            adj_uncommitted: HashMap::new(),
            deleted_vertices_uncommitted: HashMap::new(), // changed from HashMap::new() to Vec::new()
            has_uncommitted_edits: false,
        };
    }
}

impl Graph<usize> for WTDigraph {

    /// use at own risk!
    /// adds a new empty vertex to the graph,
    /// by adding an empty vector at the given index, or overwriting the entry with the same key if existant.  
    /// adds several new empty Vertices if the given index exceeds the current v_count.
    /// returns the index of the new vertex
    // todo ! why does this return a usize?
    fn add_vertex(&mut self, vertex: usize) -> usize {

        self.adj_uncommitted.insert(vertex, Vec::new()); // wipes the outgoing edges of the vertex; the only valid

        if self.vertex_exists(vertex) {
            self.delete_incoming_edges(vertex);
            return;
        } else {
            // removes vertex from deleted_vertices
            self.deleted_vertices_uncommitted
        }

        self.v_count_updated += vertex - self.v_count_updated + 1; // if the index of the newly add vertex is greater than than self.v_count we need to add all virtual vertices up to the index of `vertex`
        self.has_uncommitted_edits = true;
    }

    /// return the number of edges in the graph at the last commit.
    fn e_count(&self) -> usize {
        return self.e_count;
    }

    /// returns the number of vertices in the graph at last commit
    fn v_count(&self) -> usize {
        return self.wt_adj_len - self.deleted_vertices.len();
    }


    /// deletes the edge from 'from' to 'to'. panics if edge doens't exist. 
    /// iterates over 'from's enty in 'adj_uncommited' and deletes entries who match Add(to).
    /// if not present, enters a new entry with 'from's key and Delete(to) in 'adj_uncommited'.
    fn delete_edge(&mut self, from: usize, to: usize) {
        if !self.edge_exists_updated(from, to) {
            panic!("Edge from {} to {} doesn't exist!", from, to);
        }

        match self.adj_uncommitted.get_mut(&from) {
            Some(adj) => {
                let i: Option<usize> = adj.iter().position(|&x| x == &Edit::Add(to)); // safe because we know it exists

                match i {
                    Some(added_e) => {
                        adj.swap_remove(i);
                    }
                    None => {}
                }

                adj.push(Edit::Delete(to));
                self.e_count_updated -= 1;
            }

            None => {
                self.adj_uncommitted.insert(from, vec![Edit::Delete(to)]);
                self.e_count_updated -= 1;
            }
        }

        self.has_uncommitted_edits = true;
    }

    /// deletes the vertex at the given index
    /// panics if the vertex doesn't exist - should eventually return a Result type
    /// if the vertex exists, we mark it in the 'deleted-vertices-uncommited'-Vector, and set 'has_uncommited_edits' to true.
    fn delete_vertex(&mut self, vertex: usize) {
        if !(self.vertex_exists_updated(vertex)) {
            panic!("Vertex doesn't exist.");
        }

        self.deleted_vertices_uncommitted.insert(vertex, true);

        self.has_uncommitted_edits = true;
    }


    /// checks if the given vertex exists
    fn vertex_exists(&self, vertex: usize) -> bool {
        if self.deleted_vertices.contains_key(&vertex) {
            return false;
        }

        if vertex < self.wt_adj_len {
            return true;
        }

        return false;
    }

    /// it removes all vertices in deleted_vertices from the graph, resets deleted_vertices, thus shrinking
    /// wt_adj_len, the updated v_count AND the v_count at last commit. does not commit changes other than vertex deletion. 
    /// does commit vertex-deletion and rebuild QW-tree. (expensive!)
    /// return a list containing the deleted vertices?
    fn shrink(&mut self) -> Vec<Option<usize>> {
        // somebody else should check this. -Simon
        let mut sequence: Vec<usize> = Vec::new();
        let mut bv = BitVec::new();

        let mut current_index: usize = 0;
        let mut old_and_new_indices: Vec<Option<usize>> = Vec::new();
        // I've decided to change to output to Vec<Option<usize>>, where the index represents the old indices of the vector
        // and the values are the new indices after the shrink
        // A value of `None` means that the old Index was deleted.

        for v in 0..self.wt_adj_len_updated {
            if !self.vertex_exists_updated(v) {
                old_and_new_indices.push(None);
                continue;
            }

            if v != 0 {
                // ugly but I can't think of a better solution right now
                current_index += 1; // only increase current index, if current vertex still exists
            }

            old_and_new_indices.push(Some(current_index));

            bv.append(true); // appends a 1 to mark the beginning of a new vertex; we only do this, if the vertex still exists

            let adj: Vec<usize> = self.outgoing_edges_updated(v);

            for i in 0..adj.len() - 1 {
                bv.append(false); // appends a 0 to bitmap for every element in adj
                sequence.push(adj[i]); // moves all elements of adj into sequence
            }
        }

        // apply all other changes
        self.wt_adj_len = self.wt_adj_len_updated;

        // update deleted_vertices
        for (vertex, change) in self.deleted_vertices_uncommitted.iter() {
            if change {
                // i.e. if it was deleted
                self.deleted_vertices.insert(vertex, true);
            } else {
                // i.e. if it was readded
                self.deleted_vertices.remove(vertex);
            }
        }

        self.adj_uncommitted = HashMap::new(); // reset adj_uncommitted

        self.wt_adj = QWT256::new(&mut sequence);
        self.starting_indices = bv;

        self.deleted_vertices = HashMap::new();
        self.discard_edits(); // reset all uncommitted changes
    }

    /// returns if there is an edge from `from` to `to`
    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if !(self.vertex_exists(from) && self.vertex_exists(to)) {
            return false;
        }

        if self.outgoing_edges(from).contains(&to) {
            return true;
        }

        return false;
    }
}
impl Directed<usize> for WTDigraph {

    /// return all outgoing edges of the given vertex in a vector
    /// should probably be changed to return an iterator instea
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        if !self.vertex_exists(vertex) {
            panic!("outgoing_edges: Vertex {} doesn't exist.", vertex);
        }

        let mut outgoing: Vec<usize> = Vec::new();

        let start = self.starting_indices.select1(vertex) - vertex;
        let end = self.starting_indices.select1(vertex + 1) - (vertex + 1);

        if start > self.wt_adj.len() || start == end {
            return Vec::new();
        }

        for i in start..end {
            outgoing.push(self.wt_adj.get(i).unwrap()); // is it safe to unwrap here? I think it should be
        }

        return outgoing;
    }
    
    /// return all outgoing edges of the given vertex in a vector
    /// should probably be changed to return an iterator instead
    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        // returns a list of vertices that have outgoing edges to `vertex`
        if !self.vertex_exists(vertex) {
            panic!("incoming_edges: Vertex {} doesn't exist.", vertex);
        }

        let mut incoming: Vec<usize> = Vec::new();
        let number: usize = self.wt_adj.rank(vertex, self.wt_adj.len()).unwrap(); // safe to unwrap because vertex exists

        for i in 1..number + 1 {
            let index_in_wt = self.wt_adj.select(vertex, i);
            let pos_in_bitmap = self.starting_indices.select0(index_in_wt);
            let incoming_edge = self.starting_indices.rank1(pos_in_bitmap) - 1;
            incoming.push(incoming_edge);
        }

        return incoming;
    }

    /// deletes all outgoing edges of the given vertex
    /// should return a Result
    fn delete_outgoing_edges(&mut self, vertex: usize) {
        if !self.vertex_exists(vertex) {
            panic!("delete_outgoing_edges: Vertex {} doesn't exist.", vertex);
        }

        let outgoing: Vec<usize> = self.outgoing_edges_updated(vertex);

        for from in outgoing {
            self.delete_edge(from, vertex); // this function call updates e_count_updated
        }

        self.has_uncommitted_edits = true;
    }

    /// deletes all incoming edges of the given vertex
    /// should return a Result
    fn delete_incoming_edges(&mut self, vertex: usize) {
        if !self.vertex_exists(vertex) {
            panic!("incoming_edges: Vertex {} doesn't exist.", vertex);
        }

        let incoming: Vec<usize> = self.incoming_edges_updated(vertex); // empty list if there are no incoming edges

        for from in incoming {
            self.delete_edge(from, vertex); // this function call updates e_count_updated
        }

        self.has_uncommitted_edits = true;
    }
}

impl Unlabeled<usize> for WTDigraph {
    
    /// adds a new empty vertex at either the index following the last or at (the lowest available) previously freed index.
    /// preserves indexing and never overwrites vertices
    /// append_vertex() is not defined for labeled graphs
    /// returns the index of the new vertex
    fn append_vertex(&mut self) -> usize {
        // appends a vertex at the end of uncommitted_adj and returns the index

        let index: usize = self.wt_adj_len_updated;

        self.adj_uncommitted.insert(index, Vec::new());

        self.wt_adj_len_updated += 1;

        self.has_uncommitted_edits = true;
        return index; // changed to index-1; that's a bug! I've changed it back! -Simon
    }
}
impl Unweighted<usize> for WTDigraph {
    
    /// adds an edge between the vertices 'from' and 'to', by adding an edge from the smaller to the bigger indice in the dg.
    fn add_edge(&mut self, from: usize, to: usize) {
        // only adds to uncommitted edits
        // todo; its possible to add the same edge multiple times
        if !(self.vertex_exists_updated(from) && self.vertex_exists_updated(to)) {
            panic!("Failed to add edge.");
        }

        if self.edge_exists_updated(from, to) {
            panic!("Edge already exists.");
        }

        match self.adj_uncommitted.get_mut(&from) {
            Some(adj) => {
                adj.push(Edit::Add(to));
            }
            None => {
                self.adj_uncommitted.insert(from, vec![Edit::Add(to)]);
            }
        }

        self.has_uncommitted_edits = true;
        self.e_count_updated += 1; // added this line, else adding an edge doesn't update e_count_updated
    }
}

impl WT<usize> for WTDigraph {

    /// collect and apply all changes in adj_uncommited. rebuild QW-tree. expensive!
    /// set v_count to v_count_updated, e_count to e_count_updated, if present change labels, weights [...].
    /// some changes like deleted vertices are conserved
    fn commit_edits(&mut self) {
        // build new sequence and bitvec

        let mut sequence: Vec<usize> = Vec::new();
        let mut bv = BitVec::new();

        for v in 0..self.wt_adj_len_updated {
            bv.append(true); // appends a 1 to mark the beginning of a new vertex

            if !self.vertex_exists_updated(v) {
                continue;
            }

            let adj: Vec<usize> = self.outgoing_edges_updated(v);

            for i in 0..adj.len() - 1 {
                bv.append(false); // appends a 0 to bitmap for every element in adj
                sequence.push(adj[i]); // moves all elements of adj into sequence
            }
        }

        // apply all other changes
        self.wt_adj_len = self.wt_adj_len_updated;

        // update deleted_vertices
        for (vertex, change) in self.deleted_vertices_uncommitted.iter() {
            if change {
                // i.e. if it was deleted
                self.deleted_vertices.insert(vertex, true);
            } else {
                // i.e. if it was readded
                self.deleted_vertices.remove(vertex);
            }
        }

        self.adj_uncommitted = HashMap::new(); // reset adj_uncommitted

        self.wt_adj = QWT256::new(&mut sequence);
        self.starting_indices = bv;

        self.discard_edits(); // reset all uncommitted changes
    }


    /// this function will delete all changes since last commit by resetting the wt-digraph to its state after the last commit.
    fn discard_edits(&mut self) {
        // todo: make sure these are all fields with changes
        self.wt_adj_len_updated = self.wt_adj_len;
        self.e_count_updated = self.e_count;
        self.deleted_vertices_uncommitted = Vec::new();
        self.adj_uncommitted = HashMap::new();
        self.has_uncommitted_edits = false;
    }

    /// return true if the vertex still exists and wasn't deleted, or if it was created since since last commit.
    fn vertex_exists_updated(&self, vertex: usize) -> bool {
        // first we check if the vertex was deleted or added since last commit

        let change: Option<&bool> = self.deleted_vertices_uncommitted.get(&vertex);

        match change {
            Some(cng) => {
                return !cng; // if cng is true, vertex doesn't exist and it exists if it is false
            }
            None => {} // change is None if there were no changes to the entry of vertex in deleted_vertices since last commit
        }

        // if there wasn't a change, we check, whether the vertex was deleted in last commit
        // and whether it exists in theory, because the index is less the wt_adj_len_updated

        // explanation of the following logic: if the vertex exists in theory, because vertex < wt_adj_len_updated
        // then we only need to check whether it was deleted before last commit
        // if was, then deleted_vertices contains the index of vertex, so we need to flip that output
        // resulting in true && false if it was deleted before last commit
        return (vertex < self.wt_adj_len_updated) && !self.deleted_vertices.contains_key(&vertex);
    }

    /// return true if the edge still exists and wasn't deleted, or if it was created since since last commit.
    fn edge_exists_updated(&self, from: usize, to: usize) -> bool {
        if !(self.vertex_exists_updated(from) && self.vertex_exists_updated(to)) {
            return false;
        }

        if self.updated_outgoing_edges(from).contains(&to) {
            return true;
        }
        return false;
    }

    /// return the recent number of vertices in the graph
    fn v_count_updated(&self) -> usize {
        let mut v_count = self.v_count();

        // apply changes
        for deleted in self.deleted_vertices_uncommitted.values() {
            if deleted {
                // deleted is true if a vertex was deleted
                v_count -= 1;
            } else {
                // deleted is false if a vertex was readded
                v_count += 1;
            }
        }
        return v_count;
    }
}

impl WTDirected<usize> for WTDigraph {

    /// return all outgoing edges of the given vertex in a vector, which exist and weren't deleted, or were created since since last commit.
    /// should probably be changed to return an iterator instead
    fn outgoing_edges_updated(&self, vertex: usize) -> Vec<usize> {
        if !self.vertex_exists_updated(vertex) {
            panic!("Vertex {vertex} doesn't exist!");
        }

        let mut outgoing: Vec<usize> = self.outgoing_edges(vertex);

        let changes: &Vec<Edit<usize>> = self.adj_uncommitted.get(&vertex).unwrap_or(&Vec::new()); // if there are no changes, this is an empty list

        for change in changes {
            match change {
                Edit::Add(change) => {
                    outgoing.push(change.clone()); // changed change to change.clone() here and 4 lines below
                }
                Edit::Delete(change) => {
                    let index_of_change: usize =
                        outgoing.iter().position(|&x| x == change.clone()).unwrap(); // this returns the index of the first (and hopefully only) time, `change` appears in outgoing; panics, if change not in outgoing ... which shouldn't happen
                    outgoing.remove(index_of_change);
                }
            }
        }

        return outgoing;
    }
    
    /// return all incoming edges of the given vertex in a vector, which exist and weren't deleted, or were created since since last commit.
    /// should probably be changed to return an iterator instead
    fn incoming_edges_updated(&self, vertex: usize) -> Vec<usize> {
        // this is a very expensive function!
        // It is strongly recommend to commit and call incoming_edges instead!
        if !self.vertex_exists_updated(vertex) {
            panic!("Vertex {vertex} doesn't exist!");
        }

        let mut incoming: Vec<usize> = self.incoming_edges(vertex); // this should be an empty list, if there are no incoming edges

        let mut changes: Vec<Edit<usize>> = Vec::new(); // changed from 'let mut changes: &Vec<Edit<usize>>;'

        // we need to iterate over every vertex that has been changed since the last commit,
        // unwrap every change and see whether it contains our vertex
        for (v, v_adj) in self.adj_uncommitted.iter() {
            for change in v_adj {
                // check whether the change contains our `vertex`
                match change {
                    Edit::Add(w) => {
                        if w == &vertex {
                            changes.push(Edit::Add(v.clone())); // if change contains `vertex`, `vertex`'s incoming edge from v has changed
                        };
                    }
                    Edit::Delete(w) => {
                        if w == &vertex {
                            changes.push(Edit::Delete(v.clone())); // changed v to v.clone() here and above
                        };
                    }
                }
            }
        }

        // if I haven't missed anything, that should be all changes to the incoming edges
        // now we need to apply the changes:

        for change in changes {
            // used binding `changes` isn't initialized ; `changes` used here but it isn't initialized
            match change {
                Edit::Add(change) => {
                    incoming.push(change.clone()); // changed change to change.clone() here and 4 lines below
                }
                Edit::Delete(change) => {
                    let index_of_change: usize =
                        incoming.iter().position(|&x| x == change.clone()).unwrap(); // this returns the index of the first (and hopefully only) time, `change` appears in incoming; panics, if change not in outgoing ... which shouldn't happen
                    incoming.remove(index_of_change);
                }
            }
        }

        return incoming;
    }
}

// Optional methods:

// impl<L> GraphSearch for WTDigraph<L> {
//     fn connected(&self, from: usize, to: usize) -> bool {
//         // is a connected to b?
//         let mut list_of_outgoing_edges: VecDeque<usize> = VecDeque::new();
//         let mut visited: Vec<usize> = Vec::new();
//         list_of_outgoing_edges.append(&mut self.outgoing_edges(from).into());
//         visited.push(from);
//         while !list_of_outgoing_edges.is_empty() {
//             let v = list_of_outgoing_edges.pop_front().unwrap();
//             visited.push(v);
//             if v == to {
//                 return true;
//             }
//             for item in self.outgoing_edges(v) {
//                 if !visited.contains(&item) {
//                     // if vertex was not yet visited, add it to the queue
//                     if !list_of_outgoing_edges.contains(&item) {
//                         list_of_outgoing_edges.push_back(item);
//                     }
//                 }
//             }
//         }
//         false
//     }

//     fn shortest_path(&self, from: usize, to: usize, mode: ShortestPathAlgorithm) -> Vec<usize> {
//         todo!()
//     }

//     fn shortest_paths(&self, mode: ShortestPathAlgorithm) -> Vec<Vec<usize>> {
//         todo!()
//     }

//     fn connected_components(&self) -> Vec<Vec<usize>> {
//         todo!()
//     }
// }

// WT-Weighted Digraph - definition and methods
