use vers_vecs::RsVec;

use crate::graph::labeled_undirected::LabeledUGraph;
use crate::traits::{
    Directed, Graph, Labeled, Undirected, Unweighted, WTDirected, WTLabeled, WTUndirected, WT,
};
use crate::wt::labeled_directed::LabeledWTDigraph;
use serde::{Deserialize, Serialize};

use std::hash::Hash;

// An labeled wavelet-tree-graph with undirected edges. (L-wt-ugraph)
// The L-wt-ugraph holds a L-wt-digraph. All operations on the L-wt-digraph can be performed on the L-wt-ugraph.
// The only divergent implementations are regarding the "doubling" of edges due to no directions.
// Users can perfom fast operations on the original graph and slower operations on the recent state of the graph.
// Users can integrate the recent state of the graph into the QW-Tree by rebuilding it using the commit_edits-function.
// See module wt::labeled_directed for the L-wt-digraph struct definition. See more documentation on function-level and in the crate introduction.
// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type. Labels can have any type and are referenced.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    ldg: LabeledWTDigraph<L>,
}

impl<L> LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    pub fn from_labeled_ugraph(lug: LabeledUGraph<L>) -> Self {
        return LabeledWTUGraph {
            ldg: LabeledWTDigraph::from_labeled_digraph(lug.ldg),
        };
    }
    pub fn from(sequence: Vec<usize>, starting_indices: RsVec, labels: Vec<L>) -> Self {
        return LabeledWTUGraph {
            ldg: LabeledWTDigraph::from(sequence, starting_indices, labels),
        };
    }
}
impl<L> Graph<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    // this function needs documentation
    fn add_vertex(&mut self, vertex: L) -> usize {
        return self.ldg.add_vertex(vertex);
    }

    fn e_count(&self) -> usize {
        return self.ldg.e_count();
    }

    fn v_count(&self) -> usize {
        return self.ldg.v_count();
    }

    fn delete_edge(&mut self, from: L, to: L) {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.index_updated(&from);
        let to_index = self.index_updated(&to);
        if from_index.is_none() {
            panic!("wtlug delete_edge : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlug delete_edge : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        if from_index <= to_index {
            self.ldg.delete_edge(from, to);
        } else {
            self.ldg.delete_edge(to, from);
        }
    }

    fn delete_vertex(&mut self, vertex: L) {
        self.ldg.delete_vertex(vertex)
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        return self.ldg.vertex_exists(vertex);
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.index(&from);
        let to_index = self.index(&to);
        if from_index.is_none() {
            panic!("wtlug edge_exists : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlug edge_exists : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        if from_index <= to_index {
            return self.ldg.edge_exists(from, to);
        } else {
            return self.ldg.edge_exists(to, from);
        }
    }
}
impl<L> Undirected<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.index(&vertex);
        if vertex_index.is_none() {
            panic!("wtlug edges : Vertex doesn't exist");
        }

        // returns all edges connected to vertex
        let mut edges: Vec<L>;
        edges = self.ldg.incoming_edges(vertex.clone()); // all incoming edges of vertex
        if self.edge_exists(vertex.clone(), vertex.clone()) {
            for item in self.ldg.outgoing_edges(vertex.clone()) {
                if item != vertex {
                    edges.push(item);
                }
            }
            return edges;
        } else {
            edges.append(&mut self.ldg.outgoing_edges(vertex)); // + outgoing edges of vertex
            return edges;
        }
    }

    fn delete_edges_from(&mut self, vertex: L) {
        for item in self.edges_updated(vertex.clone()) {
            self.delete_edge(vertex.clone(), item);
        }
    }
}
impl<L> Labeled<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.ldg.edit_label(old_label, new_label);
    }

    fn label(&self, vertex: usize) -> Option<&L> {
        return self.ldg.label(vertex);
    }

    fn index(&self, label: &L) -> Option<usize> {
        return self.ldg.index(label);
    }

    fn shrink(&mut self) {
        self.ldg.shrink();
    }
}
impl<L> Unweighted<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    // this function needs documentation
    fn add_edge(&mut self, from: L, to: L) {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.index(&from);
        let to_index = self.index(&to);
        if from_index.is_none() {
            panic!("wtlug add_edge : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlug add_edge : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        if from_index <= to_index {
            self.ldg.add_edge(from, to);
        } else {
            self.ldg.add_edge(to, from);
        }
    }
}
impl<L> WT<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn commit_edits(&mut self) {
        self.ldg.commit_edits();
    }

    // fn get_uncommitted_edits(&self) -> Option<std::collections::HashMap<usize, L>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        self.ldg.discard_edits();
    }

    fn vertex_exists_updated(&self, vertex: L) -> bool {
        return self.ldg.vertex_exists_updated(vertex);
    }

    fn edge_exists_updated(&self, from: L, to: L) -> bool {
        // fixme
        // does check twice if from and to is valid
        let from_index = self.index(&from);
        let to_index = self.index(&to);
        if from_index.is_none() {
            panic!("wtlug edge_exists : from Vertex doesn't exist")
        }
        if to_index.is_none() {
            panic!("wtlug edge_exists : to Vertex doesn't exist")
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        if from_index <= to_index {
            return self.ldg.edge_exists_updated(from, to);
        } else {
            return self.ldg.edge_exists_updated(to, from);
        }
    }

    fn v_count_updated(&self) -> usize {
        return self.ldg.v_count_updated();
    }

    fn e_count_updated(&self) -> usize {
        return self.ldg.e_count_updated();
    }
}
impl<L> WTUndirected<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edges_updated(&self, vertex: L) -> Vec<L> {
        let vertex_index = self.index(&vertex);
        if vertex_index.is_none() {
            panic!("wtlug edges : Vertex doesn't exist");
        }

        let mut edges: Vec<L>;
        edges = self.ldg.incoming_edges_updated(vertex.clone()); // all incoming edges of vertex
        if self.edge_exists_updated(vertex.clone(), vertex.clone()) {
            for item in self.ldg.outgoing_edges_updated(vertex.clone()) {
                if item != vertex {
                    edges.push(item);
                }
            }
            return edges;
        } else {
            edges.append(&mut self.ldg.outgoing_edges_updated(vertex)); // + outgoing edges of vertex
            return edges;
        }
    }
}
impl<L> WTLabeled<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn label_updated(&self, index: usize) -> Option<&L> {
        return self.ldg.label_updated(index);
    }

    fn index_updated(&self, label: &L) -> Option<usize> {
        return self.ldg.index_updated(label);
    }
}
