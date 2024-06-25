use crate::graph::labeled_directed::LabeledDigraph;
use crate::traits::{Directed, Graph, Labeled, Unweighted, Weighted};
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;
// LabeledWeightedDigraph
pub struct LabeledWeightedDigraph<L, W>
where
    L: Hash + Eq,
{
    dg: LabeledDigraph<L>,
    weights: HashMap<(usize, usize), W>,
}

impl<L, W> LabeledWeightedDigraph<L, W>
where
    L: Hash + Eq + Clone,
    W: Clone,
{
    pub fn new() -> Self {
        LabeledWeightedDigraph {
            dg: LabeledDigraph::new(),
            weights: HashMap::new(),
        }
    }
    pub fn from_adjacency_list(
        v_count: usize,
        e_count: usize,
        adj: Vec<Vec<(usize, W)>>,
        labels: Vec<L>,
    ) -> Self {
        let mut hashmap_weights: HashMap<(usize, usize), W> = HashMap::new();
        if !(v_count == adj.len()) {
            panic!("v_count != adj.len()")
        }
        let mut j = 0;
        let mut adjlist: Vec<Vec<usize>> = vec![vec![]; v_count];
        for item in adj {
            for i in 0..item.len() {
                let (to, weight): (usize, W) = item[i].clone();
                hashmap_weights.insert((j, to), weight);
                adjlist[j].push(to);
            }
            j += 1;
        }
        LabeledWeightedDigraph {
            dg: LabeledDigraph::from_adjacency_list(v_count, e_count, adjlist, labels),
            weights: hashmap_weights,
        }
    }
}

impl<L, W> Graph<L> for LabeledWeightedDigraph<L, W>
where
    L: Eq + Hash + Clone + std::fmt::Display,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        self.dg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.dg.e_count()
    }

    fn v_count(&self) -> usize {
        self.dg.v_count()
    }

    fn vertex_deleted(&self, vertex: L) -> bool {
        self.dg.vertex_deleted(vertex)
    }

    fn delete_edge(&mut self, from: L, to: L) {
        let i_of_w: usize; // -- note from celine: could we use index_of_w for clarity?
        match self
            .dg
            .dg
            .adj
            .get(self.dg.get_index(from.clone()).unwrap().clone())
        {
            Some(vs) => {
                let i_of_w_opt = vs
                    .iter()
                    .position(|&x| x == self.dg.get_index(to.clone()).unwrap().to_owned());
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
        let from_index = self.dg.get_index(from.clone()).unwrap().clone();
        self.dg.dg.adj[from_index].swap_remove(i_of_w); // deletes adj entry
        self.weights.remove(&(
            self.dg.get_index(from).unwrap().clone(),
            self.dg.get_index(to).unwrap().clone(),
        )); // deletes HashMap entry of weight
        self.dg.dg.e_count -= 1;
    }

    fn delete_vertex(&mut self, vertex: L) {
        if self.dg.get_index(vertex.clone()).unwrap() < &self.dg.v_count() {
            self.delete_incoming_edges(vertex.clone());
            self.delete_outgoing_edges(vertex.clone());
            self.dg.dg.v_count -= 1;
            self.dg
                .dg
                .deleted_vertices
                .push(self.dg.get_index(vertex.clone()).unwrap().to_owned());
        } else {
            panic!("delete_vertex : Can't delete Vertex : vertex >= self.v_count")
        }
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        self.dg.vertex_exists(vertex)
    }

    fn shrink(&mut self) -> HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L, W> Directed<L> for LabeledWeightedDigraph<L, W>
where
    L: Eq + Hash + Clone + std::fmt::Display,
{
    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        self.dg.outgoing_edges(vertex)
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        self.dg.incoming_edges(vertex)
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        for to in self.outgoing_edges(vertex.clone()) {
            self.delete_edge(vertex.clone(), to);
        }
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        for from in self.incoming_edges(vertex.clone()) {
            self.delete_edge(from, vertex.clone());
        }
    }
}
impl<L, W> Labeled<L> for LabeledWeightedDigraph<L, W>
where
    L: Eq + Hash + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.dg.edit_label(old_label, new_label);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        self.dg.get_label(vertex)
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        self.dg.get_index(label)
    }
}
impl<L, W> Weighted<L, W> for LabeledWeightedDigraph<L, W>
where
    L: Eq + Hash + Clone,
    W: Clone,
{
    fn add_edge(&mut self, from: L, to: L, weight: W) {
        self.dg.add_edge(from.clone(), to.clone());
        self.weights.insert(
            (
                self.dg.get_index(from).unwrap().to_owned(),
                self.dg.get_index(to).unwrap().to_owned(),
            ),
            weight,
        );
    }

    fn edit_weight(&mut self, from: L, to: L, weight: W) {
        self.weights.insert(
            (
                self.dg.get_index(from).unwrap().to_owned(),
                self.dg.get_index(to).unwrap().to_owned(),
            ),
            weight,
        );
    }

    fn get_weight(&mut self, from: L, to: L) -> W {
        self.weights
            .get(&(
                self.dg.get_index(from).unwrap().to_owned(),
                self.dg.get_index(to).unwrap().to_owned(),
            ))
            .unwrap()
            .clone()
    }
}
