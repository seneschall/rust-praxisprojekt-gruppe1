mod common;
#[cfg(test)]
mod test {

    use wt_graphs::{
        graph::wt_directed::WTDigraph,
        traits::{Directed, Graph},
    };

    use crate::common::{setup_digraph, setup_wtdigraph};

    #[test]
    fn test_graph() {
        let mut digraph = setup_digraph("tests/tinyDG.txt");
        assert_eq!(digraph.v_count(), 13);
        assert_eq!(digraph.e_count(), 22);
        assert_eq!(digraph.outgoing_edges(2), vec![3u32, 0u32]);
        assert_eq!(digraph.outgoing_edges(1), Vec::new());
        digraph.add_edge(1, 0);
        assert_eq!(digraph.e_count(), 23);
        assert_eq!(digraph.outgoing_edges(1), vec![0u32]);
        digraph.delete_edge(1, 0);
        assert_eq!(digraph.e_count(), 22);
        assert_eq!(digraph.outgoing_edges(1), Vec::new());
    }
    #[test]
    fn test_wtdigraph_from() {
        let wtdigraph = setup_wtdigraph("tests/tinyDG.txt");
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3u32, 0u32]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
    #[test]
    fn test_wtdigraph_from_digraph() {
        let digraph = setup_digraph("tests/tinyDG.txt");

        let wtdigraph = WTDigraph::from_digraph(digraph); // creating WTDigraph using from_digraph

        assert_eq!(wtdigraph.outgoing_edges(2), vec![3u32, 0u32]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
    #[test]
    fn test_compare_outgoing_edges_wtdigraph_with_digraph() {
        let filename = "tests/mediumDG.txt";
        let digraph = setup_digraph(filename);
        let wtdigraph = setup_wtdigraph(filename);
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.outgoing_edges(i), wtdigraph.outgoing_edges(i));
        }
        assert_eq!(digraph.v_count(), wtdigraph.v_count());
        assert_eq!(digraph.e_count(), wtdigraph.e_count());
    }
    #[test]
    fn test_compare_incoming_edges_wtdigraph_with_digraph() { // incoming edges not yet implemented
                                                              // let digraph = setup_digraph("tests/tinyDG.txt");
                                                              // let wtdigraph = setup_wtdigraph("tests/tinyDG.txt");
                                                              // for i in 0..digraph.v_count(){
                                                              //     assert_eq!(digraph.incoming_edges(i), wtdigraph.incoming_edges(i));
                                                              // }
    }
}
