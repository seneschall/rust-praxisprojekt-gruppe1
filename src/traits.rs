// todo: change L to &L whereever possible!

pub trait Graph<T> {
    /// For index-based graphs: Adds new vertex at index `vertex`. If the vertex already exists, all incoming and outgoing
    /// edges will be deleted. If the index is greater than the highest used index, all indices inbetween will be
    /// created as empty vertices.
    ///
    /// For label-based graphs: Appends new vertex and saves the label `vertex`.
    ///
    /// In both cases the function returns the index.
    fn add_vertex(&mut self, vertex: T) -> usize; // adds vertex at given index; use at users own risk; if vertex doesn't exist (i.e. vertex is less than wt_adj.len()), it just adds it, if it does, it must not have incoming or outgoing edges
                                                  // for unlabeled Graphs : vertex = index; removes all incoming and outgoing edges from index and returns the index(mainly used for labeled graphs)
                                                  // for labeled Graphs : vertex = label; appends a vertex and returns the index of the vertex
    /// Returns the number of edges in graph
    fn e_count(&self) -> usize;

    /// Returns the number of vertices in graph
    fn v_count(&self) -> usize;

    /// Deletes an edge from `from` to `to`.
    /// Panics if either `from` or `to` don't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation). The deletion has to be committed.
    fn delete_edge(&mut self, from: T, to: T);

    /// Returns true if there is an edge between the vertices `from` and `to`.
    ///
    /// For wavelet tree based graphs, this operates only on the committed vertices (for a function that also
    /// operates on uncommitted vertices see `edge_exists_updated`).
    fn edge_exists(&self, from: T, to: T) -> bool;

    /// Deletes the vertex `vertex`.
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation). The deletion has to be committed.
    fn delete_vertex(&mut self, vertex: T); // should eventually be changed to return a Result type

    /// Returns true if the vertex `vertex` exists.
    ///
    /// For wavelet tree based graphs, this operates only on the committed vertices (for a function that also
    /// operates on uncommitted vertices see `vertex_exists_updated`).
    fn vertex_exists(&self, vertex: T) -> bool;
}

pub trait Directed<T> {
    /// For index based graphs: Returns a vector containing copies of all indices the vertex `vertex` has an outgoing edge to.
    ///
    /// For label based graphs: Returns a vector containing clones of the labels of all vertices the vertex `vertex` has an outgoing edge to.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs, this operates only on the committed vertices (for a function that also
    /// operates on uncommitted vertices see `outgoing_edges_updated`).
    fn outgoing_edges(&self, vertex: T) -> Vec<T>; // should probably be changed to return an iterator instead
                                                   // returns a Vec<&T> of outgoing edges of vertex

    /// For index based graphs: Returns a vector containing copies of all indices the vertex `vertex` has an incoming edge to.
    ///
    /// For label based graphs: Returns a vector containing clones of the labels of all vertices the vertex `vertex` has an incoming edge to.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs, this operates only on the committed vertices (for a function that also
    /// operates on uncommitted vertices see `incoming_edges_updated`).
    fn incoming_edges(&self, vertex: T) -> Vec<T>; // should probably be changed to return an iterator instead
                                                   // returns a Vec<&T> of incoming edges of vertex

    /// Deletes all outgoing edges of vertex `vertex`.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation). The deletion has to be committed.
    fn delete_outgoing_edges(&mut self, vertex: T); // deletes all outgoing edges of vertex; should return a Result
                                                    // deletes outgoing edges of vertex
                                                    // todo: updated!

    /// Deletes all incoming edges of vertex `vertex`.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation). The deletion has to be committed.
    fn delete_incoming_edges(&mut self, vertex: T); // deletes all incoming edges of vertex; should return a Result
                                                    // deletes incoming edges of vertex
                                                    // todo: updated!
}
pub trait Undirected<T> {
    /// For index based graphs: Returns a vector containing copies of all indices the vertex `vertex` has an edge to.
    ///
    /// For label based graphs: Returns a vector containing clones of the labels of all vertices the vertex `vertex` has an edge to.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs, this operates only on the committed vertices (for a function that also
    /// operates on uncommitted vertices see `edges_updated`).
    fn edges(&self, vertex: T) -> Vec<T>;

    /// Deletes all edges connected to vertex `vertex`.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation). The deletion has to be committed.
    fn delete_edges_from(&mut self, vertex: T);
}
pub trait Unlabeled<T> {
    /// Appends a new vertex and returns the index.
    fn append_vertex(&mut self) -> usize; //fn append_vertex(&mut self) -> usize; // adds vertex at position wt_adj.len() or at index of lowest deleted vertex (if that change hasn't been committed)
                                          // can't use append for labeled Graphs, since add_vertex works as an append for labeled graphs

    /// Removes all deleted vertices, shifts the following indices to fill the position (similiar to calling `remove` on a `Vec<T>`), and commits all changes.
    ///
    /// Returns a vector with the new indices at the index of the indices (`None` if the index was deleted).
    ///
    /// # Example
    ///
    /// ```rust
    /// // dg is a `WTDigraph` containing the vertices 0..=3 and some edges.
    /// // ...
    /// dg.delete_vertex(1);
    /// dg.delete_vertex(3);
    /// let new_indices: Vec<Option<usize>> = dg.shrink();
    /// // new_indices = [
    /// //  Some(0),
    /// //  None,
    /// //  Some(1),
    /// //  None,
    /// // ]
    ///```
    fn shrink(&mut self) -> Vec<Option<usize>>;
}

pub trait Labeled<L> {
    /// Changes the name of `old_label` to `new_label`.
    ///
    /// Panics if `old_label` doesn't exist or `new_label` already exists.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn edit_label(&mut self, old_label: L, new_label: L); // true if last item in uncommitted edits for v is Edit::DeleteSelf; should return a Result
                                                          //changes old_Label to new_label

