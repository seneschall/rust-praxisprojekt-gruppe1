//! Preludes import the necessary traits to work with different graph types.
//! Import all preludes or pick the one for the graph type you want to work with.
//!
//! # Example
//!
//! To import all data structures use:
//!
//! ```rust
//! use wt_graphs::prelude::*;
//! ```
//!
//! To import only the indexed digraphs use:
//!
//! ```rust
//! use wt_graphs::prelude::indexed_digraph::*;
//! ```
//!
//! To import only the labeled undirected graphs use:
//!
//! ```rust
//! use wt_graphs::prelude::labeled_ugraph::*;
//! ```

pub use indexed_digraph::*;
pub use indexed_ugraph::*;
pub use weighted_labeled_digraph::*;
pub use weighted_labeled_ugraph::*;

pub mod indexed_digraph {
    pub use crate::graph::directed::Digraph;
    pub use crate::graph::weighted_directed::WeightedDigraph;
    pub use crate::traits::*; // todo (if we have time): import only necessary traits
    pub use crate::wt::directed::WTDigraph;
    pub use crate::wt::weighted_directed::WeightedWTDigraph;
}

pub mod indexed_ugraph {
    pub use crate::graph::undirected::UGraph;
    pub use crate::graph::weighted_undirected::WeightedUGraph;
    pub use crate::traits::*; // todo (if we have time): import only necessary traits
    pub use crate::wt::undirected::WTUGraph;
    pub use crate::wt::weighted_undirected::WeightedWTUGraph;
}

pub mod labeled_digraph {
    pub use crate::graph::labeled_directed::LabeledDigraph;
    pub use crate::graph::labeled_weighted_undirected::LabeledWeightedUGraph;
    pub use crate::traits::*; // todo (if we have time): import only necessary traits
    pub use crate::wt::labeled_directed::LabeledWTDigraph;
    pub use crate::wt::labeled_weighted_directed::LabeledWeightedWTDigraph;
}

pub mod labeled_ugraph {
    pub use crate::graph::labeled_undirected::LabeledUGraph;
    pub use crate::graph::labeled_weighted_undirected::LabeledWeightedUGraph;
    pub use crate::traits::*; // todo (if we have time): import only necessary traits
    pub use crate::wt::labeled_undirected::LabeledWTUGraph;
    pub use crate::wt::labeled_weighted_undirected::LabeledWeightedWTUGraph;
}
