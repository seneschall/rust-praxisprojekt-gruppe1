use crate::traits::Graph;
use std::collections::HashMap;

// USE ?
use super::{Delete, Undirected, Weighted};

// UNIT-TESTS for Graph and Weighted Graph
#[cfg(test)]
mod test {

    const V_COUNT: u32 = 10;
    #[test]
    fn create_new_graph() {}
    #[test]
    fn create_graph_from_adj() {
        // code for graph
    }
    #[test]
    fn create_weighted_graph_from_adj() {
        // code for graph_weighted
    }
    #[test]
    fn add_edge_to_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn add_vertex_to_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn append_vertex_to_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn remove_edge_from_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn remove_vertex_from_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn add_label_to_graphs() {}
    #[test]
    fn edit_label_on_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn edges_on_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn print_weight_of_graph_edge() {
        // code for graph_weighted
    }
    #[test]
    fn add_edit_weight_of_graph_edge() {
        // code for graph_weighted
    }
    #[test]
    fn delete_weight_of_graph_edge() {
        // code for graph_weighted
    }
}

//Graph - definition and methods

pub struct UGraph<L>
// same as digraph?!
{
    v_count: usize,                 // number of vertices
    e_count: usize,                 // number of edges
    adj: Vec<Vec<usize>>, // adjacency list of indices -- note from group: should we set this to pub(crate)?
    node_labels: HashMap<usize, L>, // format: index of node - value of node's label
}

impl<L> UGraph<L> // same as digraph?!
{
    fn new() -> Self {
        todo!();
    }
}

impl<L> Graph<L> for UGraph<L> {
    fn add_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn add_label(&mut self, vertex: usize, label: L) {
        todo!()
    }

    fn add_ledge(&mut self, from: L, to: L) {
        todo!()
    }

    fn add_lvertex(&mut self, label: L) {
        todo!()
    }

    fn add_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn append_vertex(&mut self) -> usize {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn edit_label(&mut self, vertex: usize, change: L) {
        todo!()
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }
}

impl<L> Delete<L> for UGraph<L> {
    fn delete_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn delete_and_shift(&mut self, vertex: usize) {
        todo!()
    }
}

impl<L> Undirected<L> for UGraph<L> {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn delete_edges_from(&self, vertex: usize) {
        todo!()
    }
}

// Weighted Graph - definition and methods

pub struct WeightedUGraph<L, W> {
    ug: UGraph<L>,
    weights: HashMap<(usize, usize), W>,
}

impl<L, W> Graph<L> for WeightedUGraph<L, W> {
    fn add_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn add_ledge(&mut self, from: L, to: L) {
        todo!()
    }

    fn add_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn add_lvertex(&mut self, label: L) {
        todo!()
    }

    fn add_label(&mut self, vertex: usize, label: L) {
        todo!()
    }

    fn append_vertex(&mut self) -> usize {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn edit_label(&mut self, vertex: usize, label: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        todo!()
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }
}

impl<L, W> Delete<L> for WeightedUGraph<L, W> {
    fn delete_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn delete_and_shift(&mut self, vertex: usize) {
        todo!()
    }
}

impl<L, W> Undirected<L> for WeightedUGraph<W, L> {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn delete_edges_from(&mut self, vertex: usize) {
        todo!()
    }
}

impl<L, W> Weighted<W> for WeightedUGraph<L, W> {
    fn weight(&self, from: usize, to: usize) -> W {
        todo!()
    }

    fn edit_weight(&mut self, from: usize, to: usize, weight: W) {
        todo!()
    }
}