    /// Returns the label of the vertex at index `vertex` or `None` if it doesn't exist.
    ///
    /// For wavelet tree based graphs, this operates only on the committed vertices (for a function that also
    /// operates on uncommitted vertices see `get_label_updated`).
    fn label(&self, vertex: usize) -> Option<&L>;

    /// Returns the index of the vertex at label `vertex` or `None` if it doesn't exist.
    ///
    /// For wavelet tree based graphs, this operates only on the committed vertices (for a function that also
    /// operates on uncommitted vertices see `get_index_updated`).
    fn index(&self, label: &L) -> Option<usize>; // returns the index of the vertex with the given label
                                                 //input:Label, output Option<&usize>; check in hashmaps value

    /// Removes all deleted vertices, shifts the following indices to fill the position (similiar to calling `remove` on a `Vec<T>`), and commits all changes.
    fn shrink(&mut self); // removes all unconnected vertices from bitmap; only allowed, if has_uncommitted_edits == false; returns a Hashmap with old indices as keys and new indices as values
                          // can only be used after commit_edits; all deleted vertices will be removed ( index will shift )
                          // returns hashmap with deleted indices
                          // bitmap changes
}

pub trait Unweighted<T> {
    /// Adds a new edge from vertex `from` to vertex `to`.
    ///
    /// Panics if either `from` or `to` don't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn add_edge(&mut self, from: T, to: T);
    // adds an edge from `from` to `to`
}
pub trait Weighted<T, W> {
    /// Adds a new edge from vertex `from` to vertex `to` with weight `weight`.
    ///
    /// Panics if either `from` or `to` don't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn add_edge(&mut self, from: T, to: T, weight: W);

    /// Changes the weight of the edge from vertex `from` to vertex `to`.
    ///
    /// Panics if the edge doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn edit_weight(&mut self, from: T, to: T, weight: W);

    /// Returns the weight of the edge from vertex `from` to vertex `to`.
    ///
    /// Panics if the edge doesn't exist.
    ///
    /// For wavelet tree based graphs, this operates only on the committed vertices (for a function that also
    /// operates on uncommitted vertices see `get_weight_updated`).
    fn weight(&mut self, from: T, to: T) -> W;
}

pub trait WT<T> {
    /// Returns the number of existing vertices including uncommitted changes.
    fn v_count_updated(&self) -> usize;

    /// Returns the number of existing edges including uncommitted changes.
    fn e_count_updated(&self) -> usize;

    /// Rebuilds the wavelet tree applying all changes since last commit.
    ///
    /// Allows for efficient storage of and faster operation on the graph.
    fn commit_edits(&mut self);

    /// Returns true if there is an edge between the vertices `from` and `to`.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn edge_exists_updated(&self, from: T, to: T) -> bool;

    /// Drops all changes since last commit.
    fn discard_edits(&mut self);

    /// Returns true if the vertex `vertex` exists.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn vertex_exists_updated(&self, vertex: T) -> bool;
}

pub trait WTUndirected<T> {
    /// For index based graphs: Returns a vector containing copies of all indices the vertex `vertex` has an edge to.
    ///
    /// For label based graphs: Returns a vector containing clones of the labels of all vertices the vertex `vertex` has an edge to.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn edges_updated(&self, vertex: T) -> Vec<T>;
}

pub trait WTDirected<T> {
    /// For index based graphs: Returns a vector containing copies of all indices the vertex `vertex` has an outgoing edge to.
    ///
    /// For label based graphs: Returns a vector containing clones of the labels of all vertices the vertex `vertex` has an outgoing edge to.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn outgoing_edges_updated(&self, vertex: T) -> Vec<T>;

    /// For index based graphs: Returns a vector containing copies of all indices the vertex `vertex` has an incoming edge to.
    ///
    /// For label based graphs: Returns a vector containing clones of the labels of all vertices the vertex `vertex` has an incoming edge to.
    ///
    /// Panics if `vertex` doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn incoming_edges_updated(&self, vertex: T) -> Vec<T>;
}
pub trait WTWeighted<T, W> {
    /// Returns the weight of the edge from vertex `from` to vertex `to`.
    ///
    /// Panics if the edge doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn weight_updated(&mut self, from: T, to: T) -> W;
}

pub trait WTLabeled<L> {
    /// Returns the label of the vertex at index `vertex` or `None` if it doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn label_updated(&self, index: usize) -> Option<&L>;

    /// Returns the index of the vertex at label `vertex` or `None` if it doesn't exist.
    ///
    /// For wavelet tree based graphs the function operates on the uncommitted changes (i.e. it considers vertices as
    /// existing even if they haven't been committed after creation).
    fn index_updated(&self, label: &L) -> Option<usize>;
}

// additional graph functionality
// didn't get finished before deadline
// pub enum ShortestPathAlgorithm {
//     Dijkstra,
//     BFS,
//     BellmanFord,
//     AStar,
// }

// pub trait GraphSearch {
//     fn shortest_path(&self, from: usize, to: usize, mode: ShortestPathAlgorithm) -> Vec<usize>; // returns the shortest path from `from` to `to` using breadth first search

//     fn shortest_paths(&self, mode: ShortestPathAlgorithm) -> Vec<Vec<usize>>; // shortest paths from all vertices to all other vertices

//     fn connected_components(&self) -> Vec<Vec<usize>>; // returns all groups of vertices that are connected; makes no sense for directed graphs; default: bfs

//     fn connected(&self, from: usize, to: usize) -> bool; // is a connected to b? default: bfs
// }
