use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::traits::*;

#[test]
fn add_vertex() {

    // requires random LabeledWTUGraphs and a random mutable variable of type L for testing.
    // calls add_vertex() and verifies the results.

    // the possible ranges for values is 0..usize::MAX , which I wanted to use for testing, but since computing such large graphs is too slow,
    // I will use a "fake" max range
    const USIZE_MAX : usize = 100; 

    // initialize a random number generator
    let mut rng = thread_rng();

    // create 50 random LabeledWTUGraphs
    for i in 0..50 {

        println!("creating the {}'th LabeledWTUGraph",i+1);
        let my_v = (rand::random::<f64>() * USIZE_MAX as f64).floor() as usize;
        let vec_range = (0..my_v).collect::<Vec<usize>>();
        let mut my_e : usize = 0;
        match my_v {
            0 =>    (),
            _ =>    {
                let max_edges = my_v * (my_v-1) / 2;
                my_e = (rand::random::<f64>() * max_edges as f64).floor() as usize;
            }
        }
        println!("v and e are: {},{}",my_v,my_e);
        let mut my_adj: Vec<Vec<usize>> = vec![Vec::new();my_v];
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
                        if !my_adj[start].contains(&end) && !my_adj[end].contains(&start) {
                            break;
                        }                        
                        println!("repeated third loop!");
                    }
                    my_adj[start].push(end); 
                }
            }
        }
        let my_labels = (0..my_v).map(|x| x.to_string() ).collect::<Vec<String>>();   
        println!("labels are: {:?}", my_labels);  
        let my_lu = crate::graph::labeled_undirected::LabeledUGraph::from_adjacency_list(my_v,my_e,my_adj,my_labels);
        let mut my_lwu = crate::wt::labeled_undirected::LabeledWTUGraph::from_labeled_ugraph(my_lu);

        // this is the new variable
        let my_label = String::from("new_label");

        // run
        let my_return = my_lwu.add_vertex(my_label.clone());

        // does the expected vertex exist in the graph?
        assert_eq!(my_lwu.get_label_updated(my_return), Some(&my_label));
        assert_eq!(my_return, my_lwu.get_index_updated(&my_label).unwrap());

        // does the return equal the expected usize value?
        assert_eq!(my_lwu.ldg.dg.wt_adj_len, my_return);

        // does the additional metadata equal the expected one?
        assert_eq!(my_lwu.e_count(), my_lwu.e_count_updated());
        assert_eq!(my_lwu.v_count()+1, my_lwu.v_count_updated());
        assert_ne!(my_lwu.ldg.dg.wt_adj_len, my_lwu.ldg.dg.wt_adj_len_updated);
    
        // is the addition ready for commit?
        assert_eq!(my_lwu.ldg.dg.has_uncommitted_edits, true);
        // make adj_uncommited pub(crate) and import Edit, then we could
        // assert_eq!(my_lwu.ldg.dg.adj_uncommitted.get(my_label),[Add(my_label)]);
    }

}