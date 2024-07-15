use crate::graph::labeled_weighted_directed::LabeledWeightedDigraph;
use crate::traits::{Graph, Labeled, Undirected, Weighted};
use num::Num;
use serde::{Deserialize, Serialize};

use std::hash::Hash;

#[cfg(test)]
mod test;

// A labeled, mutable graph with undirected edges, where each edge represents a weight.
// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type.
// Labels can have any type and are referenced. Weights can have any type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq,
    W: Num,
{
    pub(crate) lwdg: LabeledWeightedDigraph<L, W>,
}

impl<L, W> LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq + Clone,
    W: Clone + Num,
{
    pub fn new() -> Self {
        LabeledWeightedUGraph {
            lwdg: LabeledWeightedDigraph::new(),
        }
    }
    pub fn from_adjacency_list(
        v_count: usize,
        e_count: usize,
        adj: Vec<Vec<(usize, W)>>,
        labels: Vec<L>,
    ) -> Self {
        return LabeledWeightedUGraph {
            lwdg: LabeledWeightedDigraph::from_adjacency_list(v_count, e_count, adj, labels),
        };
    }
}
impl<L, W> Graph<L> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq + Clone + std::fmt::Display,
    W: Num,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        self.lwdg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.lwdg.e_count()
    }

    fn v_count(&self) -> usize {
        self.lwdg.v_count()
    }

    fn delete_edge(&mut self, from: L, to: L) {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwug add_edge : from Vertex doesn't exist");
        }
        if to_index.is_none() {
            panic!("lwug add_edge : to Vertex doesn't exist");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        //todo
        if from_index <= to_index {
            self.lwdg.delete_edge(from, to);
        } else {
            self.lwdg.delete_edge(to, from);
        }
    }

    fn delete_vertex(&mut self, vertex: L) {
        self.lwdg.delete_vertex(vertex);
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        self.lwdg.vertex_exists(vertex)
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwug add_edge : from Vertex doesn't exist");
        }
        if to_index.is_none() {
            panic!("lwug add_edge : to Vertex doesn't exist");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        // todo
        if from_index <= to_index {
            return self.lwdg.edge_exists(from, to);
        } else {
            return self.lwdg.edge_exists(to, from);
        }
    }
}
impl<L, W> Undirected<L> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq + Clone + std::fmt::Display,
    W: Num,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("lwug edges : Vertex doesn't exist");
        }
        let vertex_index = vertex_index.unwrap();
        let mut edges: Vec<L> = Vec::new();
        for i in 0..vertex_index {
            if self.lwdg.ldg.dg.adj[i].contains(&vertex_index) {
                edges.push(self.get_label(i).unwrap().clone());
            }
        }
        for item in self.lwdg.ldg.dg.adj[vertex_index].clone() {
            edges.push(self.get_label(item).unwrap().clone());
        }
        edges
    }

    fn delete_edges_from(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("lwug delete_edges_from : Vertex doesn't exist");
        }
        let vertex_index = vertex_index.unwrap();
        for from in 0..vertex_index {
            if self.lwdg.ldg.dg.adj[from].contains(&vertex_index) {
                self.delete_edge(self.get_label(from).unwrap().clone(), vertex.clone());
            }
        }
        for to in self.lwdg.ldg.dg.adj[vertex_index].clone() {
            self.delete_edge(vertex.clone(), self.get_label(to).unwrap().clone());
        }
    }
}
impl<L, W> Labeled<L> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq + Clone,
    W: Num,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.lwdg.edit_label(old_label, new_label);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        self.lwdg.get_label(vertex)
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        self.lwdg.get_index(label)
    }

    fn shrink(&mut self) {
        todo!()
    }
}
impl<L, W> Weighted<L, W> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq + Clone,
    W: Clone + Num,
{
    fn add_edge(&mut self, from: L, to: L, weight: W) {
        if self.get_index(&from) <= self.get_index(&to) {
            self.lwdg.add_edge(from, to, weight);
        } else {
            self.lwdg.add_edge(to, from, weight);
        }
    }

    fn edit_weight(&mut self, from: L, to: L, weight: W) {
        if self.get_index(&from) <= self.get_index(&to) {
            self.lwdg.edit_weight(from, to, weight);
        } else {
            self.lwdg.edit_weight(to, from, weight);
        }
    }

    fn get_weight(&mut self, from: L, to: L) -> W {
        if self.get_index(&from) <= self.get_index(&to) {
            self.lwdg.get_weight(from, to)
        } else {
            self.lwdg.get_weight(to, from)
        }
    }
}
