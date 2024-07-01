use std::collections::HashMap;

use vers_vecs::RsVec;

use crate::graph::weighted_directed::WeightedDigraph;
use crate::traits::{Directed, Graph, UnLabeled, WTDirected, WTWeighted, Weighted, WT};
use crate::wt::directed::WTDigraph;
use crate::Edit;

pub struct WeightedWTDigraph<W> {
    dg: WTDigraph,
    weights_uncommitted: HashMap<(usize, usize), Edit<W>>,
    weights: HashMap<(usize, usize), W>,
}

impl<W> WeightedWTDigraph<W>{
    pub fn from_weighted_digraph(wdg : WeightedDigraph<W>) -> Self {
        return WeightedWTDigraph{
            dg: WTDigraph::from_digraph(wdg.dg),
            weights_uncommitted: HashMap::new(),
            weights: wdg.weights,
        }
    }
    pub fn from(sequence: Vec<usize>, starting_indices: RsVec, weights : HashMap<(usize,usize), W>) -> Self {
        return WeightedWTDigraph{
            dg: WTDigraph::from(sequence, starting_indices),
            weights_uncommitted: HashMap::new(),
            weights,
        }
    }
}
impl<W> Graph<usize> for WeightedWTDigraph<W> {
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
impl<W> Directed<usize> for WeightedWTDigraph<W> {
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn delete_outgoing_edges(&mut self, vertex: usize) {
        todo!()
    }

    fn delete_incoming_edges(&mut self, vertex: usize) {
        todo!()
    }
}
impl<W> UnLabeled<usize> for WeightedWTDigraph<W> {
    fn append_vertex(&mut self) -> usize {
        todo!()
    }
    
    fn shrink(&mut self) -> Vec<Option<usize>> {
        todo!()
    }
}
impl<W> Weighted<usize, W> for WeightedWTDigraph<W> {
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
impl<W> WT<usize> for WeightedWTDigraph<W> {
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
}
impl<W> WTDirected<usize> for WeightedWTDigraph<W> {
    fn outgoing_edges_updated(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn incoming_edges_updated(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }
}
impl<W> WTWeighted<usize, W> for WeightedWTDigraph<W> {
    fn get_weight_updated(&mut self, from: usize, to: usize) -> W {
        todo!()
    }
}
