use std::fs;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Debug;

// note: when reading in undirected graphs, the handling of the adj-list must be minded.

// extract the adjacency list representing all the edges between two vertices from the input file.
pub(crate) fn read_adj_directed_unweighted(filename: &str) -> Vec<Vec<usize>> {
    // opens the file
    let content = fs::read_to_string(filename).expect("Unable to open file");
    // read the file in lines
    let mut lines = content.lines();
    // read v_count
    let v_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // skip e_count
    lines.next();
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
            adjacency_list[vertex].push(adjacent);
        } else {
            eprintln!("Invalid line: {}", line);
            // here better error handling needed
        }
    } assert_eq!(v_count, adjacency_list.len());
    adjacency_list
}

// extract the adjacency list representing all the edges between two vertices from the input file.
pub(crate) fn read_adj_undirected_unweighted(filename: &str) -> Vec<Vec<usize>> {
    // opens the file
    let content = fs::read_to_string(filename).expect("Unable to open file");
    // read the file in lines
    let mut lines = content.lines();
    // read v_count
    let v_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // skip e_count
    lines.next();
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
    adjacency_list
}

// extract the adjacency list representing all the edges between two vertices from the input file, plus a hashmap storing their weights
pub(crate) fn read_adj_weighted_directed<W>(filename: &str) -> (Vec<Vec<usize>>,HashMap<(usize,usize),W>)
where W: FromStr, W: Debug {
    // opens the file
    let content = fs::read_to_string(filename).expect("Unable to open file");
    // read the file in lines
    let mut lines = content.lines();
    // read v_count
    let v_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // read e_count
    let e_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // create Vec<Vec<T>> in the size of v_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    // create HashMap<(usize,usize),W> in the size of e_count
    let mut weights: HashMap<(usize,usize),W> = HashMap::with_capacity(e_count);
    // loops over all lines
    for line in lines {
        // remove possible whitespaces and delimiters
        let line = line.trim();
        let elements : Vec<&str> = line.split(' ').collect();
        let vertex = elements[0].parse::<usize>().expect("need error handling");
        let adjacent = elements[1].parse::<usize>().expect("need error handling");
        let weight = elements[3].parse::<W>().ok().unwrap();
        adjacency_list[vertex].push(adjacent);
        weights.insert((vertex,adjacent),weight);
            // need error handling
    } assert_eq!(v_count, adjacency_list.len());
    (adjacency_list, weights)
}

// extract the adjacency list representing all the edges between two vertices from the input file, plus a hashmap storing their weights
pub(crate) fn read_adj_weighted_undirected<W>(filename: &str) -> (Vec<Vec<usize>>,HashMap<(usize,usize),W>)
where W: FromStr, W: Debug {
    // opens the file
    let content = fs::read_to_string(filename).expect("Unable to open file");
    // read the file in lines
    let mut lines = content.lines();
    // read v_count
    let v_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // read e_count
    let e_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // create Vec<Vec<T>> in the size of v_count
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; v_count]; 
    // create HashMap<(usize,usize),W> in the size of e_count
    let mut weights: HashMap<(usize,usize),W> = HashMap::with_capacity(e_count);
    // loops over all lines
    for line in lines {
        // remove possible whitespaces and delimiters
        let line = line.trim();
        let elements : Vec<&str> = line.split(' ').collect();
        let vertex = elements[0].parse::<usize>().expect("need error handling");
        let adjacent = elements[1].parse::<usize>().expect("need error handling");
        let weight = elements[3].parse::<W>().ok().unwrap();
        if vertex <= adjacent {
            adjacency_list[vertex].push(adjacent);
            weights.insert((vertex,adjacent),weight);
        }
        else {
            adjacency_list[adjacent].push(vertex);
            weights.insert((adjacent,vertex),weight);
        }
        adjacency_list[vertex].push(adjacent);
        weights.insert((vertex,adjacent),weight);
            // need error handling
    } assert_eq!(v_count, adjacency_list.len());
    (adjacency_list, weights)
}


// extract the adjacency list representing all the edges between two vertices from the input file.
pub(crate) fn read_adj_undirected(filename: &str) -> Vec<Vec<usize>> {
    // opens the file
    let content = fs::read_to_string(filename).expect("Unable to open file");
    // read the file in lines
    let mut lines = content.lines();
    // read v_count
    let v_count = lines.next().expect("-.-").parse::<usize>().expect("-.-");
    // skip e_count
    lines.next();
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
            adjacency_list[vertex].push(adjacent);
        } else {
            eprintln!("Invalid line: {}", line);
            // here better error handling needed
        }
    } assert_eq!(v_count, adjacency_list.len());
    adjacency_list
}
