use std::collections::HashMap;

use crate::graph::Graph;

use super::{Directed, Weighted};

// UNIT-TESTS for Digraph and Weighed Digraph
#[cfg(test)]
mod test {


    use std::collections::hash_map;

    use super::*;
    const V_COUNT: usize = 10;

    #[test] // impl<L> Digraph<L>
    fn test_digraph_new() {
        let digraph: Digraph<usize> = Digraph::new(10);
        assert_eq!(digraph.e_count(), 0);
        assert_eq!(digraph.v_count(), 10);
        assert_eq!(digraph.adj, vec![vec![]; digraph.v_count()]);
        assert_eq!(digraph.node_labels, HashMap::new())
    }
    #[test]
    fn test_digraph_from_adjacency_list() {
        let digraph: Digraph<usize> = Digraph::from_adjacency_list(10, 2, vec![vec![2], vec![3]]);
        assert_eq!(digraph.v_count(), 10);
        assert_eq!(digraph.e_count(), 2);
        assert_eq!(digraph.adj, vec![vec![2], vec![3]]); // edges from 0 -> 2, 1 -> 3
    }
    #[test]
    fn test_digraph_vertex_exists() {
        let digraph: Digraph<usize> = Digraph::new(10);
        for i in 0..digraph.v_count(){
            // adj has 10 entrys 0..9
            assert_eq!(digraph.vertex_exists(i), true);
        }
        assert_eq!(digraph.vertex_exists(10), false); // vertex 10 doesn't exist
    }

