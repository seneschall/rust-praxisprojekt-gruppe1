use crate::graph::directed::Digraph;
use crate::traits::{Directed, Graph, Labeled, Unlabeled, Unweighted};
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;

/// A labeled, mutable graph with directed edges. 
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type. 
/// Labels can have any type and are referenced.
pub struct LabeledDigraph<L>
where
    L: Eq + Hash,
{
    pub(crate) dg: Digraph,
    pub(crate) vec_vertex_labels: Vec<L>,
    pub(crate) hashmap_labels_vertex: HashMap<L, usize>,
}
impl<L> LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        LabeledDigraph {
            dg: Digraph::new(),
            vec_vertex_labels: Vec::new(),
            hashmap_labels_vertex: HashMap::new(),
        }
    }
    pub fn from_adjacency_list(
        v_count: usize,
        e_count: usize,
        adj: Vec<Vec<usize>>,
        labels: Vec<L>,
    ) -> Self {
        let mut vec_vertex_labels: Vec<L> = Vec::new();
        let mut hashmap_labels_vertex: HashMap<L, usize> = HashMap::new();
        if !(labels.len() == v_count) {
            panic!("Failed : v_count and labels.len() are not equal")
        } else {
            let mut i: usize = 0;
            for item in labels {
                vec_vertex_labels.push(item.clone()); // create Vec for Labels
                hashmap_labels_vertex.insert(item, i); // create HashMap for Labels
                i += 1;
            }
        }
        LabeledDigraph {
            dg: Digraph::from_adjacency_list(v_count, e_count, adj),
            vec_vertex_labels,
            hashmap_labels_vertex,
        }
    }
}
impl<L> Graph<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        if self.hashmap_labels_vertex.contains_key(&vertex) {
            panic!("ldg add_vertex : Label already in use");
        }
        let index = self.dg.append_vertex();
        self.vec_vertex_labels.push(vertex.clone());
        self.hashmap_labels_vertex.insert(vertex, index);
        index
    }

    fn e_count(&self) -> usize {
        self.dg.e_count()
    }

    fn v_count(&self) -> usize {
        self.dg.v_count()
    }

    fn delete_edge(&mut self, from: L, to: L) {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("ldg delete edge : From index is none");
        }
        if to_index.is_none() {
            panic!("ldg delete edge : To index is none");
        }
        self.dg
            .delete_edge(from_index.unwrap().clone(), to_index.unwrap().clone())
    }

    fn delete_vertex(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg delete_vertex : vertex index is none");
        }

        self.dg.delete_vertex(vertex_index.unwrap().clone());
        self.hashmap_labels_vertex.remove(&vertex).unwrap();
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            return false;
            // panic!("ldg vertex_exists : vertex index is none");
        }
        self.dg.vertex_exists(vertex_index.unwrap().clone())
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("ldg edge exists : from index is none");
        }
        if to_index.is_none() {
            panic!("ldg edge exists : to index is none");
        }
        self.dg
            .edge_exists(from_index.unwrap().clone(), to_index.unwrap().clone())
    }
}
impl<L> Directed<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg outgoing edges vertex label is none");
        }
        if !(self.vertex_exists(vertex)) {
            panic!("ldg outgoing edges of a Label which doesn't exist");
        }
        let mut outgoing_edges: Vec<L> = Vec::new();
        for item in self.dg.outgoing_edges(vertex_index.unwrap()) {
            outgoing_edges.push(self.get_label(item).unwrap().clone());
        }
        outgoing_edges
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg incoming edges : vertex is none");
        }
        if !(self.vertex_exists(vertex)) {
            panic!("ldg incoming edges of a Label which doesn't exist");
        }
        let mut incoming_edges: Vec<L> = Vec::new();
        for item in self.dg.incoming_edges(vertex_index.unwrap()) {
            incoming_edges.push(self.get_label(item).unwrap().clone());
        }
        incoming_edges
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg delete_incoming_edges : vertex is none");
        }
        self.dg.delete_outgoing_edges(vertex_index.unwrap());
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg delete_incoming_edges : vertex is none");
        }
        self.dg.delete_incoming_edges(vertex_index.unwrap());
    }
}
impl<L> Labeled<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        let old_label_index = self.get_index(&old_label);
        if old_label_index.is_none() {
            // if it's some, valid input
            panic!("ldg edit_label : old_label_index is none");
        }
        if self.get_index(&new_label).is_some() {
            // if it's none, valid input
            panic!("ldg edit_label : new_label is some"); // new label should be none
        }
        let old_label_index = old_label_index.unwrap(); // save unwrap, since old_label_index is some
        self.vec_vertex_labels[old_label_index] = new_label.clone(); // update vec

        self.hashmap_labels_vertex.remove(&old_label); // remove old entry in the hasmap
        self.hashmap_labels_vertex
            .insert(new_label, old_label_index); // insert new label with old index
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        // gets label from index of vec label
        // todo fixme checkme
        // might return some, since vec is not updated properly right now
        // double check if it is neccassary to update vec of labels, delete comment if sure
        return self.vec_vertex_labels.get(vertex);
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        //gets index from key in hashmap
        if self.hashmap_labels_vertex.contains_key(label) {
            return self.hashmap_labels_vertex.get(label).copied();
        } else {
            return None;
        }
    }

    fn shrink(&mut self) -> HashMap<L, Option<L>> {
        todo!()
    }
}
impl<L> Unweighted<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn add_edge(&mut self, from: L, to: L) {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("ldg add_edge : from index is none");
        }
        if to_index.is_none() {
            panic!("ldg add_edge : to index is none");
        }
        self.dg.add_edge(from_index.unwrap(), to_index.unwrap());
    }
}
