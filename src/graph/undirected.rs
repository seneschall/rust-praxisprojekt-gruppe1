use crate::traits::Graph;
use std::collections::HashMap;

// USE ?
use super::{Undirected, Weighted};

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

    fn add_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn add_label(&mut self, vertex: usize, label: L) {
        todo!()
    }

    fn append_vertex(&mut self) -> usize {
        todo!()
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn delete_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn edit_label(&mut self, vertex: usize, change: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        todo!()
    }
}

impl<L> Undirected for UGraph<L> {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }
}

// Weighted Graph - definition and methods

pub struct Graph_Weighted<L> {
    test: L,
}

impl<L> Graph_Weighted<L> {}

impl<L> Graph<L> for Graph_Weighted<L> {
    fn add_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn add_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn add_label(&mut self, vertex: usize, label: L) {
        todo!()
    }

    fn append_vertex(&mut self) -> usize {
        todo!()
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn delete_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn edit_label(&mut self, vertex: usize, change: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        todo!()
    }
}

impl<L> Undirected for Graph_Weighted<L> {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }
}

impl<L> Weighted for Graph_Weighted<L> {
    fn weight_of_edge(&self, from: usize, to: usize) -> f64 {
        todo!()
    }
}
