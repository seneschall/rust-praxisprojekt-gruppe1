use crate::graph::directed::Digraph;
use crate::traits::{Directed, Graph, Labeled, Unlabeled, Unweighted};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;

// A labeled, mutable graph with directed edges.
// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type.
// Labels can have any type and are referenced.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LabeledDigraph<L>
where
    L: Eq + Hash,
{
    pub(crate) dg: Digraph,
    pub(crate) index_label: Vec<L>,
    pub(crate) label_index: HashMap<L, usize>,
}
impl<L> LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        LabeledDigraph {
            dg: Digraph::new(),
            index_label: Vec::new(),
            label_index: HashMap::new(),
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
            index_label: vec_vertex_labels,
            label_index: hashmap_labels_vertex,
        }
    }
}
impl<L> Graph<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        if self.label_index.contains_key(&vertex) {
            panic!("ldg add_vertex : Label already in use");
        }
        let index = self.dg.append_vertex();
        self.index_label.push(vertex.clone());
        self.label_index.insert(vertex, index);
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
            panic!("ldg delete edge : from Vertex doesn't exist");
        }
        if to_index.is_none() {
            panic!("ldg delete edge : to Vertex doesn't exist");
        }
        self.dg
            .delete_edge(from_index.unwrap().clone(), to_index.unwrap().clone())
    }

    fn delete_vertex(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg delete_vertex : Vertex doesn't exist");
        }

        self.dg.delete_vertex(vertex_index.unwrap().clone());
        self.label_index.remove(&vertex).unwrap();
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
            panic!("ldg edge_exists : from Vertex doesn't exist");
        }
        if to_index.is_none() {
            panic!("ldg edge_exists : to Vertex doesn't exist");
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
            panic!("ldg outgoing_edges : Vertex doesn't exist");
        }
        if !(self.vertex_exists(vertex)) {
            panic!("ldg outgoing_edges : Vertex doesn't exist");
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
            panic!("ldg incoming_edges : Vertex doesn't exist");
        }
        if !(self.vertex_exists(vertex)) {
            panic!("ldg incoming_edges : Vertex doesn't exist");
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
            panic!("ldg delete_incoming_edges : Vertex doesn't exist");
        }
        self.dg.delete_outgoing_edges(vertex_index.unwrap());
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("ldg delete_incoming_edges : Vertex doesn't exist");
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
            panic!("ldg edit_label : old_label Vertex doesn't exist");
        }
        if self.get_index(&new_label).is_some() {
            // if it's none, valid input
            panic!("ldg edit_label : new_label Vertex already in use"); // new label should be none
        }
        let old_label_index = old_label_index.unwrap(); // save unwrap, since old_label_index is some
        self.index_label[old_label_index] = new_label.clone(); // update vec

        self.label_index.remove(&old_label); // remove old entry in the hasmap
        self.label_index.insert(new_label, old_label_index); // insert new label with old index
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        // gets label from index of vec label
        // todo fixme checkme
        // might return some, since vec is not updated properly right now
        // double check if it is neccassary to update vec of labels, delete comment if sure
        return self.index_label.get(vertex);
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        //gets index from key in hashmap
        if self.label_index.contains_key(label) {
            return self.label_index.get(label).copied();
        } else {
            return None;
        }
    }

    fn shrink(&mut self) {
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
            panic!("ldg add_edge : from Vertex doesn't exist");
        }
        if to_index.is_none() {
            panic!("ldg add_edge : to Vertex doesn't exist");
        }
        self.dg.add_edge(from_index.unwrap(), to_index.unwrap());
    }
}
