use crate::graph::directed::Digraph;
use crate::traits::{Directed, Graph, UnLabeled, Unweighted, WTDirected, WT};
use crate::Edit;
use num::Zero;
use qwt::{AccessUnsigned, RankUnsigned, SelectUnsigned, QWT256};
use std::collections::{HashMap, VecDeque};
use vers_vecs::{BitVec, RsVec};
// 1 MAJOR if WTGraph has no edges, subtract overflow in qwt crate

// UNIT-TESTS for WT-Digraph and WT-Weighted Digraph
#[cfg(test)]
mod test;

// // WT-Digraph - definition and methods
pub struct WTDigraph {
    v_count: usize,                                    // number of vertices
    e_count: usize,                                    // number of edges
    pub(crate) v_count_updated: usize,                 // number of vertices
    pub(crate) e_count_updated: usize,                 // number of edges
    wt_adj: QWT256<usize>,                             // the wavelet tree adjacency list
    starting_indices: RsVec,                           // starting indices of each
    deleted_vertices: Vec<usize>, // a list containing the indices of all vertices that were deleted (gets reset when shrinking)
    uncommitted_deleted_vertices: Vec<Edit<usize>>, // changes to vertices
    uncommitted_adj: HashMap<usize, Vec<Edit<usize>>>, // changes to outgoing edges
    has_uncommitted_edits: bool,
}

impl WTDigraph {
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
            v_count,
            e_count,
            v_count_updated: v_count,
            e_count_updated: e_count,
            wt_adj,
            starting_indices,
            deleted_vertices: dg.deleted_vertices,
            uncommitted_adj: HashMap::new(),
            uncommitted_deleted_vertices: Vec::new(), // changed from HashMap::new() to Vec::new()
            has_uncommitted_edits: false,
        };
    }

    pub fn from(sequence: Vec<usize>, starting_indices: RsVec) -> Self {
        let length = starting_indices.len();

        let v_count = starting_indices.rank1(length);

        let e_count = starting_indices.rank0(length);

        let wt_adj: QWT256<usize> = QWT256::from(sequence);

        return WTDigraph {
            v_count,
            e_count,
            v_count_updated: v_count,
            e_count_updated: e_count,
            wt_adj,
            starting_indices,
            deleted_vertices: Vec::new(),
            uncommitted_adj: HashMap::new(),
            uncommitted_deleted_vertices: Vec::new(), // changed from HashMap::new() to Vec::new()
            has_uncommitted_edits: false,
        };
    }
}

impl Graph<usize> for WTDigraph {
    fn add_vertex(&mut self, vertex: usize) -> usize {
        // use at own risk
        // if vertex already exists, this deletes all outgoing and incoming edges
        // if the index is greater than v_count_updated, it just adds the vertex and raises the v_count
        // to the index of vertex + 1
        // when committing, empty vertices will be inserted
        // changed alot
        if vertex >= self.v_count_updated {
            for i in 0..vertex - self.v_count_updated + 1 {
                self.uncommitted_adj
                    .insert(self.v_count_updated + i, Vec::new()); // wipes the outgoing edges of the vertex; the only valid
            }
            self.v_count_updated += vertex - self.v_count_updated + 1;
        } else {
            self.uncommitted_adj.insert(vertex, Vec::new()); // wipes the outgoing edges of the vertex; the only valid
        }
        self.v_count_updated - 1
    }

    fn e_count(&self) -> usize {
        self.e_count
    }

    fn v_count(&self) -> usize {
        self.v_count
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        //either this or we need vertex_deleted_updated
        // todo: do we need this function? We already have vertex_exists -Simon
        if self.deleted_vertices.contains(&vertex)
            || self
                .uncommitted_deleted_vertices
                .contains(&Edit::Add(vertex))
        // changed this to Edit::Add(vertex) -Simon
        {
            true
        } else {
            false
        }
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        if !self.edge_exists_updated(from, to) {
            panic!("Edge from {} to {} doesn't exist!", from, to);
        }

        match self.uncommitted_adj.get_mut(&from) {
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
                self.uncommitted_adj.insert(from, vec![Edit::Delete(to)]);
                self.e_count_updated -= 1;
            }
        }

        self.has_uncommitted_edits = true;
    }

    fn delete_vertex(&mut self, vertex: usize) {
        if !(self.vertex_exists_updated(vertex)) {
            panic!("Vertex doesn't exist.");
        }

        self.uncommitted_deleted_vertices.push(Edit::Delete(vertex)); // changed push(vertex) to push(Edit::Delete(vertex))

        // self.v_count_updated -= 1;
        self.has_uncommitted_edits = true;
    }
    // checked up to here
    fn vertex_exists(&self, vertex: usize) -> bool {
        if self.deleted_vertices.contains(&vertex) {
            return false;
        }

        if vertex < self.v_count {
            return true;
        }

        return false;
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if self.outgoing_edges(from).contains(&to) {
            return true;
        }
        return false;
    }
}
impl Directed<usize> for WTDigraph {
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        let mut v_adj: Vec<usize> = Vec::new();
        let v = vertex;

        let start = self.starting_indices.select1(v) - v;
        let end = self.starting_indices.select1(v + 1) - (v + 1);

        if start > self.wt_adj.len() || start == end {
            return Vec::new();
        }

        for i in start..end {
            v_adj.push(self.wt_adj.get(i).unwrap()); // is it safe to unwrap here? I think it should be
        }

