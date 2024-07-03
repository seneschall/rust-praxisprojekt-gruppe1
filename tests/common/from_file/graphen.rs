use std::fs;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Debug;
use core::hash::Hash;

use crate::graph::
    {directed::Digraph,labeled_directed::LabeledDigraph,weighted_directed::WeightedDigraph, labeled_weighted_directed::LabeledWeightedDigraph,
        undirected::UGraph,labeled_undirected::LabeledUGraph,weighted_undirected::WeightedUGraph,labeled_weighted_undirected::LabeledWeightedUGraph};


/// read in a digraph from file
pub fn create_digraph(filepath: &str) -> Digraph {
    let (v_count, e_count, adj) = read_adj_unlabeled_unweighted_directed(filepath);
    Digraph::from_adjacency_list(v_count, e_count, adj)
}

// helper for digraph
fn read_adj_unlabeled_unweighted_directed(filename: &str) -> (usize,usize,Vec<Vec<usize>>) {

    // opens the file and read the file in lines
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let mut lines = content.lines();

    // read v_count, e_count
    let v_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("v count not usize");
    let e_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("e count not usize");

    // create Vec<Vec<usize>> in the size of v_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 

    // loops over all lines
    for line in lines {
        let line = line.trim();

        // returns a iterator of type usize , consisting of the (two) elements in the current line, both in usize
        let mut numbers = line.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());

        if let (Some(vertex), Some(adjacent)) = (numbers.next(), numbers.next()) {
            // this line pushes the pairs onto the adj-vector        
            adjacency_list[vertex].push(adjacent);

        } else {
            eprintln!("Invalid line: {}", line);
            // here better error handling needed
        }
    } assert_eq!(v_count, adjacency_list.len());

    (v_count, e_count, adjacency_list)
}

/// read in an ugraph from file
pub fn create_ugraph(filepath: &str) -> UGraph {
    let (v_count, e_count, adj) = read_adj_unlabeled_unweighted_undirected(filepath);
    UGraph::from_adjacency_list(v_count, e_count, adj)
}


// helper for ugraph
fn read_adj_unlabeled_unweighted_undirected(filename: &str) -> (usize, usize, Vec<Vec<usize>>) {

    // opens the file and read the file in lines
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let mut lines = content.lines();

    // read v_count, e_count
    let v_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("v count not usize");
    let e_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("e count not usize");

    // create Vec<Vec<T>> in the size of v_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    // loops over all lines
    for line in lines {
        // remove possible whitespaces and delimiters
        let line = line.trim();
        // returns a iterator of type usize , consisting of the (two) elements in the current line
        let mut numbers = line.split_whitespace()
        // this line??
        .filter_map(|s| s.parse::<usize>().ok());
        // store the (two) elements in 'vertex' and 'adjacent'
        if let (Some(vertex), Some(adjacent)) = (numbers.next(), numbers.next()) {
            // this line pushes the pairs onto the adj-vector      
            // only this line would differ in an explicit implementation of read_adj_undirected   
            if vertex <= adjacent {
                adjacency_list[vertex].push(adjacent); 
            }
            else {
                adjacency_list[adjacent].push(vertex); 
            }
        } else {
            eprintln!("Invalid line: {}", line);
            // here better error handling needed
        }
    } assert_eq!(v_count, adjacency_list.len());
    (v_count, e_count, adjacency_list)
}




/// read in a weighted digraph from file
pub fn create_weighted_digraph<W>(filepath: &str) -> WeightedDigraph<W>
where W: FromStr + Debug + Clone, {
    let (v_count, e_count, adj, weights) = read_adj_unlabeled_weighted_directed::<W>(filepath);
    let mut i: usize = 0;
    let mut adj_with_weights : Vec<Vec<(usize,W)>> = Vec::new();
    for vertex in adj {
        let mut myvec = Vec::new();
        for edge in vertex{
            myvec.push((edge, weights.get(&(i,edge)).unwrap().clone()))
        }
        adj_with_weights.push(myvec);
        i += 1;
    }
    WeightedDigraph::from_adjacency_list(v_count, e_count, adj_with_weights)
}

