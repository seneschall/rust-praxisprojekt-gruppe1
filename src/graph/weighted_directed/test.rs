use super::*;
use std::f64::{INFINITY, NEG_INFINITY};

fn setupwdg() -> WeightedDigraph<f64> {
    let v_count = 5;
    let e_count = 12;
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

    let wdg: WeightedDigraph<f64> = WeightedDigraph::from_adjacency_list(v_count, e_count, adj);
    wdg
}

#[test]
fn new() {
    let wdg: WeightedDigraph<f64> = WeightedDigraph::new();
    assert!(wdg.weights.is_empty());
    assert!(wdg.dg.adj.is_empty());
    assert!(wdg.dg.deleted_vertices.is_empty());
    assert_eq!(wdg.v_count(), 0);
    assert_eq!(wdg.e_count(), 0);
}
#[test]
fn from_adjacency_list() {
    let v_count = 5;
    let e_count = 12;
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

    let wdg: WeightedDigraph<f64> =
        WeightedDigraph::from_adjacency_list(v_count.clone(), e_count.clone(), adj.clone());
    assert_eq!(wdg.e_count(), e_count);
    assert_eq!(wdg.v_count(), v_count);
    assert_eq!(wdg.dg.adj, testadj);
    assert_eq!(wdg.dg.deleted_vertices, HashMap::new());
    assert_eq!(wdg.weights, testweights);
}

#[test]
fn append_vertex() {
    let mut wdg: WeightedDigraph<f64> = WeightedDigraph::new();
    for i in 0..9 {
        assert_eq!(i, wdg.append_vertex());
    }
}
#[test]
fn add_edge() {
    let (i, j): (usize, usize) = (1, 0);
    let mut wdg = setupwdg();
    wdg.add_edge(1, 0, NEG_INFINITY);
    assert_eq!(
        wdg.weights.get_key_value(&(1, 0)).unwrap(),
        (&(i, j), &NEG_INFINITY)
    );
    assert_eq!(wdg.e_count(), 13);
}
#[test]
fn add_vertex() {
    let mut wdg = setupwdg();
    wdg.add_vertex(5);
    assert_eq!(wdg.v_count(), 6);
    assert_eq!(
        wdg.dg.adj,
        vec![
            vec![4, 3],
            vec![],
            vec![0],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
            vec![],
        ]
    );
}
#[test]
fn edit_weight() {
    let mut wdg = setupwdg();
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
    for from in wdg.dg.adj.clone() {
        for to in from {
            test_weights_hashmap.insert((j, to), weights[j].get(u).unwrap().clone());
            u += 1;
        }
        u = 0;
        j += 1;
    }
    wdg.edit_weight(0, 4, NEG_INFINITY);
    test_weights_hashmap.insert((0, 4), NEG_INFINITY);
    assert_eq!(wdg.weights, test_weights_hashmap);
}
#[test]
fn get_weight() {
    let mut wdg = setupwdg();
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
    for from in wdg.dg.adj.clone() {
        for to in from {
            test_weights_hashmap.insert((j, to), weights[j].get(u).unwrap().clone());
            assert_eq!(
                wdg.weight(j, to),
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
    let mut wdg = setupwdg();
    wdg.delete_vertex(0);
    let v_count = 4;
    let e_count = 7;
    let mut labels: Vec<String> = Vec::new();
    let mut test_labels_hashmap: HashMap<String, usize> = HashMap::new();
    let mut test_weights_hashmap: HashMap<(usize, usize), f64> = HashMap::new();
    for i in 0..5 {
        labels.push(i.to_string());
        test_labels_hashmap.insert(i.to_string(), i);
    }
    test_weights_hashmap.insert((3, 1), 0.0);
    test_weights_hashmap.insert((3, 2), 2.1);
    test_weights_hashmap.insert((3, 3), INFINITY);
    test_weights_hashmap.insert((3, 4), NEG_INFINITY);

    test_weights_hashmap.insert((4, 1), 0.0);
    test_weights_hashmap.insert((4, 2), 0.0);
    test_weights_hashmap.insert((4, 3), 0.0);
    let adj: Vec<Vec<usize>> = vec![vec![], vec![], vec![], vec![4, 1, 2, 3], vec![3, 1, 2]]; // order is not important and changes here since we use swap_remove for more efficency
    assert_eq!(wdg.e_count(), e_count);
    assert_eq!(wdg.v_count(), v_count);
    assert_eq!(wdg.dg.adj, adj);
    assert_eq!(wdg.dg.deleted_vertices.contains_key(&0), true);
    // assert_eq!(wdg.dg.deleted_vertices, vec![0]);
}
#[test]
fn delete_edge() {
    let mut wdg = setupwdg();
    let mut j = 0;
    for from in wdg.dg.adj.clone() {
        for to in from {
            wdg.delete_edge(j, to);
        }
        j += 1;
    }
    assert_eq!(wdg.dg.adj, vec![vec![]; wdg.v_count()]);
    assert_eq!(wdg.e_count(), 0);
}
#[test]
fn outgoing_edges() {
    let wdg = setupwdg();
    let testadj: Vec<Vec<usize>> = vec![
        vec![4, 3],
        vec![],
        vec![0],
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 2, 3],
    ];
    let mut outgoing_edges_to_index: Vec<usize> = Vec::new();
    for i in 0..wdg.v_count() {
        for item in wdg.outgoing_edges(i) {
            outgoing_edges_to_index.push(item);
        }
        assert_eq!(outgoing_edges_to_index, testadj[i]);
        outgoing_edges_to_index.clear();
    }
}
#[test]
fn incoming_edges() {
    let wdg = setupwdg();
    let testadj: Vec<Vec<usize>> = vec![
        vec![2, 3, 4],
        vec![3, 4],
        vec![3, 4],
        vec![0, 3, 4],
        vec![0, 3],
    ];
    let mut incoming_edges_to_index: Vec<usize> = Vec::new();
    for i in 0..wdg.v_count() {
        for item in wdg.incoming_edges(i) {
            incoming_edges_to_index.push(item);
        }
        println!("{i}");
        assert_eq!(incoming_edges_to_index, testadj[i]);
        incoming_edges_to_index.clear();
    }
}
#[test]
fn delete_outgoing_edges() {
    let mut wdg = setupwdg();
    assert_eq!(wdg.e_count(), 12);
    for i in 0..wdg.v_count() {
        wdg.delete_outgoing_edges(i);
    }
    assert_eq!(wdg.e_count(), 0);
    assert_eq!(wdg.dg.adj, vec![vec![]; 5]);
}
#[test]
fn delete_incoming_edges() {
    let mut wdg = setupwdg();
    assert_eq!(wdg.e_count(), 12);
    for i in 0..wdg.v_count() {
        wdg.delete_incoming_edges(i);
    }
    assert_eq!(wdg.e_count(), 0);
    assert_eq!(wdg.dg.adj, vec![vec![]; 5]);
}
