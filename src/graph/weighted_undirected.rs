use std::collections::HashMap;

use crate::graph::weighted_directed::WeightedDigraph;
use crate::traits::{Graph, UnLabeled, Undirected, Weighted};

#[cfg(test)]
mod test;
/// An indexed, mutable graph with undirected edges, where each edge represents a weight.
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type. 
/// Weights can have any type.
pub struct WeightedUGraph<W> {
    wdg: WeightedDigraph<W>,
}
impl<W> WeightedUGraph<W> {
    pub fn new() {
        todo!()
    }
    pub fn from_adjacency_list() {
        todo!()
    }
}

impl<W> Graph<usize> for WeightedUGraph<W> {
    fn add_vertex(&mut self, vertex: usize) -> usize {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        todo!()
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn delete_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        todo!()
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        todo!()
    }
}
impl<W> Undirected<usize> for WeightedUGraph<W> {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn delete_edges_from(&mut self, vertex: usize) {
        todo!()
    }
}
impl<W> UnLabeled<usize> for WeightedUGraph<W> {
    fn append_vertex(&mut self) -> usize {
        todo!()
    }
}
impl<W> Weighted<usize, W> for WeightedUGraph<W> {
    fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        todo!()
    }

    fn edit_weight(&mut self, from: usize, to: usize, weight: W) {
        todo!()
    }

    fn get_weight(&mut self, from: usize, to: usize) -> W {
        todo!()
    }
}