// helper for W-digraph
fn read_adj_unlabeled_weighted_directed<W>(filename: &str) -> (usize, usize, Vec<Vec<usize>>,HashMap<(usize,usize),W>)
where W: FromStr, W: Debug {

    // opens the file, read the file in lines
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let mut lines = content.lines();

    // read v_count, e_count
    let v_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("v count not usize");
    let e_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("e count not usize");

    // create adjacency list in the size of v_count and HashMap<(usize,usize),W> in the size of e_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    let mut weights: HashMap<(usize,usize),W> = HashMap::with_capacity(e_count);

    // loops over all lines
    for line in lines {
        let line = line.trim();
        let elements : Vec<&str> = line.split(' ').collect();

        // line must consist of two usize-type variables and one w-size type variables
        let vertex = elements[0].parse::<usize>().expect("need error handling");
        let adjacent = elements[1].parse::<usize>().expect("need error handling");
        let weight = elements[3].parse::<W>().ok().unwrap();

        adjacency_list[vertex].push(adjacent);
        weights.insert((vertex,adjacent),weight);
            // need error handling
    } assert_eq!(v_count, adjacency_list.len());
    
    (v_count, e_count, adjacency_list, weights)
}

/// read in a weighted ugraph from file (W-ugraph)
pub fn create_weighted_ugraph<W>(filepath: &str) -> WeightedUGraph<W> 
where W: Debug, W : FromStr, W: Copy
{
    let (e_count, v_count, adj, weights) = read_adj_unlabeled_weighted_undirected::<W>(filepath);
    let mut i: usize = 0;
    let mut adj_with_weights : Vec<Vec<(usize,W)>> = Vec::new();
    for vertex in adj {
        let mut myvec = Vec::new();
        for edge in vertex{
            myvec.push((edge, weights.get(&(i,edge)).unwrap().clone()))
        }
        adj_with_weights.push(myvec);
        i += 1;
    }
    WeightedUGraph::from_adjacency_list(v_count, e_count, adj_with_weights)
}

// helper for W-ugraph
fn read_adj_unlabeled_weighted_undirected<W>(filename: &str) -> (usize, usize, Vec<Vec<usize>>,HashMap<(usize,usize),W>)
where W: FromStr, W: Debug {

    // opens the file, read the file in lines
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let mut lines = content.lines();

    // read v_count, e_count
    let v_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("v count not usize");
    let e_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("e count not usize");

    // create Vec<Vec<T>> in the size of v_count and HashMap<(usize,usize),W> in the size of e_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    let mut weights: HashMap<(usize,usize),W> = HashMap::with_capacity(e_count);

    // loops over all lines
    for line in lines {
        let line = line.trim();

        let elements : Vec<&str> = line.split(' ').collect();

        // the line must consists of two usize-types and one W-types , in that order
        let vertex = elements[0].parse::<usize>().expect("need error handling");
        let adjacent = elements[1].parse::<usize>().expect("need error handling");
        let weight = elements[3].parse::<W>().ok().unwrap();


        if vertex <= adjacent {
            adjacency_list[vertex].push(adjacent); 
        }
        else {
            adjacency_list[adjacent].push(vertex); 
        }
        weights.insert((vertex,adjacent),weight);

    } assert_eq!(v_count, adjacency_list.len());

    (v_count, e_count, adjacency_list, weights)
    
}




/// read in an directed labeled graph from file
pub fn create_labeled_digraph<L>(filepath: &str) -> LabeledDigraph<L> 
where L: Eq + Clone + Hash + FromStr
{
    let (v_count, e_count, adj, labels, indices) = read_adj_labeled_unweighted_directed(filepath);
    LabeledDigraph::from_adjacency_list(v_count, e_count, adj, labels) 
}

