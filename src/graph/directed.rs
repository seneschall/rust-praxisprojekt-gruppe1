use crate::graph::Graph;
use core::hash;
use std::collections::HashMap;
use std::hash::Hash;

use super::{Delete, Directed, Labeled, Weighted};

// UNIT-TESTS for Digraph and Weighed Digraph
#[cfg(test)]
mod digraph {
    use super::*;
    #[test]
    fn new() {
        let digraph = Digraph::new();
        assert!(digraph.adj.is_empty());
        assert_eq!(digraph.v_count, 0);
        assert_eq!(digraph.e_count, 0);
        assert!(digraph.deleted_vertices.is_empty());
    }
    #[test]
    fn from_adjacency_list() {
        let v_count = 10;
        let e_count = 5;
        let adj = vec![vec![0]; 10];
        let digraph = Digraph::from_adjacency_list(v_count, e_count, adj.clone());
        assert_eq!(digraph.v_count(), v_count);
        assert_eq!(digraph.e_count(), e_count);
        assert_eq!(digraph.adj, adj);
        assert!(digraph.deleted_vertices.is_empty());
    }
    #[test]
    fn append_vertex() {
        let mut digraph = Digraph::new();
        for i in 0..9 {
            assert_eq!(i, digraph.append_vertex())
        }
    }
    #[test]
    fn add_edge() {
        let mut digraph = Digraph::from_adjacency_list(5, 0, vec![vec![]; 5]);
        for i in 0..digraph.v_count() {
            digraph.add_edge(i, 0);
        }
        assert_eq!(digraph.adj, vec![vec![0]; 5]);
        assert_eq!(digraph.e_count(), 5);
    }
    #[test]
    fn add_vertex() {
        let mut digraph = Digraph::new();
        for i in 0..9 {
            assert_eq!(digraph.add_vertex(i), i);
        }
        assert_eq!(digraph.add_vertex(500), 500);
    }
    #[test]
    fn vertex_exists() {
        let mut digraph = Digraph::new();
        digraph.v_count = 2;
        digraph.adj = vec![vec![]; 2];
        digraph.deleted_vertices = vec![0];
        assert_eq!(digraph.vertex_exists(0), false);
        assert_eq!(digraph.vertex_exists(1), true);
        digraph.deleted_vertices = vec![];
        assert_eq!(digraph.vertex_exists(0), true);
        digraph.v_count = 0;
        assert_eq!(digraph.vertex_exists(1), false);
        digraph = Digraph::new();
    }
    #[test]
    fn delete_vertex() {
        let mut digraph = Digraph::from_adjacency_list(5, 0, vec![vec![]; 5]);
        digraph.delete_vertex(0);
        assert_eq!(digraph.deleted_vertices, vec![0]);
        assert_eq!(digraph.v_count(), 4);
    }
    #[test]
    fn vertex_deleted() {
        let mut digraph = Digraph::from_adjacency_list(5, 0, vec![vec![]; 5]);
        assert_eq!(digraph.vertex_deleted(0), false);
        digraph.deleted_vertices = vec![0];
        assert_eq!(digraph.vertex_deleted(0), true);
        assert_eq!(digraph.vertex_deleted(1), false);
        digraph.deleted_vertices = vec![0, 1];
        assert_eq!(digraph.vertex_deleted(1), true);
    }
    #[test]
    fn delete_edge() {
        let mut digraph = Digraph::from_adjacency_list(5, 5, vec![vec![1]; 5]);
        assert_eq!(
            digraph.adj,
            vec![vec![1], vec![1], vec![1], vec![1], vec![1]]
        );
        digraph.delete_edge(0, 1);
        assert_eq!(
            digraph.adj,
            vec![vec![], vec![1], vec![1], vec![1], vec![1]]
        );
        assert_eq!(digraph.e_count, 4);
    }
    #[test]
    fn outgoing_edges() {
        let mut digraph = Digraph::from_adjacency_list(5, 5, vec![vec![1]; 5]);
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.outgoing_edges(i), vec![1]);
        }
        digraph.e_count = 25;
        digraph.adj = vec![vec![0, 1, 2, 3, 4]; 5];
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.outgoing_edges(i), vec![0, 1, 2, 3, 4]);
        }
    }
    #[test]
    fn incoming_edges() {
        let mut digraph = Digraph::from_adjacency_list(5, 25, vec![vec![0, 1, 2, 3, 4]; 5]);
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.incoming_edges(i), vec![0, 1, 2, 3, 4]);
        }
        digraph.adj = vec![vec![1]; 5];
        for i in 0..digraph.v_count() {
            if i == 1 {
                assert_eq!(digraph.incoming_edges(i), vec![0, 1, 2, 3, 4]);
            } else {
                assert_eq!(digraph.incoming_edges(i), vec![]);
            }
        }
    }
    #[test]
    fn delete_outgoing_edges() {
        let mut digraph = Digraph::from_adjacency_list(5, 5, vec![vec![1]; 5]);
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.adj[i], vec![1]);
        }
        for i in 0..digraph.v_count() {
            digraph.delete_outgoing_edges(i);
            assert_eq!(digraph.adj[i], vec![]);
        }
        assert_eq!(digraph.e_count(), 0);
        digraph.e_count = 25;
        digraph.adj = vec![vec![0, 1, 2, 3, 4]; 5];
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.adj[i], vec![0, 1, 2, 3, 4]);
        }
        for i in 0..digraph.v_count() {
            digraph.delete_outgoing_edges(i);
            assert_eq!(digraph.adj[i], vec![]);
        }
        assert_eq!(digraph.e_count(), 0);
    }
    #[test]
    fn delete_incoming_edges() {
        let mut digraph = Digraph::from_adjacency_list(5, 5, vec![vec![1]; 5]);
        for i in 0..digraph.v_count() {
            digraph.delete_incoming_edges(i);
        }
        assert_eq!(digraph.e_count(), 0);
        assert_eq!(digraph.adj, vec![vec![]; 5]);
    }
}

