//! # Introduction to the WT/-graphs-library 
//! This is a powerful tool designed to create and handle various types of graphs. \
//! You can also create normal graphs or create and query so called wt-graphs. \
//! You can call the vertices of your graph by either vertex-indice (usize datatype) or vertex-labels (generic datatype - you choose!). 
//! However, you must choose one method per graph or WT graph, as mixing both is not supported.
//! This crate is divided into two main modules: WT and graphs. Here's an overview of the key functionalities:

//! ### 1. Creating and Managing Graphs
//! Using the Graph module, users can create and manipulate eight different types of graphs, depending on whether its edges have directions, and/or weights, and whether the user wants to address the vertices in the graph by name or by index. 
//! Graphs can be initialized empty and filled later, or they can be read from a file with a specific format. Users can make changes to these graphs as needed.
//! possible todo()!? note which functions are offered here?

//! ### 2. Working with Wavelet-Tree Graphs
//! The WT module allows users to create four types of Wavelet-Tree graphs, which are graphs offering fast operations using the QWT-library-crate.
//! Wavelet-Tree graphs can be created empty, from an existing graph, or using a bitmap and sequence. 
//! Changes to WT graphs are cached and applied only after a commit, which reinitializes the graph. 
//! 
//! The module provides functions to:
//! - Retrieve information about the initialized graph
//! - perform fast operations on the initialized graph, such as outgoing edges, incoming edges, select, access, BFS, DFS, Shortest Path (fast)
//! - perform the same operations on an "updated" graph that includs the changes you have made. (kinda slow)
//! - perform and query changes on the initialized graph (fast)


//! ### 3. Reading Graph Information from Files
//! The library also offers a module called "from_file" which offers 16 different constructor-functions, one for each of the distinct wt-/graph-types that we offer.
//! These require a specifially formatted file containing the information about the graph. please read the module description for the syntax of the file. \

//! With these functionalities, the wt_graphs library provides a comprehensive and efficient solution for working with both traditional and Wavelet-Tree graphs.

pub(crate) mod traits;
pub mod labeled;
pub mod indexed;

/// This trait contains the 8 offer wt-graph objects and fast operations (thanks to QWT-library).
pub mod wt;

/// This trait contains the 8 offered graphs objects.
pub mod graph;

/// This module is offering functionality to instantiate complete graphs from files. The file needs to contain the graph's data in the following format:
/// Note that the lines are numbered for clairity. do not include the line numbers in your wt-/graph-input-file.
/// (1) <number of vertices in the graph>
/// (2) <number of edges in the graph>
/// (3)... <vertex_from>   <vertex_2>         as usize or generic label L(referenced)                               OR
/// (3)... <vertex_from>   <vertex_2>         as usize or generic type L(referenced)  <weight as type weight W>     OR
/// Note also that the number of lines in the file should be equal to the number of the edges that you declare+2.
/// () Find example API calls in the documentation for the import functions
pub mod from_file;

/// This trait will probably only exist in the development versions of this crate, since using it would mean\
/// that the user would be obliged to manually import all traits the datastructure that he's working with should implement.
pub mod traits;

// Enum(s) used by all structures and thus publicly available 
#[derive(PartialEq, Debug)]
pub enum Edit<T> {
    Add(T),
    Delete(T),
}

// note/idea from Celine : could we put "trait Graph<&L>" here just for beauty?