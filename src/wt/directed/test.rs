// use crate::graph::directed::Digraph;
// use crate::wt::directed::WTDigraph;

// use super::*;
// #[test]
// fn from_digraph() {
//     let dg = Digraph::new();
//     let wtdg = WTDigraph::from_digraph(dg.clone());
//     assert_eq!(dg.v_count(), wtdg.wt_adj_len);
//     assert_eq!(dg.e_count(), wtdg.e_count);
//     // todo
// }
// #[test]
// fn from() {
//     todo!()
// }
// #[test]
// fn add_vertex() {
//     // todo for committed changes; these tests only test uncommitted changes
//     let mut dg = Digraph::new();
//     dg.add_vertex(5);
//     dg.add_edge(0, 1); // 1 MAJOR if WTGraph has no edges, subtract overflow in qwt crate
//     let mut wtdg = WTDigraph::from_digraph(dg.clone());
//     wtdg.add_vertex(0);
//     assert_eq!(wtdg.wt_adj_len_updated, dg.v_count());
//     wtdg.add_vertex(1);
//     assert_eq!(wtdg.wt_adj_len_updated, dg.v_count());
//     wtdg.add_vertex(10);
//     assert_eq!(wtdg.wt_adj_len_updated + wtdg.deleted_vertices.len(), 11);
// }
// #[test]
// fn vertex_deleted() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(5);
//     let mut wtdg = WTDigraph::from_digraph(dg.clone());
//     wtdg.deleted_vertices.push(0);
//     assert_eq!(
//         wtdg.vertex_deleted(0),
//         true,
//         "Vertex 0 should be deleted but is not"
//     );
//     wtdg.deleted_vertices.push(4);
//     assert_eq!(
//         wtdg.vertex_deleted(4),
//         true,
//         "Vertex 4 should be deleted but is not"
//     );
// }
// #[test]
// fn delete_edge() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(5);
//     dg.add_edge(0, 1);
//     let mut wtdg = WTDigraph::from_digraph(dg.clone());
//     wtdg.delete_edge(0, 1);
//     assert_eq!(wtdg.adj_uncommitted.get(&0), Some(&vec![Edit::Add(1)]));
//     // wtdg.delete_edge(1,0); //will result in subtract with overflow
// }
// #[test]
// fn delete_vertex() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(5);
//     let mut wtdg = WTDigraph::from_digraph(dg.clone());
//     for i in 0..5 {
//         println!("{:?}", wtdg.deleted_vertices_uncommitted);
//         wtdg.delete_vertex(i);
//     }
//     assert_eq!(
//         wtdg.deleted_vertices_uncommitted,
//         vec![
//             Edit::Add(0),
//             Edit::Add(1),
//             Edit::Add(2),
//             Edit::Add(3),
//             Edit::Add(4)
//         ]
//     );
// }
// #[test]
// fn vertex_exists() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(5);
//     let mut wtdg = WTDigraph::from_digraph(dg.clone());
//     wtdg.deleted_vertices = vec![0, 1, 2, 3, 4];
//     for i in 0..5 {
//         assert_eq!(
//             wtdg.vertex_exists(i),
//             false,
//             "Vertex {i} is not deleted, but should be deleted"
//         );
//     }
//     wtdg.add_vertex(10);
//     assert!(wtdg.vertex_exists_updated(10));
// }
// // #[test]
// fn shrink() {
//     todo!()
// }
// #[test]
// fn outgoing_edges() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(10);
//     for i in 0..10 {
//         for j in 0..10 {
//             dg.add_edge(i, j);
//         }
//     }
//     let mut wtdg = WTDigraph::from_digraph(dg.clone());
//     for i in 0..10 {
//         // maybe order is different
//         assert_eq!(
//             dg.outgoing_edges(i),
//             wtdg.outgoing_edges(i),
//             "Outgoing edges from {i} doesn't match"
//         );
//     }
// }
// #[test]
// fn incoming_edges() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(10);
//     for i in 0..10 {
//         for j in 0..10 {
//             dg.add_edge(i, j);
//         }
//     }
//     let wtdg = WTDigraph::from_digraph(dg.clone());
//     for i in 0..10 {
//         assert_eq!(
//             dg.incoming_edges(i),
//             wtdg.incoming_edges(i),
//             "Incoming edges from {i} doesn't match"
//         );
//     }
// }
// #[test]
// fn delete_outgoing_edges() {
//     // // this test results in 'called `Option::unwrap()` on a `None` value'
//     // // thread 'graph::wt_directed::wtdigraph::delete_outgoing_edges' panicked at src\graph\wt_directed.rs:423:57:
//     let mut dg = Digraph::new();
//     dg.add_vertex(10);
//     for i in 0..10 {
//         for j in 0..10 {
//             dg.add_edge(i, j);
//         }
//     }
//     let mut wtdg = WTDigraph::from_digraph(dg);
//     for i in 0..10 {
//         wtdg.delete_outgoing_edges(i);
//     }
//     assert_eq!(wtdg.e_count_updated, 0);
//     for i in 0..10 {
//         assert_eq!(wtdg.outgoing_edges_updated(i), vec![]);
//     }
// }
// #[test]
// fn delete_incoming_edges() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(10);
//     for i in 0..10 {
//         for j in 0..10 {
//             dg.add_edge(i, j)
//         }
//     }
//     assert_eq!(dg.e_count, 100);
//     let mut wtdg = WTDigraph::from_digraph(dg);
//     assert_eq!(wtdg.e_count(), 100);
//     for i in 0..10 {
//         wtdg.delete_incoming_edges(i);
//     }
//     assert_eq!(wtdg.e_count_updated, 0);
//     for i in 0..10 {
//         assert_eq!(wtdg.incoming_edges_updated(i), vec![]);
//     }
// }
// #[test]
// fn append_vertex() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(5);
//     let mut wtdg = WTDigraph::from_digraph(dg);
//     let test = wtdg.append_vertex();
//     assert_eq!(
//         wtdg.has_uncommitted_edits, true,
//         "has_uncommitted_edits should be true, but is false"
//     );
//     assert_eq!(
//         wtdg.deleted_vertices_uncommitted.is_empty(),
//         true,
//         "has_uncommitted_deleted_vertices should be true, but is false"
//     );
//     assert_eq!(
//         wtdg.vertex_exists_updated(test),
//         true,
//         "vertex_exists(value) ; value = append_vertex return value"
//     );
// }
// #[test]
// fn add_edge() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(4);
//     let mut wtdg = WTDigraph::from_digraph(dg);
//     let mut test: HashMap<usize, Vec<Edit<usize>>> = HashMap::new();
//     for i in 0..5 {
//         for j in 0..5 {
//             wtdg.add_edge(i, j);
//         }
//         test.insert(
//             i,
//             vec![
//                 Edit::Add(0),
//                 Edit::Add(1),
//                 Edit::Add(2),
//                 Edit::Add(3),
//                 Edit::Add(4),
//             ],
//         );
//     }
//     assert_eq!(
//         wtdg.e_count_updated, 25,
//         "Not all edges added, e_count_updated is wrong"
//     );
//     assert_eq!(
//         wtdg.adj_uncommitted, test,
//         "HashMap uncommitted_adj is wrong"
//     );
// }
// #[test]
// fn commit_edits() {
//     todo!()
// }
// #[test]
// fn get_uncommitted_edits() {
//     todo!()
// }
// #[test]
// fn discard_edits() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(5);
//     dg.add_edge(0, 1);
//     let mut wtdg = WTDigraph::from_digraph(dg);
//     assert_eq!(wtdg.wt_adj_len_updated, 6);
//     assert_eq!(wtdg.e_count_updated, 1);
//     assert_eq!(wtdg.has_uncommitted_edits, false);
//     assert_eq!(wtdg.deleted_vertices.is_empty(), true);
//     assert_eq!(wtdg.adj_uncommitted.is_empty(), true);
//     assert_eq!(wtdg.deleted_vertices_uncommitted.is_empty(), true);
//     wtdg.add_vertex(15);
//     wtdg.add_edge(10, 5);
//     wtdg.delete_vertex(1);
//     assert_ne!(wtdg.wt_adj_len_updated, 6);
//     assert_ne!(wtdg.e_count_updated, 1);
//     assert_ne!(wtdg.has_uncommitted_edits, false);
//     assert_ne!(wtdg.adj_uncommitted.is_empty(), true);
//     assert_ne!(wtdg.deleted_vertices_uncommitted.is_empty(), true);
//     wtdg.discard_edits();
//     assert_eq!(wtdg.wt_adj_len_updated, 6);
//     assert_eq!(wtdg.e_count_updated, 1);
//     assert_eq!(wtdg.has_uncommitted_edits, false);
//     assert_eq!(wtdg.adj_uncommitted.is_empty(), true);
//     assert_eq!(wtdg.deleted_vertices_uncommitted.is_empty(), true);
// }
// #[test]
// fn vertex_exists_updated() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(4);
//     dg.add_edge(0, 1);
//     let mut wtdg = WTDigraph::from_digraph(dg);
//     for i in 0..5 {
//         assert_eq!(wtdg.vertex_exists_updated(i), true);
//         assert_eq!(wtdg.vertex_exists(i), true);
//     }
//     for i in 0..5 {
//         wtdg.delete_vertex(i);
//         assert_eq!(wtdg.vertex_exists_updated(i), false);
//         assert_eq!(wtdg.vertex_exists(i), true);
//     }
// }
// #[test]
// fn updated_outgoing_edges() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(4);
//     let mut wtdg = WTDigraph::from_digraph(dg.clone());
//     for i in 0..5 {
//         for j in 0..5 {
//             dg.add_edge(i, j);
//             wtdg.add_edge(i, j);
//         }
//     }
//     for i in 0..5 {
//         assert_eq!(wtdg.outgoing_edges_updated(i), dg.outgoing_edges(i));
//     }
// }
// #[test]
// fn updated_incoming_edges() {
//     let mut dg = Digraph::new();
//     dg.add_vertex(4);
//     dg.add_edge(0, 1);
//     let mut wtdg = WTDigraph::from_digraph(dg.clone());
//     for i in 0..5 {
//         for j in 0..5 {
//             dg.add_edge(i, j);
//             wtdg.add_edge(i, j);
//         }
//     }
//     for i in 0..5 {
//         for item in wtdg.incoming_edges_updated(i) {
//             assert_eq!(dg.incoming_edges(i).contains(&item), true);
//         }
//     }
// }
