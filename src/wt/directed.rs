use crate::graph::directed::Digraph;
use crate::traits::{Directed, Graph, UnLabeled, Unweighted, WTDirected, WT};
use crate::Edit;
use core::panic;
use num::Zero; //todo: brauchen wir das noch?
use qwt::{AccessUnsigned, RankUnsigned, SelectUnsigned, QWT256};
use std::collections::{HashMap, VecDeque};
use vers_vecs::{BitVec, RsVec};
// 1 MAJOR if WTGraph has no edges, subtract overflow in qwt crate

// UNIT-TESTS for WT-Digraph and WT-Weighted Digraph
#[cfg(test)]
mod test;

/// A structure holding an immutable Wavelet-Tree-Representation of an indexed graph with directed edges, plus information on manual changes.
/// The greatest possible of number of edges or of vertices is usize vertices, vertex-indices are also usize-data-type.
pub struct WTDigraph {
    pub(crate) wt_adj_len: usize,                      // last index + 1
    e_count: usize,                                    // number of edges
    pub(crate) wt_adj_len_updated: usize,              // last index + 1 updated
    pub(crate) e_count_updated: usize,                 // number of edges
    wt_adj: QWT256<usize>,                             // the wavelet tree adjacency list
    starting_indices: RsVec,                           // starting indices of each
    pub(crate) deleted_vertices: HashMap<usize, bool>, // key: index of vertex, value true (meaning, vertex is deleted); can never be false (gets reset when shrinking but not when committing)
    // todo: change every function that uses this
    pub(crate) deleted_vertices_uncommitted: HashMap<usize, bool>, // saves only changes to deleted_vertices; true means vertex is deleted, false means it got readded
    // todo: change every function that uses this
    adj_uncommitted: HashMap<usize, Vec<Edit<usize>>>, // changes to outgoing edges
    pub(crate) has_uncommitted_edits: bool,
}

impl WTDigraph {
    /// this function needs documentation
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

    /// this function needs documentation
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
    /// this function needs documentation
    fn add_vertex(&mut self, vertex: usize) -> usize {
        // use at own risk
        self.has_uncommitted_edits = true;
        self.adj_uncommitted.insert(vertex, Vec::new());
        // case 1 : vertex does exist; deleted_vertices_uncommitted does not contain vertex
        // case 2 : vertex does exist, deleted_vertices_uncommitted does contain verte
        // case 3 : vertex doesn't exist, deleted_vertices_uncommitted does not contain vertex
        // case 4 : vertex doesn't exist, deleted_vertices_uncommitted does contain vertex

        if self.vertex_exists(vertex) {
            if self.deleted_vertices_uncommitted.contains_key(&vertex) {
                // case 2
                self.deleted_vertices_uncommitted.remove(&vertex);
                return vertex;
            } else {
                // case 1
                self.delete_incoming_edges(vertex);
                self.deleted_vertices_uncommitted.insert(vertex, false);
                return vertex;
            }
        } else {
            if self.deleted_vertices_uncommitted.contains_key(&vertex) {
                // case 4
                self.deleted_vertices_uncommitted.insert(vertex, false);
                // self.wt_adj_len_updated += 1;
                return vertex;
            } else {
                // case 3
                if vertex >= self.wt_adj_len_updated {
                    self.deleted_vertices_uncommitted.insert(vertex, false);
                    self.wt_adj_len_updated += vertex - self.wt_adj_len_updated + 1;
                    return vertex;
                } else {
                    self.deleted_vertices_uncommitted.insert(vertex, false);
                    return vertex;
                }
            }
        }
    }

    fn e_count(&self) -> usize {
        return self.e_count;
    }

