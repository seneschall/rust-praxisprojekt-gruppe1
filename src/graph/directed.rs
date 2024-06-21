use crate::graph::Graph;
use std::collections::HashMap;
use std::hash::Hash;

use super::{Delete, Directed, Labeled, Weighted};

// UNIT-TESTS for Digraph and Weighed Digraph
#[cfg(test)]
mod test_old {

    // use super::*;
    // const V_COUNT: usize = 10;

    // #[test] // impl<L> Digraph<L>
    // fn test_digraph_new() {
    //     let digraph: Digraph<usize> = Digraph::new(10);
    //     assert_eq!(digraph.e_count(), 0);
    //     assert_eq!(digraph.v_count(), 10);
    //     assert_eq!(digraph.adj, vec![vec![]; digraph.v_count()]);
    //     assert_eq!(digraph.vertex_labels, HashMap::new())
    // }
    // #[test]
    // fn test_digraph_from_adjacency_list() {
    //     let digraph: Digraph<usize> = Digraph::from_adjacency_list(10, 2, vec![vec![2], vec![3]]);
    //     assert_eq!(digraph.v_count(), 10);
    //     assert_eq!(digraph.e_count(), 2);
    //     assert_eq!(digraph.adj, vec![vec![2], vec![3]]); // edges from 0 -> 2, 1 -> 3
    // }
    // #[test]
    // fn test_digraph_vertex_exists() {
    //     let digraph: Digraph<usize> = Digraph::new(10);
    //     for i in 0..digraph.v_count() {
    //         // adj has 10 entrys 0..9
    //         assert_eq!(digraph.vertex_exists(i), true);
    //     }
    //     assert_eq!(digraph.vertex_exists(10), false); // vertex 10 doesn't exist
    // }