// Digraph - definition and methods
pub struct Digraph {
    deleted_vertices: Vec<usize>,
    v_count: usize,                  // number of vertices
    e_count: usize,                  // number of edges
    pub(crate) adj: Vec<Vec<usize>>, // adjacency list of indices -- note from group: should we set this to pub(crate)?
}
impl Digraph {
    pub fn new() -> Self {
        Digraph {
            deleted_vertices: Vec::new(),
            v_count: 0,
            e_count: 0,
            adj: vec![vec![]; 0],
        }
    }
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<usize>>) -> Self {
        // doesn't check valid input
        if v_count == adj.len() {
            Digraph {
                deleted_vertices: Vec::new(),
                v_count,
                e_count,
                adj,
            }
        } else {
            panic!("Digraph: from_adjacency_list v_count != adj.len()");
        }
    }
    pub fn append_vertex(&mut self) -> usize {
        self.adj.push(vec![]);
        self.v_count += 1;
        self.v_count - 1
    }
}
impl Graph<usize> for Digraph {
    fn add_edge(&mut self, from: usize, to: usize) {
        if !(self.vertex_exists(from) && self.vertex_exists(to)) {
            panic!("One of vertices {}, {} doesn't exist", from, to)
        }
        self.e_count += 1;
        self.adj[from].push(to);
    }

    fn add_vertex(&mut self, vertex: usize) -> usize {
        if vertex >= self.v_count {
            for i in 0..vertex - self.v_count + 1 {
                self.adj.insert(self.v_count + i, vec![]);
            }
            self.v_count += vertex - self.v_count + 1;
        } else {
            self.adj.insert(vertex, vec![]);
            self.v_count += 1;
        }
        self.v_count - 1
    }

    fn e_count(&self) -> usize {
        self.e_count
    }

    fn v_count(&self) -> usize {
        self.v_count
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        (!self.deleted_vertices.contains(&vertex))
            && vertex < self.v_count + self.deleted_vertices.len()
    }
}
impl Delete<usize> for Digraph {
    fn delete_vertex(&mut self, vertex: usize) {
        if vertex < self.v_count {
            self.deleted_vertices.push(vertex);
            self.delete_incoming_edges(vertex);
            self.delete_outgoing_edges(vertex);
            self.v_count -= 1;
        } else {
            panic!("delete_vertex : Can't delete Vertex : vertex >= self.v_count")
        }
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        if self.deleted_vertices.contains(&vertex) {
            true
        } else {
            false
        }
    }
}

impl Directed<usize> for Digraph {
    fn delete_edge(&mut self, from: usize, to: usize) {
        let i_of_w: usize; // -- note from celine: could we use index_of_w for clarity?
        match self.adj.get(from) {
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
        self.adj[from].swap_remove(i_of_w);
        self.e_count -= 1;
    }

    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        self.adj[vertex].clone()
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        let mut incoming_edges: Vec<usize> = Vec::new();
        for i in 0..self.v_count {
            if self.adj[i].contains(&vertex) {
                incoming_edges.push(i);
            }
        }
        incoming_edges
    }

