use std::f64::{INFINITY, NEG_INFINITY};

use num::ToPrimitive;

use super::*;

fn setuplwdg() -> LabeledWeightedDigraph<String, f64> {
    let v_count = 5;
    let e_count = 12;
    let mut labels: Vec<String> = Vec::new();
    for i in 0..v_count {
        labels.push(i.to_string());
    }
    let adj: Vec<Vec<(usize, f64)>> = vec![
        vec![(4, 0.22), (3, 0.1111111)],
        vec![],
        vec![(0, 5.13131)],
        vec![
            (0, 1111.0),
            (1, 0.0),
            (2, 2.1),
            (3, INFINITY),
            (4, NEG_INFINITY),
        ],
        vec![(0, 0.0), (1, 0.0), (2, 0.0), (3, 0.0)],
    ];

    let lwdg: LabeledWeightedDigraph<String, f64> =
        LabeledWeightedDigraph::from_adjacency_list(v_count, e_count, adj, labels);
    lwdg
}

#[test]
fn new() {
    let lwdg: LabeledWeightedDigraph<String, f64> = LabeledWeightedDigraph::new();
    assert!(lwdg.weights.is_empty());
    assert!(lwdg.dg.hashmap_labels_vertex.is_empty());
    assert!(lwdg.dg.vec_vertex_labels.is_empty());
    assert!(lwdg.dg.dg.adj.is_empty());
    assert!(lwdg.dg.dg.deleted_vertices.is_empty());
    assert_eq!(lwdg.v_count(), 0);
    assert_eq!(lwdg.e_count(), 0);
}
#[test]
fn from_adjacency_list() {
    let v_count = 5;
    let e_count = 12;
    let mut labels: Vec<String> = Vec::new();
    let mut test_labels_hashmap: HashMap<String, usize> = HashMap::new();
    for i in 0..v_count {
        labels.push(i.to_string());
        test_labels_hashmap.insert(i.to_string(), i);
    }
    let adj: Vec<Vec<(usize, f64)>> = vec![
        vec![(4, 0.22), (3, 0.1111111)],
        vec![],
        vec![(0, 5.13131)],
        vec![
            (0, 1111.0),
            (1, 0.0),
            (2, 2.1),
            (3, INFINITY),
            (4, NEG_INFINITY),
        ],
        vec![(0, 0.0), (1, 0.0), (2, 0.0), (3, 0.0)],
    ];
    let testadj: Vec<Vec<usize>> = vec![
        vec![4, 3],
        vec![],
        vec![0],
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 2, 3],
    ];
    let weights: Vec<Vec<f64>> = vec![
        vec![0.22, 0.1111111],
        vec![],
        vec![5.13131],
        vec![1111.0, 0.0, 2.1, INFINITY, NEG_INFINITY],
        vec![0.0, 0.0, 0.0, 0.0],
    ];
    let mut testweights: HashMap<(usize, usize), f64> = HashMap::new();
    let mut j = 0;
    let mut u: usize = 0;
    for from in testadj.clone() {
        for to in from {
            testweights.insert((j, to), weights[j].get(u).unwrap().clone());
            u += 1;
        }
        u = 0;
        j += 1;
    }

    let lwdg: LabeledWeightedDigraph<String, f64> = LabeledWeightedDigraph::from_adjacency_list(
        v_count.clone(),
        e_count.clone(),
        adj.clone(),
        labels.clone(),
    );
    assert_eq!(lwdg.e_count(), e_count);
    assert_eq!(lwdg.v_count(), v_count);
    assert_eq!(lwdg.dg.dg.adj, testadj);
    assert_eq!(lwdg.dg.dg.deleted_vertices, vec![]);
    assert_eq!(lwdg.dg.vec_vertex_labels, labels);
    assert_eq!(lwdg.dg.hashmap_labels_vertex, test_labels_hashmap);
    assert_eq!(lwdg.weights, testweights);
}
#[test]
fn add_edge() {
    let (i, j): (usize, usize) = (1, 0);
    let mut lwdg = setuplwdg();
    lwdg.add_edge(1.to_string(), 0.to_string(), NEG_INFINITY);
    assert_eq!(
        lwdg.weights.get_key_value(&(1, 0)).unwrap(),
        (&(i, j), &NEG_INFINITY)
    );
    assert_eq!(lwdg.e_count(), 13);
}
#[test]
fn add_vertex() {
    let mut lwdg = setuplwdg();
    lwdg.add_vertex(5.to_string());
    assert_eq!(lwdg.v_count(), 6);
    assert_eq!(
        lwdg.dg
            .hashmap_labels_vertex
            .get_key_value(&5.to_string())
            .unwrap(),
        (&5.to_string(), &5)
    );
    assert_eq!(lwdg.dg.vec_vertex_labels[5], 5.to_string());
    assert_eq!(
        lwdg.dg.dg.adj,
        vec![
            vec![4, 3],
            vec![],
            vec![0],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
            vec![]
        ]
    );
}
#[test]
fn edit_weight() {
    let mut lwdg = setuplwdg();
    let mut test_weights_hashmap: HashMap<(usize, usize), f64> = HashMap::new();
    let weights: Vec<Vec<f64>> = vec![
        vec![0.22, 0.1111111],
        vec![],
        vec![5.13131],
        vec![1111.0, 0.0, 2.1, INFINITY, NEG_INFINITY],
        vec![0.0, 0.0, 0.0, 0.0],
    ];

    let mut j = 0;
    let mut u: usize = 0;
    for from in lwdg.dg.dg.adj.clone() {
        for to in from {
            test_weights_hashmap.insert((j, to), weights[j].get(u).unwrap().clone());
            u += 1;
        }
        u = 0;
        j += 1;
    }
    lwdg.edit_weight(0.to_string(), 4.to_string(), NEG_INFINITY);
    test_weights_hashmap.insert((0, 4), NEG_INFINITY);
    assert_eq!(lwdg.weights, test_weights_hashmap);
}
#[test]
fn get_weight() {
    let mut lwdg = setuplwdg();
    let weights: Vec<Vec<f64>> = vec![
        vec![0.22, 0.1111111],
        vec![],
        vec![5.13131],
        vec![1111.0, 0.0, 2.1, INFINITY, NEG_INFINITY],
        vec![0.0, 0.0, 0.0, 0.0],
    ];
    let mut test_weights_hashmap: HashMap<(usize, usize), f64> = HashMap::new();
    let mut j = 0;
    let mut u: usize = 0;
    for from in lwdg.dg.dg.adj.clone() {
        for to in from {
            test_weights_hashmap.insert((j, to), weights[j].get(u).unwrap().clone());
            assert_eq!(
                lwdg.get_weight(
                    lwdg.dg.get_label(j),
                    lwdg.dg.get_label(to)
                ),
                test_weights_hashmap.get(&(j, to)).unwrap().clone()
            );
            u += 1;
        }
        u = 0;
        j += 1;
    }
}
#[test]
fn delete_vertex() {
    let mut lwdg = setuplwdg();
    lwdg.delete_vertex(0.to_string());
    let v_count = 4;
    let e_count = 7;
    let mut labels: Vec<String> = Vec::new();
    let mut test_labels_hashmap: HashMap<String, usize> = HashMap::new();
    let mut test_weights_hashmap: HashMap<(usize, usize), f64> = HashMap::new();
    for i in 0..5 {
        labels.push(i.to_string());
        test_labels_hashmap.insert(i.to_string(), i);
    }
    test_weights_hashmap.insert((3.to_usize().unwrap(), 1.to_usize().unwrap()), 0.0);
    test_weights_hashmap.insert((3.to_usize().unwrap(), 2.to_usize().unwrap()), 2.1);
    test_weights_hashmap.insert((3.to_usize().unwrap(), 3.to_usize().unwrap()), INFINITY);
    test_weights_hashmap.insert((3.to_usize().unwrap(), 4.to_usize().unwrap()), NEG_INFINITY);

    test_weights_hashmap.insert((4.to_usize().unwrap(), 1.to_usize().unwrap()), 0.0);
    test_weights_hashmap.insert((4.to_usize().unwrap(), 2.to_usize().unwrap()), 0.0);
    test_weights_hashmap.insert((4.to_usize().unwrap(), 3.to_usize().unwrap()), 0.0);
    let adj: Vec<Vec<usize>> = vec![vec![], vec![], vec![], vec![4, 1, 2, 3], vec![3, 1, 2]]; // order is not important and changes here since we use swap_remove for more efficency

    assert_eq!(lwdg.e_count(), e_count);
    assert_eq!(lwdg.v_count(), v_count);
    assert_eq!(lwdg.dg.dg.adj, adj);
    assert_eq!(lwdg.dg.dg.deleted_vertices, vec![0]);
    assert_eq!(lwdg.dg.vec_vertex_labels, labels);
    assert_eq!(lwdg.dg.hashmap_labels_vertex, test_labels_hashmap);
}
#[test]
fn delete_edge() {
    let mut lwdg = setuplwdg();
    let mut j = 0;
    for from in lwdg.dg.dg.adj.clone() {
        for to in from {
            lwdg.delete_edge(j.to_string(), to.to_string());
        }
        j += 1;
    }
    assert_eq!(lwdg.dg.dg.adj, vec![vec![]; lwdg.v_count()]);
    assert_eq!(lwdg.e_count(), 0);
}
#[test]
fn outgoing_edges() {
    let lwdg = setuplwdg();
    let testadj: Vec<Vec<usize>> = vec![
        vec![4, 3],
        vec![],
        vec![0],
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 2, 3],
    ];
    let mut outgoing_edges_to_index: Vec<usize> = Vec::new();
    for i in 0..lwdg.v_count() {
        for item in lwdg.outgoing_edges(i.to_string()) {
            outgoing_edges_to_index.push(lwdg.dg.get_index(item));
        }
        assert_eq!(outgoing_edges_to_index, testadj[i]);
        outgoing_edges_to_index.clear();
    }
}
#[test]
fn incoming_edges() {
    let lwdg = setuplwdg();
    let testadj: Vec<Vec<usize>> = vec![
        vec![2, 3, 4],
        vec![3, 4],
        vec![3, 4],
        vec![0, 3, 4],
        vec![0, 3],
    ];
    let mut incoming_edges_to_index: Vec<usize> = Vec::new();
    for i in 0..lwdg.v_count() {
        for item in lwdg.incoming_edges(i.to_string()) {
            incoming_edges_to_index.push(lwdg.dg.get_index(item));
        }
        println!("{i}");
        assert_eq!(incoming_edges_to_index, testadj[i]);
        incoming_edges_to_index.clear();
    }
}
#[test]
fn delete_outgoing_edges() {
    let mut lwdg = setuplwdg();
    assert_eq!(lwdg.e_count(), 12);
    for i in 0..lwdg.v_count() {
        lwdg.delete_outgoing_edges(i.to_string());
    }
    assert_eq!(lwdg.e_count(), 0);
    assert_eq!(lwdg.dg.dg.adj, vec![vec![]; 5]);
}
#[test]
fn delete_incoming_edges() {
    let mut lwdg = setuplwdg();
    assert_eq!(lwdg.e_count(), 12);
    for i in 0..lwdg.v_count() {
        lwdg.delete_incoming_edges(i.to_string());
    }
    assert_eq!(lwdg.e_count(), 0);
    assert_eq!(lwdg.dg.dg.adj, vec![vec![]; 5]);
}
