use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::{prelude::WTLabeled, traits::{Graph, Labeled, WT}};

#[test]
fn add_vertex() {

    // requires random LabeledWeightedWTDigraphs and a random mutable variable of type L for testing.
    // runs add_vertex() and verifies the results.

    const USIZE_MAX : usize = 100; 
    let mut rng = thread_rng();
    
    for i in 0..50 {
            println!("creating the {}'th LabeledWeightedWTDigraph",i+1);
            let my_v = (rand::random::<f64>() * USIZE_MAX as f64).floor() as usize;
            let vec_range = (0..my_v).collect::<Vec<usize>>();
            let mut my_e : usize = 0;
            match my_v {
                0 =>    (),
                _ =>    {
                    let max_edges = my_v * (my_v-1);
                    my_e = (rand::random::<f64>() * max_edges as f64).floor() as usize;
                }
            }
            println!("v and e are: {}, {}",my_v,my_e);
            let mut my_adj: Vec<Vec<(usize, _)>> = vec![Vec::new();my_v];
            match my_v {
                0 =>    (),
                _ =>    {
                    for j in 0..my_e {
                        println!("choosing the {}'s edge",j+1);
                        let mut start = 0; let mut end = 0;
                        let mut attempts : usize = 0;
                        loop {
                            attempts += 1;
                            if attempts > 100 {
                                println!("exceeded maximum attempts in third loop");
                                break;
                            }
                            start = *vec_range.choose(&mut rng).unwrap();
                            loop {
                                end = *vec_range.choose(&mut rng).unwrap();
                                if end != start { break; }
                            }
                            if !my_adj[start].iter().any(|&(e, _)| e == end) && !my_adj[end].iter().any(|&(s, _)| s == start) {
                                break;
                            }                       
                            println!("repeated third loop!");
                        }
                        let weight = rand::random::<u8>();
                        my_adj[start].push((end,weight)); 
                    }
                }
            }
            let my_labels = (0..my_v).map(|x| x.to_string() ).collect::<Vec<String>>();   
            println!("labels are: {:?}", my_labels);  
            let my_lwd = crate::graph::labeled_weighted_directed::LabeledWeightedDigraph::from_adjacency_list(my_v,my_e,my_adj,my_labels);
            let mut my_lwwd = crate::wt::labeled_weighted_directed::LabeledWeightedWTDigraph::from_labeled_weighted_digraph(my_lwd); 


        // this is the new variable
        let my_vertex = my_lwwd.ldg.dg.wt_adj_len;
     
        // run
        let my_return = my_lwwd.add_vertex(my_vertex.to_string());

        // does the expected vertex exist in the graph?
        assert_eq!(my_vertex,my_return);
        assert_eq!(&my_vertex.to_string(),my_lwwd.label_updated(my_return).unwrap());

        assert_eq!(my_lwwd.vertex_exists(my_vertex.to_string()),false); 
        assert_eq!(my_lwwd.vertex_exists_updated(my_vertex.to_string()),true);
        
        // does the return return equal the expected usize value?
        assert_eq!(my_lwwd.ldg.dg.wt_adj_len, my_return);

        // does the additional metadata equal the expected one?
        assert_eq!(my_lwwd.e_count(), my_lwwd.e_count_updated());
        assert_eq!(my_lwwd.v_count()+1, my_lwwd.v_count_updated());
        assert_ne!(my_lwwd.ldg.dg.wt_adj_len, my_lwwd.ldg.dg.wt_adj_len_updated);
    
        // - is the addition ready for commit?
        assert_eq!(my_lwwd.ldg.dg.has_uncommitted_edits, true);
        // - make adj_uncommited pub(crate) and import Edit, then we could
        // assert_eq!(my_wd.dg.adj_uncommitted.get(my_label),[Add(my_label)]);

    }    

}
    