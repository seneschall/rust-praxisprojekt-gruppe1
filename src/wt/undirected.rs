use std::collections::HashMap;

use vers_vecs::RsVec;

use crate::graph::undirected::UGraph;
use crate::traits::{Directed, WTDirected};
use crate::traits::{Graph, UnLabeled, Undirected, Unweighted, WTUndirected, WT};
use crate::wt::directed::WTDigraph; // needed because of WTDigraph
pub struct WTUGraph {
    wtd: WTDigraph,
}
impl WTUGraph {
    pub fn from_ugraph(ugraph : UGraph) -> Self{
        return WTUGraph{
            wtd: WTDigraph::from_digraph(ugraph.dg),
        }
    }
    pub fn from(sequence: Vec<usize>, starting_indices: RsVec) -> Self {
        return WTUGraph{
            wtd: WTDigraph::from(sequence, starting_indices),
        }
    }
}
impl Graph<usize> for WTUGraph {
    fn add_vertex(&mut self, vertex: usize) -> usize {
        self.wtd.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.wtd.e_count()
    }

    fn v_count(&self) -> usize {
        self.wtd.v_count()
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.wtd.delete_edge(from, to);
        } else {
            self.wtd.delete_edge(to, from);
        }
    }

    fn delete_vertex(&mut self, vertex: usize) {
        self.wtd.delete_vertex(vertex);
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        self.wtd.vertex_exists(vertex)
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        todo!()
    }
}
impl Undirected<usize> for WTUGraph {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        // returns all edges connected to vertex
        let mut edges: Vec<usize> = Vec::new();
        edges = self.wtd.incoming_edges(vertex); // all incoming edges of vertex
        edges.append(&mut self.wtd.outgoing_edges(vertex)); // + outgoing edges of vertex
        edges
    }

    fn delete_edges_from(&mut self, vertex: usize) {
        // deletes all edges connected to vertex
        for item in self.edges(vertex) {
            self.delete_edge(vertex, item);
        }
    }
}
impl UnLabeled<usize> for WTUGraph {
    fn append_vertex(&mut self) -> usize {
        self.wtd.append_vertex()
    }

    fn shrink(&mut self) -> Vec<Option<usize>> {
        todo!()
    }
}
impl Unweighted<usize> for WTUGraph {
    fn add_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.wtd.add_edge(from, to);
        } else {
            self.wtd.add_edge(to, from);
        }
    }
}
impl WT<usize> for WTUGraph {
    fn commit_edits(&mut self) {
        self.wtd.commit_edits();
    }

    // fn get_uncommitted_edits(&self) -> Option<HashMap<usize, usize>> {
    //     self.wtd.get_uncommitted_edits()
    // }

    fn discard_edits(&mut self) {
        self.wtd.discard_edits();
    }

    fn vertex_exists_updated(&self, vertex: usize) -> bool {
        self.wtd.vertex_exists_updated(vertex)
    }

    fn edge_exists_updated(&self, from: usize, to: usize) -> bool {
        todo!()
    }

    fn v_count_updated(&self) -> usize {
        todo!()
    }
}
impl WTUndirected<usize> for WTUGraph {
    fn edges_updated(&self, vertex: usize) -> Vec<usize> {
        // todo! might be wrong
        let mut up_edges: Vec<usize> = self.wtd.incoming_edges_updated(vertex);
        up_edges.append(&mut self.wtd.outgoing_edges(vertex));
        up_edges
    }
}
