use vers_vecs::RsVec;
use crate::from_file::graphen::helper::read_adj_directed;
use crate::wt::undirected::WTUGraph;
use crate::Edit;
use crate::from_file::v_e_count;
use std::collections::HashMap;
use qwt:: QWT256;
use crate::wt::directed::WTDigraph;
use super::sequence_and_bitmap;



/// read in an undirected wt-graph encoded in a file 
pub fn create_wt_ugraph(filepath: &str) -> WTUGraph {
    let wtd = create_wt_digraph(filepath);
    WTUGraph {
        wtd,
    }
}

/// read in a directed wt-graph encoded in a file
pub fn create_wt_digraph(filepath: &str) -> WTDigraph {
    let (v_count, e_count) = v_e_count(filepath);
    let adj = read_adj_directed(filepath);
    let (sequence, bitmap) = sequence_and_bitmap(&adj);
    let starting_indices : RsVec = RsVec::from_bit_vec(bitmap);
    let wt_adj = QWT256::from(sequence);
    WTDigraph {
        wt_adj_len: v_count,
        e_count,
        wt_adj_len_updated: v_count,                         
        e_count_updated: e_count,
        wt_adj,
        starting_indices,
        deleted_vertices: vec![],
        deleted_vertices_uncommitted : Vec::<Edit<usize>>::new(),
        adj_uncommitted : HashMap::<usize, Vec::<Edit<usize>>>::new(),
        has_uncommitted_edits : false,
    }
}


// read in an undirected wt-graph with weighted edges from a file


// read in a directed wt-graph with weighted edges from a file


/// read in a undirected labeled wt-graph from file
pub fn create_wt_ugraph<L>(filepath: &str) { // -> WTUGraph<L> 
    todo!()
}

// read in a directed labeled wt-graph from file
pub fn create_wt_digraph<L>(filepath: &str) { //-> WTDigraph<L> 
    todo!()
}

/// read in a undirected labeled wt-graph with weighted edges from file
pub fn create_wt_weighted_ugraph<L,W>(filepath: &str) { // -> WTWeightedUGraph<W,L> 
    todo!()
}

/// read in an directed labeled graph with weighted edges from file
pub fn create_wt_weighted_digraph<L,W>(filepath: &str) { // -> WTWeightedDigraph<W,L>
    todo!()
}