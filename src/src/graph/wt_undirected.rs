// use ?!

// UNIT TESTS FOR WT GRAPH & WEIGHTED WT GRAPH
#cfg[(test)]
mod test {
    #[test]
    fn wtgraph_from_graph() {}
    #[test]
    fn wtgraph_from_sequence_and_bitvec() {}
    #[test]
    fn add_edges_to_wtdigraphs() {
    }

    #[test]
    fn add_vertex_to_wtdigraphs() {
    }

    #[test]
    fn append_vertex_to_wtdigraphs() {
    }

    #[test]
    fn remove_edges_from_wtdigraphs() {
    }

    #[test]
    fn remove_vertices_from_wtdigraphs()

    #[test]
    fn add_label_to_wtdigraphs() {
    }

    #[test]
    fn edit_label_on_wtdigraphs() {
    } 

    #[test]
    fn outgoing_incoming_edges_on_wtdigraphs() {}
    
    #[test]
    fn print_weight_of_wtdigraph_edge() {}
}


pub enum Edit<T>
where
    T: Unsigned + ToPrimitive,
{
    Add(T),
    Delete(T),
    AddSelf,
    DeleteSelf,
}


// IMPLEMENTATION FOR WT GRAPH

pub struct WTGraph<T, L>
// change T to usize
where
    T: Unsigned + ToPrimitive,
{
    v_count: T,                                  // number of vertices
    e_count: T,                                  // number of edges
    wt_adj: QWT256<T>,                           // the wavelet tree adjacency list
    starting_indices: RsVec,                     // starting indices of each
    uncommitted_edits: HashMap<T, Vec<Edit<T>>>, // changes not yet committed to sequence
    has_uncommitted_edits: bool,
    node_labels: HashMap<T, L>, // name given to node format: index: value
}

impl<T,L> WTGraph<T,L> {

}

impl<T,L> Graph<T, L> for WTGraph<T,L>
where
    T: Unsigned + ToPrimitive,
    {

    }

impl<T,L> Undirected<T> for WTGraph<T,L>
where
    T: Unsigned + ToPrimitive,
    {

    }

impl<T,L> WT for WTGraph_Weighted<T,L>
where 
    T: Unsigned + ToPrimitive,
    {

    }

impl<T> WTGraph for WTGraph_Weighted<T,L> 
where 
    T: Unsigned + ToPrimitive,
    {

    }


// IMPLEMENTATION FOR WEIGHTED WT GRAPH
pub struct WTGraph_Weighted<T, L>
// change T to usize
where
    T: Unsigned + ToPrimitive,
{
    v_count: T,                                  // number of vertices
    e_count: T,                                  // number of edges
    wt_adj: QWT256<T>,                           // the wavelet tree adjacency list
    starting_indices: RsVec,                     // starting indices of each
    uncommitted_edits: HashMap<T, Vec<Edit<T>>>, // changes not yet committed to sequence
    has_uncommitted_edits: bool,
    node_labels: HashMap<T, L>, // name given to node format: index: value
}

impl<T,L> WTGraph_Weighted<T,L> {}


impl<T,L> Graph<T, L> for WTGraph_Weighted<T,L>
where
    T: Unsigned + ToPrimitive,
    {

    }


impl<T,L> Undirected<T> for WTGraph_Weighted<T,L>
where
    T: Unsigned + ToPrimitive,
    {

    }

impl<T,L> Weighted<T> for WTGraph_Weighted<T,L>
where
    T: Unsigned + ToPrimitive,
    {

    }

impl<T,L> WT for WTGraph_Weighted<T,L>
where 
    T: Unsigned + ToPrimitive,
    {

    }

impl<T> WTGraph for WTGraph_Weighted<T,L> 
where 
    T: Unsigned + ToPrimitive,
    {

    }