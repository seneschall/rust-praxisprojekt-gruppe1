use std::collections::HashMap;

use crate::graph::weighted_directed::WeightedDigraph;
use crate::traits::{Graph, UnLabeled, Undirected, Weighted};

#[cfg(test)]
mod test;
pub struct WeightedUGraph<W> {
    wdg: WeightedDigraph<W>,
}
impl<W> WeightedUGraph<W>
where
    W: Clone,
{
    pub fn new() -> Self {
        WeightedUGraph {
            wdg: WeightedDigraph::new(),
        }
    }
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<(usize, W)>>) -> Self {
        WeightedUGraph {
            wdg: WeightedDigraph::from_adjacency_list(v_count, e_count, adj),
        }
    }
}


impl<W> Graph<usize> for WeightedUGraph<W> {
    fn add_vertex(&mut self, vertex: usize) -> usize {
        self.wdg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.wdg.e_count()
    }

    fn v_count(&self) -> usize {
        self.wdg.v_count()
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        //todo
        self.wdg.delete_edge(from,to);
    }

    fn delete_vertex(&mut self, vertex: usize) {
        self.wdg.delete_vertex(vertex)
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        self.wdg.vertex_exists(vertex)
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()      // erstmal unwichtig
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        self.wdg.edge_exists(from, to)
    }
}
impl<W> Undirected<usize> for WeightedUGraph<W> {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        todo!()      // erstmal unwichtig
    }

    fn delete_edges_from(&mut self, vertex: usize) {
        todo!()     // erstmal unwichtig
    }
}
impl<W> UnLabeled<usize> for WeightedUGraph<W> {
    fn append_vertex(&mut self) -> usize {
        self.wdg.append_vertex()
    }
}
impl<W> Weighted<usize, W> for WeightedUGraph<W> 
where W : Copy,
{
    fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        //todo
        self.wdg.add_edge(from, to, weight);
    }

    fn edit_weight(&mut self, from: usize, to: usize, weight: W) {
        self.wdg.edit_weight(from, to, weight);
    }

    fn get_weight(&mut self, from: usize, to: usize) -> W {
        //todo
        return self.wdg.get_weight(from,to);
    }
}