    fn delete_outgoing_edges(&mut self, vertex: usize) {
        for to in self.outgoing_edges(vertex) {
            self.delete_edge(vertex, to)
        }
    }

    fn delete_incoming_edges(&mut self, vertex: usize) {
        for from in self.incoming_edges(vertex) {
            self.delete_edge(from, vertex)
        }
    }
}

#[cfg(test)]
mod labeleddigraph {
    use super::*;
    #[test]
    fn new() {
        let ldg: LabeledDigraph<String> = LabeledDigraph::new();
        assert!(ldg.dg.adj.is_empty());
        assert_eq!(ldg.dg.v_count, 0);
        assert_eq!(ldg.dg.e_count, 0);
        assert!(ldg.dg.deleted_vertices.is_empty());
        assert!(ldg.hashmap_labels_vertex.is_empty());
        assert!(ldg.vec_vertex_labels.is_empty());
    }
    #[test]
    fn from_adjacency_list() {
        let v_count = 10;
        let e_count = 10;
        let adj = vec![vec![0]; 10];
        let mut testHashMap: HashMap<String, usize> = HashMap::new();
        let mut labels: Vec<String> = Vec::new();
        for i in 0..10 {
            labels.push(i.to_string());
            testHashMap.insert(i.to_string(), i);
        }
        let ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(v_count, e_count, adj.clone(), labels.clone());
        assert_eq!(ldg.v_count(), v_count);
        assert_eq!(ldg.e_count(), e_count);
        assert_eq!(ldg.dg.adj, adj);
        assert!(ldg.dg.deleted_vertices.is_empty());
        assert_eq!(ldg.vec_vertex_labels, labels);
        assert_eq!(ldg.hashmap_labels_vertex, testHashMap);
    }
    #[test]
    fn add_vertex() {
        let mut testHashMap: HashMap<String, usize> = HashMap::new();
        let mut ldg: LabeledDigraph<String> = LabeledDigraph::new();
        for i in 0..9 {
            assert_eq!(i, ldg.add_vertex(i.to_string()));
            assert_eq!(ldg.vec_vertex_labels[i], i.to_string());
            assert_eq!(testHashMap.insert(i.to_string(), i), None);
        }
        assert_eq!(ldg.hashmap_labels_vertex, testHashMap);
    }
    #[test]
    fn add_edge() {
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(5, 0, vec![vec![]; 5], labels.clone());
        for i in 0..ldg.v_count() {
            ldg.add_edge(i.to_string(), 0.to_string());
        }
        assert_eq!(ldg.dg.adj, vec![vec![0]; 5]);
        assert_eq!(ldg.e_count(), 5);
    }
    #[test]
    fn vertex_exists() {
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(5, 0, vec![vec![]; 5], labels.clone());
        ldg.dg.deleted_vertices = vec![0];
        assert_eq!(ldg.vertex_exists(0.to_string()), false);
        assert_eq!(ldg.vertex_exists(1.to_string()), true);
        ldg.dg.deleted_vertices = vec![];
        assert_eq!(ldg.vertex_exists(0.to_string()), true);
        ldg.dg.v_count = 0;
        assert_eq!(ldg.vertex_exists(1.to_string()), false);
    }
    #[test]
    fn delete_vertex() {
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(5, 0, vec![vec![]; 5], labels.clone());

        ldg.delete_vertex(0.to_string());
        assert_eq!(ldg.dg.deleted_vertices, vec![0]);
        assert_eq!(ldg.v_count(), 4);
    }
    #[test]
    fn vertex_deleted() {
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(5, 0, vec![vec![]; 5], labels.clone());

        assert_eq!(ldg.vertex_deleted(0.to_string()), false);
        ldg.dg.deleted_vertices = vec![0];
        assert_eq!(ldg.vertex_deleted(0.to_string()), true);
        assert_eq!(ldg.vertex_deleted(1.to_string()), false);
        ldg.dg.deleted_vertices = vec![0, 1];
        assert_eq!(ldg.vertex_deleted(1.to_string()), true);
    }
    #[test]
    fn delete_edge() {
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(5, 5, vec![vec![1]; 5], labels.clone());

        assert_eq!(
            ldg.dg.adj,
            vec![vec![1], vec![1], vec![1], vec![1], vec![1]]
        );
        ldg.delete_edge(0.to_string(), 1.to_string());
        assert_eq!(ldg.dg.adj, vec![vec![], vec![1], vec![1], vec![1], vec![1]]);
        assert_eq!(ldg.e_count(), 4);
    }
    #[test]
    fn outgoing_edges() {
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(5, 5, vec![vec![1]; 5], labels.clone());

        for i in 0..ldg.v_count() {
            assert_eq!(ldg.outgoing_edges(i.to_string()), vec![1.to_string()]);
        }
        ldg.dg.e_count = 25;
        ldg.dg.adj = vec![vec![0, 1, 2, 3, 4]; 5];
        for i in 0..ldg.v_count() {
            assert_eq!(
                ldg.outgoing_edges(i.to_string()),
                vec![
                    0.to_string(),
                    1.to_string(),
                    2.to_string(),
                    3.to_string(),
                    4.to_string()
                ]
            );
        }
    }
    #[test]
    fn incoming_edges() {
        let test: Vec<String> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> = LabeledDigraph::from_adjacency_list(
            5,
            25,
            vec![vec![0, 1, 2, 3, 4]; 5],
            labels.clone(),
        );

        for i in 0..ldg.v_count() {
            assert_eq!(
                ldg.incoming_edges(i.to_string()),
                vec![
                    0.to_string(),
                    1.to_string(),
                    2.to_string(),
                    3.to_string(),
                    4.to_string()
                ]
            );
        }
        ldg.dg.adj = vec![vec![1]; 5];
        for i in 0..ldg.v_count() {
            if i == 1 {
                assert_eq!(
                    ldg.incoming_edges(i.to_string()),
                    vec![
                        0.to_string(),
                        1.to_string(),
                        2.to_string(),
                        3.to_string(),
                        4.to_string()
                    ]
                );
            } else {
                assert_eq!(ldg.incoming_edges(i.to_string()), test);
            }
        }
    }
    #[test]
    fn delete_outgoing_edges() {
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(5, 5, vec![vec![1]; 5], labels.clone());

        for i in 0..ldg.v_count() {
            assert_eq!(ldg.dg.adj[i], vec![1]);
        }
        for i in 0..ldg.v_count() {
            ldg.delete_outgoing_edges(i.to_string());
            assert_eq!(ldg.dg.adj[i], vec![]);
        }
        assert_eq!(ldg.e_count(), 0);
        ldg.dg.e_count = 25;
        ldg.dg.adj = vec![vec![0, 1, 2, 3, 4]; 5];
        for i in 0..ldg.v_count() {
            assert_eq!(ldg.dg.adj[i], vec![0, 1, 2, 3, 4]);
        }
        for i in 0..ldg.v_count() {
            ldg.delete_outgoing_edges(i.to_string());
            assert_eq!(ldg.dg.adj[i], vec![]);
        }
        assert_eq!(ldg.e_count(), 0);
    }
    #[test]
    fn delete_incoming_edges() {
        let mut labels: Vec<String> = Vec::new();
        for i in 0..5 {
            labels.push(i.to_string());
        }
        let mut ldg: LabeledDigraph<String> =
            LabeledDigraph::from_adjacency_list(5, 5, vec![vec![1]; 5], labels.clone());

        for i in 0..ldg.v_count() {
            ldg.delete_incoming_edges(i.to_string());
        }
        assert_eq!(ldg.e_count(), 0);
        assert_eq!(ldg.dg.adj, vec![vec![]; 5]);
    }
}

