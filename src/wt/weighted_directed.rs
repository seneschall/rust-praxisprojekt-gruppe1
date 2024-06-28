use crate::traits::{Directed, Graph, UnLabeled, WTDirected, WTWeighted, Weighted, WT};
use crate::wt::directed::WTDigraph;

pub struct WeightedWTDigraph<W> {
    dg: WTDigraph,
    weights: W, // TODO so we have no compiler error
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

    fn shrink(&mut self) -> std::collections::HashMap<usize, usize> {
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
