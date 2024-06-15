use crate::graph::Digraph;
use crate::traits::*;
use num::{cast::AsPrimitive, FromPrimitive, PrimInt, ToPrimitive, Unsigned};
use qwt::{AccessUnsigned, QWT256};
use std::{collections::HashMap, hash::Hash};
use vers_vecs::{BitVec, RsVec};



// UNIT TESTS FOR WT DIGRAPH & WEIGHTED WT DIGRAPH
#cfg[(test)]
mod test {

}

#[cfg(test)]
mod test {
    use super::*;
    const V_COUNT: u32 = 10;

    #[test]
    fn wtdigraph_from_digraph() {
    }

    #[test]
    fn wtdigraph_from_sequence_and_bitvec {
    }

    #[test]
    fn add_edges_to_wtdigraphs() {
    }

    #[test]
    fn add_vertex_to_wtdigraphs() {
    }

    #[test]
    fn append_vertex_to_wtdigraphs() {
    }

    #[test]
    fn remove_edges_from_wtdigraphs() {
    }

    #[test]
    fn remove_vertices_from_wtdigraphs()

    #[test]
    fn add_label_to_wtdigraphs() {
    }

    #[test]
    fn edit_label_on_wtdigraphs() {
    } 

    #[test]
    fn outgoing_incoming_edges_on_wtdigraphs() {}
    
    #[test]
    fn print_weight_of_wtdigraph_edge() {}
}

pub enum Edit<T>
where
    T: Unsigned + ToPrimitive,
{
    Add(T),
    Delete(T),
    AddSelf,
    DeleteSelf,
}

pub struct WTDigraph<T, L>
// change T to usize
where
    T: Unsigned + ToPrimitive,
{
    v_count: T,                                  // number of vertices
    e_count: T,                                  // number of edges
    wt_adj: QWT256<T>,                           // the wavelet tree adjacency list
    starting_indices: RsVec,                     // starting indices of each
    uncommitted_edits: HashMap<T, Vec<Edit<T>>>, // changes not yet committed to sequence
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

        let v_count = starting_indices.rank1(length);
        let v_count = T::from_usize(v_count).unwrap();

        let e_count = starting_indices.rank0(length);
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
                adj.push(Edit::Add(w));
            }
            None => {
                self.uncommitted_edits.insert(v, vec![Edit::Add(w)]);
            }
        }

        self.has_uncommitted_edits = true;
    }

    fn add_vertex(&mut self, v: T) {
        // Method needs to be changed to reflect current strategy

        if v <= self.v_count - T::one() {
            // if the index of the vertex the user wants to add is smaller than the length of v_count, v exists in wt_adj
            // we now have to check, whether it was already added and or deleted

            let mut v_deleted: bool = self.vertex_deleted(v);

            if self.uncommitted_edits.get(&v).is_some() && !v_deleted {
                // if there is an entry for v in uncommitted_edits and v was not deleted, then:
                panic!("Vertex already exists.");
            }
            if v_deleted {
                // if v was deleted, that means an entry for v exists in self.uncommitted_edits
                // therefore, we'll have to push `AddSelf` to the end of the uncommitted edits of v.
                // When committing the edits, we'll only commit the changes after the final AddSelf in the changes list of v

                let mut edits_for_v: Vec<Edit<T>> = self.uncommitted_edits.get_mut(&v).unwrap();
                edits_for_v.push(Edit::AddSelf);
            }
        } else {
            self.uncommitted_edits.insert(v, vec![Edit::AddSelf]);
        }
    }

    fn add_vertex_label(&mut self, v: T, label: L) {
        if v > self.v_count - T::one() || self.vertex_deleted(v) {
            panic!("Vertex doesn't exist.");
        }

        self.node_labels.insert(v, label);
    }

    fn append_vertex(&mut self, v: T) -> T {
        todo!()
    }

    fn delete_edge(&mut self, v: T, w: T) {
        todo!()
    }

    fn delete_vertex(&mut self, v: T) {
        if v > self.v_count - T::one() {
            panic!("Vertex doesn't exist.");
        }

        match self.uncommitted_edits.get_mut(&v) {
            Some(adj) => {
                adj.push(Edit::Add(w));
            }
            None => {
                self.uncommitted_edits.insert(v, vec![Edit::Add(w)]);
            }
        }
    }

    fn e_count(&self) -> T {
        self.e_count
    }

    fn edit_label(&mut self, v: T, change: L) {
        self.node_labels.insert(v, change);
    }

    fn get_label(&self, v: T) -> Option<&L> {
        self.node_labels.get(&v)
    }

    fn v_count(&self) -> T {
        self.v_count
    }

    fn vertex_deleted(&self, v: T) -> bool {
        let mut last: Edit<T>;

        match self.uncommitted_edits.get_mut(&v) {
            Some(adj) => {
                last = adj.last();
            }
            None => {
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

        self.has_uncommitted_edits = true;
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

    fn incoming_edges(&self, vertex: T) -> Vec<T> {
        todo!()
    }
}



// WT DIGRAPH WEIGHTED


pub struct WTDigraph<T, L>
// change T to usize
where
    T: Unsigned + ToPrimitive,
{
    v_count: T,                                  // number of vertices
    e_count: T,                                  // number of edges
    wt_adj: QWT256<T>,                           // the wavelet tree adjacency list
    starting_indices: RsVec,                     // starting indices of each
    uncommitted_edits: HashMap<T, Vec<Edit<T>>>, // changes not yet committed to sequence
    has_uncommitted_edits: bool,
    node_labels: HashMap<T, L>, // name given to node format: index: value
}
