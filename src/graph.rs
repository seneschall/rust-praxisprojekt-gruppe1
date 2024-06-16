use crate::traits::*;
use num::{FromPrimitive, Integer, ToPrimitive, Unsigned};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs,
    hash::Hash,
    str::FromStr,
};
use vers_vecs::{BitVec, RsVec};

pub mod directed;
pub mod undirected;
pub mod wt_directed;
pub mod wt_undirected;

// UNIT-TESTS Graph-Einlesen aus Datei
#[cfg(test)]
mod test {
    use directed::Digraph;

    use super::*;

    const V_COUNT: usize = 10;

    #[test]
    fn create_graph_and_add_edges() {
        // let mut graph: Digraph<u32> = Digraph::new(V_COUNT);
        // graph.add_edge(3, 2);
        // graph.add_edge(5, 0);
        // assert_eq!(graph.outgoing_edges(3), vec![2u32]);
        // assert_eq!(graph.outgoing_edges(5), vec![0u32]);
        // assert_eq!(graph.e_count(), 2);
    }

    #[test]
    fn test_vertex_labels() {
        // let mut graph: Digraph<u32, String> = Digraph::new(V_COUNT);
        // graph.add_vertex_label(0, String::from("test"));
        // assert_eq!(graph.get_label(0), Some(&String::from("test")));
        // assert_eq!(graph.get_label(1), None);
    }
}

// Funktionen zum Einlesen vom Graphen aus einer Input-Datei
pub fn import_graph_properties(filename: &str) -> (usize, usize)
{
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
pub fn import_adjacency_list(
    filename: &str,
) -> Vec<Vec<usize>>
{
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
        let mut numbers = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok());

        if let (Some(vertex), Some(adjacent)) = (numbers.next(), numbers.next()) {
            adjacency_list[vertex.to_usize().unwrap()].push(adjacent);
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    adjacency_list
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
