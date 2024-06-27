use crate::graph::labeled_directed::LabeledDigraph;
use crate::traits::{Graph, Labeled, Undirected, Unweighted,Directed};
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;
pub struct LabeledUGraph<L>
where
    L: Hash + Eq,
{
    ldg: LabeledDigraph<L>,
    // vec_vertex_labels: Vec<L>,
    // hashmap_labels_vertex: HashMap<L, usize>,
}

impl<L> LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        LabeledUGraph {
            ldg: LabeledDigraph::new(),
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
        LabeledUGraph {
            ldg: LabeledDigraph::from_adjacency_list(v_count, e_count, adj, labels),
        }
    }
}

impl<L> Graph<L> for LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        self.ldg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.ldg.e_count()
    }

    fn v_count(&self) -> usize {
        self.ldg.v_count()
    }


    fn delete_edge(&mut self, from: L, to: L) {
        if self.get_index(from.clone()) <= self.get_index(to.clone()) {
            self.ldg.delete_edge(from, to);
        } else {
            self.ldg.delete_edge(to, from);
        }
    }

    fn delete_vertex(&mut self, vertex: L) {
        self.ldg.delete_vertex(vertex);
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        self.ldg.vertex_exists(vertex)
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()      // erstmal unwichtig
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        self.ldg.edge_exists(from, to)
    }
}
impl<L> Undirected<L> for LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        let mut edges: Vec<L> = Vec::new();
        for i in 0..self.get_index(vertex.clone()) {
            if self.ldg.dg.adj[i].contains(&self.get_index(vertex.clone())) {
                edges.push(self.get_label(i));
            }
        }
        for item in &self.ldg.dg.adj[self.get_index(vertex)] {
            edges.push(self.get_label(item.clone()));
        }
        edges
    }

    fn delete_edges_from(&mut self, vertex: L) {
        for from in 0..self.get_index(vertex.clone()) {
            if self.ldg.dg.adj[from].contains(&self.get_index(vertex.clone())) {
                self.delete_edge(self.get_label(from), vertex.clone());
            }
        }
        for to in self.ldg.dg.adj[self.get_index(vertex.clone())] {
            self.delete_edge(vertex.clone(), self.get_label(to));
        }
    }
}
impl<L> Labeled<L> for LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.ldg.edit_label(old_label, new_label);
    }

    fn get_label(&self, vertex: usize) -> L {
        // gets label from index of vec label
        self.ldg.get_label(vertex)
    }

    fn get_index(&self, label: L) -> usize {
        //gets index from key in hashmap
        self.ldg.get_index(label)
    }
}

impl<L> Unweighted<L> for LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn add_edge(&mut self, from: L, to: L) {
        if self.get_index(from) <= self.get_index(to) {
            self.ldg.add_edge(from, to);
        } else {
            self.ldg.add_edge(to, from);
        }
    }
}