    // #[should_panic(expected = "One of vertices 5, 10 doesn't exist")] // maybe missing counterpart expected = ..10, 5..
    // #[test] // impl<L> Graph<L> for Digraph<L>
    // fn test_digraph_add_edge() {
    //     let mut digraph: Digraph<usize> = Digraph::new(10);
    //     digraph.add_edge(0, 1);
    //     digraph.add_edge(9, 5);
    //     let mut test_adj: Vec<Vec<usize>> = vec![vec![]; digraph.v_count()];
    //     test_adj[0] = vec![1];
    //     test_adj[9] = vec![5];
    //     assert_eq!(digraph.adj, test_adj);
    //     assert_eq!(digraph.e_count(), 2);
    //     digraph.add_edge(5, 10); // panic here
    //     digraph.add_edge(10, 5); // panic here
    //     assert_eq!(digraph.adj, test_adj);
    // }
    // #[test]
    // fn test_digraph_add_vertex() {
    //     let mut digraph: Digraph<usize> = Digraph::new(1);
    //     digraph.add_vertex(2); // from [[]] to [[], [], []]
    //     assert_eq!(digraph.adj, vec![vec![], vec![], vec![]]);
    //     digraph.add_edge(2, 0);
    //     assert_eq!(digraph.adj, vec![vec![], vec![], vec![0]]);
    //     digraph.delete_edge(2, 0);
    //     digraph.add_vertex(20);
    //     assert_eq!(digraph.adj, vec![vec![]; digraph.v_count()]);
    //     assert_eq!(digraph.vertex_exists(20), true);
    // }
    // #[test]
    // fn test_digraph_add_vertex_label() {
    //     let mut digraph: Digraph<usize> = Digraph::new(1);
    //     digraph.add_label(0, 5);
    //     let mut test: HashMap<usize, usize> = HashMap::new();
    //     test.insert(0, 5);
    //     assert_eq!(digraph.vertex_labels, test);
    // }
    // #[test]
    // fn test_digraph_append_vertex() {
    //     let mut digraph: Digraph<usize> = Digraph::new(10);
    //     assert_eq!(digraph.append_vertex(), 10);
    //     assert_eq!(digraph.append_vertex(), 11);
    //     assert_eq!(digraph.outgoing_edges(10), vec![]);
    //     assert_eq!(digraph.incoming_edges(10), vec![]);
    //     assert_eq!(digraph.outgoing_edges(11), vec![]);
    //     assert_eq!(digraph.incoming_edges(11), vec![]);
    // }
    // #[test]
    // fn test_digraph_delete_edge() {
    //     let mut digraph: Digraph<usize> =
    //         Digraph::from_adjacency_list(10, 2, vec![vec![2], vec![3]]);
    //     assert_eq!(digraph.adj, vec![vec![2], vec![3]]); // edges from 0 -> 2, 1 -> 3
    //     assert_eq!(digraph.e_count(), 2);
    //     digraph.delete_edge(0, 2);
    //     assert_eq!(digraph.adj, vec![vec![], vec![3]]);
    //     digraph.delete_edge(1, 3);
    //     assert_eq!(digraph.adj, vec![vec![], vec![]]);
    //     assert_eq!(digraph.e_count(), 0);
    // }
    // #[test]
    // fn test_digraph_delete_and_shift() {
    //     let mut digraph: Digraph<usize> =
    //         Digraph::from_adjacency_list(2, 2, vec![vec![2], vec![3]]);
    //     // 0->2 , 1 ->3
    //     digraph.delete_and_shift(0);
    //     assert_eq!(digraph.adj, vec![vec![3]]);
    //     digraph.delete_and_shift(0); // adj is now vec![]
    //     digraph.add_vertex(0);
    //     assert_eq!(digraph.adj, vec![vec![]]);
    // }
    // #[test]
    // fn test_digraph_edit_label() {
    //     // edit_label & get_label
    //     let mut digraph: Digraph<usize> = Digraph::new(10);
    //     digraph.edit_label(0, 13);
    //     assert_eq!(digraph.get_label(0), Some(&13usize));
    //     digraph.edit_label(0, 10);
    //     assert_eq!(digraph.get_label(0), Some(&10usize));
    //     for i in 0..digraph.v_count() {
    //         digraph.edit_label(i, i + 100);
    //         assert_eq!(digraph.get_label(i), Some(&(i + 100)));
    //     }
    // }
    // //impl<L> Directed for Digraph<L>
    // #[test]
    // fn test_digraph_incoming_edges() {
    //     let mut digraph: Digraph<usize> = Digraph::new(10);
    //     for i in 0..digraph.v_count() {
    //         assert_eq!(digraph.incoming_edges(i), Vec::new());
    //     }
    //     for i in 0..digraph.v_count() - 1 {
    //         digraph.add_edge(i, i + 1);
    //     } // adds edges from 0 -> 1 , 1 -> 2, 2 -> 3 ...
    //     for i in 0..digraph.v_count() - 1 {
    //         assert_eq!(digraph.incoming_edges(i + 1), vec![i]);
    //         assert_eq!(digraph.adj[i], vec![i + 1]);
    //     }
    //     for i in 0..digraph.v_count() - 1 {
    //         digraph.delete_edge(i, i + 1);
    //     }
    //     for i in 0..digraph.v_count() - 1 {
    //         assert_eq!(digraph.incoming_edges(i + 1), Vec::new());
    //     }
    // }
    // #[test]
    // fn test_digraph_outgoing_edges() {
    //     let mut digraph: Digraph<usize> = Digraph::new(10);
    //     for i in 0..digraph.v_count() {
    //         assert_eq!(digraph.outgoing_edges(i), Vec::new());
    //     }
    //     for i in 0..digraph.v_count() - 1 {
    //         digraph.add_edge(i, i + 1);
    //     } // adds edges from 0 -> 1 , 1 -> 2, 2 -> 3 ...
    //     for i in 0..digraph.v_count() - 1 {
    //         assert_eq!(digraph.outgoing_edges(i), vec![i + 1]);
    //         assert_eq!(digraph.adj[i], vec![i + 1]);
    //     }
    //     for i in 0..digraph.v_count() - 1 {
    //         digraph.delete_edge(i, i + 1);
    //     }
    //     for i in 0..digraph.v_count() - 1 {
    //         assert_eq!(digraph.outgoing_edges(i), vec![]);
    //     }
    //     assert_eq!(digraph.adj, vec![vec![]; 10]);
    // }
}

