use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::traits::*;


#[test]
fn add_vertex() {

    // requires 50 random LabeledWTDigraphs and a random mutable variable of type L for testing.
    // calls add_vertex() and verifies the results.

    const USIZE_MAX : usize = 100; 
    let mut rng = thread_rng();

    // create 50 random LabeledWTDigraphs
    for i in 0..50 {

        println!("creating the {}'th LabeledWTDigraph",i+1);
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
                        loop {
                            end = *vec_range.choose(&mut rng).unwrap();
                            if end != start { break; }
                        }
                        if !my_adj[start].contains(&end) {
                                break;
                        }
                        println!("repeated third loop!");
                    }
                    my_adj[start].push(end); 
                }
            }
        }

        let my_labels = (0..my_v).map(|x| x.to_string() ).collect::<Vec<String>>();       
        let my_ldg = crate::graph::labeled_directed::LabeledDigraph::from_adjacency_list(my_v,my_e,my_adj,my_labels);
        let mut my_lwd = crate::wt::labeled_directed::LabeledWTDigraph::from_labeled_digraph(my_ldg); 

        // this is the new variable
        let my_label = String::from("new_label");
     
        // run
        let my_return = my_lwd.add_vertex(my_label.clone());

        // does the expected vertex exist in the graph?
        assert_eq!(my_return, my_lwd.get_index_updated(&my_label).unwrap());
        assert_eq!(my_lwd.get_label_updated(my_return), Some(&my_label));
        
        // does the return return equal the expected usize value?
        assert_eq!(my_lwd.dg.wt_adj_len, my_return);

        // does the additional metadata equal the expected one?
        assert_eq!(my_lwd.e_count(), my_lwd.e_count_updated());
        assert_eq!(my_lwd.v_count()+1, my_lwd.v_count_updated());
        assert_ne!(my_lwd.dg.wt_adj_len, my_lwd.dg.wt_adj_len_updated);
    
        // - is the addition ready for commit?
        assert_eq!(my_lwd.dg.has_uncommitted_edits, true);
        // - make adj_uncommited pub(crate) and import Edit, then we could
        // assert_eq!(my_wd.dg.adj_uncommitted.get(my_label),[Add(my_label)]);

    }    

}

#[test]
fn v_e_count() {

    // requires 50 random LabeledWTDigraphs and a random mutable variable of type L for testing.
    // calls e_count() and v_count() an verifies the results.

    const USIZE_MAX : usize = 100; 
    let mut rng = thread_rng();

    // create 50 random LabeledWTDigraphs
    for i in 0..50 {

        println!("creating the {}'th LabeledWTDigraph",i+1);
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
                        loop {
                            end = *vec_range.choose(&mut rng).unwrap();
                            if end != start { break; }
                        }
                        if !my_adj[start].contains(&end) {
                                break;
                        }
                        println!("repeated third loop!");
                    }
                    my_adj[start].push(end); 
                }
            }
        }

        let my_labels = (0..my_v).map(|x| x.to_string() ).collect::<Vec<String>>();       
        let my_ldg = crate::graph::labeled_directed::LabeledDigraph::from_adjacency_list(my_v,my_e,my_adj,my_labels);
        let mut my_lwd = crate::wt::labeled_directed::LabeledWTDigraph::from_labeled_digraph(my_ldg); 

        // this is the new variable
        let my_label = String::from("new_label");

        // queries the WT-graph using e_count()
        let my_e_count = my_lwd.e_count();

        // checks:
        assert_eq!(my_e_count,my_e);
    }
}

#[test]
fn delete_edge() {

    // requires random LabeledWTDigraphs and two random vertices within the graph for testing.
    // calls delete_edge() and verifies the results.

    // create 50 random LabeledWTDigraphs
    for i in 0..50 {

        println!("creating the {}'th LabeledWTDigraph",i+1);
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
                        loop {
                            end = *vec_range.choose(&mut rng).unwrap();
                            if end != start { break; }
                        }
                        if !my_adj[start].contains(&end) {
                                break;
                        }
                        println!("repeated third loop!");
                    }
                    my_adj[start].push(end); 
                }
            }
        }

        let my_labels = (0..my_v).map(|x| x.to_string() ).collect::<Vec<String>>();       
        let my_ldg = crate::graph::labeled_directed::LabeledDigraph::from_adjacency_list(my_v,my_e,my_adj,my_labels);
        let mut my_lwd = crate::wt::labeled_directed::LabeledWTDigraph::from_labeled_digraph(my_ldg); 

    // Select a random, non-empty vertice within range (0..USIZE_MAX)
    loop {
        let start = (rand::random::<f64>() * USIZE_MAX as f64).floor() as usize;
        if my_lwd.dg.outgoing_edges(start) != vec![] {
            break;
        }
    }

    // select another random vertex, that has an edge with the first one
    let end = (rand::random::<f64>() * my_lwd.dg.outgoing_edges(start).len() as f64).floor() as usize;

    // run
    my_lwd.delete_edge(start,end);

    // check
    // does the edge not exist in the graph?


    // does the additional metadata equal the expected one?
    
    // is the deletion ready for commit?
        
    }
}

#[test]
fn edge_exists() {


    // requires random LabeledWTDigraphs and a random mutable variable of type L for testing.
    // calls edge_exists() and verifies the results.

    // create 50 random LabeledWTDigraphs
    for i in 0..50 {

        println!("creating the {}'th LabeledWTDigraph",i+1);
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
                        loop {
                            end = *vec_range.choose(&mut rng).unwrap();
                            if end != start { break; }
                        }
                        if !my_adj[start].contains(&end) {
                                break;
                        }
                        println!("repeated third loop!");
                    }
                    my_adj[start].push(end); 
                }
            }
        }

        let my_labels = (0..my_v).map(|x| x.to_string() ).collect::<Vec<String>>();       
        let my_ldg = crate::graph::labeled_directed::LabeledDigraph::from_adjacency_list(my_v,my_e,my_adj,my_labels);
        let mut my_lwd = crate::wt::labeled_directed::LabeledWTDigraph::from_labeled_digraph(my_ldg); 

        // Select a random, non-empty vertice within range (0..USIZE_MAX)
        let mut start = 0usize;
        let mut end = 0usize;
        loop {
            start = (rand::random::<f64>() * USIZE_MAX as f64).floor() as usize;
            if my_lwd.(start) != vec![] {
            break;
        }
    }

    // select another random vertex, that has an edge with the first one
    let end = (rand::random::<f64>() * my_lwd.dg.outgoing_edges(start).len() as f64).floor() as usize;

    // run
    let my_result = my_lwd.edge_exists(random_edge);

    // does the return boolean equal the expected boolean?
    match my_result {
        true    => assert_eq(),
        false   => assert_eq(),
    }  
    }
}


fn delete_vertex()

fn vertex_exists()