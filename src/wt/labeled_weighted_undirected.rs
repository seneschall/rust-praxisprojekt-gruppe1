use crate::traits::{Graph, Labeled, Undirected, WTUndirected, WTWeighted, Weighted, WT};
use crate::wt::labeled_directed::LabeledWTDigraph;
use std::hash::Hash;

/// A structure holding an immutable Wavelet-Tree-Representation of a graph with directed edges and labeled vertices, where each edge represents a weight, plus information on manual changes. 
/// The greatest possible of number of edges or of vertices is usize. Labels and Weights can have any type, Labels are referenced.
pub struct LabeledWeightedUGraph<L, W>
where
    L: Hash + Clone + Eq,
{
    dg: LabeledWTDigraph<L>,
    weights: W, // TODO so we have no compiler error
}

impl<L, W> Graph<L> for LabeledWeightedDigraph<L, W>
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

    fn vertex_deleted(&self, vertex: L) -> bool {
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

    fn shrink(&mut self) -> std::collections::HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L, W> Undirected<L> for LabeledWeightedDigraph<L, W>
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
impl<L, W> Labeled<L> for LabeledWeightedDigraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        todo!()
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        todo!()
    }
}
impl<L, W> Weighted<L, W> for LabeledWeightedDigraph<L, W>
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
impl<L, W> WT<L> for LabeledWeightedDigraph<L, W>
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
}
impl<L, W> WTWeighted<L, W> for LabeledWeightedDigraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn get_weight_updated(&mut self, from: L, to: L) -> W {
        todo!()
    }
}
impl<L, W> WTUndirected<L> for LabeledWeightedDigraph<L, W>
where
    L: Hash + Clone + Eq,
{
    fn edges_updated(&self, vertex: usize) -> Vec<L> {
        todo!()
    }
}
