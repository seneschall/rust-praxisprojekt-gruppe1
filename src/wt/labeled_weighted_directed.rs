use vers_vecs::RsVec;

use crate::graph::labeled_weighted_directed::LabeledWeightedDigraph;
use crate::traits::{
    Directed, Graph, Labeled, Unweighted, WTDirected, WTLabeled, WTWeighted, Weighted, WT,
};
use crate::wt::labeled_directed::LabeledWTDigraph;
use crate::Edit;
use std::collections::HashMap;
use std::hash::Hash;

/// A structure holding an immutable Wavelet-Tree-Representation of a graph with directed edges and labeled vertices, where each edge represents a weight, plus information on manual changes. 
/// The greatest possible of number of edges or of vertices is usize. Labels and Weights can have any type, Labels are referenced.

pub struct LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
{
    ldg: LabeledWTDigraph<L>,
    weights_uncommitted: HashMap<(usize, usize), Edit<W>>,
    weights: HashMap<(usize, usize), W>,
}
impl<L, W> LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
{
    pub fn from_labeled_weighted_digraph(lwdg: LabeledWeightedDigraph<L, W>) -> Self {
        return LabeledWeightedWTDigraph {
            ldg: LabeledWTDigraph::from_labeled_digraph(lwdg.ldg),
            weights_uncommitted: HashMap::new(),
            weights: lwdg.weights,
        };
    }

    pub fn from(
        sequence: Vec<usize>,
        starting_indices: RsVec,
        labels: Vec<L>,
        weights: HashMap<(usize, usize), W>,
    ) -> Self {
        return LabeledWeightedWTDigraph {
            ldg: LabeledWTDigraph::from(sequence, starting_indices, labels),
            weights_uncommitted: HashMap::new(),
            weights,
        };
    }
}

impl<L, W> Graph<L> for LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone,
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
            panic!("lwdg add_edge : from is none");
        }
        if to_index.is_none() {
            panic!("lwdg add_edge : tois none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        self.ldg.delete_edge(from.clone(), to.clone());
        let weight = self.get_weight(from, to); // checkme fixme seems ugly this way
        self.weights_uncommitted
            .insert((from_index, to_index), Edit::Delete(weight));
    }

    fn delete_vertex(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("lwdg delete_vertex : vertex is none");
        }
        if !self.vertex_exists(vertex.clone()) {
            panic!("lwdg delete_vertex : vertex does not exist");
        }
        self.delete_incoming_edges(vertex.clone());
        self.delete_outgoing_edges(vertex.clone());
        self.ldg.delete_vertex(vertex);
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        self.ldg.vertex_exists(vertex)
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        self.ldg.edge_exists(from, to)
    }
}
impl<L, W> Directed<L> for LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone,
{
    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        self.ldg.outgoing_edges(vertex)
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        self.ldg.incoming_edges(vertex)
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        for to in self.outgoing_edges(vertex.clone()) {
            self.delete_edge(vertex.clone(), to);
        }
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        for from in self.incoming_edges(vertex.clone()) {
            self.delete_edge(from, vertex.clone());
        }
    }
}
impl<L, W> Labeled<L> for LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.ldg.edit_label(old_label, new_label);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        self.ldg.get_label(vertex)
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        self.ldg.get_index(label)
    }

    fn shrink(&mut self) -> HashMap<L, Option<L>> {
        // todo see ldg
        self.ldg.shrink();
        return HashMap::new();
    }
}
impl<L, W> Weighted<L, W> for LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone,
{
    fn add_edge(&mut self, from: L, to: L, weight: W) {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwdg add_edge : from is none");
        }
        if to_index.is_none() {
            panic!("lwdg add_edge : tois none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        self.ldg.add_edge(from, to);
        self.weights_uncommitted
            .insert((from_index, to_index), Edit::Add(weight));
    }

    fn edit_weight(&mut self, from: L, to: L, weight: W) {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwdg add_edge : from is none");
        }
        if to_index.is_none() {
            panic!("lwdg add_edge : tois none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        if !self.edge_exists(from, to) {
            panic!("edge doesn't exist");
        }
        self.weights_uncommitted
            .insert((from_index, to_index), Edit::Add(weight));
    }

    fn get_weight(&mut self, from: L, to: L) -> W {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwdg add_edge : from is none");
        }
        if to_index.is_none() {
            panic!("lwdg add_edge : to is none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        return self.weights.get(&(from_index, to_index)).unwrap().clone();
    }
}
impl<L, W> WT<L> for LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone,
{
    fn commit_edits(&mut self) {
        for ((from,to), weight) in &self.weights_uncommitted{
            match weight{
                Edit::Add(add_weight) => {
                    self.weights.insert((*from,*to), add_weight.clone());
                }
                Edit::Delete(_delete_weight) => {
                    self.weights.remove(&(*from,*to));
                }
            }
        }
        self.ldg.commit_edits();

    }

    // fn get_uncommitted_edits(&self) -> Option<std::collections::HashMap<usize, L>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        self.ldg.discard_edits();
        self.weights_uncommitted = HashMap::new();
    }

    fn vertex_exists_updated(&self, vertex: L) -> bool {
        self.ldg.vertex_exists(vertex)
    }

    fn edge_exists_updated(&self, from: L, to: L) -> bool {
        self.ldg.edge_exists_updated(from, to)
    }

    fn v_count_updated(&self) -> usize {
        self.ldg.v_count_updated()
    }
    
    fn e_count_updated(&self) -> usize {
        return self.ldg.e_count_updated();
    }
}
impl<L, W> WTWeighted<L, W> for LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
    W: Clone,
{
    fn get_weight_updated(&mut self, from: L, to: L) -> W {
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
        if self
            .weights_uncommitted
            .contains_key(&(from_index, to_index))
        {
            match self
                .weights_uncommitted
                .get(&(from_index, to_index))
                .unwrap()
            {
                Edit::Add(weight) => {
                    return weight.to_owned();
                }
                Edit::Delete(_weight) => {
                    if self.weights.contains_key(&(from_index, to_index)) {
                        return self
                            .weights
                            .get(&(from_index, to_index))
                            .unwrap()
                            .to_owned();
                    } else {
                        panic!(
                            "lwdg get_weights_updated : Something went wrong, weight is missing"
                        );
                    }
                }
            }
        } else {
            panic!("lwdg get_weights_updated : Something went wrong,weight is missing")
        }
    }
}
impl<L, W> WTDirected<L> for LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn outgoing_edges_updated(&self, vertex: L) -> Vec<L> {
        self.ldg.outgoing_edges_updated(vertex)
    }

    fn incoming_edges_updated(&self, vertex: L) -> Vec<L> {
        self.ldg.incoming_edges_updated(vertex)
    }
}

impl<L, W> WTLabeled<L> for LabeledWeightedWTDigraph<L, W>
where
    L: Hash + Eq + Clone,
{
    fn get_label_updated(&self, index: usize) -> Option<&L> {
        self.ldg.get_label_updated(index)
    }

    fn get_index_updated(&self, label: &L) -> Option<usize> {
        self.ldg.get_index_updated(label)
    }
}
