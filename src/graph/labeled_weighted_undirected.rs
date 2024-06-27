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
    L: Hash + Eq + Clone+ std::fmt::Display,
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
        //todo
        self.lwdg.delete_edge(from, to);
    }

    fn delete_vertex(&mut self, vertex: L) {
        self.lwdg.delete_vertex(vertex);
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        self.lwdg.vertex_exists(vertex)
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        self.lwdg.edge_exists(from, to)
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
    L: Hash + Eq + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.lwdg.edit_label(old_label, new_label);
    }

    fn get_label(&self, vertex: usize) -> L {
        self.lwdg.get_label(vertex)
    }

    fn get_index(&self, label: L) -> usize {
        self.lwdg.get_index(label)
    }
}
impl<L, W> Weighted<L, W> for LabeledWeightedUGraph<L, W>
where
    L: Hash + Eq + Clone,
    W: Clone,
{
    fn add_edge(&mut self, from: L, to: L, weight: W) {
        todo!()
    }

    fn edit_weight(&mut self, from: L, to: L, weight: W) {
        self.lwdg.edit_weight(from,to,weight);
    }

    fn get_weight(&mut self, from: L, to: L) -> W {
        self.lwdg.get_weight(from, to)
    }
}
