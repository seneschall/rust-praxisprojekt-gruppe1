
mod common;
#[cfg(test)]
mod test {

    use wt_graphs::{graph::wt_graph::WTDigraph, traits::{Directed, Graph}};

    use crate::common::{setup_digraph, setup_wtdigraph};



    #[test]
    fn test_graph() {
        let mut digraph = setup_digraph("tests/tinyDG.txt");
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
        let wtdigraph = setup_wtdigraph("tests/tinyDG.txt");
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3u32,0u32]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
    #[test]
    fn test_wtdigraph_from_digraph(){
        let digraph = setup_digraph("tests/tinyDG.txt");
        let wtdigraph = WTDigraph::from_digraph(digraph); // creating WTDigraph using from_digraph
        
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3u32,0u32]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
}