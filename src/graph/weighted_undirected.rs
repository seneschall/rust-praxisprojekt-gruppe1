use std::collections::HashMap;

use crate::graph::weighted_directed::WeightedDigraph;
use crate::traits::{Graph, UnLabeled, Undirected, Weighted};

#[cfg(test)]
mod test;
pub struct WeightedUGraph<W> {
    pub(crate) wdg: WeightedDigraph<W>,
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
        if from <= to {
            self.wdg.delete_edge(from, to);
        } else {
            self.wdg.delete_edge(to, from);
        }
    }

    fn delete_vertex(&mut self, vertex: usize) {
        self.wdg.delete_vertex(vertex)
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        self.wdg.vertex_exists(vertex)
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        self.wdg.edge_exists(from, to)
    }
}
impl<W> Undirected<usize> for WeightedUGraph<W> {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        todo!() // erstmal unwichtig
    }

    fn delete_edges_from(&mut self, vertex: usize) {
        todo!() // erstmal unwichtig
    }
}
impl<W> UnLabeled<usize> for WeightedUGraph<W> {
    fn append_vertex(&mut self) -> usize {
        self.wdg.append_vertex()
    }

    fn shrink(&mut self) -> Vec<Option<usize>> {
        todo!()
    }
}
impl<W> Weighted<usize, W> for WeightedUGraph<W>
where
    W: Copy,
{
    fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        if from <= to {
            self.wdg.add_edge(from, to, weight);
        } else {
            self.wdg.add_edge(to, from, weight);
        }
        
    }

    fn edit_weight(&mut self, from: usize, to: usize, weight: W) {
        if from <= to {
            self.wdg.edit_weight(from, to, weight);
        } else {
            self.wdg.edit_weight(to, from, weight);
        }

    }

    fn get_weight(&mut self, from: usize, to: usize) -> W {
        if from <= to {
            return self.wdg.get_weight(from, to);
        } else {
            return self.wdg.get_weight(to, from);
        }
        
    }
}
