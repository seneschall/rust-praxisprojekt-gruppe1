# wt_graphs (Rust Praxisprojekt – Group 1)

WTGraphs is a graph library using wavelet trees for space efficient storage.
It provides 16 different data types (8 graphs using wavelet trees for efficient storage\
and 8 graphs not using wavelet trees for quickly creating a new graph).

Graphs can either be created and accessed through indices (for efficiency) or labels (for convenience).

## The Data Types

- Not using wavelet trees
  - Index based
    - Digraph (directed graph)
    - UGraph (undirected graph)
    - WeightedDigraph (directed graph with weights)
    - WeightedUGraph (undirected graph with weights)
  - Label based
    - LabeledDigraph (directed graph)
    - LabeledUGraph (undirected graph)
    - LabeledWeightedDigraph (directed graph with weights)
    - LabeledWeightedUGraph (undirected graph with weights)
- Using wavelet trees
  - Index based
    - WTDigraph (directed graph)
    - WTUGraph (undirected graph)
    - WTWeightedDigraph (directed graph with weights)
    - WTWeightedUGraph (undirected graph with weights)
  - Label based
    - LabeledWTDigraph (directed graph)
    - LabeledWTUGraph (undirected graph)
    - LabeledWeightedWTDigraph (directed graph with weights)
    - LabeledWeightedWTUGraph (undirected graph with weights)

## Usage

Here's an example detailing:

- how to use Digraph to create an index based digraph,
- turn it into a WTDigraph,
- edit that structure,
- query the uncommitted edits,
- and how to commit the edits

```rust
// imports Digraph, WTDigraph, and the traits required
// for using these structs
use wt_graphs::prelude::indexed_digraph::*;

fn main() {
  // creates the graph as an easily editable Digraph
  let mut dg = Digraph::new();

  // populating digraph
  for i in 0..=5 {
    dg.add_vertex(i);
  }

  for i in 1..5 {
    dg.add_edge(0, i); // adds an edge from vertex 0 to vertices 1 through 4
  }

  dg.add_edge(1, 3);
  dg.add_edge(2, 4);
  dg.add_edge(5, 0);


  let mut dg = WTDigraph::from_digraph(dg); // creating a wavelet tree based digraph from dg

  let outgoing_of_zero: Vec<usize> = dg.outgoing_edges(0);
  println!("Outgoing edges of vertex at index 0: {:?}", outgoing_of_zero);

  dg.append_vertex(); // creates uncommitted vertex at index 6
  dg.add_edge(0, 6);
  dg.add_edge(6, 0);

  // the following would panic, because the vertex at index 6 hasn't been committed yet:
  // println!("Incoming edges of 6: {:?}", dg.incoming_edges(6));

  dg.vertex_exists(6); // returns false
  dg.vertex_exists_updated(6); // returns true

  let incoming_of_six: Vec<usize> = dg.incoming_edges_updated(6); // works but is inefficient
  println!("Incoming edges of index 6: {:?}", incoming_of_six);

  dg.commit_edits();

  dg.vertex_exists(6); // returns true
  
  dg.delete_vertex(0);

  dg.vertex_exists(0); // returns true
  dg.vertex_exists_updated(0); // returns false

  let new_indices: Vec<Option<usize>> = dg.shrink(); // removes 0 from graph and shifts all indices
  // new_indices = [
  //  None,
  //  Some(0),
  //  Some(1),
  //  Some(2),
  //  Some(3),
  //  Some(4),
  //  Some(5)
  // ]

  let outgoing_zero: Vec<usize> = dg.outgoing_edges(0);

  println!("outgoing edges of 0: {:?}", outgoing_zero); // prints [3] because we created an edge 1->3
  // and that was shifted one index to the left
}
```

## Credit

The library was created by:

- MartinPerezLorenzo
- Selisur
- seneschall
- VeraHannecke
