use std::str::FromStr;
use std::fmt::Debug;

use crate::graph::{directed::Digraph,weighted_directed::WeightedDigraph,undirected::UGraph,weighted_undirected::WeightedUGraph};
use crate::from_file::v_e_count;
use helper::*;


/// read in an undirected graph from file
pub fn create_ugraph(filepath: &str) -> UGraph {
    let mut dg = create_digraph(filepath);
    UGraph {
        dg,
    }
}

/// read in a directed graph from file
pub fn create_digraph(filepath: &str) -> Digraph {
    let (v_count, e_count) = v_e_count(filepath);
    let adj = read_adj_directed(filepath);
    Digraph {
        deleted_vertices : vec![],
        v_count,
        e_count,
        adj,
    }
}

/// read in an undirected graph with weighted edges from file
pub fn create_weighted_ugraph<W>(filepath: &str) -> WeightedUGraph<W> 
where W: Debug, W : FromStr, W: Copy{
    let mut wdg: WeightedDigraph<W> = create_weighted_digraph(filepath);
    WeightedUGraph {
        wdg,
    }
}

/// read in a directed graph with weighted edges from file
pub fn create_weighted_digraph<W>(filepath: &str) -> WeightedDigraph<W>
where W: FromStr, W: Debug, {
    let (_adj, weights) = read_adj_directed_weighted(filepath);
    WeightedDigraph {
        dg : create_digraph(filepath),
        weights, 
    }
}



pub(crate) mod helper {
    
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