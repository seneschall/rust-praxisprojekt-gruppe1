use crate::graph::directed::Digraph;
use crate::traits::{Directed, Graph, Unlabeled, Unweighted, Weighted};
use num::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod test;

/// An indexed, mutable graph with directed edges, where each edge represents a weight.
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type.
/// Weights can have any type.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub struct WeightedDigraph<W> {
    pub(crate) dg: Digraph,
    pub(crate) weights: HashMap<(usize, usize), W>,
}

impl<W> WeightedDigraph<W>
where
    W: Clone + Num,
{
    pub fn new() -> Self {
        WeightedDigraph {
            dg: Digraph::new(),
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
        WeightedDigraph {
            dg: Digraph::from_adjacency_list(v_count, e_count, adj_list),
            weights: hashmap_weights,
        }
    }
}

impl<W> Graph<usize> for WeightedDigraph<W> {
    fn add_vertex(&mut self, vertex: usize) -> usize {
        self.dg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.dg.e_count
    }

    fn v_count(&self) -> usize {
        self.dg.adj_len
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        let i_of_w: usize; // -- note from celine: could we use index_of_w for clarity?
        match self.dg.adj.get(from) {
            Some(vs) => {
                let i_of_w_opt = vs.iter().position(|&x| x == to);
                match i_of_w_opt {
                    Some(i) => {
                        i_of_w = i;
                    } // swap_remove more efficient than remove because the order is not important
                    None => {
                        panic!("There was no edge from {from} to {to}.");
                    }
                }
            }
            None => {
                panic!("Vertex {from} doesn't exist."); // Should be replaced by Result type
            }
        }
        self.dg.adj[from].swap_remove(i_of_w);
        self.weights.remove(&(from, i_of_w));
        self.dg.e_count -= 1;
    }

    fn delete_vertex(&mut self, vertex: usize) {
        if vertex < self.dg.adj_len {
            self.delete_incoming_edges(vertex);
            self.delete_outgoing_edges(vertex);
            self.dg.delete_vertex(vertex);
            self.dg.adj_len -= 1;
        } else {
            panic!("wdg delete_vertex : Vertex doesn't exist")
        }
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        self.dg.vertex_exists(vertex)
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        self.dg.edge_exists(from, to)
    }
}
impl<W> Directed<usize> for WeightedDigraph<W> {
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        self.dg.outgoing_edges(vertex)
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        self.dg.incoming_edges(vertex)
    }

    fn delete_outgoing_edges(&mut self, vertex: usize) {
        for to in self.outgoing_edges(vertex) {
            self.delete_edge(vertex, to)
        }
    }

    fn delete_incoming_edges(&mut self, vertex: usize) {
        for from in self.incoming_edges(vertex) {
            self.delete_edge(from, vertex);
        }
    }
}
impl<W> Unlabeled<usize> for WeightedDigraph<W> {
    fn append_vertex(&mut self) -> usize {
        self.dg.append_vertex()
    }

    fn shrink(&mut self) -> Vec<Option<usize>> {
        todo!()
    }
}
impl<W> Weighted<usize, W> for WeightedDigraph<W>
where
    W: Copy,
{
    fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        self.dg.add_edge(from, to);
        self.weights.insert((from, to), weight);
    }

    fn edit_weight(&mut self, from: usize, to: usize, weight: W) {
        self.weights.insert((from, to), weight);
    }

    fn get_weight(&mut self, from: usize, to: usize) -> W {
        self.weights.get(&(from, to)).unwrap().to_owned()
    }
}