// helper for L-digraph
fn read_adj_labeled_unweighted_directed<L>(filepath: &str) -> (usize, usize, Vec<Vec<usize>>, Vec<L>, HashMap<L,usize>) 
where L: Eq + Hash + Clone + FromStr {

    let content = fs::read_to_string(filepath).expect("Unable to open file");
    let mut lines = content.lines();
    let v_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("v count not usize");
    let e_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("e count not usize");

    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    let mut vec_vetrex_labels : Vec<L> = vec![];
    let mut hashmap_labels_vertex : HashMap<L,usize> = HashMap::new();

    // loops over all lines
    let mut i: usize = 0;
    for line in lines {

        // read out L - L 
        let line = line.trim();
        let elements : Vec<&str> = line.split(' ').collect();
        let from = elements[0].parse::<L>().ok().unwrap();
        let to = elements[1].parse::<L>().ok().unwrap();

        // this line finds the index of 'from' in the adj-list and stores it in 'from_vector'
        let mut from_vector = hashmap_labels_vertex.get(&from);
        if from_vector == None {
            from_vector = Some(&0);
        } 
        let from_vector = from_vector.unwrap();

        // this line finds the index of 'to' in the adj-list and stores it in 'to_vector'
        let mut to_vector = hashmap_labels_vertex.get(&to);
        if to_vector == None {
            to_vector = Some(&0);
        } 
        let to_vector = to_vector.unwrap();
      
        // this line pushes the edge onto the adj-vector  
        adjacency_list[*from_vector].push(*to_vector); 

        // this line updates the label-vector and index-hashmap
        if !vec_vetrex_labels.contains(&from) {
            vec_vetrex_labels.push(from.clone());
            hashmap_labels_vertex.insert(from, i);
            i += 1; 
        } 
    }
    (v_count, e_count,adjacency_list, vec_vetrex_labels, hashmap_labels_vertex)
    
} 
// assert_eq!(v_count, adjacency_list.len());

/// read in a labeled ugraph from file (L-ugraph)
pub fn create_labeled_ugraph<L>(filepath: &str) -> LabeledUGraph<L> 
where L: Eq + Clone + Hash + FromStr + Ord{

    let (v_count, e_count, adj, labels, indices) = read_adj_labeled_unweighted_undirected(filepath);
    LabeledUGraph::from_adjacency_list(v_count, e_count, adj, labels)
    }

// helper for L-ugraph
fn read_adj_labeled_unweighted_undirected<L>(filepath: &str) -> (usize, usize, Vec<Vec<usize>>, Vec<L>, HashMap<L,usize>) 
where L: Eq + Clone + Hash + FromStr + Ord
{

    let content = fs::read_to_string(filepath).expect("Unable to open file");
    let mut lines = content.lines();
    let v_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("v count not usize");
    let e_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("e count not usize");

    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    let mut vec_vetrex_labels : Vec<L> = vec![];
    let mut hashmap_labels_vertex : HashMap<L,usize> = HashMap::new();

    // loops over all lines
    let mut i: usize = 0;
    for line in lines {

        // read out L - L 
        let line = line.trim();
        let elements : Vec<&str> = line.split(' ').collect();
        let from = elements[0].parse::<L>().ok().unwrap();
        let to = elements[1].parse::<L>().ok().unwrap();

        // this line finds the index of 'from' in the adj-list and stores it in 'from_vector'
        let mut from_vector = hashmap_labels_vertex.get(&from);
        if from_vector == None {
            from_vector = Some(&0);
        } 
        let from_vector = from_vector.unwrap();

        // this line finds the index of 'to' in the adj-list and stores it in 'to_vector'
        let mut to_vector = hashmap_labels_vertex.get(&to);
        if to_vector == None {
            to_vector = Some(&0);
        } 
        let to_vector = to_vector.unwrap();

        if from_vector <= to_vector {
            // this line pushes the edge onto the adj-vector        
                adjacency_list[*from_vector].push(*to_vector); 
        }
        else {
            // this line pushes the edge onto the adj-vector  
            adjacency_list[*to_vector].push(*from_vector); 
        }

        // this line updates the label-vector and index-hashmap
        if !vec_vetrex_labels.contains(&from) {
            vec_vetrex_labels.push(from.clone());
            hashmap_labels_vertex.insert(from, i);
            i += 1; 
        } 
    }
    (v_count, e_count,adjacency_list, vec_vetrex_labels, hashmap_labels_vertex)

}
//  assert_eq!(v_count, adjacency_list.len());
    


