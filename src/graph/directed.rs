use std::collections::HashMap;

use crate::graph::Graph;

use super::{Directed, Weighted};

// UNIT-TESTS for Digraph and Weighed Digraph
#[cfg(test)]
mod test {


    use super::*;
    const V_COUNT: usize = 10;
    #[test]
    fn create_new_digraph() {
        // code for digraph_weighted
    }
    #[test]
    fn create_digraph_from_adj() {
        // code for digraph
    }
    #[test]
    fn create_weighted_digraph_from_adj() {
        // code for digraph_weighted
    }
    #[test]
    fn add_edge_to_digraphs() {
        // code for digraph
        // code for digraph_weighted
    }
    #[test]
    fn add_vertex_to_digraphs() {
        // code for digraph
        // code for digraph_weighted
    }
    #[test]
    fn append_vertex_to_digraphs() {
        // code for digraph
        // code for digraph_weighted
    }
    #[test]
    fn remove_edge_from_digraphs() {
        // code for digraph
        // code for digraph_weighted
    }
    #[test]
    fn remove_vertex_from_digraphs() {
        // code for digraph
        // code for digraph_weighted        
    }
    #[test]
    fn add_label_to_digraphs() {
        // let mut graph: Digraph<String> = Digraph::new(V_COUNT);
        // graph.add_vertex_label(0, String::from("test"));
        // assert_eq!(graph.get_label(0), Some(&String::from("test")));
        // assert_eq!(graph.get_label(1), None);
        // code for digraph_weighted  
    }
    #[test]
    fn edit_label_on_digraphs() {
        // code for digraph
        // code for digraph_weighted
    } 
    #[test]
    fn outgoing_incoming_edges_on_digraphs() {
        // code for digraph
        // code for digraph_weighted
    }
    #[test]
    fn print_weight_of_digraph_edge() {
        // code for digraph_weighted
    }
    #[test]
    fn add_edit_weight_of_digraph_edge() {
        // code for digraph_weighted
    }
    #[test]
    fn delete_weight_of_digraph_edge() {
        // code for digraph_weighted
    }
}

// Digraph - definition and methods
pub struct Digraph<L>
{
    v_count: usize,                     // number of vertices
    e_count: usize,                     // number of edges
    pub(crate) adj: Vec<Vec<usize>>,               // adjacency list of indices -- note from group: should we set this to pub(crate)?
    node_labels: HashMap<usize, L>,            // format: index of node - value of node's label
}

impl<L> Digraph<L>
{
    pub fn new(v_count: usize) -> Self {
        Digraph {
            v_count,
            e_count: 0,
            adj: vec![vec![]; v_count],
            node_labels: HashMap::new(),
        }
    }
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<usize>>) -> Self {
        // temporary, constructor with adj list -- note from celine: what is the meaning of this comment? is there a missing implementation?
        Digraph {
            v_count,
            e_count,
            adj,
            node_labels: HashMap::new(),
        }
    }
    fn vertex_exists(&self, v: usize) -> bool {
        v < self.v_count
    }
}

impl<L> Graph<L> for Digraph<L>
where
    L: Clone,
{
    fn add_edge(&mut self, v: usize, w: usize) {
        if !(self.vertex_exists(v) || self.vertex_exists(w)) {
            panic!("One of vertices {}, {} doesn't exist", v, w)
        }
        self.e_count += 1;
        self.adj[v].push(w);
    }

    fn add_vertex(&mut self, v: usize) {
        if v >= self.v_count(){
            let dummy = self.v_count();
            for i in 0..v-self.v_count(){
                self.adj.insert(dummy+i, vec![]);
                self.v_count +=1;
            }
        } else {
            self.adj.insert(v,vec![]);
            self.v_count +=1;
        }
    }

    fn add_vertex_label(&mut self, v: usize, label: L) {
        self.node_labels.insert(v, label); 
    }

    fn append_vertex(&mut self, v: usize) -> usize {
        todo!()
    }

    fn delete_edge(&mut self, v: usize, w: usize) {
        let i_of_w: usize; // -- note from celine: could we use index_of_w for clarity?
        match self.adj.get(v) {
            Some(vs) => {  
                let i_of_w_opt = vs.iter().position(|&x| x == w); // -- note from celine: can you explain this?
                // is this a nested match?
                match i_of_w_opt {
                    Some(i) => {
                        i_of_w = i;
                    } // swap_remove more efficient than remove because the order is not important
                    None => {
                        panic!("There was no edge from {v} to {w}.");
                    }
                }
            }
            None => {
                panic!("Vertex {v} doesn't exist."); // Should be replaced by Result type
            }
        }

        self.adj[v].swap_remove(i_of_w);
        self.e_count -= 1;
    }

    fn delete_vertex(&mut self, v: usize) {
        self.v_count -= 1;
        self.e_count -= self.adj[v].len();
        // need to implement incoming_edges first
        // for item in self.incoming_edges(v){
        //     self.delete_edge(item, v);
        //     self.e_count -= 1;
        // }
        self.adj.remove(v);
    }

    fn e_count(&self) -> usize {
        self.e_count
    }

    fn edit_label(&mut self, v: usize, label: L) {
        todo!() // ...
    }

    fn get_label(&self, v: usize) -> Option<&L> {
        self.node_labels.get(&v) // note from celine: can you explain this?
    }

    fn v_count(&self) -> usize {
        self.v_count
    }

    fn vertex_deleted(&self, v: usize) -> bool {
        todo!() // ...
    }
    
}

impl<L> Directed for Digraph<L>
    // no where L clone?
{
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        self.adj[vertex].clone()
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        todo!() // ...
    }
}


// Weighted Digraph definition & methods

pub struct Digraph_Weighted<L> 
{
    test : L,
}

impl<L> Digraph_Weighted<L> 
{

}

impl<L> Graph<L> for Digraph_Weighted<L> 
{
    fn add_edge(&mut self, v: usize, w: usize) {
        todo!()
    }

    fn add_vertex(&mut self, v: usize) {
        todo!()
    }

    fn add_vertex_label(&mut self, v: usize, label: L) {
        todo!()
    }

    fn append_vertex(&mut self, v: usize) -> usize {
        todo!()
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn delete_vertex(&mut self, v: usize) {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn edit_label(&mut self, v: usize, label: L) {
        todo!()
    }

    fn get_label(&self, v: usize) -> Option<&L> {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }

    fn vertex_deleted(&self, v: usize) -> bool {
        todo!()
    }
}

impl<L> Directed for Digraph_Weighted<L> 
{
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        todo!()
    }
}

impl<L> Weighted for Digraph_Weighted<L> 
{
    fn weight_of_edge(&self, from: usize, to: usize) -> f64 {
        todo!()
    }
}
