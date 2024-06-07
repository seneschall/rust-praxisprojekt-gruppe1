#[cfg(test)]
mod test {
    use wt_graphs::{graph::{create_sequence_and_bitmap, import_adjacency_list, import_graph_properties, wt_graph::WTDigraph, Digraph}, traits::{Directed, Graph}};


    #[test]
    fn test_wtdigraph() {
        //let mut graph: WTDigraph<u32, u32> = WTDigraph::from_digraph(Digraph::new(10)); // creates new graph
        //graph.add_edge(3, 2);
        //graph.add_edge(5, 0);
        let mut graph: Digraph<u32, u32> = Digraph::new(10); // creates new graph
        graph.add_edge(3, 2);
        graph.add_edge(5, 0);
        let digraph: WTDigraph<u32, u32> = WTDigraph::from_digraph(graph);
        assert_eq!(digraph.outgoing_edges(3), vec![2u32]);
        assert_eq!(digraph.outgoing_edges(5), vec![0u32]);
        //assert_eq!(graph.e_count(), 2);
        //assert_eq!(graph.v_count(), 10);
        //graph.delete_edge(3,2);  // delete not yet implemented
        //graph.delete_edge(5,0);
        //assert_ne!(graph.outgoing_edges(3), vec![2u32]);
        //assert_ne!(graph.outgoing_edges(5), vec![0u32]);
        //assert_eq!(graph.e_count(), 0);
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
        let (_e_count,v_count) = import_graph_properties(filename); //read v_count from file, e_count is not used
        let adj: Vec<Vec<u32>> = import_adjacency_list(filename); 
        let digraph : Digraph<u32,u32> = Digraph::new2(v_count,adj ); // temporary new2 to create Digraph with adj list
        let wtdigraph = WTDigraph::from_digraph(digraph); // creating WTDigraph using from_digraph
        
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3u32,0u32]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
}