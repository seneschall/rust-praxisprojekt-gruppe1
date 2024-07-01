use std::fs;
use vers_vecs::BitVec;

pub mod dg_ug;
pub mod ldg_lug;
pub mod wtd_wtu;
pub mod L_wtd_L_stu;
pub mod helper;


// this function is for internal use only
// extract the number of vertices and the number of edges in the graph from the first two lines in the file
pub(crate) fn v_e_count(filepath: &str) -> (usize, usize) {
    let content = fs::read_to_string(filepath).expect("Unable to open file");
    let mut lines = content.lines();
    let v_count = lines.next().expect("would have preferred error handling!").trim().parse::<usize>()
        .expect("couldn't find v_count (number of vertices) in first line");
            // hier error handling gewünscht
    let e_count = lines.next().expect("would have preferred error handling!").trim().parse::<usize>()
    .expect("couldn't find v_count (number of vertices) in second line");
            // hier error handling gewünscht
    (v_count, e_count)
}

// this function is for internal use only
// extract a single vector representing all the edges and a bitvector representing the vertices
pub(crate) fn sequence_and_bitmap(map: &Vec<Vec<usize>>) -> (Vec<usize>, BitVec) {
    let mut sequence = Vec::new();
    let mut bitmap = BitVec::new();

    for items in map {
        bitmap.append(true);
        for item in items {
            bitmap.append(false);
            sequence.push(item.clone());
        }
    }
    (sequence, bitmap)
}