pub struct LabeledDigraph<L>
where
    L: Eq + Hash,
{
    dg: Digraph,
    vec_vertex_labels: Vec<L>,
    hashmap_labels_vertex: HashMap<L, usize>,
}
impl<L> LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        LabeledDigraph {
            dg: Digraph::new(),
            vec_vertex_labels: Vec::new(),
            hashmap_labels_vertex: HashMap::new(),
        }
    }
    pub fn from_adjacency_list(
        v_count: usize,
        e_count: usize,
        adj: Vec<Vec<usize>>,
        labels: Vec<L>,
    ) -> Self {
        let mut vec_vertex_labels: Vec<L> = Vec::new();
        let mut hashmap_labels_vertex: HashMap<L, usize> = HashMap::new();
        if !(labels.len() == v_count) {
            panic!("Failed : v_count and labels.len() are not equal")
        } else {
            let mut i: usize = 0;
            for item in labels {
                vec_vertex_labels.push(item.clone()); // create Vec for Labels
                hashmap_labels_vertex.insert(item, i); // create HashMap for Labels
                i += 1;
            }
        }
        LabeledDigraph {
            dg: Digraph::from_adjacency_list(v_count, e_count, adj),
            vec_vertex_labels: vec_vertex_labels,
            hashmap_labels_vertex: hashmap_labels_vertex,
        }
    }
}
impl<L> Graph<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Clone,
{
    fn add_edge(&mut self, from: L, to: L) {
        self.dg.add_edge(
            self.get_index(from).unwrap().to_owned(),
            self.get_index(to).unwrap().to_owned(),
        );
    }

