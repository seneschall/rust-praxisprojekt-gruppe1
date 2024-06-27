use crate::graph::labeled_directed::LabeledDigraph;
use crate::traits::{Graph, Labeled, Undirected, Unweighted};
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;
pub struct LabeledUGraph<L>
where
    L: Hash + Eq,
{
    ldg: LabeledDigraph<L>,
    vec_vertex_labels: Vec<L>,
    hashmap_labels_vertex: HashMap<L, usize>,
}

impl<L> LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        LabeledUGraph {
            ldg: LabeledDigraph::new(),
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
        LabeledUGraph {
            ldg: LabeledDigraph::from_adjacency_list(v_count, e_count, adj, labels),
            vec_vertex_labels: vec_vertex_labels,
            hashmap_labels_vertex: hashmap_labels_vertex,
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
        if self.get_index(vertex.clone()).unwrap().clone() < self.v_count() {
            self.ldg
                .dg
                .deleted_vertices
                .push(self.get_index(vertex.clone()).unwrap().clone());
            self.delete_edges_from(vertex);
            self.ldg.dg.v_count -= 1;
        } else {
            panic!("delete_vertex : Can't delete Vertex : vertex >= self.v_count")
        }
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        self.ldg.vertex_exists(vertex)
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L> Undirected<L> for LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        let mut edges: Vec<L> = Vec::new();
        for i in 0..self.get_index(vertex.clone()).unwrap().clone() {
            if self.ldg.dg.adj[i].contains(&self.get_index(vertex.clone()).unwrap()) {
                edges.push(self.get_label(i).unwrap().clone());
            }
        }
        for item in &self.ldg.dg.adj[self.get_index(vertex).unwrap().clone()] {
            edges.push(self.get_label(item.clone()).unwrap().clone());
        }
        edges
    }

    fn delete_edges_from(&mut self, vertex: L) {
        for from in 0..self.get_index(vertex.clone()).unwrap().clone() {
            if self.ldg.dg.adj[from].contains(&self.get_index(vertex.clone()).unwrap()) {
                self.delete_edge(self.get_label(from).unwrap().clone(), vertex.clone());
            }
        }
        for to in self.ldg.dg.adj[self.get_index(vertex.clone()).unwrap().clone()].clone() {
            self.delete_edge(vertex.clone(), self.get_label(to).unwrap().clone());
        }
    }
}
impl<L> Labeled<L> for LabeledUGraph<L>
where
    L: Hash + Eq,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        self.ldg.get_label(vertex)
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        self.ldg.get_index(label)
    }
}

impl<L> Unweighted<L> for LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn add_edge(&mut self, from: L, to: L) {
        if self.get_index(from.clone()) <= self.get_index(to.clone()) {
            self.ldg.add_edge(from, to);
        } else {
            self.ldg.add_edge(to, from);
        }
    }
}
