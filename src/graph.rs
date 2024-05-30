use num::{Integer, Unsigned};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use std::str::FromStr;
use vers_vecs::{BitVec, RsVec};
use num::Unsigned;

#[cfg(test)]
mod test {
    use super::*;

    const V_COUNT: u32 = 10;

    #[test]
    fn create_graph_and_add_edges() {
        let mut graph: Digraph<u32> = Digraph::new(V_COUNT);
        graph.add_edge(3, 2);
        graph.add_edge(5, 0);
        assert_eq!(graph.outgoing_edges(3), vec![2u32]);
        assert_eq!(graph.outgoing_edges(5), vec![0u32]);
        assert_eq!(graph.e_count(), 2);
    }

    #[test]
    fn test_node_labels() {
        let mut graph: Digraph<String> = Digraph::new(V_COUNT);
        graph.add_node_label(0, String::from("test"));
        assert_eq!(graph.get_label(0), Some(String::from("test")));
        assert_eq!(graph.get_label(1), None);
    }
}

pub trait WTGraph {
    fn commit_changes();
}

pub trait Graph<L> {
    fn add_edge(&mut self, v: Vertex, w: Vertex);

    fn add_node_label(&mut self, v: Vertex, label: L);

    fn v_count(&self) -> u32;

    fn e_count(&self) -> u32;

    fn outgoing_edges(&self, vertex: Vertex) -> Vec<Vertex>;

    fn get_label(&self, v: Vertex) -> Option<L>;

    // fn delete_vertex(&mut self v: Vertex);

    // fn delete_edge(&mut self, v: Vertex, w: Vertex);

    // fn edit_label(&mut self, v: Vertext);
}

type Vertex = u32;

pub struct Digraph<T, L> {
    v_count: u32,                 // number of vertices
    e_count: u32,                 // number of edges
    adj: Vec<Vec<Vertex>>,        // adjacency list of indices
    node_labels: HashMap<u32, T>, // name given to node format: index: value
}

impl<T> Digraph<T> {
    fn new(v_count: u32) -> Self {
        Digraph {
            v_count,
            e_count: 0,
            adj: vec![vec![]; v_count as usize],
            node_labels: HashMap::new(),
        }
    }
}

impl<T: Clone> Graph<T> for Digraph<T> {
    fn add_edge(&mut self, v: Vertex, w: Vertex) {
        if !(self.vertex_exists(v) || self.vertex_exists(w)) {
            panic!("One of vertices {}, {} doesn't exist", v, w)
        };
        self.e_count += 1;
        self.adj[v as usize].push(w);
    }

    fn v_count(&self) -> u32 {
        self.v_count
    }

    fn e_count(&self) -> u32 {
        self.e_count
    }

    fn outgoing_edges(&self, vertex: Vertex) -> Vec<u32> {
        self.adj[vertex as usize].clone()
    }

    fn add_node_label(&mut self, v: Vertex, label: T) {
        self.node_labels.insert(v, label);
    }

    fn get_label(&self, v: Vertex) -> Option<T> {
        self.node_labels.get(&v).cloned()
    }
}

impl<T> Digraph<T> {
    fn vertex_exists(&self, v: Vertex) -> bool {
        v < self.v_count
    }
}

pub struct PseudoWTDigraph<T> {
    // Die Idee ist, schon mal in einer "Trockenübung" zu schauen,
    // ob die Implementierung mit Wavelet-Tree so funktionieren könnte, wie wir uns das vorstellen
    v_count: u32,                              // number of vertices
    e_count: u32,                              // number of edges
    sequence: Vec<Vertex>,                     // sequence representation of adjacency list
    starting_indices: RsVec,                   // starting indices of each
    uncommitted_edits: HashMap<u32, Vec<u32>>, // changes not yet committed to sequence
    has_uncommitted_edits: bool,
    node_labels: HashMap<u32, T>, // name given to node format: index: value
}

