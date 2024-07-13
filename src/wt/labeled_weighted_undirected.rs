use num::Num;
use vers_vecs::RsVec;

use crate::graph::labeled_weighted_undirected::LabeledWeightedUGraph;
use crate::traits::{
    Directed, Graph, Labeled, Undirected, WTDirected, WTLabeled, WTUndirected, WTWeighted,
    Weighted, WT,
};
use crate::wt::labeled_weighted_directed::LabeledWeightedWTDigraph;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::hash::Hash;

/// A structure holding an immutable Wavelet-Tree-Representation of a graph with directed edges and labeled vertices, where each edge represents a weight, plus information on manual changes.
/// The greatest possible of number of edges or of vertices is usize. Labels and Weights can have any type, Labels are referenced.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub struct LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Num,
{
    lwdg: LabeledWeightedWTDigraph<L, W>,
}
impl<L, W> LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Num,
{
    pub fn from_labeled_weighted_ugraph(lwug: LabeledWeightedUGraph<L, W>) -> Self {
        return LabeledWeightedWTUGraph {
            lwdg: LabeledWeightedWTDigraph::from_labeled_weighted_digraph(lwug.lwdg),
        };
    }
    pub fn from(
        sequence: Vec<usize>,
        starting_indices: RsVec,
        labels: Vec<L>,
        weights: HashMap<(usize, usize), W>,
    ) -> Self {
        LabeledWeightedWTUGraph {
            lwdg: LabeledWeightedWTDigraph::from(sequence, starting_indices, labels, weights),
        }
    }
}

impl<L, W> Graph<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone + Num,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        return self.lwdg.add_vertex(vertex);
    }

    fn e_count(&self) -> usize {
        return self.lwdg.e_count();
    }

    fn v_count(&self) -> usize {
        return self.lwdg.v_count();
    }

    fn delete_edge(&mut self, from: L, to: L) {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.get_index_updated(&from);
        let to_index = self.get_index_updated(&to);
        if from_index.is_none() {
            panic!("wtlwug delete_edge : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlwug delete_edge : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
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
        return self.lwdg.vertex_exists(vertex);
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("wtlwug edge_exists : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlwug edge_exists : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        if from_index <= to_index {
            return self.lwdg.edge_exists(from, to);
        } else {
            return self.lwdg.edge_exists(to, from);
        }
    }
}
impl<L, W> Undirected<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone + Num,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("wtlwug edges : Vertex doesn't exist");
        }

        // returns all edges connected to vertex
        let mut edges: Vec<L>;
        edges = self.lwdg.incoming_edges(vertex.clone()); // all incoming edges of vertex
        if self.edge_exists(vertex.clone(), vertex.clone()) {
            for item in self.lwdg.outgoing_edges(vertex.clone()) {
                if item != vertex {
                    edges.push(item);
                }
            }
            return edges;
        } else {
            edges.append(&mut self.lwdg.outgoing_edges(vertex)); // + outgoing edges of vertex
            return edges;
        }
    }

    fn delete_edges_from(&mut self, vertex: L) {
        for item in self.edges_updated(vertex.clone()) {
            self.delete_edge(vertex.clone(), item);
        }
    }
}
impl<L, W> Labeled<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Num,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.lwdg.edit_label(old_label, new_label);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        return self.lwdg.get_label(vertex);
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        return self.lwdg.get_index(label);
    }

    fn shrink(&mut self) {
        self.lwdg.shrink();
    }
}
impl<L, W> Weighted<L, W> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone + Num,
{
    fn add_edge(&mut self, from: L, to: L, weight: W) {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("wtlwug add_edge : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlwug add_edge : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        if from_index <= to_index {
            self.lwdg.add_edge(from, to, weight);
        } else {
            self.lwdg.add_edge(to, from, weight);
        }
    }

    fn edit_weight(&mut self, from: L, to: L, weight: W) {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("wtlwug edit_weight : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlwug edit_weight : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        if from_index <= to_index {
            self.lwdg.edit_weight(from, to, weight);
        } else {
            self.lwdg.edit_weight(to, from, weight);
        }
    }

    fn get_weight(&mut self, from: L, to: L) -> W {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("wtlwug get_weight : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlwug get_weight : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        if from_index <= to_index {
            return self.lwdg.get_weight(from, to);
        } else {
            return self.lwdg.get_weight(to, from);
        }
    }
}
impl<L, W> WT<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone + Num,
{
    fn commit_edits(&mut self) {
        self.lwdg.commit_edits();
    }

    // fn get_uncommitted_edits(&self) -> Option<std::collections::HashMap<usize, L>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        self.lwdg.discard_edits();
    }

    fn vertex_exists_updated(&self, vertex: L) -> bool {
        return self.lwdg.vertex_exists_updated(vertex);
    }

    fn edge_exists_updated(&self, from: L, to: L) -> bool {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("wtlwug edge_exists_updated : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlwug edge_exists_updated : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        if from_index <= to_index {
            return self.lwdg.edge_exists_updated(from, to);
        } else {
            return self.lwdg.edge_exists_updated(to, from);
        }
    }

    fn v_count_updated(&self) -> usize {
        return self.lwdg.v_count_updated();
    }

    fn e_count_updated(&self) -> usize {
        return self.lwdg.e_count_updated();
    }
}
impl<L, W> WTWeighted<L, W> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone + Num,
{
    fn get_weight_updated(&mut self, from: L, to: L) -> W {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("wtlwug get_weight_updated : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlwug get_weight_updated : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        if from_index <= to_index {
            return self.lwdg.get_weight_updated(from, to);
        } else {
            return self.lwdg.get_weight_updated(to, from);
        }
    }
}
impl<L, W> WTUndirected<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone + Num,
{
    fn edges_updated(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("wtlwug edges_updated : Vertex doesn't exist");
        }

        let mut edges: Vec<L>;
        edges = self.lwdg.incoming_edges_updated(vertex.clone()); // all incoming edges of vertex
        if self.edge_exists_updated(vertex.clone(), vertex.clone()) {
            for item in self.lwdg.outgoing_edges_updated(vertex.clone()) {
                if item != vertex {
                    edges.push(item);
                }
            }
            return edges;
        } else {
            edges.append(&mut self.lwdg.outgoing_edges_updated(vertex)); // + outgoing edges of vertex
            return edges;
        }
    }
}

impl<L, W> WTLabeled<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Clone + Hash + Eq,
    W: Num,
{
    fn get_label_updated(&self, index: usize) -> Option<&L> {
        return self.lwdg.get_label_updated(index);
    }

    fn get_index_updated(&self, label: &L) -> Option<usize> {
        return self.lwdg.get_index_updated(label);
    }
}
