use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use vers_vecs::RsVec;

use crate::graph::weighted_undirected::WeightedUGraph;
use crate::traits::{
    Directed, Graph, Undirected, Unlabeled, WTDirected, WTUndirected, WTWeighted, Weighted, WT,
};

use super::weighted_directed::WeightedWTDigraph;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub struct WeightedWTUGraph<W> {
    wdg: WeightedWTDigraph<W>,
}
impl<W> WeightedWTUGraph<W> {
    pub fn from_weighted_ugraph(wug: WeightedUGraph<W>) -> Self {
        return WeightedWTUGraph {
            wdg: WeightedWTDigraph::from_weighted_digraph(wug.wdg),
        };
    }
    pub fn from(
        sequence: Vec<usize>,
        starting_indices: RsVec,
        weights: HashMap<(usize, usize), W>,
    ) -> Self {
        return WeightedWTUGraph {
            wdg: WeightedWTDigraph::from(sequence, starting_indices, weights),
        };
    }
}
impl<W> Graph<usize> for WeightedWTUGraph<W>
where
    W: Clone,
{
    fn add_vertex(&mut self, vertex: usize) -> usize {
        return self.wdg.add_vertex(vertex);
    }

    fn e_count(&self) -> usize {
        return self.wdg.e_count();
    }

    fn v_count(&self) -> usize {
        return self.wdg.v_count();
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.wdg.delete_edge(from, to);
        } else {
            self.wdg.delete_edge(to, from);
        }
    }

    fn delete_vertex(&mut self, vertex: usize) {
        self.wdg.delete_vertex(vertex);
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        return self.wdg.vertex_exists(vertex);
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if from <= to {
            return self.wdg.edge_exists(from, to);
        } else {
            return self.wdg.edge_exists(to, from);
        }
    }
}
impl<W> Undirected<usize> for WeightedWTUGraph<W>
where
    W: Clone,
{
    fn edges(&self, vertex: usize) -> Vec<usize> {
        // returns all edges connected to vertex
        let mut edges: Vec<usize>;
        edges = self.wdg.incoming_edges(vertex); // all incoming edges of vertex
        if self.edge_exists(vertex, vertex) {
            for item in self.wdg.outgoing_edges(vertex) {
                if item != vertex {
                    edges.push(item);
                }
            }
            return edges;
        } else {
            edges.append(&mut self.wdg.outgoing_edges(vertex)); // + outgoing edges of vertex
            return edges;
        }
    }

    fn delete_edges_from(&mut self, vertex: usize) {
        // deletes all edges connected to vertex
        for item in self.edges_updated(vertex) {
            self.delete_edge(vertex, item);
        }
    }
}
impl<W> Unlabeled<usize> for WeightedWTUGraph<W> {
    fn append_vertex(&mut self) -> usize {
        return self.wdg.append_vertex();
    }

    fn shrink(&mut self) -> Vec<Option<usize>> {
        return self.wdg.shrink();
    }
}
impl<W> Weighted<usize, W> for WeightedWTUGraph<W>
where
    W: Clone,
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
impl<W> WT<usize> for WeightedWTUGraph<W>
where
    W: Clone,
{
    fn commit_edits(&mut self) {
        self.wdg.commit_edits();
    }

    // fn get_uncommitted_edits(&self) -> Option<std::collections::HashMap<usize, usize>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        self.wdg.commit_edits();
    }

    fn vertex_exists_updated(&self, vertex: usize) -> bool {
        return self.wdg.vertex_exists_updated(vertex);
    }

    fn edge_exists_updated(&self, from: usize, to: usize) -> bool {
        if from <= to {
            return self.wdg.edge_exists_updated(from, to);
        } else {
            return self.wdg.edge_exists_updated(to, from);
        }
    }

    fn v_count_updated(&self) -> usize {
        return self.wdg.v_count_updated();
    }

    fn e_count_updated(&self) -> usize {
        return self.wdg.e_count_updated();
    }
}
impl<W> WTUndirected<usize> for WeightedWTUGraph<W>
where
    W: Clone,
{
    fn edges_updated(&self, vertex: usize) -> Vec<usize> {
        let mut edges: Vec<usize>;
        edges = self.wdg.incoming_edges_updated(vertex); // all incoming edges of vertex
        if self.edge_exists_updated(vertex, vertex) {
            for item in self.wdg.outgoing_edges_updated(vertex) {
                if item != vertex {
                    edges.push(item);
                }
            }
            return edges;
        } else {
            edges.append(&mut self.wdg.outgoing_edges_updated(vertex)); // + outgoing edges of vertex
            return edges;
        }
    }
}
impl<W> WTWeighted<usize, W> for WeightedWTUGraph<W>
where
    W: Clone,
{
    fn get_weight_updated(&mut self, from: usize, to: usize) -> W {
        if from <= to {
            return self.wdg.get_weight_updated(from, to);
        } else {
            return self.wdg.get_weight_updated(to, from);
        }
    }
}
