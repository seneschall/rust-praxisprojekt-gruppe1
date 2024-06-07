#[cfg(test)]
mod test {
    use wt_graphs::{graph::{create_sequence_and_bitmap, import_adjacency_list, import_graph_properties, wt_graph::WTDigraph, Digraph}, traits::{Directed, Graph}};


    #[test]
    fn test_graph() {
        let filename = "tests/tinyDG.txt";
        let (e_count,v_count) = import_graph_properties(filename);
        let adj: Vec<Vec<u32>> = import_adjacency_list(filename); 
        let mut digraph : Digraph<u32,u32> = Digraph::new2(v_count, e_count,adj ); // temporary new2 to create Digraph with adj list
        assert_eq!(digraph.e_count(), 13);
        assert_eq!(digraph.v_count(), 22);
        assert_eq!(digraph.outgoing_edges(2), vec![3u32, 0u32]);
        assert_eq!(digraph.outgoing_edges(1), Vec::new());
        digraph.add_edge(1, 0);
        assert_eq!(digraph.e_count(), 14);
        assert_eq!(digraph.outgoing_edges(1), vec![0u32]);
        digraph.delete_edge(1,0);
        assert_eq!(digraph.e_count(),13);
        assert_eq!(digraph.outgoing_edges(1), Vec::new());

    }
    #[test]
    fn test_wtdigraph_from(){
        let filename = "tests/tinyDG.txt";
        let (sequence, starting_indices) = create_sequence_and_bitmap(&import_adjacency_list(filename)); //creating sequence and bitmap
        let wtdigraph: WTDigraph<u32,u32> = WTDigraph::from(sequence, starting_indices); // create WTDigraph using from(sequence, starting_indices)
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3u32,0u32]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
    #[test]
    fn test_wtdigraph_from_digraph(){
        let filename = "tests/tinyDG.txt";
        let (e_count,v_count) = import_graph_properties(filename);
        let adj: Vec<Vec<u32>> = import_adjacency_list(filename); 
        let digraph : Digraph<u32,u32> = Digraph::new2(v_count, e_count,adj ); // temporary new2 to create Digraph with adj list
        let wtdigraph = WTDigraph::from_digraph(digraph); // creating WTDigraph using from_digraph
        
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3u32,0u32]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
}