#[cfg(test)]
mod test_digraph {
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
    fn from_adjacency_list(){
        let v_count = 10;
        let e_count = 5;
        let adj = vec![vec![0];10];
        let digraph = Digraph::from_adjacency_list(v_count, e_count, adj.clone());
        assert_eq!(digraph.v_count(), v_count);
        assert_eq!(digraph.e_count(), e_count);
        assert_eq!(digraph.adj,adj );
        assert!(digraph.deleted_vertices.is_empty());
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
        Digraph {
            deleted_vertices: Vec::new(),
            v_count,
            e_count,
            adj,
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
        self.v_count
    }

    fn e_count(&self) -> usize {
        self.e_count
    }

    fn v_count(&self) -> usize {
        self.v_count
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        (!self.deleted_vertices.contains(&vertex)) && vertex < self.v_count
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
    L: Eq + Hash,
{
    pub fn new() -> Self {
        LabeledDigraph {
            dg: Digraph::new(),
            vec_vertex_labels: Vec::new(),
            hashmap_labels_vertex: HashMap::new(),
        }
    }
}
impl<L> Graph<L> for LabeledDigraph<L>
where
    L: Eq + Hash + Copy,
{
    fn add_edge(&mut self, from: L, to: L) {
        self.dg.add_edge(
            self.get_index(from).unwrap().to_owned(),
            self.get_index(to).unwrap().to_owned(),
        );
    }

    fn add_vertex(&mut self, vertex: L) -> usize {
        let index = self.dg.append_vertex();
        self.vec_vertex_labels.push(vertex);
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
    L: Hash + Eq + Copy,
{
    fn delete_vertex(&mut self, vertex: L) {
        self.dg
            .delete_vertex(self.get_index(vertex).unwrap().to_owned())
    }

    fn vertex_deleted(&self, vertex: L) -> bool {
        self.dg
            .vertex_deleted(self.get_index(vertex).unwrap().to_owned())
    }
}

impl<L> Directed<L> for LabeledDigraph<L>
where
    L: Hash + Eq + Clone + Copy,
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
    L: Hash + Eq + Copy,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        self.vec_vertex_labels[self
            .hashmap_labels_vertex
            .get(&old_label)
            .unwrap()
            .to_owned()] = new_label; // update Vec

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
    L: Hash + Eq,
{
    fn new() -> Self {
        LabeledWeightedDigraph {
            dg: LabeledDigraph::new(),
            weights: HashMap::new(),
        }
    }
}

impl<L, W> Weighted<L, W> for LabeledWeightedDigraph<L, W>
where
    L: Hash + Eq + Copy,
    W: Clone,
{
    fn add_edge(&mut self, from: L, to: L, weight: W) {
        self.dg.add_edge(from, to);
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
    L: Hash + Eq + Copy + std::fmt::Display,
{
    fn delete_vertex(&mut self, vertex: L) {
        if self.dg.get_index(vertex).unwrap().to_owned() < self.dg.dg.v_count {
            self.dg
                .dg
                .deleted_vertices
                .push(self.dg.get_index(vertex).unwrap().to_owned());
            self.delete_incoming_edges(vertex);
            self.delete_outgoing_edges(vertex);
            self.dg.dg.v_count -= 1;
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
    L: Hash + Eq + Copy + std::fmt::Display,
{
    fn delete_edge(&mut self, from: L, to: L) {
        let i_of_w: usize; // -- note from celine: could we use index_of_w for clarity?
        match self.dg.dg.adj.get(self.dg.get_index(from).unwrap().clone()) {
            Some(vs) => {
                let i_of_w_opt = vs
                    .iter()
                    .position(|&x| x == self.dg.get_index(to).unwrap().to_owned());
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
        let from_index = self.dg.get_index(from).unwrap().clone();
        self.dg.dg.adj[from_index].swap_remove(i_of_w);
        self.weights
            .remove(&(self.dg.get_index(from).unwrap().clone(), i_of_w));
        self.dg.dg.e_count -= 1;
    }

    fn outgoing_edges(&self, vertex: L) -> Vec<L> {
        self.dg.outgoing_edges(vertex)
    }

    fn incoming_edges(&self, vertex: L) -> Vec<L> {
        self.dg.incoming_edges(vertex)
    }

    fn delete_outgoing_edges(&mut self, vertex: L) {
        for to in self.outgoing_edges(vertex) {
            self.delete_edge(vertex, to);
        }
    }

    fn delete_incoming_edges(&mut self, vertex: L) {
        for from in self.incoming_edges(vertex) {
            self.delete_edge(from, vertex);
        }
    }
}
