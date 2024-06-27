use vers_vecs::RsVec;

use crate::graph::labeled_directed::LabeledDigraph;
use crate::traits::{Directed, Graph, Labeled, UnLabeled, Unweighted, WTDirected, WTLabeled, WT};
use crate::wt::directed::WTDigraph;
use crate::Edit;
use std::collections::HashMap;
use std::hash::Hash;

pub struct LabeledWTDigraph<L>
where
    L: Hash + Clone + Eq,
{
    dg: WTDigraph,
    index_label: Vec<L>,
    index_label_uncommitted: HashMap<usize, Edit<L>>, // this only works with a HashMap
    label_index: HashMap<L, usize>, // changed from label_index: HashMap<L, Edit<usize>>,
    label_index_uncommitted: HashMap<L, Edit<usize>>, // changed from label_index_uncommitted: HashMap<L, usize>,
}

impl<L> LabeledWTDigraph<L>
where
    L: Hash + Clone + Eq,
{
    pub fn from_digraph(dg: LabeledDigraph<L>) -> Self {
        todo!()
    }

    pub fn from(sequence: Vec<usize>, starting_indices: RsVec) -> Self {
        todo!()
    }
}
impl<L> Graph<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        let index: usize = self.dg.append_vertex(); // this also updates v_count_updated
        self.label_index_uncommitted
            .insert(vertex.clone(), Edit::Add(index)); // changed from self.label_index_uncommitted.insert(vertex, Edit::Add(index));
        self.index_label_uncommitted
            .insert(index, Edit::Add(vertex));
        // need to return v_count_updated -1, but field is private
        // it no longer is -Simon
        return self.dg.wt_adj_len_updated - 1;
    }

    fn e_count(&self) -> usize {
        return self.dg.e_count();
    }

    fn v_count(&self) -> usize {
        return self.dg.v_count();
    }

    fn vertex_deleted(&self, vertex: L) -> bool {
        todo!()
    }

    fn delete_edge(&mut self, from: L, to: L) {
        todo!()
    }

    fn delete_vertex(&mut self, vertex: L) {
        todo!()
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        let vertex: usize = self.get_index(vertex).unwrap().clone(); //added .clone()
                                                                     // todo: change vertex_exists trait so that it takes &L, not L
        return self.dg.vertex_exists(vertex);
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L> Directed<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        todo!()
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        todo!()
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        todo!()
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        todo!()
    }
}
impl<L> Labeled<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        return self.index_label.get(vertex);
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        return self.label_index.get(&label); // from return self.label_index.get(vertex);
    }
}
impl<L> Unweighted<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn add_edge(&mut self, from: L, to: L) {
        let from: usize = self.get_index_updated(from).unwrap(); //added .clone(); it's a usize! You don't have to clone!
        let to: usize = self.get_index_updated(to).unwrap(); //added .clone(); same here
        self.dg.add_edge(from, to);
    }
}
impl<L> WT<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn commit_edits(&mut self) {
        todo!()
    }

    // fn get_uncommitted_edits(&self) -> Option<HashMap<usize, L>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        todo!()
    }

    fn vertex_exists_updated(&self, vertex: L) -> bool {
        let vertex: usize = self.get_index(vertex).unwrap().clone(); //added .clone()
        return self.dg.vertex_exists_updated(vertex);
    }

    fn edge_exists_updated(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L> WTDirected<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn outgoing_edges_updated(&self, vertex: L) -> Vec<L> {
        todo!()
    }

    fn incoming_edges_updated(&self, vertex: L) -> Vec<L> {
        todo!()
    }
}

impl<L> WTLabeled<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn get_label_updated(&self, vertex: usize) -> Option<&L> {
        // check if something was changed first
        let changes: Option<&Edit<L>> = self.index_label_uncommitted.get(&vertex);
        if changes.is_some() {
            // because we know changes is some, it's safe to unwrap
            match changes.unwrap() {
                Edit::Add(label) => {
                    return Some(label);
                }
                Edit::Delete(label) => {
                    return None;
                }
            }
        }
        return self.get_label(vertex);
    }
    fn get_index_updated(&self, label: L) -> Option<&usize> {
        // check if something was changed first
        // why was this commented out?

        let changes: Option<Edit<&usize>> = self.label_index_uncommitted.get(&label); // compile error

        if changes.is_some() {
            // because we know changes is some, it's safe to unwrap
            match changes.unwrap() {
                Edit::Add(label) => {
                    return Some(label);
                }
                Edit::Delete(label) => {
                    return None;
                }
            }
        }
        return self.get_index(label);
    }
}
