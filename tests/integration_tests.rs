mod common;
use crate::common::*;
use wt_graphs::traits::*;
use wt_graphs::*;
#[cfg(test)]
mod test {

    use graph::{directed::Digraph, labeled_directed::LabeledDigraph, undirected::UGraph};
    use wt::{directed::WTDigraph, labeled_directed::LabeledWTDigraph, undirected::WTUGraph};

    use super::*;

    #[test]
    fn undirected() {
        let mut ug: UGraph = UGraph::new();
        let mut wtug: WTUGraph = WTUGraph::from_ugraph(UGraph::new());

        assert_eq!(ug.v_count(), wtug.v_count());
        for i in 0..10 {
            ug.add_vertex(i);
            wtug.add_vertex(i);
            assert_eq!(ug.v_count(), wtug.v_count_updated());
        }
        for i in 0..9 {
            ug.add_edge(i, i + 1);
            assert!(ug.edge_exists(i, i + 1));
            wtug.add_edge(i, i + 1);
            assert!(wtug.edge_exists_updated(i, i + 1));
        }
        for i in 0..10 {
            assert_eq!(ug.edges(i), wtug.edges_updated(i));
        }
        wtug.commit_edits();
        assert_eq!(ug.v_count(), wtug.v_count());
        for i in 0..10 {
            assert_eq!(ug.edges(i), wtug.edges(i), "edges");
        }
        assert_eq!(ug.e_count(), wtug.e_count());
        for i in 0..9 {
            ug.delete_edge(i, i + 1);
            assert!(!ug.edge_exists(i, i + 1));
            wtug.delete_edge(i, i + 1);
            assert!(!wtug.edge_exists_updated(i, i + 1));
        }
        wtug.commit_edits();
        assert_eq!(ug.e_count(), wtug.e_count());
        assert_eq!(ug.v_count(), wtug.v_count());
        for i in 0..10 {
            ug.delete_vertex(i);
            assert_eq!(ug.vertex_exists(i), false);
            wtug.delete_vertex(i);
            assert_eq!(wtug.vertex_exists_updated(i), false);
        }
        assert_eq!(ug.v_count(), wtug.v_count_updated(), "v_count failed");

        for i in 0..10 {
            ug.add_vertex(i);
            wtug.add_vertex(i);
            assert_eq!(ug.v_count(), wtug.v_count_updated(), "v_count failed");
            assert_eq!(ug.edges(i), wtug.edges_updated(i));
        }
        ug.add_vertex(500);
        wtug.add_vertex(500);
        assert_eq!(ug.v_count(), wtug.v_count_updated());
        ug.delete_vertex(500);
        wtug.delete_vertex(500);
        assert_eq!(ug.v_count(), 500);
        assert_eq!(ug.v_count(), wtug.v_count_updated());
        assert_eq!(ug.append_vertex(), wtug.append_vertex());
        assert_eq!(ug.v_count(), wtug.v_count_updated());
        for i in 0..10 {
            for j in 0..10 {
                ug.add_edge(i, j);
                wtug.add_edge(i, j);
            }
            println!("{i}");
            println!("ug {:?}", ug.edges(i));
            println!("wtug {:?}", wtug.edges_updated(i));
            assert_eq!(ug.edges(i).len(), wtug.edges_updated(i).len());
            for item in ug.edges(i) {
                assert!(wtug.edges_updated(i).contains(&item));
            }
            assert_eq!(ug.edges(i).len(), wtug.edges_updated(i).len());
        }
        assert_eq!(ug.v_count(), wtug.v_count_updated(), "bevor commit");
        wtug.commit_edits();
        assert_eq!(ug.v_count(), wtug.v_count());
        assert_eq!(ug.v_count(), wtug.v_count_updated(), "after commit");
        for i in 0..10 {
            assert_eq!(ug.edges(i).len(), wtug.edges_updated(i).len());
            assert_eq!(ug.edges(i).len(), wtug.edges(i).len());
            for item in ug.edges(i) {
                assert!(wtug.edges(i).contains(&item));
                assert!(wtug.edges_updated(i).contains(&item));
            }
        }
        for i in 0..10 {
            assert_eq!(ug.edges(i).len(), wtug.edges_updated(i).len());
            ug.delete_edges_from(i);
            wtug.delete_edges_from(i);
            assert_eq!(ug.edges(i).len(), wtug.edges_updated(i).len());
        }
        assert_eq!(ug.v_count(), wtug.v_count_updated());
        for i in 0..500 {
            ug.delete_vertex(i);
            wtug.delete_vertex(i);
            assert_eq!(ug.v_count(), wtug.v_count_updated());
        }
    }
    #[test]
    fn labeled_graph_digraph_and_labeled_wt_digraph() {
        let mut ldg: LabeledDigraph<String> = LabeledDigraph::new();
        let mut wtldg: LabeledWTDigraph<String> =
            LabeledWTDigraph::from_labeled_digraph(LabeledDigraph::new());
        assert_eq!(ldg.v_count(), wtldg.v_count());
        assert_eq!(ldg.e_count(), wtldg.e_count());
        for i in 0..10 {
            ldg.add_vertex(i.to_string());
            wtldg.add_vertex(i.to_string());
            assert_eq!(
                ldg.vertex_exists(i.to_string()),
                wtldg.vertex_exists_updated(i.to_string())
            );
            assert_eq!(ldg.v_count(), wtldg.v_count_updated());
        }
        for i in 0..10 {
            ldg.delete_vertex(i.to_string());
            wtldg.delete_vertex(i.to_string());
            assert_eq!(
                ldg.vertex_exists(i.to_string()),
                wtldg.vertex_exists_updated(i.to_string())
            );
            assert_eq!(ldg.v_count(), wtldg.v_count_updated());
        }
        for i in 0..10 {
            ldg.add_vertex(i.to_string());
            wtldg.add_vertex(i.to_string());
            assert_eq!(
                ldg.vertex_exists(i.to_string()),
                wtldg.vertex_exists_updated(i.to_string())
            );
            assert_eq!(ldg.v_count(), wtldg.v_count_updated());
        }
        for i in 20..30 {
            ldg.edit_label((i - 20).to_string(), i.to_string());
            wtldg.edit_label((i - 20).to_string(), i.to_string());
            assert_eq!(
                ldg.vertex_exists(i.to_string()),
                wtldg.vertex_exists_updated(i.to_string())
            );
            assert_eq!(ldg.v_count(), wtldg.v_count_updated());
        }
    }
    #[test]
    fn graph_digraph_and_wt_digraph() {
        let mut dg = Digraph::new();
        let mut wtdg = WTDigraph::from_digraph(dg.clone());
        assert_eq!(dg.v_count(), wtdg.v_count());
        for i in 0..10 {
            dg.add_vertex(i);
            wtdg.add_vertex(i);
            assert_eq!(dg.v_count(), wtdg.v_count_updated());
        }
        for i in 0..9 {
            dg.add_edge(i, i + 1);
            assert!(dg.edge_exists(i, i + 1));
            wtdg.add_edge(i, i + 1);
            assert!(wtdg.edge_exists_updated(i, i + 1));
        }
        for i in 0..10 {
            assert_eq!(dg.outgoing_edges(i), wtdg.outgoing_edges_updated(i));
            assert_eq!(dg.incoming_edges(i), wtdg.incoming_edges_updated(i));
        }
        wtdg.commit_edits();
        assert_eq!(dg.v_count(), wtdg.v_count());
        for i in 0..10 {
            assert_eq!(
                dg.outgoing_edges(i),
                wtdg.outgoing_edges(i),
                "outgoing_edges"
            );
            assert_eq!(
                dg.incoming_edges(i),
                wtdg.incoming_edges(i),
                "incoming_edges"
            );
        }
        assert_eq!(dg.e_count(), wtdg.e_count());
        for i in 0..9 {
            dg.delete_edge(i, i + 1);
            assert!(!dg.edge_exists(i, i + 1));
            wtdg.delete_edge(i, i + 1);
            assert!(!wtdg.edge_exists_updated(i, i + 1));
        }
        wtdg.commit_edits();
        assert_eq!(dg.e_count(), wtdg.e_count());
        assert_eq!(dg.v_count(), wtdg.v_count());
        for i in 0..10 {
            dg.delete_vertex(i);
            assert_eq!(dg.vertex_exists(i), false);
            wtdg.delete_vertex(i);
            assert_eq!(wtdg.vertex_exists_updated(i), false);
        }
        assert_eq!(dg.v_count(), wtdg.v_count_updated(), "v_count failed");

        for i in 0..10 {
            dg.add_vertex(i);
            wtdg.add_vertex(i);
            assert_eq!(dg.v_count(), wtdg.v_count_updated(), "v_count failed");
            assert_eq!(dg.outgoing_edges(i), wtdg.outgoing_edges_updated(i));
            assert_eq!(dg.incoming_edges(i), wtdg.incoming_edges_updated(i));
            assert_eq!(dg.incoming_edges(i), dg.outgoing_edges(i));
        }
        dg.add_vertex(500);
        wtdg.add_vertex(500);
        assert_eq!(dg.v_count(), wtdg.v_count_updated());
        dg.delete_vertex(500);
        wtdg.delete_vertex(500);
        assert_eq!(dg.v_count(), 500);
        assert_eq!(dg.v_count(), wtdg.v_count_updated());
        assert_eq!(dg.append_vertex(), wtdg.append_vertex());
        assert_eq!(dg.v_count(), wtdg.v_count_updated());
        for i in 0..10 {
            for j in 0..10 {
                dg.add_edge(i, j);
                wtdg.add_edge(i, j);
            }
            assert_eq!(
                dg.outgoing_edges(i).len(),
                wtdg.outgoing_edges_updated(i).len()
            );
            for item in dg.outgoing_edges(i) {
                assert!(wtdg.outgoing_edges_updated(i).contains(&item));
            }
            assert_eq!(
                dg.incoming_edges(i).len(),
                wtdg.incoming_edges_updated(i).len()
            );
            for item in dg.incoming_edges(i) {
                assert!(wtdg.incoming_edges_updated(i).contains(&item));
            }
        }
        assert_eq!(dg.v_count(), wtdg.v_count_updated(), "bevor commit");
        wtdg.commit_edits();
        assert_eq!(dg.v_count(), wtdg.v_count());
        assert_eq!(dg.v_count(), wtdg.v_count_updated(), "after commit");
        for i in 0..10 {
            assert_eq!(
                dg.outgoing_edges(i).len(),
                wtdg.outgoing_edges_updated(i).len()
            );
            assert_eq!(dg.outgoing_edges(i).len(), wtdg.outgoing_edges(i).len());
            for item in dg.outgoing_edges(i) {
                assert!(wtdg.outgoing_edges(i).contains(&item));
                assert!(wtdg.outgoing_edges_updated(i).contains(&item));
            }
            assert_eq!(
                dg.incoming_edges(i).len(),
                wtdg.incoming_edges_updated(i).len()
            );
            assert_eq!(dg.incoming_edges(i).len(), wtdg.incoming_edges(i).len());
            for item in dg.incoming_edges(i) {
                assert!(wtdg.incoming_edges(i).contains(&item));
                assert!(wtdg.incoming_edges_updated(i).contains(&item));
            }
        }
        for i in 0..10 {
            assert_eq!(
                dg.incoming_edges(i).len(),
                wtdg.incoming_edges_updated(i).len()
            );
            assert_eq!(
                dg.outgoing_edges(i).len(),
                wtdg.outgoing_edges_updated(i).len()
            );
            dg.delete_incoming_edges(i);
            wtdg.delete_incoming_edges(i);
            assert_eq!(
                dg.incoming_edges(i).len(),
                wtdg.incoming_edges_updated(i).len()
            );
            assert_eq!(
                dg.outgoing_edges(i).len(),
                wtdg.outgoing_edges_updated(i).len()
            );
        }
        assert_eq!(dg.v_count(), wtdg.v_count_updated());
        for i in 0..500 {
            dg.delete_vertex(i);
            wtdg.delete_vertex(i);
            assert_eq!(dg.v_count(), wtdg.v_count_updated());
        }
    }
    #[test]
    fn test_labeled_digraph() {
        let mut ldg: LabeledDigraph<&str> = LabeledDigraph::new();
        let dg: Digraph = Digraph::new();
        let index = ldg.add_vertex("2");
        assert_eq!(ldg.v_count(), 1);
        ldg.add_edge("2", "2");
        for item in ldg.outgoing_edges("2") {
            assert_eq!(item, "2");
        }
        ldg.add_vertex("3");
        ldg.add_edge("2", "3");
        ldg.add_edge("3", "2");
        ldg.delete_vertex("2");
        assert_eq!(ldg.e_count(), 0);
        ldg.add_vertex("2");
        assert_eq!(ldg.v_count(), 2);
        ldg.add_edge("2", "3");

        for item in ldg.outgoing_edges("2") {
            assert_ne!(item, "2");
        }
        // assert_eq!(1,2);
    }
    #[test]
    fn test_digraph_tinyDG() {
        let mut digraph = setup_dg("tests/tinyDG.txt");
        //check if v_count and e_count are correct
        assert_eq!(digraph.e_count(), 22);
        assert_eq!(digraph.v_count(), 13);
        //check if all outgoing edges are correct
        assert_eq!(digraph.outgoing_edges(0), vec![1, 5]);
        assert_eq!(digraph.outgoing_edges(1), vec![]);
        assert_eq!(digraph.outgoing_edges(2), vec![3, 0]);
        assert_eq!(digraph.outgoing_edges(3), vec![2, 5]);
        assert_eq!(digraph.outgoing_edges(4), vec![2, 3]);
        assert_eq!(digraph.outgoing_edges(5), vec![4]);
        assert_eq!(digraph.outgoing_edges(6), vec![0, 8, 4, 9]);
        assert_eq!(digraph.outgoing_edges(7), vec![9, 6]);
        assert_eq!(digraph.outgoing_edges(8), vec![6]);
        assert_eq!(digraph.outgoing_edges(9), vec![10, 11]);
        assert_eq!(digraph.outgoing_edges(10), vec![12]);
        assert_eq!(digraph.outgoing_edges(11), vec![12, 4]);
        assert_eq!(digraph.outgoing_edges(12), vec![9]);
        // check if all incoming edges are correct
        assert_eq!(digraph.incoming_edges(0), vec![2, 6]);
        assert_eq!(digraph.incoming_edges(1), vec![0]);
        assert_eq!(digraph.incoming_edges(2), vec![3, 4]);
        assert_eq!(digraph.incoming_edges(3), vec![2, 4]);
        assert_eq!(digraph.incoming_edges(4), vec![5, 6, 11]);
        assert_eq!(digraph.incoming_edges(5), vec![0, 3]);
        assert_eq!(digraph.incoming_edges(6), vec![7, 8]);
        assert_eq!(digraph.incoming_edges(7), vec![]);
        assert_eq!(digraph.incoming_edges(8), vec![6]);
        assert_eq!(digraph.incoming_edges(9), vec![6, 7, 12]);
        assert_eq!(digraph.incoming_edges(10), vec![9]);
        assert_eq!(digraph.incoming_edges(11), vec![9]);
        assert_eq!(digraph.incoming_edges(12), vec![10, 11]);
        // for i in 0..digraph.v_count() {
        //     println!(
        //         "assert_eq!(digraph.incoming_edges({i}), vec!{:?});",
        //         digraph.incoming_edges(i)
        //     );
        // }
    }

