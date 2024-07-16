use crate::graph::directed::Digraph;
use crate::traits::{Graph, Undirected, Unlabeled, Unweighted};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;
// An indexed, mutable graph with undirected edges. (ugraph, ug)
// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UGraph {
    pub(crate) dg: Digraph,
}

impl UGraph {
    // this function instantiiates a new empty ugraph, that must be manually filled with vertices and edges
    pub fn new() -> Self {
        UGraph { dg: Digraph::new() }
    }

    // this function can read in from a vector, but doesn't check that it's valid input, panices instead
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<usize>>) -> Self {
        UGraph {
            dg: Digraph::from_adjacency_list(v_count, e_count, adj),
        }
    }
}

impl Graph<usize> for UGraph {
    // use at own risk!
    // adds a new empty vertex to the graph,
    // by adding an empty vector at the given index, or overwriting the entry with the same key if existant.  
    // adds several new empty Vertices if the given index exceeds the current v_count.
    // returns the index of the new vertex
    // todo ! why does this return a usize?
    fn add_vertex(&mut self, vertex: usize) -> usize {
        self.dg.add_vertex(vertex)
    }

    // return the recent number of edges in the graph
    fn e_count(&self) -> usize {
        self.dg.e_count()
    }

    // return the recent number of vertices in the graph
    fn v_count(&self) -> usize {
        self.dg.v_count()
    }

    // deletes the given edge by deleting the edge from the smaller to the bigger index in dg.
    fn delete_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.dg.delete_edge(from, to);
        } else {
            self.dg.delete_edge(to, from);
        }
    }

    // deletes the vertex at the given index
    // panics if the vertex doesn't exist - should eventually return a Result type
    // if the vertex exists, we mark it in dg's deleted-vertices-Vector, then delete all it's incoming and all it's outgoing mentions.
    fn delete_vertex(&mut self, vertex: usize) {
        self.dg.delete_vertex(vertex);
    }

    // checks if the vertex at the given index exists
    fn vertex_exists(&self, vertex: usize) -> bool {
        self.dg.vertex_exists(vertex)
    }

    // returns if there is an edge between index `from` and index `to`, by searching for an edge between the smaller and the bigger in dg.
    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if from <= to {
            return self.dg.edge_exists(from, to);
        } else {
            return self.dg.edge_exists(to, from);
        }
    }
}

impl Undirected<usize> for UGraph {
    // returns all edges of the given vertex in a vector
    // should probably be changed to return an iterator instead
    // todo ! catch non-existing vertice as input
    fn edges(&self, vertex: usize) -> Vec<usize> {
        let mut edges: Vec<usize> = Vec::new();
        for i in 0..vertex {
            if self.dg.adj[i].contains(&vertex) {
                edges.push(i);
            }
        }
        edges.append(&mut self.dg.adj[vertex].clone());
        edges
    }

    // delete all edges of the given vertex
    // should return a Result
    fn delete_edges_from(&mut self, vertex: usize) {
        for from in 0..vertex {
            if self.dg.adj[from].contains(&vertex) {
                self.delete_edge(from, vertex);
            }
        }
        for to in self.dg.adj[vertex].clone() {
            self.delete_edge(vertex, to);
        }
    }
}

impl Unlabeled<usize> for UGraph {
    // adds a new empty vertex at either the index following the last or at (the lowest available) previously freed index.
    // preserves indexing and never overwrites vertices
    // append_vertex() is not defined for labeled graphs
    // returns the index of the new vertex
    fn append_vertex(&mut self) -> usize {
        self.dg.append_vertex()
    }

    // it removes all vertices in deleted_vertices from the graph, thus altering the adj-list and changing indexing.
    // this lowers adj.len() and resets it to v_count. returns a list comparing the new and old indices.
    fn shrink(&mut self) -> Vec<Option<usize>> {
        return self.dg.shrink();
    }
}

impl Unweighted<usize> for UGraph {
    // adds an edge between the vertices 'from' and 'to', by adding an edge from the smaller to the bigger indice in the dg.
    fn add_edge(&mut self, from: usize, to: usize) {
        if from <= to {
            self.dg.add_edge(from, to);
        } else {
            self.dg.add_edge(to, from);
        }
    }
}
