use crate::graph::labeled_directed::LabeledDigraph;
use crate::traits::{Directed, Graph, Unweighted};
use std::collections::HashMap;

#[test]
fn new() {
    let ldg: LabeledDigraph<String> = LabeledDigraph::new();
    assert!(ldg.dg.adj.is_empty());
    assert_eq!(ldg.dg.adj_len, 0);
    assert_eq!(ldg.dg.e_count, 0);
    assert!(ldg.dg.deleted_vertices.is_empty());
    assert!(ldg.hashmap_labels_vertex.is_empty());
    assert!(ldg.vec_vertex_labels.is_empty());
}
#[test]
fn from_adjacency_list() {
    let v_count = 10;
    let e_count = 10;
    let adj = vec![vec![0]; 10];
    let mut testhashmap: HashMap<String, usize> = HashMap::new();
    let mut labels: Vec<String> = Vec::new();
    for i in 0..10 {
        labels.push(i.to_string());
        testhashmap.insert(i.to_string(), i);
    }
    let ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(v_count, e_count, adj.clone(), labels.clone());
    assert_eq!(ldg.v_count(), v_count);
    assert_eq!(ldg.e_count(), e_count);
    assert_eq!(ldg.dg.adj, adj);
    assert!(ldg.dg.deleted_vertices.is_empty());
    assert_eq!(ldg.vec_vertex_labels, labels);
    assert_eq!(ldg.hashmap_labels_vertex, testhashmap);
}
#[test]
fn add_vertex() {
    let mut testhashmap: HashMap<String, usize> = HashMap::new();
    let mut ldg: LabeledDigraph<String> = LabeledDigraph::new();
    for i in 0..9 {
        assert_eq!(i, ldg.add_vertex(i.to_string()));
        assert_eq!(ldg.vec_vertex_labels[i], i.to_string());
        assert_eq!(testhashmap.insert(i.to_string(), i), None);
    }
    assert_eq!(ldg.hashmap_labels_vertex, testhashmap);
}
#[test]
fn add_edge() {
    let mut labels: Vec<String> = Vec::new();
    for i in 0..5 {
        labels.push(i.to_string());
    }
    let mut ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(5, 0, vec![vec![]; 5], labels.clone());
    for i in 0..ldg.v_count() {
        ldg.add_edge(i.to_string(), 0.to_string());
    }
    assert_eq!(ldg.dg.adj, vec![vec![0]; 5]);
    assert_eq!(ldg.e_count(), 5);
}
#[test]
fn vertex_exists() {
    let mut labels: Vec<String> = Vec::new();
    for i in 0..5 {
        labels.push(i.to_string());
    }
    let mut ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(5, 0, vec![vec![]; 5], labels.clone());
    ldg.dg.deleted_vertices.insert(0, true);
    assert_eq!(ldg.vertex_exists(0.to_string()), false);
    assert_eq!(ldg.vertex_exists(1.to_string()), true);
    ldg.dg.deleted_vertices = HashMap::new();
    assert_eq!(ldg.vertex_exists(0.to_string()), true);
    ldg.dg.adj_len = 0;
    assert_eq!(ldg.vertex_exists(1.to_string()), false);
}
#[test]
fn delete_vertex() {
    let mut labels: Vec<String> = Vec::new();
    for i in 0..5 {
        labels.push(i.to_string());
    }
    let mut ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(5, 0, vec![vec![]; 5], labels.clone());

    ldg.delete_vertex(0.to_string());
    assert_eq!(ldg.dg.deleted_vertices.contains_key(&0), true);
    assert_eq!(ldg.v_count(), 4);
}
#[test]
fn delete_edge() {
    let mut labels: Vec<String> = Vec::new();
    for i in 0..5 {
        labels.push(i.to_string());
    }
    let mut ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(5, 5, vec![vec![1]; 5], labels.clone());

    assert_eq!(
        ldg.dg.adj,
        vec![vec![1], vec![1], vec![1], vec![1], vec![1]]
    );
    ldg.delete_edge(0.to_string(), 1.to_string());
    assert_eq!(ldg.dg.adj, vec![vec![], vec![1], vec![1], vec![1], vec![1]]);
    assert_eq!(ldg.e_count(), 4);
}
#[test]
fn outgoing_edges() {
    let mut labels: Vec<String> = Vec::new();
    for i in 0..5 {
        labels.push(i.to_string());
    }
    let mut ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(5, 5, vec![vec![1]; 5], labels.clone());

    for i in 0..ldg.v_count() {
        assert_eq!(ldg.outgoing_edges(i.to_string()), vec![1.to_string()]);
    }
    ldg.dg.e_count = 25;
    ldg.dg.adj = vec![vec![0, 1, 2, 3, 4]; 5];
    for i in 0..ldg.v_count() {
        assert_eq!(
            ldg.outgoing_edges(i.to_string()),
            vec![
                0.to_string(),
                1.to_string(),
                2.to_string(),
                3.to_string(),
                4.to_string()
            ]
        );
    }
}
#[test]
fn incoming_edges() {
    let test: Vec<String> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    for i in 0..5 {
        labels.push(i.to_string());
    }
    let mut ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(5, 25, vec![vec![0, 1, 2, 3, 4]; 5], labels.clone());

    for i in 0..ldg.v_count() {
        assert_eq!(
            ldg.incoming_edges(i.to_string()),
            vec![
                0.to_string(),
                1.to_string(),
                2.to_string(),
                3.to_string(),
                4.to_string()
            ]
        );
    }
    ldg.dg.adj = vec![vec![1]; 5];
    for i in 0..ldg.v_count() {
        if i == 1 {
            assert_eq!(
                ldg.incoming_edges(i.to_string()),
                vec![
                    0.to_string(),
                    1.to_string(),
                    2.to_string(),
                    3.to_string(),
                    4.to_string()
                ]
            );
        } else {
            assert_eq!(ldg.incoming_edges(i.to_string()), test);
        }
    }
}
#[test]
fn delete_outgoing_edges() {
    let mut labels: Vec<String> = Vec::new();
    for i in 0..5 {
        labels.push(i.to_string());
    }
    let mut ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(5, 5, vec![vec![1]; 5], labels.clone());

    for i in 0..ldg.v_count() {
        assert_eq!(ldg.dg.adj[i], vec![1]);
    }
    for i in 0..ldg.v_count() {
        ldg.delete_outgoing_edges(i.to_string());
        assert_eq!(ldg.dg.adj[i], vec![]);
    }
    assert_eq!(ldg.e_count(), 0);
    ldg.dg.e_count = 25;
    ldg.dg.adj = vec![vec![0, 1, 2, 3, 4]; 5];
    for i in 0..ldg.v_count() {
        assert_eq!(ldg.dg.adj[i], vec![0, 1, 2, 3, 4]);
    }
    for i in 0..ldg.v_count() {
        ldg.delete_outgoing_edges(i.to_string());
        assert_eq!(ldg.dg.adj[i], vec![]);
    }
    assert_eq!(ldg.e_count(), 0);
}
#[test]
fn delete_incoming_edges() {
    let mut labels: Vec<String> = Vec::new();
    for i in 0..5 {
        labels.push(i.to_string());
    }
    let mut ldg: LabeledDigraph<String> =
        LabeledDigraph::from_adjacency_list(5, 5, vec![vec![1]; 5], labels.clone());

    for i in 0..ldg.v_count() {
        ldg.delete_incoming_edges(i.to_string());
    }
    assert_eq!(ldg.e_count(), 0);
    assert_eq!(ldg.dg.adj, vec![vec![]; 5]);
}
