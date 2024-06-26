/// This module is offering functionality to instantiate complete graphs from files. The file needs to contain the graph's data in the following format:
/// <number of vertices in the graph>
/// <number of edges in the graph>
/// A. (if the graph doesn't contain weights)
/// <vertex as usize> <adjacent vertex as usize>
/// B. (if the graph contains weights) 
/// <vertex as usize> <adjacent vertex as usize> <weight as W>
/// C. (if the graph doesn't contain weights and is labeled) 
/// <vertex as L> <adjacent vertex as L> 
/// D. (if the graph contains weights) 
/// <vertex as L> <adjacent vertex as L>  <weight as W> \
/// Find example API calls in the documentation for the import functions
pub mod from_file;
pub mod traits;
pub mod graph;
pub mod wt;

// Enums
#[derive(PartialEq, Debug)]
pub enum Edit<T> {
    Add(T),
    Delete(T),
}