    fn add_vertex(&mut self, vertex: L) -> usize {
        let index = self.dg.append_vertex();
        self.vec_vertex_labels.push(vertex.clone());
        self.hashmap_labels_vertex.insert(vertex, index);
        index
    }

    fn e_count(&self) -> usize {
        self.dg.e_count
    }

    fn v_count(&self) -> usize {
        self.dg.v_count
    }
    fn vertex_exists(&self, vertex: L) -> bool {
        self.dg
            .vertex_exists(self.get_index(vertex).unwrap().to_owned())
    }
}
impl<L> Delete<L> for LabeledDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn delete_vertex(&mut self, vertex: L) {
        self.dg
            .delete_vertex(self.get_index(vertex.clone()).unwrap().to_owned());
        // let index = self.hashmap_labels_vertex.remove(&vertex).unwrap();
        //TODO delete labels
    }

    fn vertex_deleted(&self, vertex: L) -> bool {
        self.dg
            .vertex_deleted(self.get_index(vertex).unwrap().to_owned())
    }
}

impl<L> Directed<L> for LabeledDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn delete_edge(&mut self, from: L, to: L) {
        self.dg.delete_edge(
            self.get_index(from).unwrap().to_owned(),
            self.get_index(to).unwrap().to_owned(),
        )
    }

    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        let mut outgoing_edges: Vec<L> = Vec::new();
        for item in self
            .dg
            .outgoing_edges(self.get_index(vertex).unwrap().to_owned())
        {
            outgoing_edges.push(self.get_label(item).unwrap().to_owned());
        }
        outgoing_edges
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        let mut incoming_edges: Vec<L> = Vec::new();
        for item in self
            .dg
            .incoming_edges(self.get_index(vertex).unwrap().to_owned())
        {
            incoming_edges.push(self.get_label(item).unwrap().to_owned());
        }
        incoming_edges
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        self.dg
            .delete_outgoing_edges(self.get_index(vertex).unwrap().to_owned())
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        self.dg
            .delete_incoming_edges(self.get_index(vertex).unwrap().to_owned())
    }
}

impl<L> Labeled<L> for LabeledDigraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.vec_vertex_labels[self
            .hashmap_labels_vertex
            .get(&old_label)
            .unwrap()
            .to_owned()] = new_label.clone(); // update Vec

        let value = self
            .hashmap_labels_vertex
            .remove(&old_label)
            .unwrap()
            .to_owned(); // update HashMap
        self.hashmap_labels_vertex.insert(new_label, value);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        self.vec_vertex_labels.get(vertex)
    }

    fn get_index(&self, label: L) -> Option<&usize> {
        self.hashmap_labels_vertex.get(&label)
    }
}
#[cfg(test)]
mod weighteddigraph {
    use super::*;

    fn new() {
        todo!()
    }
    fn from_adjacency_list() {
        todo!()
    }
    fn append_vertex() {
        todo!()
    }
    fn add_edge() {
        todo!()
    }
    fn add_vertex() {
        todo!()
    }
    fn edit_weight() {
        todo!()
    }
    fn get_weight() {
        todo!()
    }
    fn delete_vertex() {
        todo!()
    }
    fn vertex_deleted() {
        todo!()
    }
    fn delete_edge() {
        todo!()
    }
    fn outgoing_edges() {
        todo!()
    }

    fn incoming_edges() {
        todo!()
    }

    fn delete_outgoing_edges() {
        todo!()
    }

    fn delete_incoming_edges() {
        todo!()
    }
}

pub struct WeightedDigraph<W> {
    dg: Digraph,
    weights: HashMap<(usize, usize), W>,
}

