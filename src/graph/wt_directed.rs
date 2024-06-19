use super::Edit; // moved Edit enum to graph.rs because it's also needed for wt_undirected -Simon
use crate::graph::directed::Digraph;
use crate::traits::*;
// use num::{cast::AsPrimitive, FromPrimitive, PrimInt, ToPrimitive, Unsigned};
use qwt::{AccessUnsigned, RankUnsigned, SelectUnsigned, QWT256};
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};
use vers_vecs::{BitVec, RsVec};

// UNIT-TESTS for WT-Digraph and WT-Weighted Digraph
#[cfg(test)]
mod test {
    use super::*;
    const V_COUNT: usize = 10;
}

// WT-Digraph - definition and methods
pub struct WTDigraph<L> {
    v_count: usize,                               // number of vertices
    e_count: usize,                               // number of edges
    v_count_updated: usize,                       // number of vertices
    e_count_updated: usize,                       // number of edges
    wt_adj: QWT256<usize>,                        // the wavelet tree adjacency list
    starting_indices: RsVec,                      // starting indices of each
    uncommitted_edits: HashMap<usize, Vec<Edit>>, // changes not yet committed to sequence
    has_uncommitted_edits: bool,
    vertex_labels: HashMap<L, usize>, // name given to vertex format: {label : index}
}

impl<L> WTDigraph<L> {
    pub fn from_digraph(dg: Digraph<L>) -> Self {
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
            wt_adj, // here sequence would be replaced by wavelet tree ...
            starting_indices,
            uncommitted_edits: HashMap::new(),
            has_uncommitted_edits: false,
            vertex_labels: HashMap::new(),
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
            uncommitted_edits: HashMap::new(),
            has_uncommitted_edits: false,
            vertex_labels: HashMap::new(),
        };
    }
}

impl<L> Graph<L> for WTDigraph<L>
where
    L: Clone + Eq + PartialEq + Hash,
{
    fn add_edge(&mut self, from: usize, to: usize) {
        // only adds to uncommitted edits

        match self.uncommitted_edits.get_mut(&from) {
            Some(adj) => {
                adj.push(Edit::Add(to));
            }
            None => {
                self.uncommitted_edits.insert(from, vec![Edit::Add(to)]);
            }
        }

        self.has_uncommitted_edits = true;
    }

    fn add_vertex(&mut self, vertex: usize) {
        // todo: write comment explaining what this does
        self.uncommitted_edits.insert(vertex, vec![Edit::AddSelf]); // wipes the outgoing edges of the vertex; the only valid

        if (vertex < self.v_count_updated) && !(self.vertex_deleted(vertex)) {
            // should just call self.delete_incoming_edges()
            // but that can be implemented with the following logic
            for from in self.updated_incoming_edges(vertex) {
                // deletes all incoming edges of vertex
                // this would only makes sense if the vertex potentially has some
                self.delete_edge(from, vertex);
            }
            return;
        }
        self.v_count_updated += vertex - self.v_count_updated + 1; // if the index of the newly add vertex is greater than than self.v_count we need to add all virtual vertices up to the index of `vertex`
    }

    fn add_label(&mut self, vertex: usize, label: L) {
        if vertex > self.v_count - 1 || self.vertex_deleted(vertex) {
            panic!("Vertex doesn't exist.");
        }

        self.vertex_labels.insert(label, vertex);
    }
    fn append_vertex(&mut self) -> usize {
        todo!()
    }

    fn e_count(&self) -> usize {
        self.e_count
    }

    fn edit_label(&mut self, vertex: usize, change: L) {
        self.vertex_labels.insert(change, vertex);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        let label: Option<&L> =
            self.vertex_labels
                .iter()
                .find_map(|(key, &value)| if vertex == value { Some(key) } else { None });
        return label;
    }

    fn v_count(&self) -> usize {
        self.v_count
    }

    fn add_ledge(&mut self, from: L, to: L) {
        todo!()
    }

    fn add_lvertex(&mut self, label: L) {
        todo!()
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        self.vertex_labels.get(&label)
    }
}

