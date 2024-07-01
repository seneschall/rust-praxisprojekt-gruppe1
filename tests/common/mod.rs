use super::*;
use graph::{directed::Digraph, labeled_directed::LabeledDigraph, labeled_undirected::LabeledUGraph, labeled_weighted_directed::LabeledWeightedDigraph, labeled_weighted_undirected::LabeledWeightedUGraph, undirected::UGraph, weighted_directed::WeightedDigraph, weighted_undirected::WeightedUGraph};
use num::ToPrimitive;
use vers_vecs::RsVec;
use wt::{directed::WTDigraph, weighted_directed::WeightedWTDigraph};
use std::{collections::HashMap, fs};
use vers_vecs::BitVec;

// Directed Graphs
pub fn setup_dg(filename: &str) -> Digraph {
    let (v_count, e_count) = import_graph_properties(filename);
    let adj: Vec<Vec<usize>> = import_adjacency_list_dg(filename);
    let digraph: Digraph = Digraph::from_adjacency_list(v_count, e_count, adj);
    return digraph;
}
pub fn setup_wdg(filename: &str) -> WeightedDigraph<f32>{
    let (v_count, e_count) = import_graph_properties(filename);
    let adj : Vec<Vec<(usize, f32)>> = import_adjacency_list_wdg(filename);
    let wdg : WeightedDigraph<f32> = WeightedDigraph::from_adjacency_list(v_count, e_count, adj);
    return wdg;
}
pub fn setup_ldg(filename: &str) -> LabeledDigraph<String>{
    let (v_count, e_count) = import_graph_properties(filename);
    let (adj , labels) = import_adjacency_list_ldg(filename);
    let ldg : LabeledDigraph<String> = LabeledDigraph::from_adjacency_list(v_count, e_count, adj, labels);
    return ldg;
}
pub fn setup_lwdg(filename: &str) -> LabeledWeightedDigraph<String, f32>{
    let (v_count, e_count) = import_graph_properties(filename);
    let (adj , labels) = import_adjacency_list_lwdg(filename);
    let lwdg : LabeledWeightedDigraph<String, f32> = LabeledWeightedDigraph::from_adjacency_list(v_count, e_count, adj, labels);
    return lwdg;
}
// Undirected Graphs
pub fn setup_ug(filename: &str) -> UGraph {
    let (v_count, e_count) = import_graph_properties(filename);
    let adj: Vec<Vec<usize>> = import_adjacency_list_ug(filename);
    let ug: UGraph = UGraph::from_adjacency_list(v_count, e_count, adj);
    return ug;
}
pub fn setup_wug(filename: &str) -> WeightedUGraph<f32>{
    let (v_count, e_count) = import_graph_properties(filename);
    let adj : Vec<Vec<(usize, f32)>> = import_adjacency_list_wug(filename);
    let wug : WeightedUGraph<f32> = WeightedUGraph::from_adjacency_list(v_count, e_count, adj);
    return wug;
}
pub fn setup_lug(filename: &str) -> LabeledUGraph<String>{
    let (v_count, e_count) = import_graph_properties(filename);
    let (adj , labels) = import_adjacency_list_lug(filename);
    let lug : LabeledUGraph<String> = LabeledUGraph::from_adjacency_list(v_count, e_count, adj, labels);
    return lug;
}
pub fn setup_lwug(filename: &str) -> LabeledWeightedUGraph<String, f32>{
    let (v_count, e_count) = import_graph_properties(filename);
    let (adj , labels) = import_adjacency_list_lwdg(filename);
    let lwug : LabeledWeightedUGraph<String, f32> = LabeledWeightedUGraph::from_adjacency_list(v_count, e_count, adj, labels);
    return lwug;
}