impl<W> WeightedDigraph<W> {
    fn new() -> Self {
        WeightedDigraph {
            dg: Digraph::new(),
            weights: HashMap::new(),
        }
    }
    fn from_adjacency_list() -> Self {
        todo!()
    }
    fn append_vertex() {
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

    fn add_vertex(&mut self, vertex: usize) -> usize {
        self.dg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.dg.e_count
    }

    fn v_count(&self) -> usize {
        self.dg.v_count
    }

    fn edit_weight(&mut self, from: usize, to: usize, weight: W) {
        self.weights.insert((from, to), weight);
    }

    fn get_weight(&mut self, from: usize, to: usize) -> W {
        self.weights.get(&(from, to)).unwrap().to_owned()
    }
}

impl<W> Delete<usize> for WeightedDigraph<W> {
    fn delete_vertex(&mut self, vertex: usize) {
        if vertex < self.dg.v_count {
            self.dg.deleted_vertices.push(vertex);
            self.delete_incoming_edges(vertex);
            self.delete_outgoing_edges(vertex);
            self.dg.v_count -= 1;
        } else {
            panic!("delete_vertex : Can't delete Vertex : vertex >= self.v_count")
        }
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        self.dg.vertex_deleted(vertex)
    }
}
impl<W> Directed<usize> for WeightedDigraph<W> {
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

#[cfg(test)]
mod labeledweighteddigraph {
    use std::f64::{INFINITY, NEG_INFINITY};

    use num::ToPrimitive;

    use super::*;

