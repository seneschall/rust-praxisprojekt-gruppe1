use std::collections::HashMap;

use crate::graph::weighted_directed::WeightedDigraph;
use crate::traits::{Graph, UnLabeled, Undirected, Weighted};

#[cfg(test)]
mod test;
pub struct WeightedUGraph<W> {
    wdg: WeightedDigraph<W>,
    weights: HashMap<(usize, usize), W>,
}
impl<W> WeightedUGraph<W>
where
    W: Clone,
{
    pub fn new() -> Self {
        WeightedUGraph {
            wdg: WeightedDigraph::new(),
            weights: HashMap::new(),
        }
    }
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<(usize, W)>>) -> Self {
        let mut hashmap_weights: HashMap<(usize, usize), W> = HashMap::new();
        if !(v_count == adj.len()) {
            panic!("v_count != adj.len()")
        }
        let mut j = 0;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; v_count];
        for item in adj {
            for i in 0..item.len() {
                let (to, weight): (usize, W) = item[i].clone();
                hashmap_weights.insert((j, to), weight);
                adj_list[j].push(to);
            }
            j += 1;
        }
        WeightedUGraph {
            wdg: WeightedDigraph::from_adjacency_list(v_count, e_count, adj_list),
            weights: hashmap_weights,
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
        todo!()
    }

    fn delete_vertex(&mut self, vertex: usize) {
        self.wdg.delete_vertex(vertex)
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