// WT Directed Graphs
pub fn setup_wtdg(filename: &str) -> WTDigraph {
    let adj = import_adjacency_list_dg(filename);
    let (sequence, starting_indices) = create_sequence_and_bitmap(&adj); //creating sequence and bitmap
    let wtdigraph: WTDigraph = WTDigraph::from(sequence, RsVec::from_bit_vec(starting_indices)); // create WTDigraph using from(sequence, starting_indices)
    return wtdigraph;
}
pub fn setup_wtwdg(filename: &str) -> WeightedWTDigraph<f32>{
    let adj_weight = import_adjacency_list_wdg(filename);
    let mut weights : HashMap<(usize,usize), f32> = HashMap::new();
    let mut adj: Vec<Vec<usize>> = Vec::new();
    for from in 0..adj_weight.len(){
        adj.insert(from, vec![]);
        for (to, weight) in adj_weight[from].clone(){
            adj[from].push(to);
            weights.insert((from,to), weight);
        }
    }
    let (sequence, starting_indices) = create_sequence_and_bitmap(&adj);
    let wtwdg : WeightedWTDigraph<f32> = WeightedWTDigraph::from(sequence, RsVec::from_bit_vec(starting_indices), weights);
    return wtwdg;
}

// use output from import_adjacency_list to create a sequence for qwt and a bitmap
// ex. let (sequence, bitmap) = create_sequence_and_bitmap(&adjacency_list);
pub fn create_sequence_and_bitmap(map: &Vec<Vec<usize>>) -> (Vec<usize>, BitVec) {
    let mut sequence = Vec::new();
    let mut bitmap = BitVec::new();

    for items in map {
        bitmap.append(true);
        for item in items {
            bitmap.append(false);
            sequence.push(item.clone());
        }
    }
    (sequence, bitmap)
}


// Funktionen zum Einlesen vom Graphen aus einer Input-Datei
pub fn import_graph_properties(filename: &str) -> (usize, usize) {
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let mut lines = content.lines();

    let v_count = lines
        .next()
        .expect("Missing first line")
        .trim()
        .parse::<usize>()
        .expect("First line (number of vertices) is not a valid input");

    let e_count = lines
        .next()
        .expect("Missing second line")
        .trim()
        .parse::<usize>()
        .expect("Second line (number of edges) is not a valid input");

    (v_count, e_count)
}

// create the adjecency list from a graph in the input file
pub fn import_adjacency_list_dg(filename: &str) -> Vec<Vec<usize>> {
    let content = fs::read_to_string(filename).expect("Unable to open file");

    let mut lines = content.lines();
    let size: usize = lines
        .next()
        .expect("Missing first line")
        .trim()
        .parse()
        .expect("First line (number of vertices) is not a valid input");

    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; size]; // create Vec<Vec<T>> with the size equal to the amount of verticies

    for line in lines {
        let line = line.trim();
        let mut numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());

        if let (Some(vertex), Some(adjacent)) = (numbers.next(), numbers.next()) {
            adjacency_list[vertex.to_usize().unwrap()].push(adjacent);
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    adjacency_list
}



pub fn import_adjacency_list_wdg(filename: &str) -> Vec<Vec<(usize, f32)>> {
    todo!()
}
pub fn import_adjacency_list_ldg(filename: &str) -> (Vec<Vec<usize>>, Vec<String>) {
    todo!()
}
pub fn import_adjacency_list_lwdg(filename: &str) -> (Vec<Vec<(usize,f32)>>, Vec<String>) {
    todo!()
}


pub fn import_adjacency_list_ug(filename: &str) -> Vec<Vec<usize>> {
    todo!()
}
pub fn import_adjacency_list_wug(filename: &str) -> Vec<Vec<(usize, f32)>> {
    todo!()
}
pub fn import_adjacency_list_lug(filename: &str) -> (Vec<Vec<usize>>, Vec<String>) {
    todo!()
}
pub fn import_adjacency_list_lwug(filename: &str) -> (Vec<Vec<(usize,f32)>>, Vec<String>) {
    todo!()
}
