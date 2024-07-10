use std::collections::HashMap;

use vers_vecs::RsVec;

use crate::graph::undirected::UGraph;
use crate::traits::{Directed, WTDirected};
use crate::traits::{Graph, Undirected, Unlabeled, Unweighted, WTUndirected, WT};
use crate::wt::directed::WTDigraph;
use serde::{Deserialize, Serialize}; // needed because of WTDigraph

/// An indexed wavelet-tree-ugraph with undirected edges. (wt-ugraph)
/// The wt-ugraph holds a wt-digraph. All operations on the wt-digraph can be performed on the wt-ugraph.
/// The only divergent implementations are ...
/// Users can perfom fast operations on the original graph and slower operations on the recent state of the graph.
/// Users can integrate the recent state of the graph into the QW-Tree by rebuilding it using the commit_edits-function.
/// See module wt::directed for the wt-digraph struct definition. See more documentation on function-level and in the crate introduction.
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type. Labels can have any type and are referenced.
#[derive(Serialize, Deserialize)]
pub struct WTUGraph {
    pub(crate) wtd: WTDigraph,
}

impl WTUGraph {
    /// this function instantiiates a wt-ugraph from a given ugraph
    pub fn from_ugraph(ugraph: UGraph) -> Self {
        return WTUGraph {
            wtd: WTDigraph::from_digraph(ugraph.dg),
        };
    }

    /// this function builds a WT-ugraph from a vector sequence of bits, where each 1 represents a vertex, and each 0 another vertex adjacent to the first one (they are connected though an edge)
    /// and a usize-vector representing the id's (indices) of the adjacent vertices.
    pub fn from(sequence: Vec<usize>, starting_indices: RsVec) -> Self {
        return WTUGraph {
            wtd: WTDigraph::from(sequence, starting_indices),
        };
    }
}

impl Graph<usize> for WTUGraph {
    /// use at own risk!
    /// adds a new empty vertex to the graph,
    /// by adding an empty vector at the given index, or overwriting the entry with the same key if existant.  
    /// adds several new empty Vertices if the given index exceeds the current v_count.
    /// returns the index of the new vertex
    // todo ! why does this return a usize?
    fn add_vertex(&mut self, vertex: usize) -> usize {
        self.wtd.add_vertex(vertex)
    }

    /// returns the number of edges in the graph at last commit
    fn e_count(&self) -> usize {
        self.wtd.e_count()
    }

    /// returns the number of vertices in the graph at last commit
    fn v_count(&self) -> usize {
        self.wtd.v_count()
    }

    /// deletes the given edge by deleting the edge from the smaller to the bigger index in wtd.
    fn delete_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.wtd.delete_edge(from, to);
        } else {
            self.wtd.delete_edge(to, from);
        }
    }

    /// deletes the vertex at the given index
    /// panics if the vertex doesn't exist - should eventually return a Result type
    fn delete_vertex(&mut self, vertex: usize) {
        self.wtd.delete_vertex(vertex);
    }

    /// returns if there is an vertex with given index
    fn vertex_exists(&self, vertex: usize) -> bool {
        self.wtd.vertex_exists(vertex)
    }

    /// returns if there is an edge between `from` and `to`
    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if from <= to {
            return self.wtd.edge_exists(from, to);
        } else {
            return self.wtd.edge_exists(to, from);
        }
    }
}
impl Undirected<usize> for WTUGraph {
    /// returns all edges of the given vertex in a vector, by computing it's incoming and outgoing edges in wtd.
    /// should probably be changed to return an iterator instead
    // todo ! catch non-existing vertice as input
    fn edges(&self, vertex: usize) -> Vec<usize> {
        // returns all edges connected to vertex
        let mut edges: Vec<usize> = Vec::new();
        edges = self.wtd.incoming_edges(vertex); // all incoming edges of vertex
        if self.edge_exists(vertex, vertex) {
            for item in self.wtd.outgoing_edges(vertex) {
                if item != vertex {
                    edges.push(item);
                }
            }
            return edges;
        } else {
            edges.append(&mut self.wtd.outgoing_edges(vertex)); // + outgoing edges of vertex
            return edges;
        }
    }

    /// delete all edges of the given vertex
    /// should return a Result
    fn delete_edges_from(&mut self, vertex: usize) {
        // deletes all edges connected to vertex
        for item in self.edges_updated(vertex) {
            self.delete_edge(vertex, item);
        }
    }
}
impl Unlabeled<usize> for WTUGraph {
    /// adds a new empty vertex at either the index following the last or at (the lowest available) previously freed index.
    /// preserves indexing and never overwrites vertices
    /// append_vertex() is not defined for labeled graphs
    /// returns the index of the new vertex
    fn append_vertex(&mut self) -> usize {
        self.wtd.append_vertex()
    }

    /// it removes all vertices in deleted_vertices from the graph, resets deleted_vertices, thus shrinking
    /// wt_adj_len, the updated v_count AND the v_count at last commit. does not commit changes other than vertex deletion.
    /// does commit vertex-deletion and rebuild QW-tree. (expensive!)
    /// return a list containing the deleted vertices?
    fn shrink(&mut self) -> Vec<Option<usize>> {
        return self.wtd.shrink();
    }
}
impl Unweighted<usize> for WTUGraph {
    /// adds an edge between the vertices 'from' and 'to', by adding an edge from the smaller to the bigger indice in the dg.
    fn add_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.wtd.add_edge(from, to);
        } else {
            self.wtd.add_edge(to, from);
        }
    }
}
impl WT<usize> for WTUGraph {
    /// collect and apply all changes in adj_uncommited. rebuild QW-tree. expensive!
    /// set v_count to v_count_updated, e_count to e_count_updated, if present change labels, weights [...].
    /// some changes like deleted vertices are conserved
    fn commit_edits(&mut self) {
        self.wtd.commit_edits();
    }

    /// this function will delete all changes since last commit by resetting the wtd to its state after the last commit.
    fn discard_edits(&mut self) {
        self.wtd.discard_edits();
    }

    /// return true if the vertex still exists and wasn't deleted, or if it was created since since last commit.
    fn vertex_exists_updated(&self, vertex: usize) -> bool {
        self.wtd.vertex_exists_updated(vertex)
    }

    /// return true if the edge still exists and wasn't deleted, or if it was created since since last commit.
    fn edge_exists_updated(&self, from: usize, to: usize) -> bool {
        if from <= to {
            return self.wtd.edge_exists_updated(from, to);
        } else {
            return self.wtd.edge_exists_updated(to, from);
        }
    }

    /// return the recent number of vertices in the graph
    fn v_count_updated(&self) -> usize {
        return self.wtd.v_count_updated();
    }

    fn e_count_updated(&self) -> usize {
        return self.wtd.e_count_updated();
    }
}

impl WTUndirected<usize> for WTUGraph {
    /// return all edges of the given vertex in a vector, which exist and weren't deleted, or were created since since last commit.
    /// should probably be changed to return an iterator instead
    fn edges_updated(&self, vertex: usize) -> Vec<usize> {
        let mut edges: Vec<usize> = Vec::new();
        edges = self.wtd.incoming_edges_updated(vertex); // all incoming edges of vertex
        if self.edge_exists_updated(vertex, vertex) {
            for item in self.wtd.outgoing_edges_updated(vertex) {
                if item != vertex {
                    edges.push(item);
                }
            }
            return edges;
        } else {
            edges.append(&mut self.wtd.outgoing_edges_updated(vertex)); // + outgoing edges of vertex
            return edges;
        }
    }
}
