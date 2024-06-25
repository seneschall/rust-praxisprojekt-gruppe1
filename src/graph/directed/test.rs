use crate::graph::directed::Digraph;
use crate::traits::{Directed, Graph, UnLabeled, Unweighted};

#[test]
fn new() {
    let digraph = Digraph::new();
    assert!(digraph.adj.is_empty());
    assert_eq!(digraph.v_count, 0);
    assert_eq!(digraph.e_count, 0);
    assert!(digraph.deleted_vertices.is_empty());
}
#[test]
fn from_adjacency_list() {
    let v_count = 10;
    let e_count = 5;
    let adj = vec![vec![0]; 10];
    let digraph = Digraph::from_adjacency_list(v_count, e_count, adj.clone());
    assert_eq!(digraph.v_count(), v_count);
    assert_eq!(digraph.e_count(), e_count);
    assert_eq!(digraph.adj, adj);
    assert!(digraph.deleted_vertices.is_empty());
}
#[test]
fn append_vertex() {
    let mut digraph = Digraph::new();
    for i in 0..9 {
        assert_eq!(i, digraph.append_vertex())
    }
}
#[test]
fn add_edge() {
    let mut digraph = Digraph::from_adjacency_list(5, 0, vec![vec![]; 5]);
    for i in 0..digraph.v_count() {
        digraph.add_edge(i, 0);
    }
    assert_eq!(digraph.adj, vec![vec![0]; 5]);
    assert_eq!(digraph.e_count(), 5);
}
#[test]
fn add_vertex() {
    let mut digraph = Digraph::new();
    for i in 0..9 {
        assert_eq!(digraph.add_vertex(i), i);
    }
    assert_eq!(digraph.add_vertex(500), 500);
    assert_eq!(digraph.v_count(), 501);
    digraph.add_vertex(0);
    assert_eq!(digraph.v_count(), 501);
}
#[test]
fn vertex_exists() {
    let mut digraph = Digraph::new();
    digraph.v_count = 2;
    digraph.adj = vec![vec![]; 2];
    digraph.deleted_vertices = vec![0];
    assert_eq!(digraph.vertex_exists(0), false);
    assert_eq!(digraph.vertex_exists(1), true);
    digraph.deleted_vertices = vec![];
    assert_eq!(digraph.vertex_exists(0), true);
    digraph.v_count = 0;
    assert_eq!(digraph.vertex_exists(1), false);
}
#[test]
fn delete_vertex() {
    let mut digraph = Digraph::from_adjacency_list(5, 0, vec![vec![]; 5]);
    digraph.delete_vertex(0);
    assert_eq!(digraph.deleted_vertices, vec![0]);
    assert_eq!(digraph.v_count(), 4);
}
#[test]
fn vertex_deleted() {
    let mut digraph = Digraph::from_adjacency_list(5, 0, vec![vec![]; 5]);
    assert_eq!(digraph.vertex_deleted(0), false);
    digraph.deleted_vertices = vec![0];
    assert_eq!(digraph.vertex_deleted(0), true);
    assert_eq!(digraph.vertex_deleted(1), false);
    digraph.deleted_vertices = vec![0, 1];
    assert_eq!(digraph.vertex_deleted(1), true);
}
#[test]
fn delete_edge() {
    let mut digraph = Digraph::from_adjacency_list(5, 5, vec![vec![1]; 5]);
    assert_eq!(
        digraph.adj,
        vec![vec![1], vec![1], vec![1], vec![1], vec![1]]
    );
    digraph.delete_edge(0, 1);
    assert_eq!(
        digraph.adj,
        vec![vec![], vec![1], vec![1], vec![1], vec![1]]
    );
    assert_eq!(digraph.e_count, 4);
}
#[test]
fn outgoing_edges() {
    let mut digraph = Digraph::from_adjacency_list(5, 5, vec![vec![1]; 5]);
    for i in 0..digraph.v_count() {
        assert_eq!(digraph.outgoing_edges(i), vec![1]);
    }
    digraph.e_count = 25;
    digraph.adj = vec![vec![0, 1, 2, 3, 4]; 5];
    for i in 0..digraph.v_count() {
        assert_eq!(digraph.outgoing_edges(i), vec![0, 1, 2, 3, 4]);
    }
}
#[test]
fn incoming_edges() {
    let mut digraph = Digraph::from_adjacency_list(5, 25, vec![vec![0, 1, 2, 3, 4]; 5]);
    for i in 0..digraph.v_count() {
        assert_eq!(digraph.incoming_edges(i), vec![0, 1, 2, 3, 4]);
    }
    digraph.adj = vec![vec![1]; 5];
    for i in 0..digraph.v_count() {
        if i == 1 {
            assert_eq!(digraph.incoming_edges(i), vec![0, 1, 2, 3, 4]);
        } else {
            assert_eq!(digraph.incoming_edges(i), vec![]);
        }
    }
}
#[test]
fn delete_outgoing_edges() {
    let mut digraph = Digraph::from_adjacency_list(5, 5, vec![vec![1]; 5]);
    for i in 0..digraph.v_count() {
        assert_eq!(digraph.adj[i], vec![1]);
    }
    for i in 0..digraph.v_count() {
        digraph.delete_outgoing_edges(i);
        assert_eq!(digraph.adj[i], vec![]);
    }
    assert_eq!(digraph.e_count(), 0);
    digraph.e_count = 25;
    digraph.adj = vec![vec![0, 1, 2, 3, 4]; 5];
    for i in 0..digraph.v_count() {
        assert_eq!(digraph.adj[i], vec![0, 1, 2, 3, 4]);
    }
    for i in 0..digraph.v_count() {
        digraph.delete_outgoing_edges(i);
        assert_eq!(digraph.adj[i], vec![]);
    }
    assert_eq!(digraph.e_count(), 0);
}
#[test]
fn delete_incoming_edges() {
    let mut digraph = Digraph::from_adjacency_list(5, 5, vec![vec![1]; 5]);
    for i in 0..digraph.v_count() {
        digraph.delete_incoming_edges(i);
    }
    assert_eq!(digraph.e_count(), 0);
    assert_eq!(digraph.adj, vec![vec![]; 5]);
}
