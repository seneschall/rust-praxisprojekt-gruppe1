use crate::traits::*;
use num::{FromPrimitive, Integer, ToPrimitive, Unsigned};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs,
    hash::Hash,
    str::FromStr,
};

mod wt_graph;

#[cfg(test)]
mod test {
    use super::*;

    const V_COUNT: u32 = 10;

    #[test]
    fn create_graph_and_add_edges() {
        let mut graph: Digraph<u32, u32> = Digraph::new(V_COUNT);
        graph.add_edge(3, 2);
        graph.add_edge(5, 0);
        assert_eq!(graph.outgoing_edges(3), vec![2u32]);
        assert_eq!(graph.outgoing_edges(5), vec![0u32]);
        assert_eq!(graph.e_count(), 2);
    }

    #[test]
    fn test_node_labels() {
        let mut graph: Digraph<u32, String> = Digraph::new(V_COUNT);
        graph.add_node_label(0, String::from("test"));
        assert_eq!(graph.get_label(0), Some(&String::from("test")));
        assert_eq!(graph.get_label(1), None);
    }
}

// Defining data structures

pub struct Digraph<T, L>
// pub(crate) allows us to read adj in wt_graph without making it public for users of the library
// i.e. it makes it crate private
where
    T: Unsigned + ToPrimitive,
{
    v_count: T,                 // number of vertices
    e_count: T,                 // number of edges
    adj: Vec<Vec<T>>,           // adjacency list of indices
    node_labels: HashMap<T, L>, // name given to node format: index: value
}

impl<T, L> Digraph<T, L>
where
    T: Unsigned + ToPrimitive + Copy + Integer,
{
    fn new(v_count: T) -> Self {
        Digraph {
            v_count,
            e_count: T::zero(),
            adj: vec![vec![]; v_count.to_usize().unwrap()],
            node_labels: HashMap::new(),
        }
    }

    fn vertex_exists(&self, v: T) -> bool {
        v < self.v_count
    }
}

impl<T, L> Graph<T, L> for Digraph<T, L>
where
    T: Unsigned + ToPrimitive + Integer + Display + Copy + Hash,
    L: Clone,
{
    fn add_edge(&mut self, v: T, w: T) {
        if !(self.vertex_exists(v) || self.vertex_exists(w)) {
            panic!("One of vertices {}, {} doesn't exist", v, w)
        };
        self.e_count = self.e_count() + T::one();
        self.adj[v.to_usize().unwrap()].push(w);
    }

    fn add_node_label(&mut self, v: T, label: L) {
        self.node_labels.insert(v, label);
    }

    fn delete_edge(&mut self, v: T, w: T) {
        let i_of_w: usize;
        match self.adj.get(v.to_usize().unwrap()) {
            Some(vs) => {
                let i_of_w_opt = vs.iter().position(|&x| x == w);

                match i_of_w_opt {
                    Some(i) => {
                        i_of_w = i;
                    } // swap_remove more efficient than remove because the order is not important
                    None => {
                        panic!("There was no edge from {v} to {w}.");
                    }
                }
            }
            None => {
                panic!("Vertex {v} doesn't exist."); // Should be replaced by Result type
            }
        }

        self.adj[v.to_usize().unwrap()].swap_remove(i_of_w);
    }

    fn delete_vertex(&mut self, v: T) {
        todo!()
    }

    fn e_count(&self) -> T {
        self.e_count
    }

    fn edit_label(&mut self, v: T) {
        todo!()
    }

    fn get_label(&self, v: T) -> Option<&L> {
        self.node_labels.get(&v)
    }

    fn v_count(&self) -> T {
        self.v_count
    }
}

impl<T, L> Directed<T> for Digraph<T, L>
where
    T: Unsigned + ToPrimitive + Clone,
{
    fn outgoing_edges(&self, vertex: T) -> Vec<T> {
        self.adj[vertex.to_usize().unwrap()].clone()
    }

    fn incoming_edges(&self, vertex: T) -> Vec<T> {
        todo!()
    }
}

// Veras Funktionen:

pub fn import_graph_properties<T: FromStr + Debug + Unsigned>(filename: &str) -> (T, T)
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

pub fn import_adjacency_list<T: Eq + Hash + Clone + Debug + FromStr + Unsigned>(
    filename: &str,
) -> HashMap<T, Vec<T>>
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

pub fn create_sequence<T: Clone + Unsigned>(map: &HashMap<T, Vec<T>>) -> Vec<T> {
    let mut sequence = Vec::new();

    for items in map.values() {
        for item in items {
            sequence.push(item.clone());
        }
    }
    sequence
}
