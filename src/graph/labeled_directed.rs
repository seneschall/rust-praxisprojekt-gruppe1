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
            vec_vertex_labels: vec_vertex_labels,
            hashmap_labels_vertex: hashmap_labels_vertex,
        }
    }
}
impl<L> Graph<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        if self.hashmap_labels_vertex.contains_key(&vertex) {
            panic!("add_vertex Label already in use");
        }
        let index = self.dg.append_vertex();
        self.vec_vertex_labels.push(vertex.clone());
        self.hashmap_labels_vertex.insert(vertex, index);
        index
    }

    fn e_count(&self) -> usize {
        self.dg.e_count
    }

    fn v_count(&self) -> usize {
        self.dg.v_count
    }

    fn vertex_deleted(&self, vertex: L) -> bool {
        self.dg
            .vertex_deleted(self.get_index(vertex).unwrap().clone())
    }

    fn delete_edge(&mut self, from: L, to: L) {
        self.dg.delete_edge(
            self.get_index(from).unwrap().to_owned(),
            self.get_index(to).unwrap().to_owned(),
        )
    }

    fn delete_vertex(&mut self, vertex: L) {
        self.dg
            .delete_vertex(self.get_index(vertex.clone()).unwrap().to_owned());
        self.hashmap_labels_vertex.remove(&vertex).unwrap();
        // only delete entry in hashmap
        // if we delete vec of labels, index is wrong
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        self.dg
            .vertex_exists(self.get_index(vertex).unwrap().to_owned())
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L> Directed<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        if !(self.vertex_exists(vertex.clone())) {
            panic!("outgoing edges of a Label which doesn't exist");
        }
        let mut outgoing_edges: Vec<L> = Vec::new();
        for item in self
            .dg
            .outgoing_edges(self.get_index(vertex).unwrap().to_owned())
        {
            outgoing_edges.push(self.get_label(item).unwrap().to_owned());
        }
        outgoing_edges
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        if !(self.vertex_exists(vertex.clone())) {
            panic!("incoming edges of a Label which doesn't exist");
        }
        let mut incoming_edges: Vec<L> = Vec::new();
        for item in self
            .dg
            .incoming_edges(self.get_index(vertex).unwrap().to_owned())
        {
            incoming_edges.push(self.get_label(item).unwrap().to_owned());
        }
        incoming_edges
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        self.dg
            .delete_outgoing_edges(self.get_index(vertex).unwrap().to_owned())
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        self.dg
            .delete_incoming_edges(self.get_index(vertex).unwrap().to_owned())
    }
}
impl<L> Labeled<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.vec_vertex_labels[self
            .hashmap_labels_vertex
            .get(&old_label)
            .unwrap()
            .to_owned()] = new_label.clone(); // update Vec

        let value = self
            .hashmap_labels_vertex
            .remove(&old_label)
            .unwrap()
            .to_owned(); // update HashMap
        self.hashmap_labels_vertex.insert(new_label, value);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        // gets label from index of vec labels
        if self.dg.vertex_exists(vertex) {
            return self.vec_vertex_labels.get(vertex);
        } else {
            panic!("get_label : vertex is deleted");
        }
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        //gets index from key in hashmap
        if self.hashmap_labels_vertex.contains_key(&label) {
            self.hashmap_labels_vertex.get(&label)
        } else {
            panic!("get_index : Label not valid or deleted");
        }
    }
}
impl<L> Unweighted<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn add_edge(&mut self, from: L, to: L) {
        self.dg.add_edge(
            self.get_index(from).unwrap().to_owned(),
            self.get_index(to).unwrap().to_owned(),
        );
    }
}
