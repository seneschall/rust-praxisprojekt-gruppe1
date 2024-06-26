





/// read in a undirected labeled graph from file
pub fn create_ugraph<L>(filepath: &str) { // -> UGraph<L> 
    todo!()
}

/// read in an directed labeled graph from file
pub fn create_digraph<L>(filepath: &str) { //-> Digraph<L> 
    todo!()
}

/// read in a undirected labeled graph with weighted edges from file
pub fn create_weighted_ugraph<L,W>(filepath: &str) { // -> WeightedUGraph<W,L> 
    todo!()
}

/// read in an directed labeled graph with weighted edges from file
pub fn create_weighted_digraph<L,W>(filepath: &str) { // -> WeightedDigraph<W,L>
    todo!()
}
















/* use std::str::FromStr;
use std::fmt::Debug;

use crate::graph::{labeled_directed::LabeledDigraph,
    labeled_weighted_directed::LabeledWeightedDigraph,
    labeled_undirected::LabeledUGraph,
    labeled_weighted_undirected::LabeledWeightedUGraph};
use crate::import::v_e_count;
use crate::traits::{Unweighted, Weighted};
use helper::*;


/// read in a undirected graph encoded in a file
pub fn create_ugraph<L>(filepath: &str) -> LabeledUGraph<L> {
    let mut dg = create_digraph(filepath);
    let mut counter = 0;
    let mut changes: Vec<(usize,usize)> = vec![];
    for list in &dg.adj {
        for edge in list {
            changes.push((*edge,counter));
        }
        counter +=1;
    }
    for (edge,counter) in changes {
        dg.add_edge(edge, counter);
    }
    UGraph {
        dg,
    }
}

/// can read in a directed graph encoded in a file
pub fn create_digraph<L>(filepath: &str) -> LabeledDigraph<L> {
    let (v_count, e_count) = v_e_count(filepath);
    let adj = read_adj_directed(filepath);
    Digraph {
        deleted_vertices : vec![],
        v_count,
        e_count,
        adj,
    }
}

/// read in a undirected graph with weighted edges encoded in a file
pub fn create_weighted_ugraph<W>(filepath: &str) -> WeightedUGraph<W> 
where W: Debug, W : FromStr, W: Copy{
    let mut wdg: WeightedDigraph<W> = create_weighted_digraph(filepath);
    let mut counter = 0;
    let mut changes: Vec<(usize,usize)> = vec![];
    for list in &wdg.dg.adj {
        for edge in list {
            changes.push((*edge,counter));
        }
        counter +=1;
    }
    for i in 0..wdg.dg.e_count {
        for j in 0..wdg.dg.v_count {
            if let (&(counter,edge),&weight) = wdg.weights.get_key_value(&(i,j)).unwrap() {
                wdg.add_edge(edge, counter, weight)
            }
        }
    }
    for (edge,counter) in changes {
        wdg.dg.add_edge(edge, counter);
    }
    WeightedUGraph {
        wdg,
    }
}

/// read in a directed graph with weighted edges encoded in a file
pub fn create_weighted_digraph<W>(filepath: &str) -> WeightedDigraph<W>
where W: FromStr, W: Debug, {
    let (adj, weights) = read_adj_directed_weighted(filepath);
    WeightedDigraph {
        dg : create_digraph(filepath),
        weights, 
    }
}



pub mod helper {
    
use std::fs;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Debug;

// extract the adjacency list representing all the edges between two vertices from the input file.
pub(crate) fn read_adj_directed(filename: &str) -> Vec<Vec<usize>> {
    // opens the file
    let content = fs::read_to_string(filename).expect("Unable to open file");
    // read the file in lines
    let mut lines = content.lines();
    // read v_count
    let v_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // skip e_count
    lines.next();
    // create Vec<Vec<T>> in the size of v_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    // loops over all lines
    for line in lines {
        // remove possible whitespaces and delimiters
        let line = line.trim();
        // returns a iterator of type usize , consisting of the (two) elements in the current line
        let mut numbers = line.split_whitespace()
        // this line??
        .filter_map(|s| s.parse::<usize>().ok());
        // store the (two) elements in 'vertex' and 'adjacent'
        if let (Some(vertex), Some(adjacent)) = (numbers.next(), numbers.next()) {
            // this line pushes the pairs onto the adj-vector      
            // only this line would differ in an explicit implementation of read_adj_undirected   
            adjacency_list[vertex].push(adjacent);
        } else {
            eprintln!("Invalid line: {}", line);
            // here better error handling needed
        }
    } assert_eq!(v_count, adjacency_list.len());
    adjacency_list
}

// extract the adjacency list representing all the edges between two vertices from the input file, plus a hashmap storing their weights
pub(crate) fn read_adj_directed_weighted<W>(filename: &str) -> (Vec<Vec<usize>>,HashMap<(usize,usize),W>)
where W: FromStr, W: Debug {
    // opens the file
    let content = fs::read_to_string(filename).expect("Unable to open file");
    // read the file in lines
    let mut lines = content.lines();
    // read v_count
    let v_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // read e_count
    let e_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // create Vec<Vec<T>> in the size of v_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    // create HashMap<(usize,usize),W> in the size of e_count
    let mut weights: HashMap<(usize,usize),W> = HashMap::with_capacity(e_count);
    // loops over all lines
    for line in lines {
        // remove possible whitespaces and delimiters
        let line = line.trim();
        let elements : Vec<&str> = line.split(' ').collect();
        let vertex = elements[0].parse::<usize>().expect("need error handling");
        let adjacent = elements[1].parse::<usize>().expect("need error handling");
        let weight = elements[3].parse::<W>().ok().unwrap();
        adjacency_list[vertex].push(adjacent);
        weights.insert((vertex,adjacent),weight);
            // need error handling
    } assert_eq!(v_count, adjacency_list.len());
    (adjacency_list, weights)
}

}

*/