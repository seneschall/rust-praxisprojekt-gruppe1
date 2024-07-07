use vers_vecs::RsVec;

use crate::graph::labeled_directed::LabeledDigraph;
use crate::traits::{Directed, Graph, Labeled, Unlabeled, Unweighted, WTDirected, WTLabeled, WT};
use crate::wt::directed::WTDigraph;
use crate::Edit;
use std::collections::HashMap;
use std::hash::Hash;

/// An labeled wavelet-tree-graph with directed edges. (L-wt-digraph)
/// The L-wt-digraph holds a wt-digraph and data encoding the digraph's vertice-labels, at the last commit and in the recent, uncommited stage.
/// Users can perfom fast operations on the original graph and slower operations on the recent state of the graph.
/// Users can integrate the recent state of the graph into the QW-Tree by rebuilding it using the commit_edits-function.
/// See module wt::directed for the WT-digraph struct definition. See more documentation on function-level and in the crate introduction.
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type. Labels can have any type and are referenced.
pub struct LabeledWTDigraph<L>
where
    L: Hash + Clone + Eq,
{
    pub(crate) dg: WTDigraph,
    index_label: Vec<L>,
    index_label_uncommitted: HashMap<usize, Edit<L>>, // this only works with a HashMap
    label_index: HashMap<L, usize>, // changed from label_index: HashMap<L, Edit<usize>>,
    label_index_uncommitted: HashMap<L, Edit<usize>>, // changed from label_index_uncommitted: HashMap<L, usize>,
}