impl<L> WTDelete<L> for WTDigraph<L> {
    fn delete_edge(&mut self, from: usize, to: usize) {
        match self.uncommitted_edits.get_mut(&from) {
            Some(adj) => {
                adj.push(Edit::Add(to));
            }
            None => {
                self.uncommitted_edits.insert(from, vec![Edit::Delete(to)]);
            }
        }

        self.has_uncommitted_edits = true;
    }

    fn delete_vertex(&mut self, vertex: usize) {
        if vertex > self.v_count - 1 {
            panic!("Vertex doesn't exist.");
        }

        match self.uncommitted_edits.get_mut(&vertex) {
            Some(adj) => {
                adj.push(Edit::Add(vertex));
            }
            None => {
                self.uncommitted_edits
                    .insert(vertex, vec![Edit::Add(vertex)]);
            }
        }
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        let last_opt: Option<&Edit>;

        match self.uncommitted_edits.get(&vertex) {
            Some(adj) => {
                last_opt = adj.last();
            }
            None => {
                return false;
            }
        }

        let last: &Edit;

        match last_opt {
            Some(edit) => {
                last = edit;
            }
            None => {
                // last_opt will be None if adj of vertex is an empty list
                // in that case the vertex wasn't deleted
                return false;
            }
        }

        match last {
            Edit::DeleteSelf => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

impl<L> Directed<L> for WTDigraph<L> {
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        let mut v_adj: Vec<usize> = Vec::new();
        let v = vertex; // this won't work if v is of type u128

        let start = self.starting_indices.select1(v) - v; // this won't work if v is of type u128
        let end = self.starting_indices.select1(v + 1) - (v + 1); // neither will this

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
        let number: usize = (self.wt_adj.rank(vertex, self.wt_adj.len())).unwrap() + 1;

        for i in 1..number {
            let indeximwt = self.wt_adj.select(vertex, i).unwrap();
            let posinbitmap = self.starting_indices.select0(indeximwt);
            let einsenzaehlen = self.starting_indices.rank1(posinbitmap) - 1;
            v_inc.push(einsenzaehlen);
            //v_inc.push((self.starting_indices.rank1(self.starting_indices.select0(self.wt_adj.select(vertex,i).unwrap()))-1).as_())
        }
        v_inc
    }

    fn delete_outgoing_edges(&mut self, vertex: usize) {
        todo!()
    }

    fn delete_incoming_edges(&mut self, vertex: usize) {
        todo!()
    }
}
impl<L> GraphSearch for WTDigraph<L> {
    fn connected(&self, from: usize, to: usize) -> bool {
        // is a connected to b?
        let mut list_of_outgoing_edges: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<usize> = Vec::new();
        list_of_outgoing_edges.append(&mut self.outgoing_edges(from).into());
        visited.push(from);
        while !list_of_outgoing_edges.is_empty() {
            let v = list_of_outgoing_edges.pop_front().unwrap();
            visited.push(v);
            if v == to {
                return true;
            }
            for item in self.outgoing_edges(v) {
                if !visited.contains(&item) {
                    // if vertex was not yet visited, add it to the queue
                    if !list_of_outgoing_edges.contains(&item) {
                        list_of_outgoing_edges.push_back(item);
                    }
                }
            }
        }
        false
    }

    fn shortest_path(&self, from: usize, to: usize, mode: ShortestPathAlgorithm) -> Vec<usize> {
        todo!()
    }

    fn shortest_paths(&self, mode: ShortestPathAlgorithm) -> Vec<Vec<usize>> {
        todo!()
    }

    fn connected_components(&self) -> Vec<Vec<usize>> {
        todo!()
    }
}
// WT-Weighted Digraph - definition and methods

impl<L> WTDirected for WTDigraph<L> {
    fn updated_outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn updated_incoming_edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }
}
