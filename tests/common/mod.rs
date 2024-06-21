use vers_vecs::RsVec;
use wt_graphs::graph::{
    create_sequence_and_bitmap, directed::Digraph, import_adjacency_list, import_graph_properties,
    wt_directed::WTDigraph,
};

pub fn setup_digraph(filename: &str) -> Digraph<usize> {
    let (v_count, e_count) = import_graph_properties(filename);
    let adj: Vec<Vec<usize>> = import_adjacency_list(filename);
    let digraph: Digraph<usize> = Digraph::from_adjacency_list(v_count, e_count, adj); // temporary new2 to create Digraph with adj list
    digraph
}
pub fn setup_wtdigraph(filename: &str) -> WTDigraph<usize> {
    let (sequence, starting_indices) = create_sequence_and_bitmap(&import_adjacency_list(filename)); //creating sequence and bitmap
    let wtdigraph: WTDigraph<usize> =
        WTDigraph::from(sequence, RsVec::from_bit_vec(starting_indices)); // create WTDigraph using from(sequence, starting_indices)
    wtdigraph
}
