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