impl<L> LabeledWTDigraph<L>
where
    L: Hash + Clone + Eq,
{
    pub fn from_labeled_digraph(ldg: LabeledDigraph<L>) -> Self {
        return LabeledWTDigraph {
            index_label: ldg.vec_vertex_labels,
            index_label_uncommitted: HashMap::new(),
            label_index: ldg.hashmap_labels_vertex,
            label_index_uncommitted: HashMap::new(),
            dg: WTDigraph::from_digraph(ldg.dg),
        };
    }
    pub fn from(sequence: Vec<usize>, starting_indices: RsVec, labels: Vec<L>) -> Self {
        let mut label_index: HashMap<L, usize> = HashMap::new();
        for i in 0..labels.len() {
            label_index.insert(labels[i].clone(), i);
        }

        return LabeledWTDigraph {
            dg: WTDigraph::from(sequence, starting_indices),
            index_label: labels,
            index_label_uncommitted: HashMap::new(),
            label_index: label_index,
            label_index_uncommitted: HashMap::new(),
        };
    }
}
impl<L> Graph<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        let vertex_index = self.get_index_updated(&vertex);
        if vertex_index.is_some() {
            panic!("ldg add_vertex : vertex is not none");
        }
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

    fn delete_edge(&mut self, from: L, to: L) {
        let from_index = self.get_index_updated(&from);
        let to_index = self.get_index_updated(&to);
        if from_index.is_none() {
            panic!("ldg edge_exists : from_index is none")
        }
        if to_index.is_none() {
            panic!("ldg edge_exists : to_index is none")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        self.dg.delete_edge(from_index, to_index);
    }

    fn delete_vertex(&mut self, vertex: L) {
        let vertex_index = self.get_index_updated(&vertex);
        if vertex_index.is_none() {
            panic!("ldg delete_vertex : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        self.dg.delete_vertex(vertex_index);
        // checkme
        // need extra checking or is insert enough?
        self.index_label_uncommitted
            .insert(vertex_index, Edit::Delete(vertex.clone()));
        self.label_index_uncommitted
            .insert(vertex, Edit::Delete(vertex_index));
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            return false;
            // panic!("ldg vertex_exists : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        return self.dg.vertex_exists(vertex_index);
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        let from_index = self.get_index_updated(&from);
        let to_index = self.get_index_updated(&to);
        if from_index.is_none() {
            panic!("ldg edge_exists : from_index is none")
        }
        if to_index.is_none() {
            panic!("ldg edge_exists : to_index is none")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        return self.dg.edge_exists(from_index, to_index);
    }
}
impl<L> Directed<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg outgoing_edges : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        let outgoing_edges = self.dg.outgoing_edges(vertex_index);
        let mut outgoing_edges_labeled: Vec<L> = Vec::new();
        for item in outgoing_edges {
            outgoing_edges_labeled.push(self.get_label(item).unwrap().to_owned());
        }
        return outgoing_edges_labeled;
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg incoming_edges : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        let incoming_edges = self.dg.incoming_edges(vertex_index);
        let mut incoming_edges_labeled: Vec<L> = Vec::new();
        for item in incoming_edges {
            incoming_edges_labeled.push(self.get_label(item).unwrap().to_owned());
        }
        return incoming_edges_labeled;
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg delete_outgoing_edges : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        self.dg.delete_outgoing_edges(vertex_index);
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg delete_incoming_edges : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        self.dg.delete_incoming_edges(vertex_index);
    }
}
impl<L> Labeled<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        let old_label_index: Option<usize> = self.get_index_updated(&old_label);
        let new_label_index: Option<usize> = self.get_index_updated(&new_label);
        if old_label_index.is_none() {
            panic!("ldg edit_label : old_label_index is none");
        }
        if new_label_index.is_some() {
            panic!("ldg edit_label : new_label_index is some");
        }
        let old_label_index = old_label_index.unwrap().to_owned();
        // old_label and new_label are valid
        // check in label_index_uncommitted;
        if self.label_index_uncommitted.contains_key(&old_label) {
            // Label got an entry in label_index_uncommitted
            match self.label_index_uncommitted.get(&old_label).unwrap() {
                Edit::Delete(_) => {
                    // Label was deleted from the index
                    // this case is not valid
                    panic!("ldg edit_label : old_label was deleted.");
                }
                Edit::Add(_) => {
                    // Label was added to an index
                    // this case is valid
                    self.label_index_uncommitted
                        .insert(new_label.clone(), Edit::Add(old_label_index));
                    // mark new_label as added
                    self.label_index_uncommitted
                        .insert(old_label, Edit::Delete(old_label_index));
                    // mark old_label as deleted

                    // updated index_label_uncommitted
                    self.index_label_uncommitted
                        .insert(old_label_index, Edit::Add(new_label));
                    return;
                }
            }
        } else {
            if self.label_index.contains_key(&old_label) {
                // label_index has a key for old_label
                // this key is not in uncommitted, since it was checked first
                let index = self.label_index.get(&old_label).unwrap().to_owned();
                //update label_index_uncommitted and index_label_uncommitted
                self.label_index_uncommitted
                    .insert(new_label.clone(), Edit::Add(index));
                self.index_label_uncommitted
                    .insert(index, Edit::Add(new_label));

                return;
            }
        }
        // todo fixme checkme if it panics I missed something
        panic!("ldg edit_label : Missed something");
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        return self.index_label.get(vertex);
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        return self.label_index.get(&label).copied();
    }

    fn shrink(&mut self) -> HashMap<L, Option<L>> {
        // todo
        // updates index_label
        let old_and_new_indices = self.dg.shrink();
        let mut new_index_labels: Vec<L> = Vec::new();
        for i in 0..old_and_new_indices.len() {
            if old_and_new_indices[i].is_some() {
                new_index_labels.insert(i, self.index_label[i].clone());
            }
        }
        self.index_label = new_index_labels;
        return HashMap::new();
    }
}
impl<L> Unweighted<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn add_edge(&mut self, from: L, to: L) {
        let from_index = self.get_index_updated(&from);
        let to_index = self.get_index_updated(&to);
        if from_index.is_none() {
            panic!("ldg edge_exists_updated : from_index is none")
        }
        if to_index.is_none() {
            panic!("ldg edge_exists_updated : to_index is none")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        self.dg.add_edge(from_index, to_index);
    }
}
impl<L> WT<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn commit_edits(&mut self) {
        // first append new indices to the
        // increase Vec<L> to v_count_updated
        // todo: checkme if this is correct
        let mut new_index_label: Vec<L> = Vec::new();
        let mut new_label_index: HashMap<L, usize> = HashMap::new();
        for i in 0..self.dg.wt_adj_len_updated {
            if self.index_label_uncommitted.contains_key(&i) {
                // change of labels at index i
                let change = self.index_label_uncommitted.get(&i).unwrap(); // save to unwrap here
                match change {
                    Edit::Add(new_label) => {
                        // new label
                        new_index_label.push(new_label.to_owned());
                        new_label_index.insert(new_label.to_owned(), i);
                    }
                    Edit::Delete(new_label) => {
                        //this index is marked as deleted
                        //this is a special case, we can't delete the index, because of index shift
                        //checkme fixme todo
                        //we have to check in deleted_vertices if this entry is valid
                        new_index_label.push(new_label.to_owned());
                        new_label_index.remove(new_label);
                    }
                }
            } else {
                // pushes index/label pairs that weren't changed since last commit
                if i <= self.label_index.len() {
                    let label: &L = self.index_label.get(i).unwrap();
                    new_index_label.push(label.clone());
                    new_label_index.insert(label.clone(), i);
                }
            }
        }
        // for now just rebuilding label_index from index_label
        self.index_label_uncommitted = HashMap::new();
        self.label_index_uncommitted = HashMap::new();
        self.index_label = new_index_label;
        self.label_index = new_label_index;
        self.dg.commit_edits();
    }

    // fn get_uncommitted_edits(&self) -> Option<HashMap<usize, L>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        self.dg.discard_edits();
        self.label_index_uncommitted = HashMap::new();
        self.index_label_uncommitted = HashMap::new();
    }

    fn vertex_exists_updated(&self, vertex: L) -> bool {
        let vertex_index = self.get_index_updated(&vertex);
        if vertex_index.is_none() {
            return false;
            // panic!("ldg vertex_exists_updated : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        return self.dg.vertex_exists_updated(vertex_index);
    }

    fn edge_exists_updated(&self, from: L, to: L) -> bool {
        let from_index = self.get_index_updated(&from);
        let to_index = self.get_index_updated(&to);
        if from_index.is_none() {
            panic!("ldg edge_exists_updated : from_index is none")
        }
        if to_index.is_none() {
            panic!("ldg edge_exists_updated : to_index is none")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        return self.dg.edge_exists_updated(from_index, to_index);
    }

    fn v_count_updated(&self) -> usize {
        return self.dg.v_count_updated();
    }

    fn e_count_updated(&self) -> usize {
        return self.dg.e_count_updated();
    }
}
impl<L> WTDirected<L> for LabeledWTDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn outgoing_edges_updated(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index_updated(&vertex);
        if vertex_index.is_none() {
            panic!("ldg outgoing_edges_updated : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        let outgoing_edges_updated = self.dg.outgoing_edges_updated(vertex_index);
        let mut outgoing_edges_updated_labeled: Vec<L> = Vec::new();
        for item in outgoing_edges_updated {
            outgoing_edges_updated_labeled.push(self.get_label_updated(item).unwrap().clone())
        }
        return outgoing_edges_updated_labeled;
    }

    fn incoming_edges_updated(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index_updated(&vertex);
        if vertex_index.is_none() {
            panic!("ldg incoming_edges_updated : vertex_index is none");
        }
        let vertex_index = vertex_index.unwrap();
        let incoming_edges_updated = self.dg.incoming_edges_updated(vertex_index);
        let mut incoming_edges_updated_labeled: Vec<L> = Vec::new();
        for item in incoming_edges_updated {
            incoming_edges_updated_labeled.push(self.get_label_updated(item).unwrap().clone());
        }
        return incoming_edges_updated_labeled;
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
    fn get_index_updated(&self, label: &L) -> Option<usize> {
        // check if something was changed first
        // why was this commented out?

        let changes: Option<&Edit<usize>> = self.label_index_uncommitted.get(label); // compile error

        if changes.is_some() {
            // because we know changes is some, it's safe to unwrap
            match changes.unwrap() {
                Edit::Add(label) => {
                    return Some(label.to_owned()); // to_owned() since its label is from type &usize
                }
                Edit::Delete(_) => {
                    return None;
                }
            }
        }
        return self.get_index(label);
    }
}