    #[should_panic(expected = "One of vertices 5, 10 doesn't exist")] // maybe missing counterpart expected = ..10, 5..
    #[test] // impl<L> Graph<L> for Digraph<L>
    fn test_digraph_add_edge() {
        let mut digraph: Digraph<usize> = Digraph::new(10);
        digraph.add_edge(0, 1);
        digraph.add_edge(9, 5);
        let mut test_adj : Vec<Vec<usize>> = vec![vec![]; digraph.v_count()];
        test_adj[0] = vec![1];
        test_adj[9] = vec![5];
        assert_eq!(digraph.adj, test_adj);
        assert_eq!(digraph.e_count(), 2);
        digraph.add_edge(5, 10); // panic here
        digraph.add_edge(10,5);  // panic here
        assert_eq!(digraph.adj, test_adj);
    }
    #[test]
    fn test_digraph_add_vertex(){
        let mut digraph: Digraph<usize> = Digraph::new(1);
        digraph.add_vertex(2); // from [[]] to [[], [], []]
        assert_eq!(digraph.adj, vec![vec![], vec![], vec![]]);
        digraph.add_edge(2, 0);
        assert_eq!(digraph.adj, vec![vec![], vec![], vec![0]]);
        digraph.delete_edge(2,0);
        digraph.add_vertex(20);
        assert_eq!(digraph.adj, vec![vec![];digraph.v_count()]);
        assert_eq!(digraph.vertex_exists(20), true);
    }
    #[test]
    fn test_digraph_add_vertex_label(){
        let mut digraph: Digraph<usize> = Digraph::new(1);
        digraph.add_vertex_label(0, 5);
        let mut test: HashMap<usize,usize> = HashMap::new();
        test.insert(0,5);
        assert_eq!(digraph.node_labels, test); 
    }
    #[test]
    fn test_digraph_append_vertex() {
        let mut digraph: Digraph<usize> = Digraph::new(10);
        assert_eq!(digraph.append_vertex(0), 10);
        assert_eq!(digraph.append_vertex(0), 11);
        assert_eq!(digraph.outgoing_edges(10), vec![]);
        assert_eq!(digraph.incoming_edges(10), vec![]);
        assert_eq!(digraph.outgoing_edges(11), vec![]);
        assert_eq!(digraph.incoming_edges(11), vec![]);
    }
    #[test]
    fn test_digraph_delete_edge(){
        let mut digraph: Digraph<usize> = Digraph::from_adjacency_list(10, 2, vec![vec![2], vec![3]]);
        assert_eq!(digraph.adj, vec![vec![2], vec![3]]); // edges from 0 -> 2, 1 -> 3
        assert_eq!(digraph.e_count(), 2);
        digraph.delete_edge(0, 2);
        assert_eq!(digraph.adj, vec![vec![], vec![3]]);
        digraph.delete_edge(1,3);
        assert_eq!(digraph.adj, vec![vec![], vec![]]);
        assert_eq!(digraph.e_count(), 0);
    }
    #[test]
    fn test_digraph_delete_vertex(){
        let mut digraph: Digraph<usize> = Digraph::from_adjacency_list(2, 2, vec![vec![2], vec![3]]);
        // 0->2 , 1 ->3
        digraph.delete_vertex(0);
        assert_eq!(digraph.adj, vec![vec![3]]);
        digraph.delete_vertex(0); // adj is now vec![]
        digraph.add_vertex(0);
        assert_eq!(digraph.adj, vec![vec![]]);
    }
    #[test]
    fn test_digraph_edit_label() { // edit_label & get_label
        let mut digraph: Digraph<usize> = Digraph::new(10);
        digraph.edit_label(0, 13);
        assert_eq!(digraph.get_label(0), Some(&13usize));
        digraph.edit_label(0, 10);
        assert_eq!(digraph.get_label(0), Some(&10usize));
        for i in 0..digraph.v_count() {
            digraph.edit_label(i, i + 100);
            assert_eq!(digraph.get_label(i), Some(&(i + 100)));
        }
    }
    #[test]
    fn test_digraph_vertex_deleted() {
        let mut digraph: Digraph<usize> = Digraph::new(10);
        for i in 0..digraph.v_count() {
            // new graph without any vertices, all should be empty
            assert_eq!(digraph.vertex_deleted(i), true)
        }
        for i in 0..digraph.v_count()-1 {
            // add edge to all vertices 0 -> 1, 1-> 2, 2-> 3 ...
            digraph.add_edge(i, i + 1);
        }
        digraph.add_edge(digraph.v_count()-1, 0);
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.vertex_deleted(i), false);
        }
        assert_eq!(digraph.e_count(), 10);
        for i in 0..digraph.v_count() {
            digraph.delete_vertex(0); // deletes all vertices
        }
        assert_eq!(digraph.v_count(), 0);
        assert_eq!(digraph.e_count(), 0);
    }


    //impl<L> Directed for Digraph<L>
    #[test]
    fn test_digraph_incoming_edges() {
        let mut digraph: Digraph<usize> = Digraph::new(10);
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.incoming_edges(i), Vec::new());
        }
        for i in 0..digraph.v_count() - 1 {
            digraph.add_edge(i, i + 1);
        } // adds edges from 0 -> 1 , 1 -> 2, 2 -> 3 ...
        for i in 0..digraph.v_count() - 1 {
            assert_eq!(digraph.incoming_edges(i + 1), vec![i]);
            assert_eq!(digraph.adj[i], vec![i+1]);
        }
        for i in 0..digraph.v_count() -1{
            digraph.delete_edge(i, i+1);
        }
        for i in 0..digraph.v_count()-1{
            assert_eq!(digraph.incoming_edges(i+1), Vec::new());
        }
    }
    #[test]
    fn test_digraph_outgoing_edges(){
        let mut digraph: Digraph<usize> = Digraph::new(10);
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.outgoing_edges(i), Vec::new());
        }
        for i in 0..digraph.v_count() - 1 {
            digraph.add_edge(i, i + 1);
        } // adds edges from 0 -> 1 , 1 -> 2, 2 -> 3 ...
        for i in 0..digraph.v_count()-1 {
            assert_eq!(digraph.outgoing_edges(i), vec![i+1]);
            assert_eq!(digraph.adj[i], vec![i+1]);
        }
        for i in 0..digraph.v_count() -1 {
            digraph.delete_edge(i,i+1);
        }
        for i in 0..digraph.v_count() -1{
            assert_eq!(digraph.outgoing_edges(i), vec![]);
        }
        assert_eq!(digraph.adj, vec![vec![]; 10]);
    }


}

// Digraph - definition and methods
pub struct Digraph<L> {
    v_count: usize,                  // number of vertices
    e_count: usize,                  // number of edges
    pub(crate) adj: Vec<Vec<usize>>, // adjacency list of indices -- note from group: should we set this to pub(crate)?
    node_labels: HashMap<usize, L>,  // format: index of node - value of node's label
}

