use crate::graph::labeled_directed::LabeledDigraph;
use crate::traits::{Directed, Graph, Labeled, Undirected, Unweighted};
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;
/// A labeled, mutable graph with undirected edges.
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type.
/// Labels can have any type and are referenced.
pub struct LabeledUGraph<L>
where
    L: Hash + Eq,
{
    pub(crate) ldg: LabeledDigraph<L>,
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
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lug delete_edge : from index is none");
        }
        if to_index.is_none() {
            panic!("lug delete_edge : to index is none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        if from_index <= to_index {
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

    fn edge_exists(&self, from: L, to: L) -> bool {
        self.ldg.edge_exists(from, to)
    }
}
impl<L> Undirected<L> for LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("lug edges : vertex is none");
        }
        let vertex_index = vertex_index.unwrap();
        let mut edges: Vec<L> = Vec::new();
        for i in 0..vertex_index {
            if self.ldg.dg.adj[i].contains(&vertex_index) {
                edges.push(self.get_label(i).unwrap().clone());
            }
        }
        for item in self.ldg.dg.adj[vertex_index].clone() {
            edges.push(self.get_label(item).unwrap().clone());
        }
        edges
    }

    fn delete_edges_from(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("lug delete_edges_from : vertex is none");
        }
        let vertex_index = vertex_index.unwrap();
        for from in 0..vertex_index {
            if self.ldg.dg.adj[from].contains(&vertex_index) {
                self.delete_edge(self.get_label(from).unwrap().clone(), vertex.clone());
            }
        }
        for to in self.ldg.dg.adj[vertex_index].clone() {
            self.delete_edge(vertex.clone(), self.get_label(to).unwrap().clone());
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

    fn get_label(&self, vertex: usize) -> Option<&L> {
        // gets label from index of vec label
        self.ldg.get_label(vertex)
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        //gets index from key in hashmap
        self.ldg.get_index(label)
    }

    fn shrink(&mut self){
        todo!()
    }
}

impl<L> Unweighted<L> for LabeledUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn add_edge(&mut self, from: L, to: L) {
        if self.get_index(&from) <= self.get_index(&to) {
            self.ldg.add_edge(from, to);
        } else {
            self.ldg.add_edge(to, from);
        }
    }
}
