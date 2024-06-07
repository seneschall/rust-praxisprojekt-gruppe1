#[cfg(test)]
mod test {
    use vers_vecs::RsVec;
    use wt_graphs::{graph::{create_sequence_and_bitmap, import_adjacency_list, wt_graph::WTDigraph, Digraph}, traits::{Directed, Graph}};


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
    fn test_wtdigraph_from_file(){
        let filename = "tests/tinyDG.txt";
        let (sequence, starting_indices) = create_sequence_and_bitmap(&import_adjacency_list(filename));
        let wtdigraph: WTDigraph<u32,u32> = WTDigraph::from(sequence, starting_indices);
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3u32,0u32]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
    }
}
