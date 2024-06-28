use std::collections::HashMap;

// todo: change L to &L whereever possible!

pub trait Graph<T> {
    // will be implemented by all graphs
    fn add_vertex(&mut self, vertex: T) -> usize; // adds vertex at given index; use at users own risk; if vertex doesn't exist (i.e. vertex is less than wt_adj.len()), it just adds it, if it does, it must not have incoming or outgoing edges
                                                  // for unlabeled Graphs : vertex = index; removes all incoming and outgoing edges from index and returns the index(mainly used for labeled graphs)
                                                  // for labeled Graphs : vertex = label; appends a vertex and returns the index of the vertex
    fn e_count(&self) -> usize;
    // returns the number of edges in graph
    fn v_count(&self) -> usize;
    // returns the number of vertices in graph
    // fn vertex_deleted(&self, vertex: T) -> bool;
    // checks if the vertex is deleted, deleted vertices are stored in deleted_vertices
    fn delete_edge(&mut self, from: T, to: T);
    // deletes an edge from `from` to `to`
    fn edge_exists(&self, from: T, to: T) -> bool;
    // returns if there is an edge from `from` to `to`
    fn delete_vertex(&mut self, vertex: T); // should eventually be changed to return a Result type
                                            // for unlabeled : vertex = index; deletes a vertex at index
                                            // for labeled : vertex = label, convert label to index, then use delete_vertex of non label parent
    fn vertex_exists(&self, vertex: T) -> bool;

    fn shrink(&mut self) -> HashMap<usize, usize>; // removes all unconnected vertices from bitmap; only allowed, if has_uncommitted_edits == false; returns a Hashmap with old indices as keys and new indices as values
                                                   // can only be used after commit_edits; all deleted vertices will be removed ( index will shift )
                                                   // returns hashmap with deleted indices
                                                   // bitmap changes
}
pub trait Directed<T> {
    fn outgoing_edges(&self, vertex: T) -> Vec<T>; // should probably be changed to return an iterator instead
                                                   // returns a Vec<T> of outgoing edges of vertex
    fn incoming_edges(&self, vertex: T) -> Vec<T>; // should probably be changed to return an iterator instead
                                                   // returns a Vec<T> of incoming edges of vertex
    fn delete_outgoing_edges(&mut self, vertex: T); // deletes all outgoing edges of vertex; should return a Result
                                                    // deletes outgoing edges of vertex
    fn delete_incoming_edges(&mut self, vertex: T); // deletes all incoming edges of vertex; should return a Result
                                                    // deletes incoming edges of vertex
}
pub trait Undirected<T> {
    fn edges(&self, vertex: T) -> Vec<T>;
    //returns all edges from vertex
    fn delete_edges_from(&mut self, vertex: T);
    //deletes all edges from vertex
}
pub trait UnLabeled<T> {
    fn append_vertex(&mut self) -> usize; //fn append_vertex(&mut self) -> usize; // adds vertex at position wt_adj.len() or at index of lowest deleted vertex (if that change hasn't been committed)
                                          // can't use append for labeled Graphs, since add_vertex works as an append for labeled graphs
}
pub trait Labeled<L> {
    fn edit_label(&mut self, old_label: L, new_label: L); // true if last item in uncommitted edits for v is Edit::DeleteSelf; should return a Result
                                                          //changes old_Label to new_label
    fn get_label(&self, vertex: usize) -> Option<&L>;
    //input:index, output Option<&L>; check in vec[vertex] for label
    fn get_index(&self, label: &L) -> Option<usize>; // returns the index of the vertex with the given label
                                                     //input:Label, output Option<&usize>; check in hashmaps value
}
pub trait Unweighted<T> {
    fn add_edge(&mut self, from: T, to: T);
    // adds an edge from `from` to `to`
}
pub trait Weighted<T, W> {
    fn add_edge(&mut self, from: T, to: T, weight: W);
    // adds an edge from `from` to `to` with weight `weight`
    fn edit_weight(&mut self, from: T, to: T, weight: W);
    // edits the weight from `from` to `to` with weight `weight`
    fn get_weight(&mut self, from: T, to: T) -> W;
    // returns the weight from `from` to `to`
}
pub trait WT<T> {
    //todo, uncommited changes for edges missing?
    fn v_count_updated(&self) -> usize;

    fn commit_edits(&mut self);
    // commits all edits, including labels, weights, edges, vertices; wt will be rebuild here
    // fn get_uncommitted_edits(&self) -> ?;
    // returns all uncommmited_edits since last commit
    fn edge_exists_updated(&self, from: T, to: T) -> bool;
    // returns if the edge exists with uncommitted changes
    fn discard_edits(&mut self);
    // drops all uncommitted changes
    fn vertex_exists_updated(&self, vertex: T) -> bool;
    // same as vertex_exists but with uncommitted edits
}
pub trait WTUndirected<T> {
    fn edges_updated(&self, vertex: usize) -> Vec<T>;
    // returns all edges from vertex, including uncommitted edits, might be expensive
}
pub trait WTDirected<T> {
    fn outgoing_edges_updated(&self, vertex: T) -> Vec<T>;
    // if there are no incoming edges, this returns an empty list
    // returns all outgoing edges from vertex, including uncommitted edits, might be expensive
    fn incoming_edges_updated(&self, vertex: T) -> Vec<T>;
    // if there are no outgoing edges, this returns an empty list
    // returns all incoming edges from vertex, including uncommitted edits, might be expensive
}
pub trait WTWeighted<T, W> {
    fn get_weight_updated(&mut self, from: T, to: T) -> W;
    // returns the updated weight from `from` to `to`
}
pub trait WTLabeled<L> {
    fn get_label_updated(&self, index: usize) -> Option<&L>;
    fn get_index_updated(&self, label: &L) -> Option<usize>;
}

// additional graph functionality
pub enum ShortestPathAlgorithm {
    Dijkstra,
    BFS,
    BellmanFord,
    AStar,
}

pub trait GraphSearch {
    fn shortest_path(&self, from: usize, to: usize, mode: ShortestPathAlgorithm) -> Vec<usize>; // returns the shortest path from `from` to `to` using breadth first search

    fn shortest_paths(&self, mode: ShortestPathAlgorithm) -> Vec<Vec<usize>>; // shortest paths from all vertices to all other vertices

    fn connected_components(&self) -> Vec<Vec<usize>>; // returns all groups of vertices that are connected; makes no sense for directed graphs; default: bfs

    fn connected(&self, from: usize, to: usize) -> bool; // is a connected to b? default: bfs
}