    fn setuplwdg() -> LabeledWeightedDigraph<String, f64> {
        let v_count = 5;
        let e_count = 12;
        let mut labels: Vec<String> = Vec::new();
        for i in 0..v_count {
            labels.push(i.to_string());
        }
        let adj: Vec<Vec<(usize, f64)>> = vec![
            vec![(4, 0.22), (3, 0.1111111)],
            vec![],
            vec![(0, 5.13131)],
            vec![
                (0, 1111.0),
                (1, 0.0),
                (2, 2.1),
                (3, INFINITY),
                (4, NEG_INFINITY),
            ],
            vec![(0, 0.0), (1, 0.0), (2, 0.0), (3, 0.0)],
        ];

        let lwdg: LabeledWeightedDigraph<String, f64> =
            LabeledWeightedDigraph::from_adjacency_list(v_count, e_count, adj, labels);
        lwdg
    }
    // dg: LabeledDigraph<L>,
    // weights: HashMap<(usize, usize), W>,
    #[test]
    fn new() {
        let lwdg: LabeledWeightedDigraph<String, f64> = LabeledWeightedDigraph::new();
        assert!(lwdg.weights.is_empty());
        assert!(lwdg.dg.hashmap_labels_vertex.is_empty());
        assert!(lwdg.dg.vec_vertex_labels.is_empty());
        assert!(lwdg.dg.dg.adj.is_empty());
        assert!(lwdg.dg.dg.deleted_vertices.is_empty());
        assert_eq!(lwdg.v_count(), 0);
        assert_eq!(lwdg.e_count(), 0);
    }
    #[test]
    fn from_adjacency_list() {
        let v_count = 5;
        let e_count = 12;
        let mut labels: Vec<String> = Vec::new();
        let mut test_labels_hashmap: HashMap<String, usize> = HashMap::new();
        for i in 0..v_count {
            labels.push(i.to_string());
            test_labels_hashmap.insert(i.to_string(), i);
        }
        let adj: Vec<Vec<(usize, f64)>> = vec![
            vec![(4, 0.22), (3, 0.1111111)],
            vec![],
            vec![(0, 5.13131)],
            vec![
                (0, 1111.0),
                (1, 0.0),
                (2, 2.1),
                (3, INFINITY),
                (4, NEG_INFINITY),
            ],
            vec![(0, 0.0), (1, 0.0), (2, 0.0), (3, 0.0)],
        ];
        let testadj: Vec<Vec<usize>> = vec![
            vec![4, 3],
            vec![],
            vec![0],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
        ];
        let weights: Vec<Vec<f64>> = vec![
            vec![0.22, 0.1111111],
            vec![],
            vec![5.13131],
            vec![1111.0, 0.0, 2.1, INFINITY, NEG_INFINITY],
            vec![0.0, 0.0, 0.0, 0.0],
        ];
        let mut testweights: HashMap<(usize, usize), f64> = HashMap::new();
        let mut j = 0;
        let mut u: usize = 0;
        for from in testadj.clone() {
            for to in from {
                testweights.insert((j, to), weights[j].get(u).unwrap().clone());
                u += 1;
            }
            u = 0;
            j += 1;
        }

        let lwdg: LabeledWeightedDigraph<String, f64> = LabeledWeightedDigraph::from_adjacency_list(
            v_count.clone(),
            e_count.clone(),
            adj.clone(),
            labels.clone(),
        );
        assert_eq!(lwdg.e_count(), e_count);
        assert_eq!(lwdg.v_count(), v_count);
        assert_eq!(lwdg.dg.dg.adj, testadj);
        assert_eq!(lwdg.dg.dg.deleted_vertices, vec![]);
        assert_eq!(lwdg.dg.vec_vertex_labels, labels);
        assert_eq!(lwdg.dg.hashmap_labels_vertex, test_labels_hashmap);
        assert_eq!(lwdg.weights, testweights);
    }
    #[test]
    fn add_edge() {
        let (i, j): (usize, usize) = (1, 0);
        let mut lwdg = setuplwdg();
        lwdg.add_edge(1.to_string(), 0.to_string(), NEG_INFINITY);
        assert_eq!(
            lwdg.weights.get_key_value(&(1, 0)).unwrap(),
            (&(i, j), &NEG_INFINITY)
        );
        assert_eq!(lwdg.e_count(), 13);
    }
    #[test]
    fn add_vertex() {
        let mut lwdg = setuplwdg();
        lwdg.add_vertex(5.to_string());
        assert_eq!(lwdg.v_count(), 6);
        assert_eq!(
            lwdg.dg
                .hashmap_labels_vertex
                .get_key_value(&5.to_string())
                .unwrap(),
            (&5.to_string(), &5)
        );
        assert_eq!(lwdg.dg.vec_vertex_labels[5], 5.to_string());
        assert_eq!(
            lwdg.dg.dg.adj,
            vec![
                vec![4, 3],
                vec![],
                vec![0],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3],
                vec![]
            ]
        );
    }
    #[test]
    fn edit_weight() {
        let mut lwdg = setuplwdg();
        let mut test_weights_hashmap: HashMap<(usize, usize), f64> = HashMap::new();
        let weights: Vec<Vec<f64>> = vec![
            vec![0.22, 0.1111111],
            vec![],
            vec![5.13131],
            vec![1111.0, 0.0, 2.1, INFINITY, NEG_INFINITY],
            vec![0.0, 0.0, 0.0, 0.0],
        ];

        let mut j = 0;
        let mut u: usize = 0;
        for from in lwdg.dg.dg.adj.clone() {
            for to in from {
                test_weights_hashmap.insert((j, to), weights[j].get(u).unwrap().clone());
                u += 1;
            }
            u = 0;
            j += 1;
        }
        lwdg.edit_weight(0.to_string(), 4.to_string(), NEG_INFINITY);
        test_weights_hashmap.insert((0, 4), NEG_INFINITY);
        assert_eq!(lwdg.weights, test_weights_hashmap);
    }
    #[test]
    fn get_weight() {
        let mut lwdg = setuplwdg();
        let weights: Vec<Vec<f64>> = vec![
            vec![0.22, 0.1111111],
            vec![],
            vec![5.13131],
            vec![1111.0, 0.0, 2.1, INFINITY, NEG_INFINITY],
            vec![0.0, 0.0, 0.0, 0.0],
        ];
        let mut test_weights_hashmap: HashMap<(usize, usize), f64> = HashMap::new();
        let mut j = 0;
        let mut u: usize = 0;
        for from in lwdg.dg.dg.adj.clone() {
            for to in from {
                test_weights_hashmap.insert((j, to), weights[j].get(u).unwrap().clone());
                assert_eq!(
                    lwdg.get_weight(
                        lwdg.dg.get_label(j).unwrap().clone(),
                        lwdg.dg.get_label(to).unwrap().clone()
                    ),
                    test_weights_hashmap.get(&(j, to)).unwrap().clone()
                );
                u += 1;
            }
            u = 0;
            j += 1;
        }
    }
    #[test]
    fn delete_vertex() {
        let mut lwdg = setuplwdg();
        lwdg.delete_vertex(0.to_string());
        let v_count = 4;
        let e_count = 7;
        let mut labels: Vec<String> = Vec::new();
        let mut test_labels_hashmap: HashMap<String, usize> = HashMap::new();
        let mut test_weights_hashmap: HashMap<(usize, usize), f64> = HashMap::new();
        for i in 0..5 {
            labels.push(i.to_string());
            test_labels_hashmap.insert(i.to_string(), i);
        }
        test_weights_hashmap.insert((3.to_usize().unwrap(), 1.to_usize().unwrap()), 0.0);
        test_weights_hashmap.insert((3.to_usize().unwrap(), 2.to_usize().unwrap()), 2.1);
        test_weights_hashmap.insert((3.to_usize().unwrap(), 3.to_usize().unwrap()), INFINITY);
        test_weights_hashmap.insert((3.to_usize().unwrap(), 4.to_usize().unwrap()), NEG_INFINITY);

        test_weights_hashmap.insert((4.to_usize().unwrap(), 1.to_usize().unwrap()), 0.0);
        test_weights_hashmap.insert((4.to_usize().unwrap(), 2.to_usize().unwrap()), 0.0);
        test_weights_hashmap.insert((4.to_usize().unwrap(), 3.to_usize().unwrap()), 0.0);
        let adj: Vec<Vec<usize>> = vec![vec![], vec![], vec![], vec![4, 1, 2, 3], vec![3, 1, 2]]; // order is not important and changes here since we use swap_remove for more efficency
        let weights: Vec<Vec<f64>> = vec![
            vec![],
            vec![0.22, 0.1111111],
            vec![],
            vec![],
            vec![0.0, 2.1, INFINITY, NEG_INFINITY],
            vec![0.0, 0.0, 0.0],
        ];

        assert_eq!(lwdg.e_count(), e_count);
        assert_eq!(lwdg.v_count(), v_count);
        assert_eq!(lwdg.dg.dg.adj, adj);
        assert_eq!(lwdg.dg.dg.deleted_vertices, vec![0]);
        assert_eq!(lwdg.dg.vec_vertex_labels, labels);
        assert_eq!(lwdg.dg.hashmap_labels_vertex, test_labels_hashmap);
    }
    #[test]
    fn vertex_deleted() {
        let mut lwdg = setuplwdg();
        lwdg.dg.dg.deleted_vertices.push(2);
        assert_eq!(lwdg.vertex_deleted(lwdg.dg.get_label(2).unwrap().clone()),true);
    }
    #[test]
    fn delete_edge() {
        let mut lwdg = setuplwdg();
        let testadj: Vec<Vec<usize>> = vec![
            vec![4, 3],
            vec![],
            vec![0],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
        ];
        let mut j = 0;
        let mut u: usize = 0;
        for from in lwdg.dg.dg.adj.clone() {
            for to in from {
                lwdg.delete_edge(j.to_string(), to.to_string());
            }
            u = 0;
            j += 1;
        }
        assert_eq!(lwdg.dg.dg.adj, vec![vec![];lwdg.v_count()]);
        assert_eq!(lwdg.e_count(), 0);
    }

