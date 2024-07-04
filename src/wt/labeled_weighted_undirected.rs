use vers_vecs::RsVec;

use crate::graph::labeled_weighted_undirected::LabeledWeightedUGraph;
use crate::traits::{
    Graph, Labeled, Undirected, WTLabeled, WTUndirected, WTWeighted, Weighted, WT,
};
use crate::wt::labeled_weighted_directed::LabeledWeightedWTDigraph;

use std::collections::HashMap;
use std::hash::Hash;

/// A structure holding an immutable Wavelet-Tree-Representation of a graph with directed edges and labeled vertices, where each edge represents a weight, plus information on manual changes. 
/// The greatest possible of number of edges or of vertices is usize. Labels and Weights can have any type, Labels are referenced.
pub struct LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
{
    lwdg: LabeledWeightedWTDigraph<L, W>,
}
impl<L, W> LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
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
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }

    fn delete_edge(&mut self, from: L, to: L) {
        todo!()
    }

    fn delete_vertex(&mut self, vertex: L) {
        todo!()
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L, W> Undirected<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        todo!()
    }

    fn delete_edges_from(&mut self, vertex: L) {
        todo!()
    }
}
impl<L, W> Labeled<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        todo!()
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        todo!()
    }

    fn shrink(&mut self) -> std::collections::HashMap<L, Option<L>> {
        todo!()
    }
}
impl<L, W> Weighted<L, W> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn add_edge(&mut self, from: L, to: L, weight: W) {
        todo!()
    }

    fn edit_weight(&mut self, from: L, to: L, weight: W) {
        todo!()
    }

    fn get_weight(&mut self, from: L, to: L) -> W {
        todo!()
    }
}
impl<L, W> WT<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn commit_edits(&mut self) {
        todo!()
    }

    // fn get_uncommitted_edits(&self) -> Option<std::collections::HashMap<usize, L>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        todo!()
    }

    fn vertex_exists_updated(&self, vertex: L) -> bool {
        todo!()
    }

    fn edge_exists_updated(&self, from: L, to: L) -> bool {
        todo!()
    }

    fn v_count_updated(&self) -> usize {
        todo!()
    }
    
    fn e_count_updated(&self) -> usize {
        todo!()
    }
}
impl<L, W> WTWeighted<L, W> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn get_weight_updated(&mut self, from: L, to: L) -> W {
        todo!()
    }
}
impl<L, W> WTUndirected<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn edges_updated(&self, vertex: usize) -> Vec<L> {
        todo!()
    }
}

impl<L, W> WTLabeled<L> for LabeledWeightedWTUGraph<L, W>
where
    L: Clone + Hash + Eq,
{
    fn get_label_updated(&self, index: usize) -> Option<&L> {
        todo!()
    }

    fn get_index_updated(&self, label: &L) -> Option<usize> {
        todo!()
    }
}