impl<L> Digraph<L> {
    pub fn new(v_count: usize) -> Self {
        Digraph {
            v_count,
            e_count: 0,
            adj: vec![vec![]; v_count],
            node_labels: HashMap::new(),
        }
    }
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<usize>>) -> Self {
        Digraph {
            v_count,
            e_count,
            adj,
            node_labels: HashMap::new(),
        }
    }
    fn vertex_exists(&self, vertex: usize) -> bool {
        vertex < self.v_count
    }
}

impl<L> Graph<L> for Digraph<L>
where
    L: Clone,
{
    fn add_edge(&mut self, from: usize, to: usize) {
        if !(self.vertex_exists(from) && self.vertex_exists(to)) {
            panic!("One of vertices {}, {} doesn't exist", from, to)
        }
        self.e_count += 1;
        self.adj[from].push(to);
    }

    fn add_vertex(&mut self, vertex: usize) {
        if vertex >= self.v_count() {
            let dummy = self.v_count();
            for i in 0..vertex - self.v_count()+1 {
                self.adj.insert(dummy + i, vec![]);
                self.v_count += 1;
            }
        } else {
            self.adj.insert(vertex, vec![]);
            self.v_count += 1;
        }
    }

    fn add_vertex_label(&mut self, vertex: usize, label: L) {
        self.node_labels.insert(vertex, label);
    }

    fn append_vertex(&mut self, vertex: usize) -> usize {
        // question value of vertex ?
        // IF value of vertex doesn't matter
        // you just want to append a vertex and return the index of the new vertex
        self.adj.push(vec![]);
        self.v_count += 1;
        self.v_count - 1 //len-1 = index
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        let i_of_w: usize; // -- note from celine: could we use index_of_w for clarity?
        match self.adj.get(from) {
            Some(vs) => {
                let i_of_w_opt = vs.iter().position(|&x| x == to); // -- note from celine: can you explain this?
                                                                   // is this a nested match?
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

    fn delete_vertex(&mut self, vertex: usize) {
        for item in self.incoming_edges(vertex) {
            self.delete_edge(item, vertex);
        }
        for item in self.outgoing_edges(vertex){
            self.delete_edge(vertex, item);
        }
        self.v_count -= 1;
        self.adj.remove(vertex);
    }

    fn e_count(&self) -> usize {
        self.e_count
    }

    fn edit_label(&mut self, vertex: usize, label: L) {
        self.node_labels.insert(vertex, label);
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        self.node_labels.get(&vertex) // note from celine: can you explain this?
    }

    fn v_count(&self) -> usize {
        self.v_count
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        // doesn't work here like in wt_digraph
        // returning true if vertex has no incoming or outgoing edges
        // question how should this work?
        // delete vertex removes the entry at the given index
        // all vertices to the right are shifted one index to the left
        // => can't check if vertex is deleted
        // you can only check if vertex has no incoming or outgoing edges
        // user might be confused with v_count if delete vertex isn't deleting
        if self.outgoing_edges(vertex).is_empty() {
            if self.incoming_edges(vertex).is_empty() {
                return true;
            }
        }
        false
    }
}

impl<L> Directed for Digraph<L>
// no where L clone?
{
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
}

// Weighted Digraph definition & methods

pub struct Digraph_Weighted<L> {
    test: L,
}

impl<L> Digraph_Weighted<L> {}

impl<L> Graph<L> for Digraph_Weighted<L> {
    fn add_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn add_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn add_vertex_label(&mut self, vertex: usize, label: L) {
        todo!()
    }

    fn append_vertex(&mut self, vertex: usize) -> usize {
        todo!()
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn delete_vertex(&mut self, vertex: usize) {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn edit_label(&mut self, vertex: usize, label: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }

    fn vertex_deleted(&self, vertex: usize) -> bool {
        todo!()
    }
}

impl<L> Directed for Digraph_Weighted<L> {
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }
}

impl<L> Weighted for Digraph_Weighted<L> {
    fn weight_of_edge(&self, from: usize, to: usize) -> f64 {
        todo!()
    }
}
