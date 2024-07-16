use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::traits::{Graph,WT, Labeled};

#[test]
fn add_vertex() {

    // requires random LabeledWeightedWTDigraphs and a random mutable variable of type L for testing.
    // runs add_vertex() and verifies the results.

    const USIZE_MAX : usize = 100; 
    
    for i in 0..50 {
            println!("creating the {}'th LabeledWeightedWTDigraph",i+1);
            let mut rng = thread_rng();
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
                            println!("start is: {:?}", start); 
                            loop {
                                end = *vec_range.choose(&mut rng).unwrap();
                                if end != start { break; }
                            }
                            if !my_adj[start].contains(&end) {
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
            let my_lwd = crate::graph::labeled_weighted_directed::LabeledDigraph::from_adjacency_list(my_v,my_e,my_adj,my_labels);
            let mut my_lwwd = crate::wt::labeled_weighted_directed::LabeledWeightedWTDigraph::from_weighted_digraph(my_lwd); 


        // this is the new variable
        let my_vertex = my_wwd.dg.wt_adj_len;
     
        // run
        let my_return = my_wwd.add_vertex(my_vertex.clone());

        // does the expected vertex exist in the graph?
        assert_eq!(my_wwd.vertex_exists(my_return),false); // but returns false 
        assert_eq!(my_wwd.vertex_exists_updated(my_return),true);
        
        // does the return return equal the expected usize value?
        assert_eq!(my_wwd.dg.wt_adj_len, my_return);

        // does the additional metadata equal the expected one?
        assert_eq!(my_wwd.e_count(), my_wwd.e_count_updated());
        assert_eq!(my_wwd.v_count()+1, my_wwd.v_count_updated());
        assert_ne!(my_wwd.dg.wt_adj_len, my_wwd.dg.wt_adj_len_updated);
    
        // - is the addition ready for commit?
        assert_eq!(my_wwd.dg.has_uncommitted_edits, true);
        // - make adj_uncommited pub(crate) and import Edit, then we could
        // assert_eq!(my_wd.dg.adj_uncommitted.get(my_label),[Add(my_label)]);

    }    

}
    