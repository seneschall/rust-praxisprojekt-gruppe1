use crate::graph::labeled_weighted_directed::LabeledWeightedDigraph;
use crate::traits::{Graph, Labeled, Undirected, Weighted};
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;
pub struct LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq,
{
    lwdg: LabeledWeightedDigraph<L, W>,
}

impl<L, W> LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq,
{
    pub fn new() {
        todo!()
    }
    pub fn from_adjacency_list() {
        todo!()
    }
}
impl<L, W> Graph<L> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq,
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

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L, W> Undirected<L> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        todo!()
    }

    fn delete_edges_from(&mut self, vertex: L) {
        todo!()
    }
}
impl<L, W> Labeled<L> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq,
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
impl<L, W> Weighted<L, W> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq,
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
