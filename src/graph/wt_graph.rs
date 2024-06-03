use crate::graph::Digraph;
use crate::traits::*;
use num::{cast::AsPrimitive, FromPrimitive, PrimInt, ToPrimitive, Unsigned};
use qwt::{AccessUnsigned, QWT256};
use std::{collections::HashMap, hash::Hash};
use vers_vecs::{BitVec, RsVec};

pub struct WTDigraph<T, L>
where
    T: Unsigned + ToPrimitive,
{
    v_count: T,                            // number of vertices
    e_count: T,                            // number of edges
    wt_adj: QWT256<T>,                     // the wavelet tree adjacency list
    starting_indices: RsVec,               // starting indices of each
    uncommitted_edits: HashMap<T, Vec<T>>, // changes not yet committed to sequence
    has_uncommitted_edits: bool,
    node_labels: HashMap<T, L>, // name given to node format: index: value
}

impl<T, L> WTDigraph<T, L>
where
    T: Unsigned + ToPrimitive + FromPrimitive + Copy + PrimInt + AsPrimitive<u8>,
    u8: AsPrimitive<T>,
{
    pub fn from_digraph(dg: Digraph<T, L>) -> Self {
        let mut bv = BitVec::new();
        let mut e_count: T = T::zero();
        let v_count = dg.adj.len();
        let mut sequence: Vec<T> = Vec::new();

        for (v, v_adj) in dg.adj.iter().enumerate() {
            // iterate over all vertices (v) in adj
            bv.append(true);
            for val in v_adj.iter() {
                // iterate over the values in the adjacency list of v
                sequence.push(*val);
                bv.append(false); // append 0 to bv for each element in adjacency list of v
                e_count = e_count + T::one();
            }
        }
        let starting_indices = RsVec::from_bit_vec(bv);

        let wt_adj: QWT256<T> = QWT256::from(sequence);

        return WTDigraph {
            v_count: T::from_usize(v_count).unwrap(),
            e_count,
            wt_adj, // here sequence would be replaced by wavelet tree
            starting_indices,
            uncommitted_edits: HashMap::new(),
            has_uncommitted_edits: false,
            node_labels: HashMap::new(),
        };
    }

    pub fn from(sequence: Vec<T>, starting_indices: RsVec) -> Self {
        let length = starting_indices.len();

        let v_count = starting_indices.rank1(length - 1);
        let v_count = T::from_usize(v_count).unwrap();

        let e_count = starting_indices.rank0(length - 1);
        let e_count = T::from_usize(e_count).unwrap();

        let wt_adj: QWT256<T> = QWT256::from(sequence);

        return WTDigraph {
            v_count,
            e_count,
            wt_adj,
            starting_indices,
            uncommitted_edits: HashMap::new(),
            has_uncommitted_edits: false,
            node_labels: HashMap::new(),
        };
    }
}

impl<T, L> Graph<T, L> for WTDigraph<T, L>
where
    T: Unsigned + ToPrimitive + PrimInt + Hash + Copy,
    L: Clone,
{
    fn add_edge(&mut self, v: T, w: T) {
        // only adds to uncommitted edits

        match self.uncommitted_edits.get_mut(&v) {
            Some(adj) => {
                adj.push(w);
            }
            None => {
                self.uncommitted_edits.insert(v, vec![w]);
            }
        }
    }

    fn add_node_label(&mut self, v: T, label: L) {
        if v > self.v_count - T::one() {
            panic!("Vertex doesn't exist.");
        }

        self.node_labels.insert(v, label);
    }

    fn delete_edge(&mut self, v: T, w: T) {
        todo!()
    }

    fn delete_vertex(&mut self, v: T) {
        todo!()
    }

    fn e_count(&self) -> T {
        self.e_count
    }

    fn edit_label(&mut self, v: T) {
        todo!()
    }

    fn get_label(&self, v: T) -> Option<&L> {
        self.node_labels.get(&v)
    }

    fn v_count(&self) -> T {
        self.v_count
    }
}

impl<T, L> Directed<T> for WTDigraph<T, L>
where
    T: Unsigned + ToPrimitive + Copy + AsPrimitive<u8> + PrimInt,
    u8: AsPrimitive<T>,
{
    fn outgoing_edges(&self, vertex: T) -> Vec<T> {
        let mut v_adj: Vec<T> = Vec::new();
        let v = vertex.to_usize().unwrap(); // this won't work if v is of type u128

        let start = self.starting_indices.select1(v) + v; // this won't work if v is of type u128
        let end = self.starting_indices.select1(v + 1) + v + 1; // neither will this

        if start > self.wt_adj.len() || start == end {
            return Vec::new();
        }

        for i in start..end {
            v_adj.push(self.wt_adj.get(i).unwrap()); // is it safe to unwrap here? I think it should be
        }

        return v_adj;
    }

    fn incoming_edges(&self, vertex: T) -> Vec<T> {
        todo!()
    }
}