        return v_adj;
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        // returns a list of vertices that have outgoing edges to `vertex`
        let mut v_inc: Vec<usize> = Vec::new();
        let number: Option<usize> = self.wt_adj.rank(vertex, self.wt_adj.len());
        if number.is_some() {
            for i in 1..number.unwrap() + 1 {
                let indeximwt = self.wt_adj.select(vertex, i).unwrap();
                let posinbitmap = self.starting_indices.select0(indeximwt);
                let einsenzaehlen = self.starting_indices.rank1(posinbitmap) - 1;
                v_inc.push(einsenzaehlen);
                //v_inc.push(self.starting_indices.rank1(self.starting_indices.select0(self.wt_adj.select(vertex, i).unwrap()),) - 1)
            }
        }
        v_inc
    }

    fn delete_outgoing_edges(&mut self, vertex: usize) {
        let outgoing: Vec<usize> = self.outgoing_edges_updated(vertex);
        let num_of_deleted_vertices = outgoing.len() - 1;
        for from in outgoing {
            self.delete_edge(from, vertex);
        }
        self.e_count_updated -= num_of_deleted_vertices; // this line breaks it
                                                         // todo: this line breaks what? -Simon
        self.has_uncommitted_edits = true;
    }

    fn delete_incoming_edges(&mut self, vertex: usize) {
        let incoming: Vec<usize> = self.incoming_edges_updated(vertex);
        if incoming.is_empty() {
            return;
        } // if incoming is empty next line is subtract overflow

        let num_of_deleted_vertices = incoming.len() - 1;

        for from in incoming {
            self.delete_edge(from, vertex);
        }

        self.e_count_updated -= num_of_deleted_vertices; // this line breaks e_count_updated
                                                         // todo: how does it break it? -Simon
        self.has_uncommitted_edits = true;
    }
}
impl UnLabeled<usize> for WTDigraph {
    fn append_vertex(&mut self) -> usize {
        // appends a vertex at the end of uncommitted_adj and returns the index

        let index: usize = self.v_count_updated;

        self.uncommitted_adj.insert(index, Vec::new());

        self.v_count_updated += 1;

        self.has_uncommitted_edits = true;
        return index; // changed to index-1; that's a bug! I've changed it back! -Simon
    }
}
impl Unweighted<usize> for WTDigraph {
    fn add_edge(&mut self, from: usize, to: usize) {
        // only adds to uncommitted edits
        // todo; its possible to add the same edge multiple times
        if !(self.vertex_exists_updated(from) && self.vertex_exists_updated(to)) {
            panic!("Failed to add edge.");
        }

        if self.edge_exists_updated(from, to) {
            panic!("Edge already exists.");
        }

        match self.uncommitted_adj.get_mut(&from) {
            Some(adj) => {
                adj.push(Edit::Add(to));
            }
            None => {
                self.uncommitted_adj.insert(from, vec![Edit::Add(to)]);
            }
        }

        self.has_uncommitted_edits = true;
        self.e_count_updated += 1; // added this line, else adding an edge doesn't update e_count_updated
    }
}

impl WT<usize> for WTDigraph {
    fn commit_edits(&mut self) {}

    // fn get_uncommitted_edits(&self) -> Option<HashMap<usize, usize>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        self.v_count_updated = self.v_count();
        self.e_count_updated = self.e_count();
        self.deleted_vertices = Vec::new();
        self.uncommitted_deleted_vertices = Vec::new();
        self.uncommitted_adj = HashMap::new();
        self.has_uncommitted_edits = false;
    }

    fn vertex_exists_updated(&self, vertex: usize) -> bool {
        if self
            .uncommitted_deleted_vertices
            .contains(&(Edit::Add(vertex)))
        {
            return false;
        }

        if self
            .uncommitted_deleted_vertices
            .contains(&(Edit::Delete(vertex)))
        {
            return true;
        }
        // problem here
        // v_count_updated counts the current number of valid vertices
        // after deleting vertices v_count_updated is wrong, since it decreases
        // v_count_updated maybe shouldn decrease?
        // it shouldn't decrease. -Simon
        if vertex < self.v_count_updated && self.vertex_exists(vertex) {
            return true;
        }

        return false;
    }

    fn edge_exists_updated(&self, from: usize, to: usize) -> bool {
        if self.updated_outgoing_edges(from).contains(&to) {
            return true;
        }
        return false;
    }
}

impl WTDirected<usize> for WTDigraph {
    fn outgoing_edges_updated(&self, vertex: usize) -> Vec<usize> {
        if !self.vertex_exists_updated(vertex) {
            panic!("Vertex {vertex} doesn't exist!");
        }

        let mut outgoing: Vec<usize> = self.outgoing_edges(vertex);

        let changes: &Vec<Edit<usize>> = self.uncommitted_adj.get(&vertex).unwrap_or(&Vec::new()); // if there are no changes, this is an empty list

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

    fn incoming_edges_updated(&self, vertex: usize) -> Vec<usize> {
        // this is a very expensive function!
        // I would strongly recommend to commit before trying to use this!
        if !self.vertex_exists_updated(vertex) {
            panic!("Vertex {vertex} doesn't exist!");
        }

        let mut incoming: Vec<usize> = self.incoming_edges(vertex);

        let mut changes: Vec<Edit<usize>> = Vec::new(); // changed from 'let mut changes: &Vec<Edit<usize>>;'

        // we need to iterate over every vertex that has been changed since the last commit,
        // unwrap every change and see whether it contains our vertex
        for (v, v_adj) in self.uncommitted_adj.iter() {
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
