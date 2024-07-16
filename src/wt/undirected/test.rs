use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::traits::*;

#[test]
fn add_vertex() {

    // requires random WTUGraphs and a random mutable variable of type usize for testing.
    // calls add_vertex() and verifies the results.

    const USIZE_MAX : usize = 10; 
    let mut rng = thread_rng();

    // create 50 random WTUgraphs
    for i in 0..50 {

        println!("creating the {}'th WTUGraph",i+1);
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
        println!("v and e are: {}, {}",my_v,my_e);
        let mut my_adj: Vec<Vec<usize>> = vec![Vec::new();my_v];
        let mut e : usize = 0;
        match my_v {
            0 =>    (),
            _ =>    {
                for j in 0..my_e {
                    println!("choosing the {}'s edge",j+1);
                    let mut start = 0; let mut end = 0;
                    let mut attempts : usize = 0;
                    loop {
                        println!("entered third loop!");
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
        let my_ug = crate::graph::undirected::UGraph::from_adjacency_list(my_v, my_e, my_adj);
        let mut my_wu = crate::wt::undirected::WTUGraph::from_ugraph(my_ug);
        // this is the new variable
        let my_vertex = my_wu.wtd.wt_adj_len;

        // runs:
        let my_return = my_wu.add_vertex(my_vertex);

        // does the expected vertex exist in the graph?
        assert_eq!(my_wu.vertex_exists(my_return),false); // but returns false 
        assert_eq!(my_wu.vertex_exists_updated(my_return),true);

        // does the additional metadata equal the expected one?
        assert_eq!(my_wu.e_count(), my_wu.e_count_updated());
        assert_eq!(my_wu.v_count()+1, my_wu.v_count_updated());
        assert_ne!(my_wu.wtd.wt_adj_len, my_wu.wtd.wt_adj_len_updated);
    
        // is the addition ready for commit?
        assert_eq!(my_wu.wtd.has_uncommitted_edits, true);
        // make adj_uncommited pub(crate) and import Edit, then we could
        // assert_eq!(my_wu.wtd.adj_uncommitted.get(my_vertex),[Add(my_vertex)]);
    }
    
}

