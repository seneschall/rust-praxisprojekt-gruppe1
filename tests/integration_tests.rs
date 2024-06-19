mod common;
#[cfg(test)]
mod test {

    use wt_graphs::{
        graph::{wt_directed::WTDigraph, Edit},
        traits::{Directed, Graph, GraphSearch},
    };

    use crate::common::{setup_digraph, setup_wtdigraph};

    #[test]
    fn test_directed_add_vertex() {
        let mut digraph = setup_digraph("tests/tinyDG.txt");
        let mut digraph2 = setup_digraph("tests/tinyDG.txt");
        digraph.add_vertex(0);
        assert_eq!(digraph.outgoing_edges(1), digraph2.outgoing_edges(0));
        assert_eq!(digraph.v_count(), digraph2.v_count() + 1);
        digraph.add_vertex(100);
        assert_eq!(digraph.v_count(), 101);
    }
    #[test]
    fn test_directed_delete_vertex() {
        let mut digraph = setup_digraph("tests/tinyDG.txt");
        let mut digraph2 = setup_digraph("tests/tinyDG.txt");
        for i in 0..digraph2.v_count() {
            assert_eq!(digraph.outgoing_edges(i), digraph2.outgoing_edges(i))
        }
        // digraph2.delete_vertex(0);
        for i in 0..digraph2.v_count() {
            assert_eq!(digraph.outgoing_edges(0), digraph2.outgoing_edges(0));
        }
    }
    #[test]
    fn wtdigraph_connected() {
        let wtdigraph = setup_wtdigraph("tests/tinyDG.txt");
        assert_eq!(wtdigraph.connected(0, 1), true);
        assert_eq!(wtdigraph.connected(8, 12), true);
        assert_eq!(wtdigraph.connected(1, 0), false);
        assert_eq!(wtdigraph.connected(0, 7), false);
        assert_eq!(wtdigraph.connected(12, 7), false);
    }
    #[test]
    fn test_wtgraph_incoming_edges() {
        let wtdigraph = setup_wtdigraph("tests/tinyDG.txt");

        assert_eq!(wtdigraph.incoming_edges(0), vec![2usize, 6]);
        assert_eq!(wtdigraph.incoming_edges(1), vec![0usize]);
        assert_eq!(wtdigraph.incoming_edges(2), vec![3usize, 4]);
        assert_eq!(wtdigraph.incoming_edges(3), vec![2usize, 4]);
        assert_eq!(wtdigraph.incoming_edges(4), vec![5usize, 6, 11]);
        assert_eq!(wtdigraph.incoming_edges(5), vec![0usize, 3]);
        assert_eq!(wtdigraph.incoming_edges(6), vec![7usize, 8]);
        assert_eq!(wtdigraph.incoming_edges(7), Vec::new());
        assert_eq!(wtdigraph.incoming_edges(8), vec![6usize]);
        assert_eq!(wtdigraph.incoming_edges(9), vec![6usize, 7, 12]);
        assert_eq!(wtdigraph.incoming_edges(10), vec![9usize]);
        assert_eq!(wtdigraph.incoming_edges(11), vec![9usize]);
        assert_eq!(wtdigraph.incoming_edges(12), vec![10usize, 11]);
    }
    #[test]
    fn test_graph() {
        let mut digraph = setup_digraph("tests/tinyDG.txt");
        assert_eq!(digraph.v_count(), 13);
        assert_eq!(digraph.e_count(), 22);
        assert_eq!(digraph.outgoing_edges(2), vec![3usize, 0]);
        assert_eq!(digraph.outgoing_edges(1), Vec::new());
        digraph.add_edge(1, 0);
        assert_eq!(digraph.e_count(), 23);
        assert_eq!(digraph.outgoing_edges(1), vec![0usize]);
        digraph.delete_edge(1, 0);
        assert_eq!(digraph.e_count(), 22);
        assert_eq!(digraph.outgoing_edges(1), Vec::new());
    }
    #[test]
    fn test_wtdigraph_from() {
        let wtdigraph = setup_wtdigraph("tests/tinyDG.txt");
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3usize, 0]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
    #[test]
    fn test_wtdigraph_from_digraph() {
        let digraph = setup_digraph("tests/tinyDG.txt");

        let wtdigraph = WTDigraph::from_digraph(digraph); // creating WTDigraph using from_digraph

        assert_eq!(wtdigraph.outgoing_edges(2), vec![3usize, 0]);
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
}
