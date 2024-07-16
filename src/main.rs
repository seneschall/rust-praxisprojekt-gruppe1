use rand::{distributions::Uniform, prelude::*};
use std::time::{Duration, Instant};
use wt_graphs::prelude::*;

// Create graphs to run benchmarks on

fn create_random_digraph(v_count: usize) -> Digraph {
    let mut rand_rng = thread_rng(); // allows us to create random value
    let range = Uniform::new(0, v_count - 1); // allows us to pull samples of random numbers from this

    let mut adj: Vec<Vec<usize>> = Vec::new();
    let mut e_count: usize = 0;

    for i in 0..v_count {
        let outgoing_e_count: usize = rand_rng.sample(&range); // in the worst case an edge has outgoing edges to all others
        let edges = random_edges(outgoing_e_count, v_count, i);
        adj.push(edges);
        e_count += outgoing_e_count;
    }

    return Digraph::from_adjacency_list(v_count, e_count, adj);
}

fn create_random_wtdigraph(v_count: usize) -> WTDigraph {
    let dg: Digraph = create_random_digraph(v_count);
    return WTDigraph::from_digraph(dg);
}

// Some quick benchmarking functions

fn bench_wtdigraph_outgoing_edges(dg: &WTDigraph, rand_vertices: &Vec<usize>) -> Duration {
    let now = Instant::now();

    for v in rand_vertices {
        let outgoing: Vec<usize> = dg.outgoing_edges(v.to_owned());
        // println!("Outgoing edges of {}: {:?}", v, outgoing);
    }

    let elapsed = now.elapsed();
    return elapsed;
}

fn bench_digraph_outgoing_edges(dg: &Digraph, rand_vertices: &Vec<usize>) -> Duration {
    let now = Instant::now();

    for v in rand_vertices {
        let outgoing: Vec<usize> = dg.outgoing_edges(v.to_owned());
        // println!("Outgoing edges of {}: {:?}", v, outgoing);
    }

    let elapsed = now.elapsed();
    return elapsed;
}

fn bench_wtdigraph_incoming_edges(dg: &WTDigraph, rand_vertices: &Vec<usize>) -> Duration {
    let now = Instant::now();

    for v in rand_vertices {
        let incoming: Vec<usize> = dg.incoming_edges(v.to_owned());
        // println!("Incoming edges of {}: {:?}", v, incoming);
    }

    let elapsed = now.elapsed();
    return elapsed;
}

fn bench_digraph_incoming_edges(dg: &Digraph, rand_vertices: &Vec<usize>) -> Duration {
    let now = Instant::now();

    for v in rand_vertices {
        let incoming: Vec<usize> = dg.incoming_edges(v.to_owned());
        // println!("Incoming edges of {}: {:?}", v, incoming);
    }

    let elapsed = now.elapsed();
    return elapsed;
}

fn random_edges(len: usize, v_count: usize, exclude_vertex: usize) -> Vec<usize> {
    // generates a list of edges with length `len` for a vertex (that will be excluded from this list with `exclude_vertex`)
    // for a graph with v_count `v_count`
    let mut rng = thread_rng();
    let mut range = Uniform::new(0, v_count);

    // let mut edges: Vec<usize> = Vec::new();

    // for v in 0..len {
    //     loop {
    //         let num: usize = rng.sample(&range);
    //         if (!edges.contains(&num)) && (num != exclude_vertex) {
    //             edges.push(num);
    //             break;
    //         }
    //     }
    // }

    let edges: Vec<usize> = (0..len).map(|_| rng.sample(&range)).collect();
    // code taken from: https://stackoverflow.com/questions/48218459/how-do-i-generate-a-vector-of-random-numbers-in-a-range

    return edges;
}

// Statistics

fn rand_num(lower_bound: usize, upper_bound: usize) -> usize {
    todo!()
}

fn mean(data: &[usize]) -> f32 {
    // based on rust cookbook:
    // https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html

    let count = data.len();
    let sum = data.iter().sum::<usize>() as f32;
    let mean = match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    };
    return mean.unwrap();
}

const V_COUNT: usize = 10_000;

fn main() {
    // creation of graphs

    let now: Instant = Instant::now();
    let dg: Digraph = create_random_digraph(V_COUNT);
    let time_to_create_dg = now.elapsed();

    let now: Instant = Instant::now();
    let wtdg = WTDigraph::from_digraph(dg.clone());
    let time_to_create_wtdg = now.elapsed();

    println!(
        "Time to create digraph: {:.3?}\nTime to create wt digraph: {:.3?}",
        time_to_create_dg, time_to_create_wtdg
    );

    let num_of_rand_verts: usize = V_COUNT / 2;
    let random_vertices = random_edges(num_of_rand_verts, num_of_rand_verts, V_COUNT);

    // outgoing edges

    let now: Instant = Instant::now();
    bench_digraph_outgoing_edges(&dg, &random_vertices);
    let dg_out = now.elapsed();

    let now: Instant = Instant::now();
    bench_wtdigraph_outgoing_edges(&wtdg, &random_vertices);
    let wtdg_out = now.elapsed();

    println!("Time to get outgoing edges in digraph: {:.3?}\nTime to get outgoing edges in wt digraph: {:.3?}", dg_out, wtdg_out);

    // incoming edges

    let now: Instant = Instant::now();
    bench_digraph_incoming_edges(&dg, &random_vertices);
    let dg_out = now.elapsed();

    let now: Instant = Instant::now();
    bench_wtdigraph_incoming_edges(&wtdg, &random_vertices);
    let wtdg_out = now.elapsed();

    println!("Time to get incoming edges in digraph: {:.3?}\nTime to get incoming edges in wt digraph: {:.3?}", dg_out, wtdg_out);
}
