use std::collections::HashMap;

use vers_vecs::RsVec;

use crate::graph::weighted_undirected::WeightedUGraph;
use crate::traits::{Graph, Unlabeled, Undirected, WTUndirected, WTWeighted, Weighted, WT};
use crate::wt::undirected::WTUGraph;

use super::weighted_directed::WeightedWTDigraph;

pub struct WeightedWTUGraph<W> {
    wug: WeightedWTDigraph<W>,
}
impl<W> WeightedWTUGraph<W> {
    pub fn from_weighted_ugraph(wug: WeightedUGraph<W>) -> Self {
        return WeightedWTUGraph {
            wug: WeightedWTDigraph::from_weighted_digraph(wug.wdg),
        };
    }
    pub fn from(
        sequence: Vec<usize>,
        starting_indices: RsVec,
        weights: HashMap<(usize, usize), W>,
    ) -> Self {
        return WeightedWTUGraph {
            wug: WeightedWTDigraph::from(sequence, starting_indices, weights),
        };
    }
}
impl<W> Graph<usize> for WeightedWTUGraph<W> {
    fn add_vertex(&mut self, vertex: usize) -> usize {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn v_count(&self) -> usize {
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

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        todo!()
    }
}
impl<W> Undirected<usize> for WeightedWTUGraph<W> {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn delete_edges_from(&mut self, vertex: usize) {
        todo!()
    }
}
impl<W> Unlabeled<usize> for WeightedWTUGraph<W> {
    fn append_vertex(&mut self) -> usize {
        todo!()
    }

    fn shrink(&mut self) -> Vec<Option<usize>> {
        todo!()
    }
}
impl<W> Weighted<usize, W> for WeightedWTUGraph<W> {
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
impl<W> WT<usize> for WeightedWTUGraph<W> {
    fn commit_edits(&mut self) {
        todo!()
    }

    // fn get_uncommitted_edits(&self) -> Option<std::collections::HashMap<usize, usize>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        todo!()
    }

    fn vertex_exists_updated(&self, vertex: usize) -> bool {
        todo!()
    }

    fn edge_exists_updated(&self, from: usize, to: usize) -> bool {
        todo!()
    }

    fn v_count_updated(&self) -> usize {
        todo!()
    }
    
    fn e_count_updated(&self) -> usize {
        todo!()
    }
}
impl<W> WTUndirected<usize> for WeightedWTUGraph<W> {
    fn edges_updated(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }
}
impl<W> WTWeighted<usize, W> for WeightedWTUGraph<W> {
    fn get_weight_updated(&mut self, from: usize, to: usize) -> W {
        todo!()
    }
}
