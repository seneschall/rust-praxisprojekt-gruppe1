// USE ?

// UNIT-TESTS for Graph and Weighted Graph
#[cfg(test)]
mod test {
    use super::*;
    const V_COUNT: u32 = 10;
    #[test]
    fn create_new_graph() {
        let mut graph: Graph<u32, u32> = Graph::new(V_COUNT);
        graph.add_edge(3, 2);
        graph.add_edge(5, 0);
        assert_eq!(graph.edges(3), vec![2u32]);
        assert_eq!(graph.edges(5), vec![0u32]);
        assert_eq!(graph.e_count(), 2);
        // code for graph_weighted
    }
    #[test]
    fn create_graph_from_adj() {
        // code for graph
    }
    #[test]
    fn create_weighted_graph_from_adj() {
        // code for graph_weighted
    }
    #[test]
    fn add_edge_to_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn add_vertex_to_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn append_vertex_to_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn remove_edge_from_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn remove_vertex_from_graphs() {
        // code for graph
        // code for graph_weighted        
    }
    #[test]
    fn add_label_to_graphs() {
        let mut graph: Graph<u32, String> = Graph::new(V_COUNT);
        graph.add_vertex_label(0, String::from("test"));
        assert_eq!(graph.get_label(0), Some(&String::from("test")));
        assert_eq!(graph.get_label(1), None);
        // code for graph_weighted
    }
    #[test]
    fn edit_label_on_graphs() {
        // code for graph
        // code for graph_weighted
    } 
    #[test]
    fn edges_on_graphs() {
        // code for graph
        // code for graph_weighted
    }
    #[test]
    fn print_weight_of_graph_edge() {
        // code for graph_weighted
    }
    #[test]
    fn add_edit_weight_of_graph_edge() {
        // code for graph_weighted
    }
    #[test]
    fn delete_weight_of_graph_edge() {
        // code for graph_weighted
    }
}

// Graph - definition and methods

pub struct Graph<L> // same as digraph?!
{

}

impl<L> Graph<L>  // same as digraph?!
{
    
}

impl<L> Graph<L> for Graph<L>
{

}

impl<L> Undirected for Graph<L>
{
  
}


// Weighted Graph - definition and methods

pub struct Graph_Weighted<L> 
{
 
}

impl<L> Graph_Weighted<L> 
{
  
}

impl<L> Graph<L> for Graph_Weighted<L>
{

}

impl<L> Undirected for Graph_Weighted<L>
{
  
}

impl<L> Weighted for Graph_Weighted<L>
