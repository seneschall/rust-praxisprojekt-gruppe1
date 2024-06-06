#[cfg(test)]
mod test {
    use wt_graphs::{graph::Digraph, wt_graph::WTDigraph, Directed, Graph};

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
}