    fn v_count(&self) -> usize {
        return self.wt_adj_len - self.deleted_vertices.len();
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        if !self.edge_exists_updated(from, to) {
            panic!("Edge from {} to {} doesn't exist!", from, to);
        }
        match self.adj_uncommitted.get_mut(&from) {
            Some(adj) => {
                let i: Option<usize> = adj.iter().position(|x| x == &Edit::Add(to)); // safe because we know it exists

                match i {
                    Some(_added_e) => {
                        adj.swap_remove(i.unwrap());
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

    fn delete_vertex(&mut self, vertex: usize) {
        if !(self.vertex_exists_updated(vertex)) {
            panic!("Vertex doesn't exist.");
        }

        self.deleted_vertices_uncommitted.insert(vertex, true);

        self.has_uncommitted_edits = true;
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        if self.deleted_vertices.contains_key(&vertex) {
            return false;
        }

        if vertex < self.wt_adj_len {
            return true;
        }

        return false;
    }

    /// this function needs documentation

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
    /// this function needs documentation
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
    /// this function needs documentation
    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        // returns a list of vertices that have outgoing edges to `vertex`
        if !self.vertex_exists(vertex) {
            panic!("incoming_edges: Vertex {} doesn't exist.", vertex);
        }
        if self.e_count == 0 {
            return Vec::new(); // if e_count is 0, number will result in subtract overflow
        }
        let mut incoming: Vec<usize> = Vec::new();
        let number: usize = self.wt_adj.rank(vertex, self.wt_adj.len()).unwrap(); // safe to unwrap because vertex exists

        for i in 1..number + 1 {
            let index_in_wt = self.wt_adj.select(vertex, i).unwrap();
            let pos_in_bitmap = self.starting_indices.select0(index_in_wt);
            let incoming_edge = self.starting_indices.rank1(pos_in_bitmap) - 1;
            incoming.push(incoming_edge);
        }

        return incoming;
    }

    fn delete_outgoing_edges(&mut self, vertex: usize) {
        if !self.vertex_exists(vertex) {
            panic!("delete_outgoing_edges: Vertex {} doesn't exist.", vertex);
        }

        let outgoing: Vec<usize> = self.outgoing_edges_updated(vertex);

        for to in outgoing {
            self.delete_edge(vertex, to); // this function call updates e_count_updated
        }

        self.has_uncommitted_edits = true;
    }

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

impl UnLabeled<usize> for WTDigraph {
    /// this function needs documentation
    fn append_vertex(&mut self) -> usize {
        // appends a vertex at the end of uncommitted_adj and returns the index
        return self.add_vertex(self.wt_adj_len_updated);
        // let index: usize = self.wt_adj_len_updated;

        // self.adj_uncommitted.insert(index, Vec::new());

        // self.wt_adj_len_updated += 1;

        // self.has_uncommitted_edits = true;
        // return index; // changed to index-1; that's a bug! I've changed it back! -Simon
    }

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
            if *change {
                // i.e. if it was deleted
                self.deleted_vertices.insert(*vertex, true);
            } else {
                // i.e. if it was readded
                self.deleted_vertices.remove(vertex);
            }
        }

        self.adj_uncommitted = HashMap::new(); // reset adj_uncommitted

        self.wt_adj = QWT256::new(&mut sequence);
        self.starting_indices = RsVec::from_bit_vec(bv);

        self.deleted_vertices = HashMap::new();
        self.discard_edits(); // reset all uncommitted changes
        return old_and_new_indices;
    }
}
impl Unweighted<usize> for WTDigraph {
    /// this function needs documentation
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
    /// this function needs documentation
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
            for i in 0..adj.len() {
                bv.append(false); // appends a 0 to bitmap for every element in adj
                sequence.push(adj[i]); // moves all elements of adj into sequence
            }
        }
        // apply all other changes
        self.wt_adj_len = self.wt_adj_len_updated;
        self.e_count = self.e_count_updated;
        // update deleted_vertices
        for (vertex, change) in self.deleted_vertices_uncommitted.iter() {
            if *change {
                // i.e. if it was deleted
                self.deleted_vertices.insert(*vertex, true);
            } else {
                // i.e. if it was readded
                self.deleted_vertices.remove(vertex);
            }
        }
        self.adj_uncommitted = HashMap::new(); // reset adj_uncommitted
        self.wt_adj = QWT256::new(&mut sequence);
        self.starting_indices = RsVec::from_bit_vec(bv);

        self.discard_edits(); // reset all uncommitted changes
    }

    // fn get_uncommitted_edits(&self) -> Option<HashMap<usize, usize>> {
    //     todo!()
    // }

    /// this function needs documentation
    fn discard_edits(&mut self) {
        // todo: make sure these are all fields with changes
        self.wt_adj_len_updated = self.wt_adj_len;
        self.e_count_updated = self.e_count;
        self.deleted_vertices_uncommitted = HashMap::new();
        self.adj_uncommitted = HashMap::new();
        self.has_uncommitted_edits = false;
    }

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

    fn edge_exists_updated(&self, from: usize, to: usize) -> bool {
        if !(self.vertex_exists_updated(from) && self.vertex_exists_updated(to)) {
            return false;
        }

        if self.outgoing_edges_updated(from).contains(&to) {
            return true;
        }
        return false;
    }

    fn v_count_updated(&self) -> usize {
        // wt_adj_len_updated = last index +1
        let mut v_count_updated = self.wt_adj_len_updated;
        // apply changes
        for deleted in self.deleted_vertices_uncommitted.values() {
            if *deleted {
                // deleted is true if a vertex was deleted
                v_count_updated -= 1;
            }
        }
        for deleted in self.deleted_vertices.values() {
            if *deleted {
                // deleted is true if a vertex was deleted
                v_count_updated -= 1;
            }
        }
        return v_count_updated;
    }
}

impl WTDirected<usize> for WTDigraph {
    /// this function needs documentation
    fn outgoing_edges_updated(&self, vertex: usize) -> Vec<usize> {
        if !self.vertex_exists_updated(vertex) {
            panic!("Vertex {vertex} doesn't exist!");
        }
        let mut outgoing: Vec<usize> = Vec::new();
        if self.vertex_exists(vertex) {
            outgoing = self.outgoing_edges(vertex);
        }
        let binding = Vec::new();
        let changes: &Vec<Edit<usize>> = self.adj_uncommitted.get(&vertex).unwrap_or(&binding); // if there are no changes, this is an empty list

        for change in changes {
            match change {
                Edit::Add(change) => {
                    outgoing.push(change.clone()); // changed change to change.clone() here and 4 lines below
                }
                Edit::Delete(change) => {
                    let index_of_change = outgoing.iter().position(|&x| x == change.clone()); // this returns the index of the first (and hopefully only) time, `change` appears in outgoing; panics, if change not in outgoing ... which shouldn't happen
                    match index_of_change {
                        Some(_) => {
                            outgoing.remove(index_of_change.unwrap());
                        }
                        None => {}
                    }
                }
            }
        }

        return outgoing;
    }
    /// this function needs documentation
    fn incoming_edges_updated(&self, vertex: usize) -> Vec<usize> {
        // this is a very expensive function!
        // It is strongly recommend to commit and call incoming_edges instead!
        if !self.vertex_exists_updated(vertex) {
            panic!("Vertex {vertex} doesn't exist!");
        }
        let mut incoming: Vec<usize> = Vec::new();
        if self.vertex_exists(vertex) {
            incoming = self.incoming_edges(vertex);
        }

        // this should be an empty list, if there are no incoming edges

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
