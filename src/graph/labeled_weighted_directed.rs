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
    pub(crate) ldg: LabeledDigraph<L>,
    pub(crate) weights: HashMap<(usize, usize), W>,
}

impl<L, W> LabeledWeightedDigraph<L, W>
where
    L: Hash + Eq + Clone,
    W: Clone,
{
    pub fn new() -> Self {
        LabeledWeightedDigraph {
            ldg: LabeledDigraph::new(),
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
            ldg: LabeledDigraph::from_adjacency_list(v_count, e_count, adjlist, labels),
            weights: hashmap_weights,
        }
    }
}

impl<L, W> Graph<L> for LabeledWeightedDigraph<L, W>
where
    L: Eq + Hash + Clone,
{
    fn add_vertex(&mut self, vertex: L) -> usize {
        self.ldg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.ldg.e_count()
    }

    fn v_count(&self) -> usize {
        self.ldg.v_count()
    }

    fn delete_edge(&mut self, from: L, to: L) {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwdg add_edge : from is none");
        }
        if to_index.is_none() {
            panic!("lwdg add_edge : tois none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        self.ldg.dg.delete_edge(from_index, to_index);
        self.weights.remove(&(from_index, to_index));
    }

    fn delete_vertex(&mut self, vertex: L) {
        let vertex_index = self.get_index(&vertex);
        if vertex_index.is_none() {
            panic!("lwdg delete_vertex : vertex is none");
        }
        if !self.vertex_exists(vertex.clone()) {
            panic!("lwdg delete_vertex : vertex does not exist");
        }
        self.delete_incoming_edges(vertex.clone());
        self.delete_outgoing_edges(vertex.clone());
        self.ldg.delete_vertex(vertex);
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        self.ldg.vertex_exists(vertex)
    }


    fn edge_exists(&self, from: L, to: L) -> bool {
        self.ldg.edge_exists(from, to)
    }
}
impl<L, W> Directed<L> for LabeledWeightedDigraph<L, W>
where
    L: Eq + Hash + Clone,
{
    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        self.ldg.outgoing_edges(vertex)
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        self.ldg.incoming_edges(vertex)
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        // // next lines might not be needed
        // let vertex_index = self.get_index(vertex);
        // if vertex_index.is_none(){
        //     panic!("lwdg delete_incoming_edges : vertex is none");
        // }
        // let vertex_index = vertex_index.unwrap();
        for to in self.outgoing_edges(vertex.clone()) {
            self.delete_edge(vertex.clone(), to);
        }
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        // // next lines might not be needed
        // let vertex_index = self.get_index(vertex);
        // if vertex_index.is_none(){
        //     panic!("lwdg delete_incoming_edges : vertex is none");
        // }
        // let vertex_index = vertex_index.unwrap();

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
        self.ldg.edit_label(old_label, new_label);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        self.ldg.get_label(vertex)
    }

    fn get_index(&self, label: &L) -> Option<usize> {
        self.ldg.get_index(label)
    }
    
    fn shrink(&mut self) -> HashMap<L, Option<L>> {
        todo!()
    }
}
impl<L, W> Weighted<L, W> for LabeledWeightedDigraph<L, W>
where
    L: Eq + Hash + Clone,
    W: Clone,
{
    fn add_edge(&mut self, from: L, to: L, weight: W) {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwdg add_edge : from is none");
        }
        if to_index.is_none() {
            panic!("lwdg add_edge : tois none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        self.ldg.add_edge(from, to);
        self.weights.insert((from_index, to_index), weight);
    }

    fn edit_weight(&mut self, from: L, to: L, weight: W) {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwdg add_edge : from is none");
        }
        if to_index.is_none() {
            panic!("lwdg add_edge : tois none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();
        if !self.edge_exists(from, to) {
            panic!("edge doesn't exist");
        }
        self.weights.insert((from_index, to_index), weight);
    }

    fn get_weight(&mut self, from: L, to: L) -> W {
        let from_index = self.get_index(&from);
        let to_index = self.get_index(&to);
        if from_index.is_none() {
            panic!("lwdg add_edge : from is none");
        }
        if to_index.is_none() {
            panic!("lwdg add_edge : to is none");
        }
        let from_index = from_index.unwrap();
        let to_index = to_index.unwrap();

        return self.weights.get(&(from_index, to_index)).unwrap().clone();
    }
}
