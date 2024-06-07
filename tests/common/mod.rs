use wt_graphs::graph::{create_sequence_and_bitmap, import_adjacency_list, import_graph_properties, wt_graph::WTDigraph, Digraph};

pub fn setup_digraph(filename : &str) -> Digraph<u32,u32>{
    let (e_count,v_count) = import_graph_properties(filename);
    let adj: Vec<Vec<u32>> = import_adjacency_list(filename); 
    let digraph : Digraph<u32,u32> = Digraph::new2(v_count, e_count,adj ); // temporary new2 to create Digraph with adj list
    digraph
}
pub fn setup_wtdigraph(filename : &str) -> WTDigraph<u32,u32>{
    let (sequence, starting_indices) = create_sequence_and_bitmap(&import_adjacency_list(filename)); //creating sequence and bitmap
    let wtdigraph: WTDigraph<u32,u32> = WTDigraph::from(sequence, starting_indices); // create WTDigraph using from(sequence, starting_indices)
    wtdigraph
}