/// read in a weighted labeled digraph from file (W-L-digraph)
pub fn create_labeled_weighted_digraph<L,W>(filepath: &str) -> LabeledWeightedDigraph<L,W> 
where W: FromStr + Debug + Hash + Clone + Eq ,L : Hash + Eq + Clone + Debug + FromStr {
    let (v_count, e_count, adj, weights, labels, indices) = read_adj_labeled_weighted_directed::<L,W>(filepath);
    let mut i: usize = 0;
    let mut adj_with_weights : Vec<Vec<(usize,W)>> = Vec::new();
    for vertex in adj {
        let mut myvec = Vec::new();
        for edge in vertex {
            myvec.push((edge, weights.get(&(i,edge)).unwrap().clone()))
        }
        adj_with_weights.push(myvec);
        i += 1;
    }
    LabeledWeightedDigraph::from_adjacency_list(v_count, e_count, adj_with_weights, labels)
}

// helper for W-L-digraph
fn read_adj_labeled_weighted_directed<L,W>(filename: &str) -> (usize, usize, Vec<Vec<usize>>,HashMap<(usize,usize),W>, Vec<L>, HashMap<L,usize>)
where W: FromStr + Debug, L : Hash + Clone + FromStr + Debug + FromStr + Eq {

    // opens the file, read the file in lines
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let mut lines = content.lines();

    // read v_count, e_count
    let v_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("v count not usize");
    let e_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("e count not usize");

    // create adjacency list in the size of v_count and HashMap<(usize,usize),W> in the size of e_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    let mut weights: HashMap<(usize,usize),W> = HashMap::with_capacity(e_count);
    let mut vec_vetrex_labels : Vec<L> = vec![];
    let mut hashmap_labels_vertex : HashMap<L,usize> = HashMap::new();

    // loops over all lines
    let mut i: usize = 0;
    for line in lines {

        // read out L - L - W
        let line = line.trim();
        let elements : Vec<&str> = line.split(' ').collect();
        let from = elements[0].parse::<L>().ok().unwrap();
        let to = elements[1].parse::<L>().ok().unwrap();
        let weight = elements[3].parse::<W>().ok().unwrap();

        // this line finds the index of 'from' in the adj-list and stores it in 'from_vector'
        let mut from_vector = hashmap_labels_vertex.get(&from);
        if from_vector == None {
            from_vector = Some(&0);
        } 
        let from_vector = from_vector.unwrap();

        // this line finds the index of 'to' in the adj-list and stores it in 'to_vector'
        let mut to_vector = hashmap_labels_vertex.get(&to);
        if to_vector == None {
            to_vector = Some(&0);
        } 
        let to_vector = to_vector.unwrap();

        // this pushes the edge onto the adj-list
        adjacency_list[*from_vector].push(*to_vector);
        // this pushes the edge and it's weight onto the weight hashmap
        weights.insert((*from_vector, *to_vector), weight);
        // this fills the vec_vertex_labels and hashmap_vertex_labels
        if !vec_vetrex_labels.contains(&from) {
            vec_vetrex_labels.push(from.clone());
                hashmap_labels_vertex.insert(from, i);
                i += 1;
            }  

    } assert_eq!(v_count, adjacency_list.len());
    
    (v_count, e_count, adjacency_list, weights, vec_vetrex_labels, hashmap_labels_vertex)

}