    #[test]
    fn test_directed_add_vertex() {
        let mut digraph = setup_dg("tests/tinyDG.txt");
        let mut digraph2 = setup_dg("tests/tinyDG.txt");
        digraph.add_vertex(0);
        assert_eq!(digraph.v_count(), digraph2.v_count());
        digraph.add_vertex(100);
        assert_eq!(digraph.v_count(), 101);
    }
    #[test]
    fn test_directed_delete_vertex() {
        let mut digraph = setup_dg("tests/tinyDG.txt");
        let mut digraph2 = setup_dg("tests/tinyDG.txt");
        for i in 0..digraph2.v_count() {
            assert_eq!(digraph.outgoing_edges(i), digraph2.outgoing_edges(i))
        }
        // digraph2.delete_vertex(0);
        for i in 0..digraph2.v_count() {
            assert_eq!(digraph.outgoing_edges(0), digraph2.outgoing_edges(0));
        }
    }
    // #[test]
    // fn wtdigraph_connected() {
    //     let wtdigraph = setup_wtdg("tests/tinyDG.txt");
    //     assert_eq!(wtdigraph.connected(0, 1), true);
    //     assert_eq!(wtdigraph.connected(8, 12), true);
    //     assert_eq!(wtdigraph.connected(1, 0), false);
    //     assert_eq!(wtdigraph.connected(0, 7), false);
    //     assert_eq!(wtdigraph.connected(12, 7), false);
    // }
    #[test]
    fn test_wtgraph_incoming_edges() {
        let wtdigraph = setup_wtdg("tests/tinyDG.txt");

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
        let mut digraph = setup_dg("tests/tinyDG.txt");
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
        let wtdigraph = setup_wtdg("tests/tinyDG.txt");
        assert_eq!(wtdigraph.outgoing_edges(2), vec![3usize, 0]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
    #[test]
    fn test_wtdigraph_from_digraph() {
        let digraph = setup_dg("tests/tinyDG.txt");

        let wtdigraph = WTDigraph::from_digraph(digraph); // creating WTDigraph using from_digraph

        assert_eq!(wtdigraph.outgoing_edges(2), vec![3usize, 0]);
        assert_eq!(wtdigraph.outgoing_edges(1), Vec::new());
        assert_eq!(wtdigraph.e_count(), 22);
        assert_eq!(wtdigraph.v_count(), 13);
    }
    #[test]
    fn test_compare_outgoing_edges_wtdigraph_with_digraph() {
        let filename = "tests/mediumDG.txt";
        let digraph = setup_dg(filename);
        let wtdigraph = setup_wtdg(filename);
        for i in 0..digraph.v_count() {
            assert_eq!(digraph.outgoing_edges(i), wtdigraph.outgoing_edges(i));
        }
        assert_eq!(digraph.v_count(), wtdigraph.v_count());
        assert_eq!(digraph.e_count(), wtdigraph.e_count());
    }
}