impl<T> PseudoWTDigraph<T> {
    pub fn from_digraph(dg: Digraph<T>) -> Self {
        let mut bv = BitVec::new();
        let mut e_count: u32 = 0;
        let v_count = dg.adj.len();
        let mut sequence: Vec<Vertex> = Vec::new();
        let mut adj: Vec<Vec<Vertex>> = vec![Vec::new(); 250];

        for (v, v_adj) in dg.adj.iter().enumerate() {
            // iterate over all vertices (v) in adj
            bv.append(true);
            for val in v_adj.iter() {
                // iterate over the values in the adjacency list of v
                sequence.push(*val);
                bv.append(false); // append 0 to bv for each element in adjacency list of v
                e_count += 1;
            }
        }
        let starting_indices = RsVec::from_bit_vec(bv);

        return PseudoWTDigraph {
            v_count: v_count as u32,
            e_count,
            sequence,
            starting_indices,
            uncommitted_edits: HashMap::new(),
            has_uncommitted_edits: false,
            node_labels: HashMap::new(),
        };
    }

    pub fn from(sequence: Vec<Vertex>, starting_indices: RsVec) -> Self {
        let length = starting_indices.len();
        let v_count = starting_indices.rank1(length - 1) as u32;
        let e_count = starting_indices.rank0(length - 1) as u32;

        return PseudoWTDigraph {
            v_count,
            e_count,
            sequence,
            starting_indices,
            uncommitted_edits: HashMap::new(),
            has_uncommitted_edits: false,
            node_labels: HashMap::new(),
        };
    }
}

impl<T: Clone> Graph<T> for PseudoWTDigraph<T> {
    fn add_edge(&mut self, v: Vertex, w: Vertex) {
        todo!()
    }

    fn add_node_label(&mut self, v: Vertex, label: T) {
        if v > self.v_count - 1 {
            panic!("Vertex doesn't exist.");
        }

        self.node_labels.insert(v, label);
    }

    fn v_count(&self) -> u32 {
        self.v_count
    }

    fn e_count(&self) -> u32 {
        self.e_count
    }

    fn outgoing_edges(&self, vertex: Vertex) -> Vec<Vertex> {
        let mut v_adj: Vec<Vertex> = Vec::new();
        let v = vertex as usize;

        let start = self.starting_indices.select1(v) + v; // statt der 1 müsste hier glaub ich rank1(vertex) stehen; ausprobieren
        let end = self.starting_indices.select1(v + 1) + v + 1; // if this value is bigger than sequence.len(), vers_vecs will return len + 1

        if start > self.sequence.len() || start == end {
            return Vec::new();
        }

        for i in start..end {
            v_adj.push(self.sequence[i]); // this won't work for qwt of course; we'll have to use get() there
        }

        return v_adj;
    }

    fn get_label(&self, v: Vertex) -> Option<T> {
        self.node_labels.get(&v).cloned()
    }
}

// Veras Funktionen:

fn import_graph_properties<T: FromStr + Debug + Unsigned>(filename: &str) -> (T, T)
where
    <T as FromStr>::Err: Debug,
{
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let mut lines = content.lines();

    let v_count = lines
        .next()
        .expect("Missing first line")
        .trim()
        .parse::<T>()
        .expect("First line (number of vertices) is not a valid input");

    let e_count = lines
        .next()
        .expect("Missing second line")
        .trim()
        .parse::<T>()
        .expect("Second line (number of edges) is not a valid input");

    (v_count, e_count)
}

fn import_adjacency_list<T: Eq + Hash + Clone + Debug + FromStr + Unsigned>(filename: &str,) -> HashMap<T, Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    let content = fs::read_to_string(filename).expect("Unable to open file");

    let mut adjacency_list: HashMap<T, Vec<T>> = HashMap::new();
    let mut lines = content.lines().skip(2);

    for line in lines {
        let line = line.trim();
        let mut numbers = line.split_whitespace().filter_map(|s| s.parse::<T>().ok());

        if let (Some(vertex), Some(adjacent)) = (numbers.next(), numbers.next()) {
            adjacency_list
                .entry(vertex.clone())
                .or_insert(Vec::new())
                .push(adjacent.clone());
            adjacency_list.entry(adjacent).or_insert(Vec::new());
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    adjacency_list
}

fn create_sequence<T: Clone + Unsigned>(map: &HashMap<T, Vec<T>>) -> Vec<T> {
    let mut sequence = Vec::new();

    for items in map.values() {
        for item in items {
            sequence.push(item.clone());
        }
    }
    sequence
}