/// read in a weighted labeled ugraph from file (W-L-ugraph)
pub fn create_weighted_labeled_ugraph<L,W>(filepath: &str) -> LabeledWeightedUGraph<L,W> 
where W: FromStr + Debug + Hash + Eq + Clone  , L : Hash + Eq + Clone + Debug + FromStr {
    let (v_count, e_count, adj, weights, labels, indices) = read_adj_labeled_weighted_directed::<L,W>(filepath);
    let mut i: usize = 0;
    let mut adj_with_weights : Vec<Vec<(usize,W)>> = Vec::new();
    for vertex in adj {
        let mut myvec : Vec<(usize,W)>= Vec::new();
        for edge in vertex {
            myvec.push((edge, weights.get(&(i,edge)).unwrap().clone()))
        }
        adj_with_weights.push(myvec);
        i += 1;
    }
    LabeledWeightedUGraph::from_adjacency_list(v_count, e_count, adj_with_weights, labels)
}

// helper for W-L-ugraph
fn read_adj_labeled_weighted_undirected<L,W>(filename: &str) -> (usize, usize, Vec<Vec<usize>>,HashMap<(usize,usize),W>, Vec<L>, HashMap<L,usize>)
where W: FromStr + Debug, L : Hash + Clone + FromStr + Debug + FromStr + Eq {

    // opens the file, read the file in lines
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let mut lines = content.lines();

    // read v_count, e_count
    let v_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("v count not usize");
    let e_count = lines.next().expect("v count not found in first line").parse::<usize>().expect("e count not usize");

    // create adjacency list in the size of v_count and HashMap<(usize,usize),W> in the size of e_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    let mut weights: HashMap<(usize,usize),W> = HashMap::with_capacity(e_count);
    let mut vec_vetrex_labels : Vec<L> = vec![];
    let mut hashmap_labels_vertex : HashMap<L,usize> = HashMap::new();

    // loops over all lines
    let mut i: usize = 0;
    for line in lines {

        // read out L - L - W
        let line = line.trim();
        let elements : Vec<&str> = line.split(' ').collect();
        let from = elements[0].parse::<L>().ok().unwrap();
        let to = elements[1].parse::<L>().ok().unwrap();
        let weight = elements[3].parse::<W>().ok().unwrap();

        // this line finds the index of 'from' in the adj-list and stores it in 'from_vector'
        let mut from_vector = hashmap_labels_vertex.get(&from);
        if from_vector == None {
            from_vector = Some(&0);
        } 
        let from_vector = from_vector.unwrap();

        // this line finds the index of 'to' in the adj-list and stores it in 'to_vector'
        let mut to_vector = hashmap_labels_vertex.get(&to);
        if to_vector == None {
            to_vector = Some(&0);
        } 
        let to_vector = to_vector.unwrap();

        // this pushes the edge onto the adj-list
        if *from_vector <= *to_vector {
            // this line pushes the pairs onto the adj-vector        
            adjacency_list[*from_vector].push(*to_vector); 
        }
        else {
            // this line pushes the pairs onto the adj-vector  
            adjacency_list[*to_vector].push(*from_vector); 
        }

        // this pushes the edge and it's weight onto the weight hashmap
        weights.insert((*from_vector, *to_vector), weight);
        // this fills the vec_vertex_labels and hashmap_vertex_labels
        if !vec_vetrex_labels.contains(&from) {
            vec_vetrex_labels.push(from.clone());
                hashmap_labels_vertex.insert(from, i);
                i += 1;
            }  

    } assert_eq!(v_count, adjacency_list.len());
    
    (v_count, e_count, adjacency_list, weights, vec_vetrex_labels, hashmap_labels_vertex)
    
}