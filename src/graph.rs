use num::{FromPrimitive, Integer, Num, ToPrimitive, Unsigned};
use qwt::QWT256;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs,
    hash::Hash,
    str::FromStr,
};
use vers_vecs::{BitVec, RsVec};

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

// Defining traits

pub trait WTGraph<T>
where
    T: Unsigned + ToPrimitive,
{
    fn commit_changes();

    // fn delete_vertex(&mut self v: T);

    // fn delete_edge(&mut self, v: T, w: T);

    // fn edit_label(&mut self, v: T);
}

pub trait Graph<T, L>
where
    T: Unsigned + ToPrimitive,
{
    fn add_edge(&mut self, v: T, w: T);

    fn add_node_label(&mut self, v: T, label: L);

    fn v_count(&self) -> T;

    fn e_count(&self) -> T;

    fn get_label(&self, v: T) -> Option<&L>;
}

pub trait Directed<T>
where
    T: Unsigned + ToPrimitive,
{
    fn outgoing_edges(&self, vertex: T) -> Vec<T>; // should probably be changed to return an iterator instead
    fn incoming_edges(&self, vertex: T) -> Vec<T>; // likewise here
}

pub trait Undirected<T>
where
    T: Unsigned + ToPrimitive,
{
    fn edges(&self, vertex: T) -> Vec<T>;
}

pub trait Weighted<T>
where
    T: Unsigned + ToPrimitive,
{
    fn weight_of_edge(&self, from: T, to: T) -> f64;
}

// Defining data structures

pub struct Digraph<T, L>
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

    fn v_count(&self) -> T {
        self.v_count
    }

    fn e_count(&self) -> T {
        self.e_count
    }

    fn add_node_label(&mut self, v: T, label: L) {
        self.node_labels.insert(v, label);
    }

    fn get_label(&self, v: T) -> Option<&L> {
        self.node_labels.get(&v)
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

pub struct PseudoWTDigraph<T, L>
where
    T: Unsigned + ToPrimitive,
{
    // Die Idee ist, schon mal in einer "Trockenübung" zu schauen,
    // ob die Implementierung mit Wavelet-Tree so funktionieren könnte, wie wir uns das vorstellen
    v_count: T,                            // number of vertices
    e_count: T,                            // number of edges
    sequence: Vec<T>,                      // sequence representation of adjacency list
    starting_indices: RsVec,               // starting indices of each
    uncommitted_edits: HashMap<T, Vec<T>>, // changes not yet committed to sequence
    has_uncommitted_edits: bool,
    node_labels: HashMap<T, L>, // name given to node format: index: value
}

impl<T, L> PseudoWTDigraph<T, L>
where
    T: Unsigned + ToPrimitive + FromPrimitive + Copy,
{
    pub fn from_digraph(dg: Digraph<T, L>) -> Self {
        let mut bv = BitVec::new();
        let mut e_count: T = T::zero();
        let v_count = dg.adj.len();
        let mut sequence: Vec<T> = Vec::new();

        for (v, v_adj) in dg.adj.iter().enumerate() {
            // iterate over all vertices (v) in adj
            bv.append(true);
            for val in v_adj.iter() {
                // iterate over the values in the adjacency list of v
                sequence.push(*val);
                bv.append(false); // append 0 to bv for each element in adjacency list of v
                e_count = e_count + T::one();
            }
        }
        let starting_indices = RsVec::from_bit_vec(bv);

        // At this point wavelet tree would be created from sequence

        return PseudoWTDigraph {
            v_count: T::from_usize(v_count).unwrap(),
            e_count,
            sequence, // here sequence would be replaced by wavelet tree
            starting_indices,
            uncommitted_edits: HashMap::new(),
            has_uncommitted_edits: false,
            node_labels: HashMap::new(),
        };
    }

    pub fn from(sequence: Vec<T>, starting_indices: RsVec) -> Self {
        let length = starting_indices.len();

        let v_count = starting_indices.rank1(length - 1);
        let v_count = T::from_usize(v_count).unwrap();

        let e_count = starting_indices.rank0(length - 1);
        let e_count = T::from_usize(e_count).unwrap();

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

impl<T, L> Graph<T, L> for PseudoWTDigraph<T, L>
where
    T: Unsigned + ToPrimitive + Integer + Hash + Copy,
    L: Clone,
{
    fn add_edge(&mut self, v: T, w: T) {
        todo!()
    }

    fn add_node_label(&mut self, v: T, label: L) {
        if v > self.v_count - T::one() {
            panic!("Vertex doesn't exist.");
        }

        self.node_labels.insert(v, label);
    }

    fn v_count(&self) -> T {
        self.v_count
    }

    fn e_count(&self) -> T {
        self.e_count
    }

    fn get_label(&self, v: T) -> Option<&L> {
        self.node_labels.get(&v)
    }
}

impl<T, L> Directed<T> for PseudoWTDigraph<T, L>
where
    T: Unsigned + ToPrimitive + Copy,
{
    fn outgoing_edges(&self, vertex: T) -> Vec<T> {
        let mut v_adj: Vec<T> = Vec::new();
        let v = vertex.to_usize().unwrap(); // this won't work if v is of type u128

        let start = self.starting_indices.select1(v) + v; // this won't work if v is of type u128
        let end = self.starting_indices.select1(v + 1) + v + 1; // neither will this

        if start > self.sequence.len() || start == end {
            return Vec::new();
        }

        for i in start..end {
            v_adj.push(self.sequence[i]); // this won't work for qwt of course; we'll have to use get() there
        }

        return v_adj;
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