    fn outgoing_edges() {
        let mut lwdg = setuplwdg();
        let testadj: Vec<Vec<usize>> = vec![
            vec![4, 3],
            vec![],
            vec![0],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
        ];
        //todo 
    }

    fn incoming_edges() {
        todo!()
    }
    #[test]
    fn delete_outgoing_edges() {
        let mut lwdg = setuplwdg();
        assert_eq!(lwdg.e_count(), 12);
        for i in 0..lwdg.v_count() {
            lwdg.delete_outgoing_edges(i.to_string());
        }
        assert_eq!(lwdg.e_count(), 0);
    }
    #[test]
    fn delete_incoming_edges() {
        let mut lwdg = setuplwdg();
        assert_eq!(lwdg.e_count(), 12);
        for i in 0..lwdg.v_count() {
            lwdg.delete_incoming_edges(i.to_string());
        }
        assert_eq!(lwdg.e_count(), 0);
    }
}
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
    fn new() -> Self {
        LabeledWeightedDigraph {
            dg: LabeledDigraph::new(),
            weights: HashMap::new(),
        }
    }
    fn from_adjacency_list(
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

impl<L, W> Weighted<L, W> for LabeledWeightedDigraph<L, W>
where
    L: Hash + Eq + Clone,
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

    fn add_vertex(&mut self, vertex: L) -> usize {
        self.dg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.dg.e_count()
    }

    fn v_count(&self) -> usize {
        self.dg.v_count()
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

impl<L, W> Delete<L> for LabeledWeightedDigraph<L, W>
where
    L: Hash + Eq + Clone + std::fmt::Display,
{
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

    fn vertex_deleted(&self, vertex: L) -> bool {
        self.dg.vertex_deleted(vertex)
    }
}

impl<L, W> Directed<L> for LabeledWeightedDigraph<L, W>
where
    L: Hash + Eq + Clone + std::fmt::Display,
{
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
