use std::collections::HashMap;
use crate::traits::{Directed, Graph, Unlabeled, Unweighted, WTDirected, WTWeighted, Weighted, WT};
use crate::wt::directed::WTDigraph;
use crate::Edit;
use core::clone::Clone;

/// A structure holding an immutable Wavelet-Tree-Representation of an indexed graph with directed edges, where each edge represents a weight, plus information on manual changes. 
/// The greatest possible of number of edges or of vertices is usize vertices, vertex-indices are also usize-data-type. Weights can have any type.
pub struct WeightedWTDigraph<W> {
    dg: WTDigraph,
    weights: HashMap<(usize, usize), W>,
    weights_updated: HashMap<(usize, usize), W>,
}

impl<W> Graph<usize> for WeightedWTDigraph<W> {
    fn add_vertex(&mut self, vertex: &usize) -> usize {
        return self.dg.add_vertex(&vertex);
    }

    fn e_count(&self) -> usize {
        return self.dg.e_count();
    }

    fn v_count(&self) -> usize {
        return self.dg.v_count();
    }

    fn delete_edge(&mut self, from: &usize, to: &usize) {
        self.dg.delete_edge(&from, &to);
        self.weights_updated.remove(&(*from,*to));
    }

    fn delete_vertex(&mut self, vertex: &usize) {
        self.dg.delete_vertex(&vertex);
        let outgoing: Vec<&usize> = self.outgoing_edges(vertex);
        let incoming: Vec<&usize> = self.incoming_edges(vertex);
        for item in outgoing {
            self.weights_updated.remove(&(*vertex,item));
        }
        for item in incoming {
            self.weights_updated.remove(&(item,*vertex));
        }
    }

    fn vertex_exists(&self, vertex: &usize) -> bool {
        return self.dg.vertex_exists(vertex);
    }

    fn shrink(&mut self) -> std::collections::HashMap<usize, usize> {
        return self.dg.shrink();
    }

    fn edge_exists(&self, from: &usize, to: &usize) -> bool {
        return self.dg.edge_exists(from, to);
    }
}
impl<W> Directed<usize> for WeightedWTDigraph<W> {
    fn outgoing_edges(&self, vertex: &usize) -> Vec<&usize> {
        return self.dg.outgoing_edges(vertex)
    }

    fn incoming_edges(&self, vertex: &usize) -> Vec<&usize> {
        return  self.dg.incoming_edges(vertex)
    }

    fn delete_outgoing_edges(&mut self, vertex: &usize) {
        let edges = self.dg.outgoing_edges(vertex);
        for item in edges {
            self.delete_edge(vertex, item);
        }
    }

    fn delete_incoming_edges(&mut self, vertex: &usize) {
        let vertices = self.dg.incoming_edges(vertex);
        for item in vertices {
            self.delete_edge(item, vertex);
        }
    }
}

impl<W> Unlabeled<usize> for WeightedWTDigraph<W> {
    fn append_vertex(&mut self) -> usize {
        return self.dg.append_vertex();
    }
}
impl<W> Weighted<usize, W> for WeightedWTDigraph<W> {

    fn add_edge(&mut self, from: &usize, to: &usize, weight: W) {
        // only adds to uncommitted edits
        // todo; its possible to add the same edge multiple times
        if !(self.vertex_exists_updated(from) && self.vertex_exists_updated(to)) {
            panic!("Failed to add edge.");
        }

        if self.edge_exists_updated(from, to) {
            panic!("Edge already exists.");
        }

        match self.dg.adj_uncommitted.get_mut(&from) {
            Some(adj) => {
                adj.push(Edit::Add(to));
                self.weights_updated.insert((*from,*to), weight);
            }
            None => {
                self.dg.adj_uncommitted.insert(*from, vec![Edit::Add(to)]);
                self.weights_updated.insert((*from,*to), weight);
            }
        }
        self.dg.has_uncommitted_edits = true;
        self.dg.e_count_updated += 1; // added this line, else adding an edge doesn't update e_count_updated
    }
    
    fn edit_weight(&mut self, from: &usize, to: &usize, weight: W) {
        self.weights_updated.insert((*from, *to),weight);
    }
    
    fn get_weight(&mut self, from: &usize, to: &usize) -> W  {
        if !(self.edge_exists(from, to)) {
            panic!("the edge doesn't exist");
        }
        return (*(self.weights.get(&(*from,*to)).unwrap())).clone();
    }
}

impl<W> WT<usize> for WeightedWTDigraph<W> {
    fn commit_edits(&mut self) {
        todo!()
    }

    fn discard_edits(&mut self) {
        self.dg.discard_edits();
    }

    fn vertex_exists_updated(&self, vertex: &usize) -> bool {
        return self.dg.vertex_exists_updated(vertex);
    }

    fn edge_exists_updated(&self, from: &usize, to: &usize) -> bool {
        return self.dg.edge_exists_updated(from, to);
    }
    
    fn v_count_updated(&self) -> usize {
        return self.dg.v_count_updated();
    }
}
impl<W> WTDirected<usize> for WeightedWTDigraph<W> {
    fn outgoing_edges_updated(&self, vertex: &usize) -> Vec<usize> {
        return self.dg.outgoing_edges_updated(vertex);
    }

    fn incoming_edges_updated(&self, vertex: &usize) -> Vec<usize> {
        return self.dg.incoming_edges_updated(vertex);
    }
}
impl<W> WTWeighted<usize, W> for WeightedWTDigraph<W> 
where W: Clone {
    fn get_weight_updated(&mut self, from: &usize, to: &usize) -> W {
        if !self.edge_exists_updated(from, to) {
            panic!("this edge doesn't exist anymore, it doesn't have any weight");
        }
        else {
            if !self.edge_exists(from, to) {
                return (*(self.weights_updated.get(&(*from,*to)).unwrap())).clone();
            }
            else {
                return self.get_weight(from, to);
            }
        }
    }
}
